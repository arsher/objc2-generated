// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::unportable_markdown)]
#![allow(rustdoc::invalid_html_tags)]

#[link(name = "EventKit", kind = "framework")]
extern "C" {}

#[cfg(feature = "EKAlarm")]
#[path = "EKAlarm.rs"]
mod __EKAlarm;
#[cfg(feature = "EKCalendar")]
#[path = "EKCalendar.rs"]
mod __EKCalendar;
#[cfg(feature = "EKCalendarItem")]
#[path = "EKCalendarItem.rs"]
mod __EKCalendarItem;
#[cfg(feature = "EKError")]
#[path = "EKError.rs"]
mod __EKError;
#[cfg(feature = "EKEvent")]
#[path = "EKEvent.rs"]
mod __EKEvent;
#[cfg(feature = "EKEventStore")]
#[path = "EKEventStore.rs"]
mod __EKEventStore;
#[cfg(feature = "EKObject")]
#[path = "EKObject.rs"]
mod __EKObject;
#[cfg(feature = "EKParticipant")]
#[path = "EKParticipant.rs"]
mod __EKParticipant;
#[cfg(feature = "EKRecurrenceDayOfWeek")]
#[path = "EKRecurrenceDayOfWeek.rs"]
mod __EKRecurrenceDayOfWeek;
#[cfg(feature = "EKRecurrenceEnd")]
#[path = "EKRecurrenceEnd.rs"]
mod __EKRecurrenceEnd;
#[cfg(feature = "EKRecurrenceRule")]
#[path = "EKRecurrenceRule.rs"]
mod __EKRecurrenceRule;
#[cfg(feature = "EKReminder")]
#[path = "EKReminder.rs"]
mod __EKReminder;
#[cfg(feature = "EKSource")]
#[path = "EKSource.rs"]
mod __EKSource;
#[cfg(feature = "EKStructuredLocation")]
#[path = "EKStructuredLocation.rs"]
mod __EKStructuredLocation;
#[cfg(feature = "EKTypes")]
#[path = "EKTypes.rs"]
mod __EKTypes;
#[cfg(feature = "EKVirtualConferenceDescriptor")]
#[path = "EKVirtualConferenceDescriptor.rs"]
mod __EKVirtualConferenceDescriptor;
#[cfg(feature = "EKVirtualConferenceProvider")]
#[path = "EKVirtualConferenceProvider.rs"]
mod __EKVirtualConferenceProvider;
#[cfg(feature = "EventKitDefines")]
#[path = "EventKitDefines.rs"]
mod __EventKitDefines;

#[cfg(all(feature = "EKAlarm", feature = "EKObject"))]
pub use self::__EKAlarm::EKAlarm;
#[cfg(all(feature = "EKCalendar", feature = "EKObject"))]
pub use self::__EKCalendar::EKCalendar;
#[cfg(all(feature = "EKCalendarItem", feature = "EKObject"))]
pub use self::__EKCalendarItem::EKCalendarItem;
#[cfg(feature = "EKError")]
pub use self::__EKError::EKErrorCode;
#[cfg(feature = "EKError")]
pub use self::__EKError::EKErrorDomain;
#[cfg(all(feature = "EKCalendarItem", feature = "EKEvent", feature = "EKObject"))]
pub use self::__EKEvent::EKEvent;
#[cfg(feature = "EKEvent")]
pub use self::__EKEvent::EKEventAvailability;
#[cfg(feature = "EKEvent")]
pub use self::__EKEvent::EKEventStatus;
#[cfg(all(
    feature = "EKCalendarItem",
    feature = "EKEvent",
    feature = "EKEventStore",
    feature = "EKObject",
    feature = "block2"
))]
pub use self::__EKEventStore::EKEventSearchCallback;
#[cfg(feature = "EKEventStore")]
pub use self::__EKEventStore::EKEventStore;
#[cfg(feature = "EKEventStore")]
pub use self::__EKEventStore::EKEventStoreChangedNotification;
#[cfg(all(feature = "EKEventStore", feature = "block2"))]
pub use self::__EKEventStore::EKEventStoreRequestAccessCompletionHandler;
#[cfg(feature = "EKEventStore")]
pub use self::__EKEventStore::EKSpan;
#[cfg(feature = "EKObject")]
pub use self::__EKObject::EKObject;
#[cfg(all(feature = "EKObject", feature = "EKParticipant"))]
pub use self::__EKParticipant::EKParticipant;
#[cfg(feature = "EKRecurrenceDayOfWeek")]
pub use self::__EKRecurrenceDayOfWeek::EKRecurrenceDayOfWeek;
#[cfg(feature = "EKRecurrenceEnd")]
pub use self::__EKRecurrenceEnd::EKRecurrenceEnd;
#[cfg(all(feature = "EKObject", feature = "EKRecurrenceRule"))]
pub use self::__EKRecurrenceRule::EKRecurrenceRule;
#[cfg(all(
    feature = "EKCalendarItem",
    feature = "EKObject",
    feature = "EKReminder"
))]
pub use self::__EKReminder::EKReminder;
#[cfg(all(feature = "EKObject", feature = "EKSource"))]
pub use self::__EKSource::EKSource;
#[cfg(all(feature = "EKObject", feature = "EKStructuredLocation"))]
pub use self::__EKStructuredLocation::EKStructuredLocation;
#[cfg(feature = "EKTypes")]
pub use self::__EKTypes::EKAlarmProximity;
#[cfg(feature = "EKTypes")]
pub use self::__EKTypes::EKAlarmType;
#[cfg(feature = "EKTypes")]
pub use self::__EKTypes::EKAuthorizationStatus;
#[cfg(feature = "EKTypes")]
pub use self::__EKTypes::EKCalendarEventAvailabilityMask;
#[cfg(feature = "EKTypes")]
pub use self::__EKTypes::EKCalendarType;
#[cfg(feature = "EKTypes")]
pub use self::__EKTypes::EKEntityMask;
#[cfg(feature = "EKTypes")]
pub use self::__EKTypes::EKEntityType;
#[cfg(feature = "EKTypes")]
pub use self::__EKTypes::EKParticipantRole;
#[cfg(feature = "EKTypes")]
pub use self::__EKTypes::EKParticipantScheduleStatus;
#[cfg(feature = "EKTypes")]
pub use self::__EKTypes::EKParticipantStatus;
#[cfg(feature = "EKTypes")]
pub use self::__EKTypes::EKParticipantType;
#[cfg(feature = "EKTypes")]
pub use self::__EKTypes::EKRecurrenceFrequency;
#[cfg(feature = "EKTypes")]
pub use self::__EKTypes::EKReminderPriority;
#[cfg(feature = "EKTypes")]
pub use self::__EKTypes::EKSourceType;
#[cfg(feature = "EKTypes")]
pub use self::__EKTypes::EKWeekday;
#[cfg(feature = "EKVirtualConferenceDescriptor")]
pub use self::__EKVirtualConferenceDescriptor::EKVirtualConferenceDescriptor;
#[cfg(feature = "EKVirtualConferenceDescriptor")]
pub use self::__EKVirtualConferenceDescriptor::EKVirtualConferenceRoomTypeDescriptor;
#[cfg(feature = "EKVirtualConferenceDescriptor")]
pub use self::__EKVirtualConferenceDescriptor::EKVirtualConferenceRoomTypeIdentifier;
#[cfg(feature = "EKVirtualConferenceDescriptor")]
pub use self::__EKVirtualConferenceDescriptor::EKVirtualConferenceURLDescriptor;
#[cfg(feature = "EKVirtualConferenceProvider")]
pub use self::__EKVirtualConferenceProvider::EKVirtualConferenceProvider;
