//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/cavaluefunctionname?language=objc)
// NS_TYPED_ENUM
pub type CAValueFunctionName = NSString;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/cavaluefunction?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CAValueFunction;
);

unsafe impl NSCoding for CAValueFunction {}

unsafe impl NSObjectProtocol for CAValueFunction {}

unsafe impl NSSecureCoding for CAValueFunction {}

extern_methods!(
    unsafe impl CAValueFunction {
        #[method_id(@__retain_semantics Other functionWithName:)]
        pub unsafe fn functionWithName(name: &CAValueFunctionName) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<CAValueFunctionName>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CAValueFunction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcavaluefunctionrotatex?language=objc)
    pub static kCAValueFunctionRotateX: &'static CAValueFunctionName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcavaluefunctionrotatey?language=objc)
    pub static kCAValueFunctionRotateY: &'static CAValueFunctionName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcavaluefunctionrotatez?language=objc)
    pub static kCAValueFunctionRotateZ: &'static CAValueFunctionName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcavaluefunctionscale?language=objc)
    pub static kCAValueFunctionScale: &'static CAValueFunctionName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcavaluefunctionscalex?language=objc)
    pub static kCAValueFunctionScaleX: &'static CAValueFunctionName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcavaluefunctionscaley?language=objc)
    pub static kCAValueFunctionScaleY: &'static CAValueFunctionName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcavaluefunctionscalez?language=objc)
    pub static kCAValueFunctionScaleZ: &'static CAValueFunctionName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcavaluefunctiontranslate?language=objc)
    pub static kCAValueFunctionTranslate: &'static CAValueFunctionName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcavaluefunctiontranslatex?language=objc)
    pub static kCAValueFunctionTranslateX: &'static CAValueFunctionName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcavaluefunctiontranslatey?language=objc)
    pub static kCAValueFunctionTranslateY: &'static CAValueFunctionName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcavaluefunctiontranslatez?language=objc)
    pub static kCAValueFunctionTranslateZ: &'static CAValueFunctionName;
}
