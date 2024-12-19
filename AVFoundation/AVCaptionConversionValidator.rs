//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionconversionvalidatorstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVCaptionConversionValidatorStatus(pub NSInteger);
impl AVCaptionConversionValidatorStatus {
    #[doc(alias = "AVCaptionConversionValidatorStatusUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "AVCaptionConversionValidatorStatusValidating")]
    pub const Validating: Self = Self(1);
    #[doc(alias = "AVCaptionConversionValidatorStatusCompleted")]
    pub const Completed: Self = Self(2);
    #[doc(alias = "AVCaptionConversionValidatorStatusStopped")]
    pub const Stopped: Self = Self(3);
}

unsafe impl Encode for AVCaptionConversionValidatorStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVCaptionConversionValidatorStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionconversionvalidator?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptionConversionValidator;
);

unsafe impl NSObjectProtocol for AVCaptionConversionValidator {}

extern_methods!(
    unsafe impl AVCaptionConversionValidator {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(all(
            feature = "AVCaption",
            feature = "AVCaptionSettings",
            feature = "objc2-core-media"
        ))]
        #[method_id(@__retain_semantics Other captionConversionValidatorWithCaptions:timeRange:conversionSettings:)]
        pub unsafe fn captionConversionValidatorWithCaptions_timeRange_conversionSettings(
            captions: &NSArray<AVCaption>,
            time_range: CMTimeRange,
            conversion_settings: &NSDictionary<AVCaptionSettingsKey, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "AVCaption",
            feature = "AVCaptionSettings",
            feature = "objc2-core-media"
        ))]
        #[method_id(@__retain_semantics Init initWithCaptions:timeRange:conversionSettings:)]
        pub unsafe fn initWithCaptions_timeRange_conversionSettings(
            this: Allocated<Self>,
            captions: &NSArray<AVCaption>,
            time_range: CMTimeRange,
            conversion_settings: &NSDictionary<AVCaptionSettingsKey, AnyObject>,
        ) -> Retained<Self>;

        #[method(status)]
        pub unsafe fn status(&self) -> AVCaptionConversionValidatorStatus;

        #[cfg(feature = "AVCaption")]
        #[method_id(@__retain_semantics Other captions)]
        pub unsafe fn captions(&self) -> Retained<NSArray<AVCaption>>;

        #[cfg(feature = "objc2-core-media")]
        #[method(timeRange)]
        pub unsafe fn timeRange(&self) -> CMTimeRange;

        #[cfg(feature = "block2")]
        #[method(validateCaptionConversionWithWarningHandler:)]
        pub unsafe fn validateCaptionConversionWithWarningHandler(
            &self,
            handler: &block2::Block<dyn Fn(*mut AVCaptionConversionWarning)>,
        );

        #[method(stopValidating)]
        pub unsafe fn stopValidating(&self);

        #[method_id(@__retain_semantics Other warnings)]
        pub unsafe fn warnings(&self) -> Retained<NSArray<AVCaptionConversionWarning>>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionconversionwarningtype?language=objc)
// NS_TYPED_ENUM
pub type AVCaptionConversionWarningType = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionconversionwarningtypeexcessmediadata?language=objc)
    pub static AVCaptionConversionWarningTypeExcessMediaData:
        &'static AVCaptionConversionWarningType;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionconversionwarning?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptionConversionWarning;
);

unsafe impl Send for AVCaptionConversionWarning {}

unsafe impl Sync for AVCaptionConversionWarning {}

unsafe impl NSObjectProtocol for AVCaptionConversionWarning {}

extern_methods!(
    unsafe impl AVCaptionConversionWarning {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other warningType)]
        pub unsafe fn warningType(&self) -> Retained<AVCaptionConversionWarningType>;

        #[method(rangeOfCaptions)]
        pub unsafe fn rangeOfCaptions(&self) -> NSRange;

        #[method_id(@__retain_semantics Other adjustment)]
        pub unsafe fn adjustment(&self) -> Option<Retained<AVCaptionConversionAdjustment>>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionconversionadjustmenttype?language=objc)
// NS_TYPED_ENUM
pub type AVCaptionConversionAdjustmentType = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionconversionadjustmenttypetimerange?language=objc)
    pub static AVCaptionConversionAdjustmentTypeTimeRange:
        &'static AVCaptionConversionAdjustmentType;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionconversionadjustment?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptionConversionAdjustment;
);

unsafe impl Send for AVCaptionConversionAdjustment {}

unsafe impl Sync for AVCaptionConversionAdjustment {}

unsafe impl NSObjectProtocol for AVCaptionConversionAdjustment {}

extern_methods!(
    unsafe impl AVCaptionConversionAdjustment {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Other adjustmentType)]
        pub unsafe fn adjustmentType(&self) -> Retained<AVCaptionConversionAdjustmentType>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionconversiontimerangeadjustment?language=objc)
    #[unsafe(super(AVCaptionConversionAdjustment, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptionConversionTimeRangeAdjustment;
);

unsafe impl Send for AVCaptionConversionTimeRangeAdjustment {}

unsafe impl Sync for AVCaptionConversionTimeRangeAdjustment {}

unsafe impl NSObjectProtocol for AVCaptionConversionTimeRangeAdjustment {}

extern_methods!(
    unsafe impl AVCaptionConversionTimeRangeAdjustment {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "objc2-core-media")]
        #[method(startTimeOffset)]
        pub unsafe fn startTimeOffset(&self) -> CMTime;

        #[cfg(feature = "objc2-core-media")]
        #[method(durationOffset)]
        pub unsafe fn durationOffset(&self) -> CMTime;
    }
);
