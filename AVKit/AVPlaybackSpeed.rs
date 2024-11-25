//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avkit/avplaybackspeed?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVPlaybackSpeed;
);

unsafe impl NSObjectProtocol for AVPlaybackSpeed {}

extern_methods!(
    unsafe impl AVPlaybackSpeed {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other systemDefaultSpeeds)]
        pub unsafe fn systemDefaultSpeeds() -> Retained<NSArray<AVPlaybackSpeed>>;

        #[method_id(@__retain_semantics Init initWithRate:localizedName:)]
        pub unsafe fn initWithRate_localizedName(
            this: Allocated<Self>,
            rate: c_float,
            localized_name: &NSString,
        ) -> Retained<Self>;

        #[method(rate)]
        pub unsafe fn rate(&self) -> c_float;

        #[method_id(@__retain_semantics Other localizedName)]
        pub unsafe fn localizedName(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other localizedNumericName)]
        pub unsafe fn localizedNumericName(&self) -> Retained<NSString>;
    }
);
