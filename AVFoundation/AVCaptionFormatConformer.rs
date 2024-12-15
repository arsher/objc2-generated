//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptionformatconformer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptionFormatConformer;
);

unsafe impl NSObjectProtocol for AVCaptionFormatConformer {}

extern_methods!(
    unsafe impl AVCaptionFormatConformer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "AVCaptionSettings")]
        #[method_id(@__retain_semantics Other captionFormatConformerWithConversionSettings:)]
        pub unsafe fn captionFormatConformerWithConversionSettings(
            conversion_settings: &NSDictionary<AVCaptionSettingsKey, AnyObject>,
        ) -> Retained<Self>;

        #[cfg(feature = "AVCaptionSettings")]
        #[method_id(@__retain_semantics Init initWithConversionSettings:)]
        pub unsafe fn initWithConversionSettings(
            this: Allocated<Self>,
            conversion_settings: &NSDictionary<AVCaptionSettingsKey, AnyObject>,
        ) -> Retained<Self>;

        #[method(conformsCaptionsToTimeRange)]
        pub unsafe fn conformsCaptionsToTimeRange(&self) -> bool;

        #[method(setConformsCaptionsToTimeRange:)]
        pub unsafe fn setConformsCaptionsToTimeRange(&self, conforms_captions_to_time_range: bool);

        #[cfg(feature = "AVCaption")]
        #[method_id(@__retain_semantics Other conformedCaptionForCaption:error:_)]
        pub unsafe fn conformedCaptionForCaption_error(
            &self,
            caption: &AVCaption,
        ) -> Result<Retained<AVCaption>, Retained<NSError>>;
    }
);
