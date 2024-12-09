//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-metal")]
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/ciimageprocessorkernel?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CIImageProcessorKernel;
);

unsafe impl NSObjectProtocol for CIImageProcessorKernel {}

extern_methods!(
    unsafe impl CIImageProcessorKernel {
        #[method(processWithInputs:arguments:output:error:_)]
        pub unsafe fn processWithInputs_arguments_output_error(
            inputs: Option<&NSArray<ProtocolObject<dyn CIImageProcessorInput>>>,
            arguments: Option<&NSDictionary<NSString, AnyObject>>,
            output: &ProtocolObject<dyn CIImageProcessorOutput>,
        ) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(roiForInput:arguments:outputRect:)]
        pub unsafe fn roiForInput_arguments_outputRect(
            input: c_int,
            arguments: Option<&NSDictionary<NSString, AnyObject>>,
            output_rect: CGRect,
        ) -> CGRect;

        #[cfg(all(feature = "CIVector", feature = "objc2-core-foundation"))]
        #[method_id(@__retain_semantics Other roiTileArrayForInput:arguments:outputRect:)]
        pub unsafe fn roiTileArrayForInput_arguments_outputRect(
            input: c_int,
            arguments: Option<&NSDictionary<NSString, AnyObject>>,
            output_rect: CGRect,
        ) -> Retained<NSArray<CIVector>>;

        #[cfg(feature = "CIImage")]
        #[method(formatForInputAtIndex:)]
        pub unsafe fn formatForInputAtIndex(input: c_int) -> CIFormat;

        #[cfg(feature = "CIImage")]
        #[method(outputFormat)]
        pub unsafe fn outputFormat() -> CIFormat;

        #[method(outputIsOpaque)]
        pub unsafe fn outputIsOpaque() -> bool;

        #[method(synchronizeInputs)]
        pub unsafe fn synchronizeInputs() -> bool;

        #[cfg(all(feature = "CIImage", feature = "objc2-core-foundation"))]
        #[method_id(@__retain_semantics Other applyWithExtent:inputs:arguments:error:_)]
        pub unsafe fn applyWithExtent_inputs_arguments_error(
            extent: CGRect,
            inputs: Option<&NSArray<CIImage>>,
            args: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Result<Retained<CIImage>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CIImageProcessorKernel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/ciimageprocessorinput?language=objc)
    pub unsafe trait CIImageProcessorInput {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(region)]
        unsafe fn region(&self) -> CGRect;

        #[method(bytesPerRow)]
        unsafe fn bytesPerRow(&self) -> usize;

        #[cfg(feature = "CIImage")]
        #[method(format)]
        unsafe fn format(&self) -> CIFormat;

        #[method(baseAddress)]
        unsafe fn baseAddress(&self) -> NonNull<c_void>;

        #[cfg(feature = "objc2-metal")]
        #[method_id(@__retain_semantics Other metalTexture)]
        unsafe fn metalTexture(&self) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        #[method(digest)]
        unsafe fn digest(&self) -> u64;

        #[method(roiTileIndex)]
        unsafe fn roiTileIndex(&self) -> NSUInteger;

        #[method(roiTileCount)]
        unsafe fn roiTileCount(&self) -> NSUInteger;
    }

    unsafe impl ProtocolType for dyn CIImageProcessorInput {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coreimage/ciimageprocessoroutput?language=objc)
    pub unsafe trait CIImageProcessorOutput {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(region)]
        unsafe fn region(&self) -> CGRect;

        #[method(bytesPerRow)]
        unsafe fn bytesPerRow(&self) -> usize;

        #[cfg(feature = "CIImage")]
        #[method(format)]
        unsafe fn format(&self) -> CIFormat;

        #[method(baseAddress)]
        unsafe fn baseAddress(&self) -> NonNull<c_void>;

        #[cfg(feature = "objc2-metal")]
        #[method_id(@__retain_semantics Other metalTexture)]
        unsafe fn metalTexture(&self) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        #[cfg(feature = "objc2-metal")]
        #[method_id(@__retain_semantics Other metalCommandBuffer)]
        unsafe fn metalCommandBuffer(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLCommandBuffer>>>;

        #[method(digest)]
        unsafe fn digest(&self) -> u64;
    }

    unsafe impl ProtocolType for dyn CIImageProcessorOutput {}
);
