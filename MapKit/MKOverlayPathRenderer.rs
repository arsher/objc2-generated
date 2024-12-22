//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkoverlaypathrenderer?language=objc)
    #[unsafe(super(MKOverlayRenderer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MKOverlayRenderer")]
    pub struct MKOverlayPathRenderer;
);

#[cfg(feature = "MKOverlayRenderer")]
unsafe impl NSObjectProtocol for MKOverlayPathRenderer {}

extern_methods!(
    #[cfg(feature = "MKOverlayRenderer")]
    unsafe impl MKOverlayPathRenderer {
        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other fillColor)]
        pub unsafe fn fillColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        /// Setter for [`fillColor`][Self::fillColor].
        #[method(setFillColor:)]
        pub unsafe fn setFillColor(&self, fill_color: Option<&NSColor>);

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[method_id(@__retain_semantics Other strokeColor)]
        pub unsafe fn strokeColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        /// Setter for [`strokeColor`][Self::strokeColor].
        #[method(setStrokeColor:)]
        pub unsafe fn setStrokeColor(&self, stroke_color: Option<&NSColor>);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(lineWidth)]
        pub unsafe fn lineWidth(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`lineWidth`][Self::lineWidth].
        #[method(setLineWidth:)]
        pub unsafe fn setLineWidth(&self, line_width: CGFloat);

        #[cfg(feature = "objc2-core-graphics")]
        #[method(lineJoin)]
        pub unsafe fn lineJoin(&self) -> CGLineJoin;

        #[cfg(feature = "objc2-core-graphics")]
        /// Setter for [`lineJoin`][Self::lineJoin].
        #[method(setLineJoin:)]
        pub unsafe fn setLineJoin(&self, line_join: CGLineJoin);

        #[cfg(feature = "objc2-core-graphics")]
        #[method(lineCap)]
        pub unsafe fn lineCap(&self) -> CGLineCap;

        #[cfg(feature = "objc2-core-graphics")]
        /// Setter for [`lineCap`][Self::lineCap].
        #[method(setLineCap:)]
        pub unsafe fn setLineCap(&self, line_cap: CGLineCap);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(miterLimit)]
        pub unsafe fn miterLimit(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`miterLimit`][Self::miterLimit].
        #[method(setMiterLimit:)]
        pub unsafe fn setMiterLimit(&self, miter_limit: CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(lineDashPhase)]
        pub unsafe fn lineDashPhase(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`lineDashPhase`][Self::lineDashPhase].
        #[method(setLineDashPhase:)]
        pub unsafe fn setLineDashPhase(&self, line_dash_phase: CGFloat);

        #[method_id(@__retain_semantics Other lineDashPattern)]
        pub unsafe fn lineDashPattern(&self) -> Option<Retained<NSArray<NSNumber>>>;

        /// Setter for [`lineDashPattern`][Self::lineDashPattern].
        #[method(setLineDashPattern:)]
        pub unsafe fn setLineDashPattern(&self, line_dash_pattern: Option<&NSArray<NSNumber>>);

        #[method(shouldRasterize)]
        pub unsafe fn shouldRasterize(&self) -> bool;

        /// Setter for [`shouldRasterize`][Self::shouldRasterize].
        #[method(setShouldRasterize:)]
        pub unsafe fn setShouldRasterize(&self, should_rasterize: bool);

        #[method(createPath)]
        pub unsafe fn createPath(&self);

        #[cfg(feature = "objc2-core-graphics")]
        #[method(path)]
        pub unsafe fn path(&self) -> CGPathRef;

        #[cfg(feature = "objc2-core-graphics")]
        /// Setter for [`path`][Self::path].
        #[method(setPath:)]
        pub unsafe fn setPath(&self, path: CGPathRef);

        #[method(invalidatePath)]
        pub unsafe fn invalidatePath(&self);

        #[cfg(all(
            feature = "MKGeometry",
            feature = "objc2-core-foundation",
            feature = "objc2-core-graphics"
        ))]
        #[method(applyStrokePropertiesToContext:atZoomScale:)]
        pub unsafe fn applyStrokePropertiesToContext_atZoomScale(
            &self,
            context: CGContextRef,
            zoom_scale: MKZoomScale,
        );

        #[cfg(all(
            feature = "MKGeometry",
            feature = "objc2-core-foundation",
            feature = "objc2-core-graphics"
        ))]
        #[method(applyFillPropertiesToContext:atZoomScale:)]
        pub unsafe fn applyFillPropertiesToContext_atZoomScale(
            &self,
            context: CGContextRef,
            zoom_scale: MKZoomScale,
        );

        #[cfg(feature = "objc2-core-graphics")]
        #[method(strokePath:inContext:)]
        pub unsafe fn strokePath_inContext(&self, path: CGPathRef, context: CGContextRef);

        #[cfg(feature = "objc2-core-graphics")]
        #[method(fillPath:inContext:)]
        pub unsafe fn fillPath_inContext(&self, path: CGPathRef, context: CGContextRef);
    }
);

extern_methods!(
    /// Methods declared on superclass `MKOverlayRenderer`
    #[cfg(feature = "MKOverlayRenderer")]
    unsafe impl MKOverlayPathRenderer {
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
    #[cfg(feature = "MKOverlayRenderer")]
    unsafe impl MKOverlayPathRenderer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
