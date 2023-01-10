//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation;
use crate::Foundation;

typed_enum!(
    pub type CATextLayerTruncationMode = Foundation::NSString;
);

typed_enum!(
    pub type CATextLayerAlignmentMode = Foundation::NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CATextLayer;

    unsafe impl ClassType for CATextLayer {
        #[inherits(NSObject)]
        type Super = CoreAnimation::CALayer;
    }
);

extern_methods!(
    #[cfg(feature = "CoreAnimation_CATextLayer")]
    unsafe impl CATextLayer {
        #[method_id(@__retain_semantics Other string)]
        pub unsafe fn string(&self) -> Option<Id<Object, Shared>>;

        #[method(setString:)]
        pub unsafe fn setString(&self, string: Option<&Object>);

        #[method(fontSize)]
        pub unsafe fn fontSize(&self) -> CGFloat;

        #[method(setFontSize:)]
        pub unsafe fn setFontSize(&self, fontSize: CGFloat);

        #[method(isWrapped)]
        pub unsafe fn isWrapped(&self) -> bool;

        #[method(setWrapped:)]
        pub unsafe fn setWrapped(&self, wrapped: bool);

        #[method_id(@__retain_semantics Other truncationMode)]
        pub unsafe fn truncationMode(&self)
            -> Id<CoreAnimation::CATextLayerTruncationMode, Shared>;

        #[method(setTruncationMode:)]
        pub unsafe fn setTruncationMode(
            &self,
            truncationMode: &CoreAnimation::CATextLayerTruncationMode,
        );

        #[method_id(@__retain_semantics Other alignmentMode)]
        pub unsafe fn alignmentMode(&self) -> Id<CoreAnimation::CATextLayerAlignmentMode, Shared>;

        #[method(setAlignmentMode:)]
        pub unsafe fn setAlignmentMode(
            &self,
            alignmentMode: &CoreAnimation::CATextLayerAlignmentMode,
        );

        #[method(allowsFontSubpixelQuantization)]
        pub unsafe fn allowsFontSubpixelQuantization(&self) -> bool;

        #[method(setAllowsFontSubpixelQuantization:)]
        pub unsafe fn setAllowsFontSubpixelQuantization(
            &self,
            allowsFontSubpixelQuantization: bool,
        );
    }
);

extern_static!(kCATruncationNone: &'static CoreAnimation::CATextLayerTruncationMode);

extern_static!(kCATruncationStart: &'static CoreAnimation::CATextLayerTruncationMode);

extern_static!(kCATruncationEnd: &'static CoreAnimation::CATextLayerTruncationMode);

extern_static!(kCATruncationMiddle: &'static CoreAnimation::CATextLayerTruncationMode);

extern_static!(kCAAlignmentNatural: &'static CoreAnimation::CATextLayerAlignmentMode);

extern_static!(kCAAlignmentLeft: &'static CoreAnimation::CATextLayerAlignmentMode);

extern_static!(kCAAlignmentRight: &'static CoreAnimation::CATextLayerAlignmentMode);

extern_static!(kCAAlignmentCenter: &'static CoreAnimation::CATextLayerAlignmentMode);

extern_static!(kCAAlignmentJustified: &'static CoreAnimation::CATextLayerAlignmentMode);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "CoreAnimation_CATextLayer")]
    unsafe impl CoreAnimation::CATextLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(
            this: Option<Allocated<Self>>,
            layer: &Object,
        ) -> Id<Self, Shared>;
    }
);
