//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MKOverlayRenderer;

    unsafe impl ClassType for MKOverlayRenderer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for MKOverlayRenderer {}

extern_methods!(
    unsafe impl MKOverlayRenderer {
        #[cfg(all(feature = "MKAnnotation", feature = "MKOverlay"))]
        #[method_id(@__retain_semantics Init initWithOverlay:)]
        pub unsafe fn initWithOverlay(
            this: Allocated<Self>,
            overlay: &ProtocolObject<dyn MKOverlay>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "MKAnnotation", feature = "MKOverlay"))]
        #[method_id(@__retain_semantics Other overlay)]
        pub unsafe fn overlay(&self) -> Retained<ProtocolObject<dyn MKOverlay>>;

        #[cfg(feature = "MKGeometry")]
        #[method(pointForMapPoint:)]
        pub unsafe fn pointForMapPoint(&self, map_point: MKMapPoint) -> CGPoint;

        #[cfg(feature = "MKGeometry")]
        #[method(mapPointForPoint:)]
        pub unsafe fn mapPointForPoint(&self, point: CGPoint) -> MKMapPoint;

        #[cfg(feature = "MKGeometry")]
        #[method(rectForMapRect:)]
        pub unsafe fn rectForMapRect(&self, map_rect: MKMapRect) -> CGRect;

        #[cfg(feature = "MKGeometry")]
        #[method(mapRectForRect:)]
        pub unsafe fn mapRectForRect(&self, rect: CGRect) -> MKMapRect;

        #[cfg(feature = "MKGeometry")]
        #[method(canDrawMapRect:zoomScale:)]
        pub unsafe fn canDrawMapRect_zoomScale(
            &self,
            map_rect: MKMapRect,
            zoom_scale: MKZoomScale,
        ) -> bool;

        #[method(setNeedsDisplay)]
        pub unsafe fn setNeedsDisplay(&self);

        #[cfg(feature = "MKGeometry")]
        #[method(setNeedsDisplayInMapRect:)]
        pub unsafe fn setNeedsDisplayInMapRect(&self, map_rect: MKMapRect);

        #[cfg(feature = "MKGeometry")]
        #[method(setNeedsDisplayInMapRect:zoomScale:)]
        pub unsafe fn setNeedsDisplayInMapRect_zoomScale(
            &self,
            map_rect: MKMapRect,
            zoom_scale: MKZoomScale,
        );

        #[method(alpha)]
        pub unsafe fn alpha(&self) -> CGFloat;

        #[method(setAlpha:)]
        pub unsafe fn setAlpha(&self, alpha: CGFloat);

        #[method(contentScaleFactor)]
        pub unsafe fn contentScaleFactor(&self) -> CGFloat;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MKOverlayRenderer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    #[cfg(feature = "MKGeometry")]
    pub fn MKRoadWidthAtZoomScale(zoom_scale: MKZoomScale) -> CGFloat;
}
