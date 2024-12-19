//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/callkit/cxproviderconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CXProviderConfiguration;
);

unsafe impl NSCopying for CXProviderConfiguration {}

unsafe impl CopyingHelper for CXProviderConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CXProviderConfiguration {}

extern_methods!(
    unsafe impl CXProviderConfiguration {
        #[deprecated = "No longer supported"]
        #[method_id(@__retain_semantics Other localizedName)]
        pub unsafe fn localizedName(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other ringtoneSound)]
        pub unsafe fn ringtoneSound(&self) -> Option<Retained<NSString>>;

        #[method(setRingtoneSound:)]
        pub unsafe fn setRingtoneSound(&self, ringtone_sound: Option<&NSString>);

        #[method_id(@__retain_semantics Other iconTemplateImageData)]
        pub unsafe fn iconTemplateImageData(&self) -> Option<Retained<NSData>>;

        #[method(setIconTemplateImageData:)]
        pub unsafe fn setIconTemplateImageData(&self, icon_template_image_data: Option<&NSData>);

        #[method(maximumCallGroups)]
        pub unsafe fn maximumCallGroups(&self) -> NSUInteger;

        #[method(setMaximumCallGroups:)]
        pub unsafe fn setMaximumCallGroups(&self, maximum_call_groups: NSUInteger);

        #[method(maximumCallsPerCallGroup)]
        pub unsafe fn maximumCallsPerCallGroup(&self) -> NSUInteger;

        #[method(setMaximumCallsPerCallGroup:)]
        pub unsafe fn setMaximumCallsPerCallGroup(&self, maximum_calls_per_call_group: NSUInteger);

        #[method(includesCallsInRecents)]
        pub unsafe fn includesCallsInRecents(&self) -> bool;

        #[method(setIncludesCallsInRecents:)]
        pub unsafe fn setIncludesCallsInRecents(&self, includes_calls_in_recents: bool);

        #[method(supportsVideo)]
        pub unsafe fn supportsVideo(&self) -> bool;

        #[method(setSupportsVideo:)]
        pub unsafe fn setSupportsVideo(&self, supports_video: bool);

        #[method_id(@__retain_semantics Other supportedHandleTypes)]
        pub unsafe fn supportedHandleTypes(&self) -> Retained<NSSet<NSNumber>>;

        #[method(setSupportedHandleTypes:)]
        pub unsafe fn setSupportedHandleTypes(&self, supported_handle_types: &NSSet<NSNumber>);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init initWithLocalizedName:)]
        pub unsafe fn initWithLocalizedName(
            this: Allocated<Self>,
            localized_name: &NSString,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CXProviderConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
