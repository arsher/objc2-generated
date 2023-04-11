//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::EventKit::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "EventKit_EKRecurrenceDayOfWeek")]
    pub struct EKRecurrenceDayOfWeek;

    #[cfg(feature = "EventKit_EKRecurrenceDayOfWeek")]
    unsafe impl ClassType for EKRecurrenceDayOfWeek {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "EventKit_EKRecurrenceDayOfWeek")]
unsafe impl NSCoding for EKRecurrenceDayOfWeek {}

#[cfg(feature = "EventKit_EKRecurrenceDayOfWeek")]
unsafe impl NSCopying for EKRecurrenceDayOfWeek {}

#[cfg(feature = "EventKit_EKRecurrenceDayOfWeek")]
unsafe impl NSObjectProtocol for EKRecurrenceDayOfWeek {}

#[cfg(feature = "EventKit_EKRecurrenceDayOfWeek")]
unsafe impl NSSecureCoding for EKRecurrenceDayOfWeek {}

extern_methods!(
    #[cfg(feature = "EventKit_EKRecurrenceDayOfWeek")]
    unsafe impl EKRecurrenceDayOfWeek {
        #[method_id(@__retain_semantics Other dayOfWeek:)]
        pub unsafe fn dayOfWeek(day_of_the_week: EKWeekday) -> Id<Self>;

        #[method_id(@__retain_semantics Other dayOfWeek:weekNumber:)]
        pub unsafe fn dayOfWeek_weekNumber(
            day_of_the_week: EKWeekday,
            week_number: NSInteger,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithDayOfTheWeek:weekNumber:)]
        pub unsafe fn initWithDayOfTheWeek_weekNumber(
            this: Option<Allocated<Self>>,
            day_of_the_week: EKWeekday,
            week_number: NSInteger,
        ) -> Id<Self>;

        #[method(dayOfTheWeek)]
        pub unsafe fn dayOfTheWeek(&self) -> EKWeekday;

        #[method(weekNumber)]
        pub unsafe fn weekNumber(&self) -> NSInteger;
    }
);
