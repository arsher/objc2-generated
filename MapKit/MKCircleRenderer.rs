//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
    pub struct MKCircleRenderer;

    #[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
    unsafe impl ClassType for MKCircleRenderer {
        #[inherits(MKOverlayRenderer, NSObject)]
        type Super = MKOverlayPathRenderer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
unsafe impl NSObjectProtocol for MKCircleRenderer {}

extern_methods!(
    #[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
    unsafe impl MKCircleRenderer {
        #[cfg(all(feature = "MKCircle", feature = "MKShape"))]
        #[method_id(@__retain_semantics Init initWithCircle:)]
        pub unsafe fn initWithCircle(this: Allocated<Self>, circle: &MKCircle) -> Retained<Self>;

        #[cfg(all(feature = "MKCircle", feature = "MKShape"))]
        #[method_id(@__retain_semantics Other circle)]
        pub unsafe fn circle(&self) -> Retained<MKCircle>;

        #[method(strokeStart)]
        pub unsafe fn strokeStart(&self) -> CGFloat;

        #[method(setStrokeStart:)]
        pub unsafe fn setStrokeStart(&self, stroke_start: CGFloat);

        #[method(strokeEnd)]
        pub unsafe fn strokeEnd(&self) -> CGFloat;

        #[method(setStrokeEnd:)]
        pub unsafe fn setStrokeEnd(&self, stroke_end: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `MKOverlayRenderer`
    #[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
    unsafe impl MKCircleRenderer {
        #[cfg(all(feature = "MKAnnotation", feature = "MKOverlay"))]
        #[method_id(@__retain_semantics Init initWithOverlay:)]
        pub unsafe fn initWithOverlay(
            this: Allocated<Self>,
            overlay: &ProtocolObject<dyn MKOverlay>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
    unsafe impl MKCircleRenderer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
