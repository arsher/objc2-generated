//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/cagradientlayertype?language=objc)
// NS_TYPED_ENUM
pub type CAGradientLayerType = NSString;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/cagradientlayer?language=objc)
    #[unsafe(super(CALayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CALayer")]
    pub struct CAGradientLayer;
);

#[cfg(all(feature = "CALayer", feature = "CAMediaTiming"))]
unsafe impl CAMediaTiming for CAGradientLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSCoding for CAGradientLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSObjectProtocol for CAGradientLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSSecureCoding for CAGradientLayer {}

extern_methods!(
    #[cfg(feature = "CALayer")]
    unsafe impl CAGradientLayer {
        #[method_id(@__retain_semantics Other colors)]
        pub unsafe fn colors(&self) -> Option<Retained<NSArray>>;

        #[method(setColors:)]
        pub unsafe fn setColors(&self, colors: Option<&NSArray>);

        #[method_id(@__retain_semantics Other locations)]
        pub unsafe fn locations(&self) -> Option<Retained<NSArray<NSNumber>>>;

        #[method(setLocations:)]
        pub unsafe fn setLocations(&self, locations: Option<&NSArray<NSNumber>>);

        #[method(startPoint)]
        pub unsafe fn startPoint(&self) -> CGPoint;

        #[method(setStartPoint:)]
        pub unsafe fn setStartPoint(&self, start_point: CGPoint);

        #[method(endPoint)]
        pub unsafe fn endPoint(&self) -> CGPoint;

        #[method(setEndPoint:)]
        pub unsafe fn setEndPoint(&self, end_point: CGPoint);

        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Retained<CAGradientLayerType>;

        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: &CAGradientLayerType);
    }
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "CALayer")]
    unsafe impl CAGradientLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(this: Allocated<Self>, layer: &AnyObject) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CALayer")]
    unsafe impl CAGradientLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcagradientlayeraxial?language=objc)
    pub static kCAGradientLayerAxial: &'static CAGradientLayerType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcagradientlayerradial?language=objc)
    pub static kCAGradientLayerRadial: &'static CAGradientLayerType;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/kcagradientlayerconic?language=objc)
    pub static kCAGradientLayerConic: &'static CAGradientLayerType;
}
