use crate::dataflow::{
    operators::{Max, Min},
    utils::{Diff, DifferentialLogBundle},
    FilterMap,
};
use abomonation_derive::Abomonation;
use ddshow_types::{differential_logging::DifferentialEvent, OperatorId, WorkerId};
#[cfg(not(feature = "timely-next"))]
use differential_dataflow::difference::DiffPair;
use differential_dataflow::{
    operators::{CountTotal, Join, Reduce},
    AsCollection, Collection,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use timely::dataflow::{operators::Enter, Scope, Stream};

pub fn arrangement_stats<S>(
    scope: &mut S,
    differential_trace: &Stream<S, DifferentialLogBundle>,
) -> Collection<S, ((WorkerId, OperatorId), ArrangementStats), Diff>
where
    S: Scope<Timestamp = Duration>,
{
    scope.region_named("Collect arrangement statistics", |region| {
        let differential_trace = differential_trace.enter(region);

        let merge_diffs = differential_trace
            .filter_map(|(time, worker, event)| match event {
                DifferentialEvent::Batch(batch) => Some((
                    ((worker, batch.operator), (batch.length as isize, 1)),
                    time,
                    1,
                )),
                DifferentialEvent::Merge(merge) => merge.complete.map(|complete_size| {
                    (
                        ((worker, merge.operator), (complete_size as isize, 0)),
                        time,
                        1,
                    )
                }),

                DifferentialEvent::MergeShortfall(_)
                | DifferentialEvent::Drop(_)
                | DifferentialEvent::TraceShare(_) => None,
            })
            .as_collection();

        let spline_levels = differential_trace
            .filter_map(|(time, worker, event)| match event {
                DifferentialEvent::Merge(merge) => merge.complete.map(|complete_size| {
                    (
                        ((worker, merge.operator), (time, merge.scale, complete_size)),
                        time,
                        1isize,
                    )
                }),

                DifferentialEvent::Batch(_)
                | DifferentialEvent::MergeShortfall(_)
                | DifferentialEvent::Drop(_)
                | DifferentialEvent::TraceShare(_) => None,
            })
            .as_collection()
            .reduce(|_, levels, output| {
                output.push((
                    levels
                        .iter()
                        .flat_map(|&(&scale, diff)| (0..diff).map(move |_| scale))
                        .collect::<Vec<_>>(),
                    1isize,
                ));
            });

        #[cfg(feature = "timely-next")]
        let merge_stats = merge_diffs
            .explode(|(key, (size, batches))| {
                let (min, max) = (Min::new(size), Max::new(size));

                Some((key, (1, size, batches, min, max)))
            })
            .count_total()
            .map(|(key, (_count, _total, batches, min, max))| {
                let stats = ArrangementStats {
                    max_size: max.value as usize,
                    min_size: min.value as usize,
                    batches: batches as usize,
                };

                (key, stats)
            });

        #[cfg(not(feature = "timely-next"))]
        let merge_stats = merge_diffs
            .explode(|(key, (size, batches))| {
                let (min, max) = (Min::new(size), Max::new(size));

                Some((
                    key,
                    DiffPair::new(
                        1,
                        DiffPair::new(size, DiffPair::new(batches, DiffPair::new(min, max))),
                    ),
                ))
            })
            .count_total()
            .map(
                |(
                    key,
                    DiffPair {
                        element1: _count,
                        element2:
                            DiffPair {
                                element1: _total,
                                element2:
                                    DiffPair {
                                        element1: batches,
                                        element2:
                                            DiffPair {
                                                element1: min,
                                                element2: max,
                                            },
                                    },
                            },
                    },
                )| {
                    let stats = ArrangementStats {
                        max_size: max.value as usize,
                        min_size: min.value as usize,
                        batches: batches as usize,
                        spline_levels: Vec::new(),
                    };

                    (key, stats)
                },
            );

        merge_stats
            .join_map(&spline_levels, |&key, stats, spline_levels| {
                let stats = ArrangementStats {
                    spline_levels: spline_levels.clone(),
                    ..stats.clone()
                };

                (key, stats)
            })
            .leave_region()
    })
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Abomonation, Deserialize, Serialize,
)]
pub struct ArrangementStats {
    pub max_size: usize,
    pub min_size: usize,
    pub batches: usize,
    /// Merge time, merge scale and the completed size of the merge
    pub spline_levels: Vec<(Duration, usize, usize)>,
    // TODO: Max/min/average batch size
    // TODO: Arrangement growth trend (Linear, logarithmic, quadratic, etc.)?
}
