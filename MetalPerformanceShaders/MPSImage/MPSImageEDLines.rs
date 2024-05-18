//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpsimageedlines?language=objc)
    #[unsafe(super(MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MPSKernel")]
    pub struct MPSImageEDLines;
);

#[cfg(feature = "MPSKernel")]
unsafe impl NSCoding for MPSImageEDLines {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSCopying for MPSImageEDLines {}

#[cfg(feature = "MPSKernel")]
unsafe impl CopyingHelper for MPSImageEDLines {
    type Result = Self;
}

#[cfg(feature = "MPSKernel")]
unsafe impl NSObjectProtocol for MPSImageEDLines {}

#[cfg(feature = "MPSKernel")]
unsafe impl NSSecureCoding for MPSImageEDLines {}

extern_methods!(
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSImageEDLines {
        #[method_id(@__retain_semantics Init initWithDevice:gaussianSigma:minLineLength:maxLines:detailRatio:gradientThreshold:lineErrorThreshold:mergeLocalityThreshold:)]
        pub unsafe fn initWithDevice_gaussianSigma_minLineLength_maxLines_detailRatio_gradientThreshold_lineErrorThreshold_mergeLocalityThreshold(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
            gaussian_sigma: c_float,
            min_line_length: c_ushort,
            max_lines: NSUInteger,
            detail_ratio: c_ushort,
            gradient_threshold: c_float,
            line_error_threshold: c_float,
            merge_locality_threshold: c_float,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[method(encodeToCommandBuffer:sourceTexture:destinationTexture:endpointBuffer:endpointOffset:)]
        pub unsafe fn encodeToCommandBuffer_sourceTexture_destinationTexture_endpointBuffer_endpointOffset(
            &self,
            command_buffer: &ProtocolObject<dyn MTLCommandBuffer>,
            source: &ProtocolObject<dyn MTLTexture>,
            dest: Option<&ProtocolObject<dyn MTLTexture>>,
            endpoint_buffer: &ProtocolObject<dyn MTLBuffer>,
            endpoint_offset: NSUInteger,
        );

        #[method(clipRectSource)]
        pub unsafe fn clipRectSource(&self) -> MTLRegion;

        #[method(setClipRectSource:)]
        pub unsafe fn setClipRectSource(&self, clip_rect_source: MTLRegion);

        #[method(gaussianSigma)]
        pub unsafe fn gaussianSigma(&self) -> c_float;

        #[method(minLineLength)]
        pub unsafe fn minLineLength(&self) -> c_ushort;

        #[method(setMinLineLength:)]
        pub unsafe fn setMinLineLength(&self, min_line_length: c_ushort);

        #[method(maxLines)]
        pub unsafe fn maxLines(&self) -> NSUInteger;

        #[method(setMaxLines:)]
        pub unsafe fn setMaxLines(&self, max_lines: NSUInteger);

        #[method(detailRatio)]
        pub unsafe fn detailRatio(&self) -> c_ushort;

        #[method(setDetailRatio:)]
        pub unsafe fn setDetailRatio(&self, detail_ratio: c_ushort);

        #[method(gradientThreshold)]
        pub unsafe fn gradientThreshold(&self) -> c_float;

        #[method(setGradientThreshold:)]
        pub unsafe fn setGradientThreshold(&self, gradient_threshold: c_float);

        #[method(lineErrorThreshold)]
        pub unsafe fn lineErrorThreshold(&self) -> c_float;

        #[method(setLineErrorThreshold:)]
        pub unsafe fn setLineErrorThreshold(&self, line_error_threshold: c_float);

        #[method(mergeLocalityThreshold)]
        pub unsafe fn mergeLocalityThreshold(&self) -> c_float;

        #[method(setMergeLocalityThreshold:)]
        pub unsafe fn setMergeLocalityThreshold(&self, merge_locality_threshold: c_float);
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSImageEDLines {
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MPSKernel")]
    unsafe impl MPSImageEDLines {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
