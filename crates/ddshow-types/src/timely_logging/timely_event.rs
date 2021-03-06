use crate::timely_logging::{
    ApplicationEvent, ChannelsEvent, CommChannelsEvent, GuardedMessageEvent, GuardedProgressEvent,
    InputEvent, MessagesEvent, OperatesEvent, ParkEvent, PushProgressEvent, ScheduleEvent,
    ShutdownEvent,
};
#[cfg(feature = "enable_abomonation")]
use abomonation_derive::Abomonation;
#[cfg(feature = "rkyv")]
use bytecheck::CheckBytes;
#[cfg(feature = "rkyv")]
use rkyv_dep as rkyv;
#[cfg(feature = "rkyv")]
use rkyv_dep::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
#[cfg(feature = "serde")]
use serde_dep::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use std::fmt::Debug;
use timely::logging::TimelyEvent as RawTimelyEvent;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvSerialize, RkyvDeserialize))]
#[cfg_attr(feature = "rkyv", archive(strict, derive(CheckBytes)))]
#[cfg_attr(feature = "serde", derive(SerdeSerialize, SerdeDeserialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_dep"))]
#[cfg_attr(feature = "enable_abomonation", derive(Abomonation))]
pub enum TimelyEvent {
    Operates(OperatesEvent),
    Channels(ChannelsEvent),
    PushProgress(PushProgressEvent),
    Messages(MessagesEvent),
    Schedule(ScheduleEvent),
    Shutdown(ShutdownEvent),
    Application(ApplicationEvent),
    GuardedMessage(GuardedMessageEvent),
    GuardedProgress(GuardedProgressEvent),
    CommChannels(CommChannelsEvent),
    Input(InputEvent),
    Park(ParkEvent),
    Text(String),
}

impl TimelyEvent {
    /// Returns `true` if the timely_event is [`TimelyEvent::Operates`].
    pub const fn is_operates(&self) -> bool {
        matches!(self, Self::Operates(..))
    }

    /// Returns `true` if the timely_event is [`TimelyEvent::Channels`].
    pub const fn is_channels(&self) -> bool {
        matches!(self, Self::Channels(..))
    }

    /// Returns `true` if the timely_event is [`TimelyEvent::PushProgress`].
    pub const fn is_push_progress(&self) -> bool {
        matches!(self, Self::PushProgress(..))
    }

    /// Returns `true` if the timely_event is [`TimelyEvent::Messages`].
    pub const fn is_messages(&self) -> bool {
        matches!(self, Self::Messages(..))
    }

    /// Returns `true` if the timely_event is [`TimelyEvent::Schedule`].
    pub const fn is_schedule(&self) -> bool {
        matches!(self, Self::Schedule(..))
    }

    /// Returns `true` if the timely_event is [`TimelyEvent::Shutdown`].
    pub const fn is_shutdown(&self) -> bool {
        matches!(self, Self::Shutdown(..))
    }

    /// Returns `true` if the timely_event is [`TimelyEvent::Application`].
    pub const fn is_application(&self) -> bool {
        matches!(self, Self::Application(..))
    }

    /// Returns `true` if the timely_event is [`TimelyEvent::GuardedMessage`].
    pub const fn is_guarded_message(&self) -> bool {
        matches!(self, Self::GuardedMessage(..))
    }

    /// Returns `true` if the timely_event is [`TimelyEvent::GuardedProgress`].
    pub const fn is_guarded_progress(&self) -> bool {
        matches!(self, Self::GuardedProgress(..))
    }

    /// Returns `true` if the timely_event is [`TimelyEvent::CommChannels`].
    pub const fn is_comm_channels(&self) -> bool {
        matches!(self, Self::CommChannels(..))
    }

    /// Returns `true` if the timely_event is [`TimelyEvent::Input`].
    pub const fn is_input(&self) -> bool {
        matches!(self, Self::Input(..))
    }

    /// Returns `true` if the timely_event is [`TimelyEvent::Park`].
    pub const fn is_park(&self) -> bool {
        matches!(self, Self::Park(..))
    }

    /// Returns `true` if the timely_event is [`TimelyEvent::Text`].
    pub const fn is_text(&self) -> bool {
        matches!(self, Self::Text(..))
    }

    pub const fn as_operates(&self) -> Option<&OperatesEvent> {
        if let Self::Operates(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_operates(self) -> Result<OperatesEvent, Self> {
        if let Self::Operates(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub const fn as_channels(&self) -> Option<&ChannelsEvent> {
        if let Self::Channels(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_channels(self) -> Result<ChannelsEvent, Self> {
        if let Self::Channels(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub const fn as_push_progress(&self) -> Option<&PushProgressEvent> {
        if let Self::PushProgress(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_push_progress(self) -> Result<PushProgressEvent, Self> {
        if let Self::PushProgress(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub const fn as_messages(&self) -> Option<&MessagesEvent> {
        if let Self::Messages(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_messages(self) -> Result<MessagesEvent, Self> {
        if let Self::Messages(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub const fn as_schedule(&self) -> Option<&ScheduleEvent> {
        if let Self::Schedule(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_schedule(self) -> Result<ScheduleEvent, Self> {
        if let Self::Schedule(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub const fn as_shutdown(&self) -> Option<&ShutdownEvent> {
        if let Self::Shutdown(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_shutdown(self) -> Result<ShutdownEvent, Self> {
        if let Self::Shutdown(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub const fn as_application(&self) -> Option<&ApplicationEvent> {
        if let Self::Application(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_application(self) -> Result<ApplicationEvent, Self> {
        if let Self::Application(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub const fn as_guarded_message(&self) -> Option<&GuardedMessageEvent> {
        if let Self::GuardedMessage(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_guarded_message(self) -> Result<GuardedMessageEvent, Self> {
        if let Self::GuardedMessage(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub const fn as_guarded_progress(&self) -> Option<&GuardedProgressEvent> {
        if let Self::GuardedProgress(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_guarded_progress(self) -> Result<GuardedProgressEvent, Self> {
        if let Self::GuardedProgress(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub const fn as_comm_channels(&self) -> Option<&CommChannelsEvent> {
        if let Self::CommChannels(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_comm_channels(self) -> Result<CommChannelsEvent, Self> {
        if let Self::CommChannels(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub const fn as_input(&self) -> Option<&InputEvent> {
        if let Self::Input(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_input(self) -> Result<InputEvent, Self> {
        if let Self::Input(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub const fn as_park(&self) -> Option<&ParkEvent> {
        if let Self::Park(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_park(self) -> Result<ParkEvent, Self> {
        if let Self::Park(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    pub const fn as_text(&self) -> Option<&String> {
        if let Self::Text(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_text(self) -> Result<String, Self> {
        if let Self::Text(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

impl From<RawTimelyEvent> for TimelyEvent {
    fn from(event: RawTimelyEvent) -> Self {
        match event {
            RawTimelyEvent::Operates(operates) => Self::Operates(operates.into()),
            RawTimelyEvent::Channels(channels) => Self::Channels(channels.into()),
            RawTimelyEvent::PushProgress(progress) => Self::PushProgress(progress.into()),
            RawTimelyEvent::Messages(inner) => Self::Messages(inner.into()),
            RawTimelyEvent::Schedule(inner) => Self::Schedule(inner.into()),
            RawTimelyEvent::Shutdown(inner) => Self::Shutdown(inner.into()),
            RawTimelyEvent::Application(inner) => Self::Application(inner.into()),
            RawTimelyEvent::GuardedMessage(inner) => Self::GuardedMessage(inner.into()),
            RawTimelyEvent::GuardedProgress(inner) => Self::GuardedProgress(inner.into()),
            RawTimelyEvent::CommChannels(inner) => Self::CommChannels(inner.into()),
            RawTimelyEvent::Input(inner) => Self::Input(inner.into()),
            RawTimelyEvent::Park(inner) => Self::Park(inner.into()),
            RawTimelyEvent::Text(text) => Self::Text(text),
        }
    }
}

impl From<ParkEvent> for TimelyEvent {
    fn from(v: ParkEvent) -> Self {
        Self::Park(v)
    }
}

impl From<InputEvent> for TimelyEvent {
    fn from(v: InputEvent) -> Self {
        Self::Input(v)
    }
}

impl From<CommChannelsEvent> for TimelyEvent {
    fn from(v: CommChannelsEvent) -> Self {
        Self::CommChannels(v)
    }
}

impl From<GuardedProgressEvent> for TimelyEvent {
    fn from(v: GuardedProgressEvent) -> Self {
        Self::GuardedProgress(v)
    }
}

impl From<GuardedMessageEvent> for TimelyEvent {
    fn from(v: GuardedMessageEvent) -> Self {
        Self::GuardedMessage(v)
    }
}

impl From<ApplicationEvent> for TimelyEvent {
    fn from(v: ApplicationEvent) -> Self {
        Self::Application(v)
    }
}

impl From<ShutdownEvent> for TimelyEvent {
    fn from(v: ShutdownEvent) -> Self {
        Self::Shutdown(v)
    }
}

impl From<ScheduleEvent> for TimelyEvent {
    fn from(v: ScheduleEvent) -> Self {
        Self::Schedule(v)
    }
}

impl From<MessagesEvent> for TimelyEvent {
    fn from(v: MessagesEvent) -> Self {
        Self::Messages(v)
    }
}

impl From<PushProgressEvent> for TimelyEvent {
    fn from(v: PushProgressEvent) -> Self {
        Self::PushProgress(v)
    }
}

impl From<ChannelsEvent> for TimelyEvent {
    fn from(v: ChannelsEvent) -> Self {
        Self::Channels(v)
    }
}

impl From<OperatesEvent> for TimelyEvent {
    fn from(v: OperatesEvent) -> Self {
        Self::Operates(v)
    }
}
