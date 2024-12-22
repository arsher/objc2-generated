//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// This enumerated type is used to indicate the currently granted authorization status for a specific
/// entity type.
///
/// may access the service.
///
/// The user cannot change this application’s status, possibly due to
/// active restrictions such as parental controls being in place.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekauthorizationstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EKAuthorizationStatus(pub NSInteger);
impl EKAuthorizationStatus {
    #[doc(alias = "EKAuthorizationStatusNotDetermined")]
    pub const NotDetermined: Self = Self(0);
    #[doc(alias = "EKAuthorizationStatusRestricted")]
    pub const Restricted: Self = Self(1);
    #[doc(alias = "EKAuthorizationStatusDenied")]
    pub const Denied: Self = Self(2);
    #[doc(alias = "EKAuthorizationStatusFullAccess")]
    pub const FullAccess: Self = Self(3);
    #[doc(alias = "EKAuthorizationStatusWriteOnly")]
    pub const WriteOnly: Self = Self(4);
    #[deprecated = "Check for full access or write only access"]
    #[doc(alias = "EKAuthorizationStatusAuthorized")]
    pub const Authorized: Self = Self(EKAuthorizationStatus::FullAccess.0);
}

unsafe impl Encode for EKAuthorizationStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for EKAuthorizationStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekweekday?language=objc)
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EKWeekday(pub NSInteger);
impl EKWeekday {
    #[doc(alias = "EKWeekdaySunday")]
    pub const Sunday: Self = Self(1);
    #[doc(alias = "EKWeekdayMonday")]
    pub const Monday: Self = Self(2);
    #[doc(alias = "EKWeekdayTuesday")]
    pub const Tuesday: Self = Self(3);
    #[doc(alias = "EKWeekdayWednesday")]
    pub const Wednesday: Self = Self(4);
    #[doc(alias = "EKWeekdayThursday")]
    pub const Thursday: Self = Self(5);
    #[doc(alias = "EKWeekdayFriday")]
    pub const Friday: Self = Self(6);
    #[doc(alias = "EKWeekdaySaturday")]
    pub const Saturday: Self = Self(7);
    #[deprecated = "Use EKWeekdaySunday instead"]
    pub const EKSunday: Self = Self(EKWeekday::Sunday.0);
    #[deprecated = "Use EKWeekdayMonday instead"]
    pub const EKMonday: Self = Self(EKWeekday::Monday.0);
    #[deprecated = "Use EKWeekdayTuesday instead"]
    pub const EKTuesday: Self = Self(EKWeekday::Tuesday.0);
    #[deprecated = "Use EKWeekdayWednesday instead"]
    pub const EKWednesday: Self = Self(EKWeekday::Wednesday.0);
    #[deprecated = "Use EKWeekdayThursday instead"]
    pub const EKThursday: Self = Self(EKWeekday::Thursday.0);
    #[deprecated = "Use EKWeekdayFriday instead"]
    pub const EKFriday: Self = Self(EKWeekday::Friday.0);
    #[deprecated = "Use EKWeekdaySaturday instead"]
    pub const EKSaturday: Self = Self(EKWeekday::Saturday.0);
}

unsafe impl Encode for EKWeekday {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for EKWeekday {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// The frequency of a recurrence
///
/// EKRecurrenceFrequency designates the unit of time used to describe the recurrence.
/// It has four possible values, which correspond to recurrence rules that are defined
/// in terms of days, weeks, months, and years.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekrecurrencefrequency?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EKRecurrenceFrequency(pub NSInteger);
impl EKRecurrenceFrequency {
    #[doc(alias = "EKRecurrenceFrequencyDaily")]
    pub const Daily: Self = Self(0);
    #[doc(alias = "EKRecurrenceFrequencyWeekly")]
    pub const Weekly: Self = Self(1);
    #[doc(alias = "EKRecurrenceFrequencyMonthly")]
    pub const Monthly: Self = Self(2);
    #[doc(alias = "EKRecurrenceFrequencyYearly")]
    pub const Yearly: Self = Self(3);
}

unsafe impl Encode for EKRecurrenceFrequency {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for EKRecurrenceFrequency {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Value representing the type of attendee.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekparticipanttype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EKParticipantType(pub NSInteger);
impl EKParticipantType {
    #[doc(alias = "EKParticipantTypeUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "EKParticipantTypePerson")]
    pub const Person: Self = Self(1);
    #[doc(alias = "EKParticipantTypeRoom")]
    pub const Room: Self = Self(2);
    #[doc(alias = "EKParticipantTypeResource")]
    pub const Resource: Self = Self(3);
    #[doc(alias = "EKParticipantTypeGroup")]
    pub const Group: Self = Self(4);
}

unsafe impl Encode for EKParticipantType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for EKParticipantType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Value representing the role of a meeting participant.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekparticipantrole?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EKParticipantRole(pub NSInteger);
impl EKParticipantRole {
    #[doc(alias = "EKParticipantRoleUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "EKParticipantRoleRequired")]
    pub const Required: Self = Self(1);
    #[doc(alias = "EKParticipantRoleOptional")]
    pub const Optional: Self = Self(2);
    #[doc(alias = "EKParticipantRoleChair")]
    pub const Chair: Self = Self(3);
    #[doc(alias = "EKParticipantRoleNonParticipant")]
    pub const NonParticipant: Self = Self(4);
}

unsafe impl Encode for EKParticipantRole {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for EKParticipantRole {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Value representing the status of a meeting invite.
///
///
/// invitation has been sent yet.
///
/// sent.
///
/// no way of determing if it was successfully
/// delivered.
///
/// successfully delivered.
///
/// source doesn't recognize the recipient.
///
/// insufficient privileges.
///
/// likely due to a temporary failure.
///
/// we're unsure how to deliver it. This is a
/// permanent failure.
///
/// scheduling with the participant isn't
/// allowed. This is a permanent failure.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekparticipantschedulestatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EKParticipantScheduleStatus(pub NSInteger);
impl EKParticipantScheduleStatus {
    #[doc(alias = "EKParticipantScheduleStatusNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "EKParticipantScheduleStatusPending")]
    pub const Pending: Self = Self(1);
    #[doc(alias = "EKParticipantScheduleStatusSent")]
    pub const Sent: Self = Self(2);
    #[doc(alias = "EKParticipantScheduleStatusDelivered")]
    pub const Delivered: Self = Self(3);
    #[doc(alias = "EKParticipantScheduleStatusRecipientNotRecognized")]
    pub const RecipientNotRecognized: Self = Self(4);
    #[doc(alias = "EKParticipantScheduleStatusNoPrivileges")]
    pub const NoPrivileges: Self = Self(5);
    #[doc(alias = "EKParticipantScheduleStatusDeliveryFailed")]
    pub const DeliveryFailed: Self = Self(6);
    #[doc(alias = "EKParticipantScheduleStatusCannotDeliver")]
    pub const CannotDeliver: Self = Self(7);
    #[doc(alias = "EKParticipantScheduleStatusRecipientNotAllowed")]
    pub const RecipientNotAllowed: Self = Self(8);
}

unsafe impl Encode for EKParticipantScheduleStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for EKParticipantScheduleStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Value representing the status of a meeting participant.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekparticipantstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EKParticipantStatus(pub NSInteger);
impl EKParticipantStatus {
    #[doc(alias = "EKParticipantStatusUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "EKParticipantStatusPending")]
    pub const Pending: Self = Self(1);
    #[doc(alias = "EKParticipantStatusAccepted")]
    pub const Accepted: Self = Self(2);
    #[doc(alias = "EKParticipantStatusDeclined")]
    pub const Declined: Self = Self(3);
    #[doc(alias = "EKParticipantStatusTentative")]
    pub const Tentative: Self = Self(4);
    #[doc(alias = "EKParticipantStatusDelegated")]
    pub const Delegated: Self = Self(5);
    #[doc(alias = "EKParticipantStatusCompleted")]
    pub const Completed: Self = Self(6);
    #[doc(alias = "EKParticipantStatusInProcess")]
    pub const InProcess: Self = Self(7);
}

unsafe impl Encode for EKParticipantStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for EKParticipantStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// An enum representing the type of a calendar.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekcalendartype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EKCalendarType(pub NSInteger);
impl EKCalendarType {
    #[doc(alias = "EKCalendarTypeLocal")]
    pub const Local: Self = Self(0);
    #[doc(alias = "EKCalendarTypeCalDAV")]
    pub const CalDAV: Self = Self(1);
    #[doc(alias = "EKCalendarTypeExchange")]
    pub const Exchange: Self = Self(2);
    #[doc(alias = "EKCalendarTypeSubscription")]
    pub const Subscription: Self = Self(3);
    #[doc(alias = "EKCalendarTypeBirthday")]
    pub const Birthday: Self = Self(4);
}

unsafe impl Encode for EKCalendarType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for EKCalendarType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekcalendareventavailabilitymask?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EKCalendarEventAvailabilityMask(pub NSUInteger);
bitflags::bitflags! {
    impl EKCalendarEventAvailabilityMask: NSUInteger {
        const EKCalendarEventAvailabilityNone = 0;
        const EKCalendarEventAvailabilityBusy = 1<<0;
        const EKCalendarEventAvailabilityFree = 1<<1;
        const EKCalendarEventAvailabilityTentative = 1<<2;
        const EKCalendarEventAvailabilityUnavailable = 1<<3;
    }
}

unsafe impl Encode for EKCalendarEventAvailabilityMask {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for EKCalendarEventAvailabilityMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/eventkit/eksourcetype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EKSourceType(pub NSInteger);
impl EKSourceType {
    #[doc(alias = "EKSourceTypeLocal")]
    pub const Local: Self = Self(0);
    #[doc(alias = "EKSourceTypeExchange")]
    pub const Exchange: Self = Self(1);
    #[doc(alias = "EKSourceTypeCalDAV")]
    pub const CalDAV: Self = Self(2);
    #[doc(alias = "EKSourceTypeMobileMe")]
    pub const MobileMe: Self = Self(3);
    #[doc(alias = "EKSourceTypeSubscribed")]
    pub const Subscribed: Self = Self(4);
    #[doc(alias = "EKSourceTypeBirthdays")]
    pub const Birthdays: Self = Self(5);
}

unsafe impl Encode for EKSourceType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for EKSourceType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// A value which specifies an entity type of event or reminder.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekentitytype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EKEntityType(pub NSUInteger);
impl EKEntityType {
    #[doc(alias = "EKEntityTypeEvent")]
    pub const Event: Self = Self(0);
    #[doc(alias = "EKEntityTypeReminder")]
    pub const Reminder: Self = Self(1);
}

unsafe impl Encode for EKEntityType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for EKEntityType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// A bitmask based on EKEntityType that can be used to specify multiple entities at once.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekentitymask?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EKEntityMask(pub NSUInteger);
bitflags::bitflags! {
    impl EKEntityMask: NSUInteger {
        #[doc(alias = "EKEntityMaskEvent")]
        const Event = 1<<EKEntityType::Event.0;
        #[doc(alias = "EKEntityMaskReminder")]
        const Reminder = 1<<EKEntityType::Reminder.0;
    }
}

unsafe impl Encode for EKEntityMask {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for EKEntityMask {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// A value indicating whether an alarm is triggered by entering or exiting a geofence.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekalarmproximity?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EKAlarmProximity(pub NSInteger);
impl EKAlarmProximity {
    #[doc(alias = "EKAlarmProximityNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "EKAlarmProximityEnter")]
    pub const Enter: Self = Self(1);
    #[doc(alias = "EKAlarmProximityLeave")]
    pub const Leave: Self = Self(2);
}

unsafe impl Encode for EKAlarmProximity {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for EKAlarmProximity {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// A value which specifies the action that occurs when the alarm is triggered.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekalarmtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EKAlarmType(pub NSInteger);
impl EKAlarmType {
    #[doc(alias = "EKAlarmTypeDisplay")]
    pub const Display: Self = Self(0);
    #[doc(alias = "EKAlarmTypeAudio")]
    pub const Audio: Self = Self(1);
    #[doc(alias = "EKAlarmTypeProcedure")]
    pub const Procedure: Self = Self(2);
    #[doc(alias = "EKAlarmTypeEmail")]
    pub const Email: Self = Self(3);
}

unsafe impl Encode for EKAlarmType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for EKAlarmType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// A priority for a reminder.
///
/// RFC 5545 allows priority to be specified with an integer in the range of 0-9,
/// with 0 representing an undefined priority, 1 the highest priority, and 9 the lowest priority.
/// Clients are encouraged to use these values when setting a reminders's priority,
/// but is is possible to specify any integer value from 0 to 9.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekreminderpriority?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EKReminderPriority(pub NSUInteger);
impl EKReminderPriority {
    #[doc(alias = "EKReminderPriorityNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "EKReminderPriorityHigh")]
    pub const High: Self = Self(1);
    #[doc(alias = "EKReminderPriorityMedium")]
    pub const Medium: Self = Self(5);
    #[doc(alias = "EKReminderPriorityLow")]
    pub const Low: Self = Self(9);
}

unsafe impl Encode for EKReminderPriority {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for EKReminderPriority {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
