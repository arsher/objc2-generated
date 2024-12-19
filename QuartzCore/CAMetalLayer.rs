//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-metal")]
use objc2_metal::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/cametaldrawable?language=objc)
    #[cfg(feature = "objc2-metal")]
    pub unsafe trait CAMetalDrawable: MTLDrawable {
        #[method_id(@__retain_semantics Other texture)]
        unsafe fn texture(&self) -> Retained<ProtocolObject<dyn MTLTexture>>;

        #[cfg(feature = "CALayer")]
        #[method_id(@__retain_semantics Other layer)]
        unsafe fn layer(&self) -> Retained<CAMetalLayer>;
    }

    #[cfg(feature = "objc2-metal")]
    unsafe impl ProtocolType for dyn CAMetalDrawable {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/quartzcore/cametallayer?language=objc)
    #[unsafe(super(CALayer, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CALayer")]
    pub struct CAMetalLayer;
);

#[cfg(all(feature = "CALayer", feature = "CAMediaTiming"))]
unsafe impl CAMediaTiming for CAMetalLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSCoding for CAMetalLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSObjectProtocol for CAMetalLayer {}

#[cfg(feature = "CALayer")]
unsafe impl NSSecureCoding for CAMetalLayer {}

extern_methods!(
    #[cfg(feature = "CALayer")]
    unsafe impl CAMetalLayer {
        #[cfg(feature = "objc2-metal")]
        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Option<Retained<ProtocolObject<dyn MTLDevice>>>;

        #[cfg(feature = "objc2-metal")]
        #[method(setDevice:)]
        pub unsafe fn setDevice(&self, device: Option<&ProtocolObject<dyn MTLDevice>>);

        #[cfg(feature = "objc2-metal")]
        #[method_id(@__retain_semantics Other preferredDevice)]
        pub unsafe fn preferredDevice(&self) -> Option<Retained<ProtocolObject<dyn MTLDevice>>>;

        #[cfg(feature = "objc2-metal")]
        #[method(pixelFormat)]
        pub unsafe fn pixelFormat(&self) -> MTLPixelFormat;

        #[cfg(feature = "objc2-metal")]
        #[method(setPixelFormat:)]
        pub unsafe fn setPixelFormat(&self, pixel_format: MTLPixelFormat);

        #[method(framebufferOnly)]
        pub unsafe fn framebufferOnly(&self) -> bool;

        #[method(setFramebufferOnly:)]
        pub unsafe fn setFramebufferOnly(&self, framebuffer_only: bool);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(drawableSize)]
        pub unsafe fn drawableSize(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setDrawableSize:)]
        pub unsafe fn setDrawableSize(&self, drawable_size: CGSize);

        #[cfg(feature = "objc2-metal")]
        #[method_id(@__retain_semantics Other nextDrawable)]
        pub unsafe fn nextDrawable(&self) -> Option<Retained<ProtocolObject<dyn CAMetalDrawable>>>;

        #[method(maximumDrawableCount)]
        pub unsafe fn maximumDrawableCount(&self) -> NSUInteger;

        #[method(setMaximumDrawableCount:)]
        pub unsafe fn setMaximumDrawableCount(&self, maximum_drawable_count: NSUInteger);

        #[method(presentsWithTransaction)]
        pub unsafe fn presentsWithTransaction(&self) -> bool;

        #[method(setPresentsWithTransaction:)]
        pub unsafe fn setPresentsWithTransaction(&self, presents_with_transaction: bool);

        #[cfg(feature = "objc2-core-graphics")]
        #[method(colorspace)]
        pub unsafe fn colorspace(&self) -> CGColorSpaceRef;

        #[cfg(feature = "objc2-core-graphics")]
        #[method(setColorspace:)]
        pub unsafe fn setColorspace(&self, colorspace: CGColorSpaceRef);

        #[method(wantsExtendedDynamicRangeContent)]
        pub unsafe fn wantsExtendedDynamicRangeContent(&self) -> bool;

        #[method(setWantsExtendedDynamicRangeContent:)]
        pub unsafe fn setWantsExtendedDynamicRangeContent(
            &self,
            wants_extended_dynamic_range_content: bool,
        );

        #[cfg(feature = "CAEDRMetadata")]
        #[method_id(@__retain_semantics Other EDRMetadata)]
        pub unsafe fn EDRMetadata(&self) -> Option<Retained<CAEDRMetadata>>;

        #[cfg(feature = "CAEDRMetadata")]
        #[method(setEDRMetadata:)]
        pub unsafe fn setEDRMetadata(&self, edr_metadata: Option<&CAEDRMetadata>);

        #[method(displaySyncEnabled)]
        pub unsafe fn displaySyncEnabled(&self) -> bool;

        #[method(setDisplaySyncEnabled:)]
        pub unsafe fn setDisplaySyncEnabled(&self, display_sync_enabled: bool);

        #[method(allowsNextDrawableTimeout)]
        pub unsafe fn allowsNextDrawableTimeout(&self) -> bool;

        #[method(setAllowsNextDrawableTimeout:)]
        pub unsafe fn setAllowsNextDrawableTimeout(&self, allows_next_drawable_timeout: bool);

        #[method_id(@__retain_semantics Other developerHUDProperties)]
        pub unsafe fn developerHUDProperties(&self) -> Option<Retained<NSDictionary>>;

        #[method(setDeveloperHUDProperties:)]
        pub unsafe fn setDeveloperHUDProperties(
            &self,
            developer_hud_properties: Option<&NSDictionary>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "CALayer")]
    unsafe impl CAMetalLayer {
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
    unsafe impl CAMetalLayer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
