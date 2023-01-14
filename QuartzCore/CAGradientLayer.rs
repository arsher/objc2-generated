//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

typed_enum!(
    pub type CAGradientLayerType = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CAGradientLayer")]
    pub struct CAGradientLayer;

    #[cfg(feature = "CoreAnimation_CAGradientLayer")]
    unsafe impl ClassType for CAGradientLayer {
        #[inherits(NSObject)]
        type Super = CALayer;
    }
);

extern_methods!(
    #[cfg(feature = "CoreAnimation_CAGradientLayer")]
    unsafe impl CAGradientLayer {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other colors)]
        pub unsafe fn colors(&self) -> Option<Id<NSArray, Shared>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setColors:)]
        pub unsafe fn setColors(&self, colors: Option<&NSArray>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other locations)]
        pub unsafe fn locations(&self) -> Option<Id<NSArray<NSNumber>, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method(setLocations:)]
        pub unsafe fn setLocations(&self, locations: Option<&NSArray<NSNumber>>);

        #[method(startPoint)]
        pub unsafe fn startPoint(&self) -> CGPoint;

        #[method(setStartPoint:)]
        pub unsafe fn setStartPoint(&self, startPoint: CGPoint);

        #[method(endPoint)]
        pub unsafe fn endPoint(&self) -> CGPoint;

        #[method(setEndPoint:)]
        pub unsafe fn setEndPoint(&self, endPoint: CGPoint);

        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Id<CAGradientLayerType, Shared>;

        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: &CAGradientLayerType);
    }
);

extern_static!(kCAGradientLayerAxial: &'static CAGradientLayerType);

extern_static!(kCAGradientLayerRadial: &'static CAGradientLayerType);

extern_static!(kCAGradientLayerConic: &'static CAGradientLayerType);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "CoreAnimation_CAGradientLayer")]
    unsafe impl CAGradientLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(
            this: Option<Allocated<Self>>,
            layer: &Object,
        ) -> Id<Self, Shared>;
    }
);
