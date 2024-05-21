//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
    pub struct MKMultiPolygonRenderer;

    #[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
    unsafe impl ClassType for MKMultiPolygonRenderer {
        #[inherits(MKOverlayRenderer, NSObject)]
        type Super = MKOverlayPathRenderer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
unsafe impl NSObjectProtocol for MKMultiPolygonRenderer {}

extern_methods!(
    #[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
    unsafe impl MKMultiPolygonRenderer {
        #[cfg(all(feature = "MKMultiPolygon", feature = "MKShape"))]
        #[method_id(@__retain_semantics Init initWithMultiPolygon:)]
        pub unsafe fn initWithMultiPolygon(
            this: Allocated<Self>,
            multi_polygon: &MKMultiPolygon,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MKMultiPolygon", feature = "MKShape"))]
        #[method_id(@__retain_semantics Other multiPolygon)]
        pub unsafe fn multiPolygon(&self) -> Retained<MKMultiPolygon>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MKOverlayRenderer`
    #[cfg(all(feature = "MKOverlayPathRenderer", feature = "MKOverlayRenderer"))]
    unsafe impl MKMultiPolygonRenderer {
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
    unsafe impl MKMultiPolygonRenderer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
