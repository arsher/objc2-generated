//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcaptureslider?language=objc)
    #[unsafe(super(AVCaptureControl, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AVCaptureControl")]
    pub struct AVCaptureSlider;
);

#[cfg(feature = "AVCaptureControl")]
unsafe impl NSObjectProtocol for AVCaptureSlider {}

extern_methods!(
    #[cfg(feature = "AVCaptureControl")]
    unsafe impl AVCaptureSlider {
        #[method_id(@__retain_semantics Init initWithLocalizedTitle:symbolName:minValue:maxValue:)]
        pub unsafe fn initWithLocalizedTitle_symbolName_minValue_maxValue(
            this: Allocated<Self>,
            localized_title: &NSString,
            symbol_name: &NSString,
            min_value: c_float,
            max_value: c_float,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLocalizedTitle:symbolName:minValue:maxValue:step:)]
        pub unsafe fn initWithLocalizedTitle_symbolName_minValue_maxValue_step(
            this: Allocated<Self>,
            localized_title: &NSString,
            symbol_name: &NSString,
            min_value: c_float,
            max_value: c_float,
            step: c_float,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLocalizedTitle:symbolName:values:)]
        pub unsafe fn initWithLocalizedTitle_symbolName_values(
            this: Allocated<Self>,
            localized_title: &NSString,
            symbol_name: &NSString,
            values: &NSArray<NSNumber>,
        ) -> Retained<Self>;

        #[method(value)]
        pub unsafe fn value(&self) -> c_float;

        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: c_float);

        #[method_id(@__retain_semantics Other localizedValueFormat)]
        pub unsafe fn localizedValueFormat(&self) -> Option<Retained<NSString>>;

        #[method(setLocalizedValueFormat:)]
        pub unsafe fn setLocalizedValueFormat(&self, localized_value_format: Option<&NSString>);

        #[method_id(@__retain_semantics Other prominentValues)]
        pub unsafe fn prominentValues(&self) -> Retained<NSArray<NSNumber>>;

        #[method(setProminentValues:)]
        pub unsafe fn setProminentValues(&self, prominent_values: &NSArray<NSNumber>);

        #[method_id(@__retain_semantics Other localizedTitle)]
        pub unsafe fn localizedTitle(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other symbolName)]
        pub unsafe fn symbolName(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other accessibilityIdentifier)]
        pub unsafe fn accessibilityIdentifier(&self) -> Option<Retained<NSString>>;

        #[method(setAccessibilityIdentifier:)]
        pub unsafe fn setAccessibilityIdentifier(
            &self,
            accessibility_identifier: Option<&NSString>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `AVCaptureControl`
    #[cfg(feature = "AVCaptureControl")]
    unsafe impl AVCaptureSlider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);