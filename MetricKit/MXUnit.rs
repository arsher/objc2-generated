//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxunitsignalbars?language=objc)
    #[unsafe(super(NSDimension, NSUnit, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MXUnitSignalBars;
);

unsafe impl NSCoding for MXUnitSignalBars {}

unsafe impl NSCopying for MXUnitSignalBars {}

unsafe impl CopyingHelper for MXUnitSignalBars {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MXUnitSignalBars {}

unsafe impl NSSecureCoding for MXUnitSignalBars {}

extern_methods!(
    unsafe impl MXUnitSignalBars {
        #[method_id(@__retain_semantics Other bars)]
        pub unsafe fn bars() -> Retained<MXUnitSignalBars>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSDimension`
    unsafe impl MXUnitSignalBars {
        #[method_id(@__retain_semantics Init initWithSymbol:converter:)]
        pub unsafe fn initWithSymbol_converter(
            this: Allocated<Self>,
            symbol: &NSString,
            converter: &NSUnitConverter,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other baseUnit)]
        pub unsafe fn baseUnit() -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSUnit`
    unsafe impl MXUnitSignalBars {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithSymbol:)]
        pub unsafe fn initWithSymbol(this: Allocated<Self>, symbol: &NSString) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxunitaveragepixelluminance?language=objc)
    #[unsafe(super(NSDimension, NSUnit, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MXUnitAveragePixelLuminance;
);

unsafe impl NSCoding for MXUnitAveragePixelLuminance {}

unsafe impl NSCopying for MXUnitAveragePixelLuminance {}

unsafe impl CopyingHelper for MXUnitAveragePixelLuminance {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MXUnitAveragePixelLuminance {}

unsafe impl NSSecureCoding for MXUnitAveragePixelLuminance {}

extern_methods!(
    unsafe impl MXUnitAveragePixelLuminance {
        #[method_id(@__retain_semantics Other apl)]
        pub unsafe fn apl() -> Retained<MXUnitAveragePixelLuminance>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSDimension`
    unsafe impl MXUnitAveragePixelLuminance {
        #[method_id(@__retain_semantics Init initWithSymbol:converter:)]
        pub unsafe fn initWithSymbol_converter(
            this: Allocated<Self>,
            symbol: &NSString,
            converter: &NSUnitConverter,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other baseUnit)]
        pub unsafe fn baseUnit() -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSUnit`
    unsafe impl MXUnitAveragePixelLuminance {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithSymbol:)]
        pub unsafe fn initWithSymbol(this: Allocated<Self>, symbol: &NSString) -> Retained<Self>;
    }
);
