//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/eventkit/ekalarm?language=objc)
    #[unsafe(super(EKObject, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "EKObject")]
    pub struct EKAlarm;
);

#[cfg(feature = "EKObject")]
unsafe impl NSCopying for EKAlarm {}

#[cfg(feature = "EKObject")]
unsafe impl CopyingHelper for EKAlarm {
    type Result = Self;
}

#[cfg(feature = "EKObject")]
unsafe impl NSObjectProtocol for EKAlarm {}

extern_methods!(
    #[cfg(feature = "EKObject")]
    unsafe impl EKAlarm {
        #[method_id(@__retain_semantics Other alarmWithAbsoluteDate:)]
        pub unsafe fn alarmWithAbsoluteDate(date: &NSDate) -> Retained<EKAlarm>;

        #[method_id(@__retain_semantics Other alarmWithRelativeOffset:)]
        pub unsafe fn alarmWithRelativeOffset(offset: NSTimeInterval) -> Retained<EKAlarm>;

        #[method(relativeOffset)]
        pub unsafe fn relativeOffset(&self) -> NSTimeInterval;

        #[method(setRelativeOffset:)]
        pub unsafe fn setRelativeOffset(&self, relative_offset: NSTimeInterval);

        #[method_id(@__retain_semantics Other absoluteDate)]
        pub unsafe fn absoluteDate(&self) -> Option<Retained<NSDate>>;

        #[method(setAbsoluteDate:)]
        pub unsafe fn setAbsoluteDate(&self, absolute_date: Option<&NSDate>);

        #[cfg(feature = "EKStructuredLocation")]
        #[method_id(@__retain_semantics Other structuredLocation)]
        pub unsafe fn structuredLocation(&self) -> Option<Retained<EKStructuredLocation>>;

        #[cfg(feature = "EKStructuredLocation")]
        #[method(setStructuredLocation:)]
        pub unsafe fn setStructuredLocation(
            &self,
            structured_location: Option<&EKStructuredLocation>,
        );

        #[cfg(feature = "EKTypes")]
        #[method(proximity)]
        pub unsafe fn proximity(&self) -> EKAlarmProximity;

        #[cfg(feature = "EKTypes")]
        #[method(setProximity:)]
        pub unsafe fn setProximity(&self, proximity: EKAlarmProximity);

        #[cfg(feature = "EKTypes")]
        #[method(type)]
        pub unsafe fn r#type(&self) -> EKAlarmType;

        #[method_id(@__retain_semantics Other emailAddress)]
        pub unsafe fn emailAddress(&self) -> Option<Retained<NSString>>;

        #[method(setEmailAddress:)]
        pub unsafe fn setEmailAddress(&self, email_address: Option<&NSString>);

        #[method_id(@__retain_semantics Other soundName)]
        pub unsafe fn soundName(&self) -> Option<Retained<NSString>>;

        #[method(setSoundName:)]
        pub unsafe fn setSoundName(&self, sound_name: Option<&NSString>);

        #[deprecated]
        #[method_id(@__retain_semantics Other url)]
        pub unsafe fn url(&self) -> Option<Retained<NSURL>>;

        #[deprecated]
        #[method(setUrl:)]
        pub unsafe fn setUrl(&self, url: Option<&NSURL>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "EKObject")]
    unsafe impl EKAlarm {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
