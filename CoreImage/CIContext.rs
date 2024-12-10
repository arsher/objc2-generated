//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;
#[cfg(feature = "objc2-core-video")]
use objc2_core_video::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-io-surface")]
use objc2_io_surface::*;
#[cfg(feature = "objc2-metal")]
use objc2_metal::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cicontextoption?language=objc)
// NS_TYPED_ENUM
pub type CIContextOption = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kcicontextoutputcolorspace?language=objc)
    pub static kCIContextOutputColorSpace: &'static CIContextOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kcicontextworkingcolorspace?language=objc)
    pub static kCIContextWorkingColorSpace: &'static CIContextOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kcicontextworkingformat?language=objc)
    pub static kCIContextWorkingFormat: &'static CIContextOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kcicontexthighqualitydownsample?language=objc)
    pub static kCIContextHighQualityDownsample: &'static CIContextOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kcicontextoutputpremultiplied?language=objc)
    pub static kCIContextOutputPremultiplied: &'static CIContextOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kcicontextcacheintermediates?language=objc)
    pub static kCIContextCacheIntermediates: &'static CIContextOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kcicontextusesoftwarerenderer?language=objc)
    pub static kCIContextUseSoftwareRenderer: &'static CIContextOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kcicontextpriorityrequestlow?language=objc)
    pub static kCIContextPriorityRequestLow: &'static CIContextOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kcicontextallowlowpower?language=objc)
    pub static kCIContextAllowLowPower: &'static CIContextOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kcicontextname?language=objc)
    pub static kCIContextName: &'static CIContextOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kcicontextmemorylimit?language=objc)
    pub static kCIContextMemoryLimit: &'static CIContextOption;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/cicontext?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIContext;
);

unsafe impl NSObjectProtocol for CIContext {}

extern_methods!(
    unsafe impl CIContext {
        #[cfg(feature = "objc2-core-graphics")]
        #[method_id(@__retain_semantics Other contextWithCGContext:options:)]
        pub unsafe fn contextWithCGContext_options(
            cgctx: CGContextRef,
            options: Option<&NSDictionary<CIContextOption, AnyObject>>,
        ) -> Retained<CIContext>;

        #[method_id(@__retain_semantics Other contextWithOptions:)]
        pub unsafe fn contextWithOptions(
            options: Option<&NSDictionary<CIContextOption, AnyObject>>,
        ) -> Retained<CIContext>;

        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context() -> Retained<CIContext>;

        #[method_id(@__retain_semantics Init initWithOptions:)]
        pub unsafe fn initWithOptions(
            this: Allocated<Self>,
            options: Option<&NSDictionary<CIContextOption, AnyObject>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-metal")]
        #[method_id(@__retain_semantics Other contextWithMTLDevice:)]
        pub unsafe fn contextWithMTLDevice(
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<CIContext>;

        #[cfg(feature = "objc2-metal")]
        #[method_id(@__retain_semantics Other contextWithMTLDevice:options:)]
        pub unsafe fn contextWithMTLDevice_options(
            device: &ProtocolObject<dyn MTLDevice>,
            options: Option<&NSDictionary<CIContextOption, AnyObject>>,
        ) -> Retained<CIContext>;

        #[cfg(feature = "objc2-metal")]
        #[method_id(@__retain_semantics Other contextWithMTLCommandQueue:)]
        pub unsafe fn contextWithMTLCommandQueue(
            command_queue: &ProtocolObject<dyn MTLCommandQueue>,
        ) -> Retained<CIContext>;

        #[cfg(feature = "objc2-metal")]
        #[method_id(@__retain_semantics Other contextWithMTLCommandQueue:options:)]
        pub unsafe fn contextWithMTLCommandQueue_options(
            command_queue: &ProtocolObject<dyn MTLCommandQueue>,
            options: Option<&NSDictionary<CIContextOption, AnyObject>>,
        ) -> Retained<CIContext>;

        #[cfg(feature = "objc2-core-graphics")]
        #[method(workingColorSpace)]
        pub unsafe fn workingColorSpace(&self) -> CGColorSpaceRef;

        #[cfg(feature = "CIImage")]
        #[method(workingFormat)]
        pub unsafe fn workingFormat(&self) -> CIFormat;

        #[cfg(all(feature = "CIImage", feature = "objc2-core-foundation"))]
        #[deprecated]
        #[method(drawImage:atPoint:fromRect:)]
        pub unsafe fn drawImage_atPoint_fromRect(
            &self,
            image: &CIImage,
            at_point: CGPoint,
            from_rect: CGRect,
        );

        #[cfg(all(feature = "CIImage", feature = "objc2-core-foundation"))]
        #[method(drawImage:inRect:fromRect:)]
        pub unsafe fn drawImage_inRect_fromRect(
            &self,
            image: &CIImage,
            in_rect: CGRect,
            from_rect: CGRect,
        );

        #[cfg(all(
            feature = "CIImage",
            feature = "objc2-core-foundation",
            feature = "objc2-core-graphics"
        ))]
        #[method(render:toBitmap:rowBytes:bounds:format:colorSpace:)]
        pub unsafe fn render_toBitmap_rowBytes_bounds_format_colorSpace(
            &self,
            image: &CIImage,
            data: NonNull<c_void>,
            row_bytes: isize,
            bounds: CGRect,
            format: CIFormat,
            color_space: CGColorSpaceRef,
        );

        #[cfg(all(
            feature = "CIImage",
            feature = "objc2-core-foundation",
            feature = "objc2-core-graphics",
            feature = "objc2-io-surface"
        ))]
        #[method(render:toIOSurface:bounds:colorSpace:)]
        pub unsafe fn render_toIOSurface_bounds_colorSpace(
            &self,
            image: &CIImage,
            surface: IOSurfaceRef,
            bounds: CGRect,
            color_space: CGColorSpaceRef,
        );

        #[cfg(all(feature = "CIImage", feature = "objc2-core-video"))]
        #[method(render:toCVPixelBuffer:)]
        pub unsafe fn render_toCVPixelBuffer(&self, image: &CIImage, buffer: CVPixelBufferRef);

        #[cfg(all(
            feature = "CIImage",
            feature = "objc2-core-foundation",
            feature = "objc2-core-graphics",
            feature = "objc2-core-video"
        ))]
        #[method(render:toCVPixelBuffer:bounds:colorSpace:)]
        pub unsafe fn render_toCVPixelBuffer_bounds_colorSpace(
            &self,
            image: &CIImage,
            buffer: CVPixelBufferRef,
            bounds: CGRect,
            color_space: CGColorSpaceRef,
        );

        #[cfg(all(
            feature = "CIImage",
            feature = "objc2-core-foundation",
            feature = "objc2-core-graphics",
            feature = "objc2-metal"
        ))]
        #[method(render:toMTLTexture:commandBuffer:bounds:colorSpace:)]
        pub unsafe fn render_toMTLTexture_commandBuffer_bounds_colorSpace(
            &self,
            image: &CIImage,
            texture: &ProtocolObject<dyn MTLTexture>,
            command_buffer: Option<&ProtocolObject<dyn MTLCommandBuffer>>,
            bounds: CGRect,
            color_space: CGColorSpaceRef,
        );

        #[method(reclaimResources)]
        pub unsafe fn reclaimResources(&self);

        #[method(clearCaches)]
        pub unsafe fn clearCaches(&self);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(inputImageMaximumSize)]
        pub unsafe fn inputImageMaximumSize(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(outputImageMaximumSize)]
        pub unsafe fn outputImageMaximumSize(&self) -> CGSize;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIContext {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// createCGImage
    unsafe impl CIContext {}
);

extern_methods!(
    /// OfflineGPUSupport
    unsafe impl CIContext {
        #[method(offlineGPUCount)]
        pub unsafe fn offlineGPUCount() -> c_uint;

        #[deprecated = "Core Image OpenGL API deprecated. (Define CI_SILENCE_GL_DEPRECATION to silence these warnings)"]
        #[method_id(@__retain_semantics Other contextForOfflineGPUAtIndex:)]
        pub unsafe fn contextForOfflineGPUAtIndex(index: c_uint) -> Option<Retained<CIContext>>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/coreimage/ciimagerepresentationoption?language=objc)
// NS_TYPED_ENUM
pub type CIImageRepresentationOption = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimagerepresentationavdepthdata?language=objc)
    pub static kCIImageRepresentationAVDepthData: &'static CIImageRepresentationOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimagerepresentationdepthimage?language=objc)
    pub static kCIImageRepresentationDepthImage: &'static CIImageRepresentationOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimagerepresentationdisparityimage?language=objc)
    pub static kCIImageRepresentationDisparityImage: &'static CIImageRepresentationOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimagerepresentationavportraiteffectsmatte?language=objc)
    pub static kCIImageRepresentationAVPortraitEffectsMatte: &'static CIImageRepresentationOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimagerepresentationportraiteffectsmatteimage?language=objc)
    pub static kCIImageRepresentationPortraitEffectsMatteImage:
        &'static CIImageRepresentationOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimagerepresentationavsemanticsegmentationmattes?language=objc)
    pub static kCIImageRepresentationAVSemanticSegmentationMattes:
        &'static CIImageRepresentationOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimagerepresentationsemanticsegmentationskinmatteimage?language=objc)
    pub static kCIImageRepresentationSemanticSegmentationSkinMatteImage:
        &'static CIImageRepresentationOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimagerepresentationsemanticsegmentationhairmatteimage?language=objc)
    pub static kCIImageRepresentationSemanticSegmentationHairMatteImage:
        &'static CIImageRepresentationOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimagerepresentationsemanticsegmentationteethmatteimage?language=objc)
    pub static kCIImageRepresentationSemanticSegmentationTeethMatteImage:
        &'static CIImageRepresentationOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimagerepresentationsemanticsegmentationglassesmatteimage?language=objc)
    pub static kCIImageRepresentationSemanticSegmentationGlassesMatteImage:
        &'static CIImageRepresentationOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimagerepresentationsemanticsegmentationskymatteimage?language=objc)
    pub static kCIImageRepresentationSemanticSegmentationSkyMatteImage:
        &'static CIImageRepresentationOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimagerepresentationhdrimage?language=objc)
    pub static kCIImageRepresentationHDRImage: &'static CIImageRepresentationOption;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/kciimagerepresentationhdrgainmapimage?language=objc)
    pub static kCIImageRepresentationHDRGainMapImage: &'static CIImageRepresentationOption;
}

extern_methods!(
    /// ImageRepresentation
    unsafe impl CIContext {
        #[cfg(all(feature = "CIImage", feature = "objc2-core-graphics"))]
        #[method_id(@__retain_semantics Other TIFFRepresentationOfImage:format:colorSpace:options:)]
        pub unsafe fn TIFFRepresentationOfImage_format_colorSpace_options(
            &self,
            image: &CIImage,
            format: CIFormat,
            color_space: CGColorSpaceRef,
            options: &NSDictionary<CIImageRepresentationOption, AnyObject>,
        ) -> Option<Retained<NSData>>;

        #[cfg(all(feature = "CIImage", feature = "objc2-core-graphics"))]
        #[method_id(@__retain_semantics Other JPEGRepresentationOfImage:colorSpace:options:)]
        pub unsafe fn JPEGRepresentationOfImage_colorSpace_options(
            &self,
            image: &CIImage,
            color_space: CGColorSpaceRef,
            options: &NSDictionary<CIImageRepresentationOption, AnyObject>,
        ) -> Option<Retained<NSData>>;

        #[cfg(all(feature = "CIImage", feature = "objc2-core-graphics"))]
        #[method_id(@__retain_semantics Other HEIFRepresentationOfImage:format:colorSpace:options:)]
        pub unsafe fn HEIFRepresentationOfImage_format_colorSpace_options(
            &self,
            image: &CIImage,
            format: CIFormat,
            color_space: CGColorSpaceRef,
            options: &NSDictionary<CIImageRepresentationOption, AnyObject>,
        ) -> Option<Retained<NSData>>;

        #[cfg(all(feature = "CIImage", feature = "objc2-core-graphics"))]
        #[method_id(@__retain_semantics Other HEIF10RepresentationOfImage:colorSpace:options:error:_)]
        pub unsafe fn HEIF10RepresentationOfImage_colorSpace_options_error(
            &self,
            image: &CIImage,
            color_space: CGColorSpaceRef,
            options: &NSDictionary<CIImageRepresentationOption, AnyObject>,
        ) -> Result<Retained<NSData>, Retained<NSError>>;

        #[cfg(all(feature = "CIImage", feature = "objc2-core-graphics"))]
        #[method_id(@__retain_semantics Other PNGRepresentationOfImage:format:colorSpace:options:)]
        pub unsafe fn PNGRepresentationOfImage_format_colorSpace_options(
            &self,
            image: &CIImage,
            format: CIFormat,
            color_space: CGColorSpaceRef,
            options: &NSDictionary<CIImageRepresentationOption, AnyObject>,
        ) -> Option<Retained<NSData>>;

        #[cfg(feature = "CIImage")]
        #[method_id(@__retain_semantics Other OpenEXRRepresentationOfImage:options:error:_)]
        pub unsafe fn OpenEXRRepresentationOfImage_options_error(
            &self,
            image: &CIImage,
            options: &NSDictionary<CIImageRepresentationOption, AnyObject>,
        ) -> Result<Retained<NSData>, Retained<NSError>>;

        #[cfg(all(feature = "CIImage", feature = "objc2-core-graphics"))]
        #[method(writeTIFFRepresentationOfImage:toURL:format:colorSpace:options:error:_)]
        pub unsafe fn writeTIFFRepresentationOfImage_toURL_format_colorSpace_options_error(
            &self,
            image: &CIImage,
            url: &NSURL,
            format: CIFormat,
            color_space: CGColorSpaceRef,
            options: &NSDictionary<CIImageRepresentationOption, AnyObject>,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "CIImage", feature = "objc2-core-graphics"))]
        #[method(writePNGRepresentationOfImage:toURL:format:colorSpace:options:error:_)]
        pub unsafe fn writePNGRepresentationOfImage_toURL_format_colorSpace_options_error(
            &self,
            image: &CIImage,
            url: &NSURL,
            format: CIFormat,
            color_space: CGColorSpaceRef,
            options: &NSDictionary<CIImageRepresentationOption, AnyObject>,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "CIImage", feature = "objc2-core-graphics"))]
        #[method(writeJPEGRepresentationOfImage:toURL:colorSpace:options:error:_)]
        pub unsafe fn writeJPEGRepresentationOfImage_toURL_colorSpace_options_error(
            &self,
            image: &CIImage,
            url: &NSURL,
            color_space: CGColorSpaceRef,
            options: &NSDictionary<CIImageRepresentationOption, AnyObject>,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "CIImage", feature = "objc2-core-graphics"))]
        #[method(writeHEIFRepresentationOfImage:toURL:format:colorSpace:options:error:_)]
        pub unsafe fn writeHEIFRepresentationOfImage_toURL_format_colorSpace_options_error(
            &self,
            image: &CIImage,
            url: &NSURL,
            format: CIFormat,
            color_space: CGColorSpaceRef,
            options: &NSDictionary<CIImageRepresentationOption, AnyObject>,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(all(feature = "CIImage", feature = "objc2-core-graphics"))]
        #[method(writeHEIF10RepresentationOfImage:toURL:colorSpace:options:error:_)]
        pub unsafe fn writeHEIF10RepresentationOfImage_toURL_colorSpace_options_error(
            &self,
            image: &CIImage,
            url: &NSURL,
            color_space: CGColorSpaceRef,
            options: &NSDictionary<CIImageRepresentationOption, AnyObject>,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "CIImage")]
        #[method(writeOpenEXRRepresentationOfImage:toURL:options:error:_)]
        pub unsafe fn writeOpenEXRRepresentationOfImage_toURL_options_error(
            &self,
            image: &CIImage,
            url: &NSURL,
            options: &NSDictionary<CIImageRepresentationOption, AnyObject>,
        ) -> Result<(), Retained<NSError>>;
    }
);

extern_methods!(
    /// CIDepthBlurEffect
    unsafe impl CIContext {
        #[cfg(feature = "CIFilter")]
        #[method_id(@__retain_semantics Other depthBlurEffectFilterForImageURL:options:)]
        pub unsafe fn depthBlurEffectFilterForImageURL_options(
            &self,
            url: &NSURL,
            options: Option<&NSDictionary>,
        ) -> Option<Retained<CIFilter>>;

        #[cfg(feature = "CIFilter")]
        #[method_id(@__retain_semantics Other depthBlurEffectFilterForImageData:options:)]
        pub unsafe fn depthBlurEffectFilterForImageData_options(
            &self,
            data: &NSData,
            options: Option<&NSDictionary>,
        ) -> Option<Retained<CIFilter>>;
    }
);
