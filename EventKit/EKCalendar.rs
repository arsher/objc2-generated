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
    #[cfg(feature = "EventKit_EKCalendar")]
    pub struct EKCalendar;

    #[cfg(feature = "EventKit_EKCalendar")]
    unsafe impl ClassType for EKCalendar {
        #[inherits(NSObject)]
        type Super = EKObject;
    }
);

extern_methods!(
    #[cfg(feature = "EventKit_EKCalendar")]
    unsafe impl EKCalendar {
        #[cfg(feature = "EventKit_EKEventStore")]
        #[method_id(@__retain_semantics Other calendarWithEventStore:)]
        pub unsafe fn calendarWithEventStore(event_store: &EKEventStore) -> Id<EKCalendar, Shared>;

        #[cfg(feature = "EventKit_EKEventStore")]
        #[method_id(@__retain_semantics Other calendarForEntityType:eventStore:)]
        pub unsafe fn calendarForEntityType_eventStore(
            entity_type: EKEntityType,
            event_store: &EKEventStore,
        ) -> Id<EKCalendar, Shared>;

        #[cfg(feature = "EventKit_EKSource")]
        #[method_id(@__retain_semantics Other source)]
        pub unsafe fn source(&self) -> Option<Id<EKSource, Shared>>;

        #[cfg(feature = "EventKit_EKSource")]
        #[method(setSource:)]
        pub unsafe fn setSource(&self, source: Option<&EKSource>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other calendarIdentifier)]
        pub unsafe fn calendarIdentifier(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[method(type)]
        pub unsafe fn r#type(&self) -> EKCalendarType;

        #[method(allowsContentModifications)]
        pub unsafe fn allowsContentModifications(&self) -> bool;

        #[method(isSubscribed)]
        pub unsafe fn isSubscribed(&self) -> bool;

        #[method(isImmutable)]
        pub unsafe fn isImmutable(&self) -> bool;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Id<NSColor, Shared>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: Option<&NSColor>);

        #[method(supportedEventAvailabilities)]
        pub unsafe fn supportedEventAvailabilities(&self) -> EKCalendarEventAvailabilityMask;

        #[method(allowedEntityTypes)]
        pub unsafe fn allowedEntityTypes(&self) -> EKEntityMask;
    }
);