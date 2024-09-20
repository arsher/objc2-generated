//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRasterizationRateSampleArray;

    unsafe impl ClassType for MTLRasterizationRateSampleArray {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for MTLRasterizationRateSampleArray {}

extern_methods!(
    unsafe impl MTLRasterizationRateSampleArray {
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(&self, index: NSUInteger) -> Retained<NSNumber>;

        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(&self, value: &NSNumber, index: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLRasterizationRateSampleArray {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRasterizationRateLayerDescriptor;

    unsafe impl ClassType for MTLRasterizationRateLayerDescriptor {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for MTLRasterizationRateLayerDescriptor {}

unsafe impl CopyingHelper for MTLRasterizationRateLayerDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLRasterizationRateLayerDescriptor {}

extern_methods!(
    unsafe impl MTLRasterizationRateLayerDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "MTLTypes")]
        #[method_id(@__retain_semantics Init initWithSampleCount:)]
        pub unsafe fn initWithSampleCount(
            this: Allocated<Self>,
            sample_count: MTLSize,
        ) -> Retained<Self>;

        #[cfg(feature = "MTLTypes")]
        #[method_id(@__retain_semantics Init initWithSampleCount:horizontal:vertical:)]
        pub unsafe fn initWithSampleCount_horizontal_vertical(
            this: Allocated<Self>,
            sample_count: MTLSize,
            horizontal: NonNull<c_float>,
            vertical: NonNull<c_float>,
        ) -> Retained<Self>;

        #[cfg(feature = "MTLTypes")]
        #[method(maxSampleCount)]
        pub unsafe fn maxSampleCount(&self) -> MTLSize;

        #[method(horizontalSampleStorage)]
        pub unsafe fn horizontalSampleStorage(&self) -> NonNull<c_float>;

        #[method(verticalSampleStorage)]
        pub unsafe fn verticalSampleStorage(&self) -> NonNull<c_float>;

        #[method_id(@__retain_semantics Other horizontal)]
        pub unsafe fn horizontal(&self) -> Retained<MTLRasterizationRateSampleArray>;

        #[method_id(@__retain_semantics Other vertical)]
        pub unsafe fn vertical(&self) -> Retained<MTLRasterizationRateSampleArray>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLRasterizationRateLayerDescriptor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    unsafe impl MTLRasterizationRateLayerDescriptor {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRasterizationRateLayerArray;

    unsafe impl ClassType for MTLRasterizationRateLayerArray {
        type Super = NSObject;
    }
);

unsafe impl NSObjectProtocol for MTLRasterizationRateLayerArray {}

extern_methods!(
    unsafe impl MTLRasterizationRateLayerArray {
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            layer_index: NSUInteger,
        ) -> Option<Retained<MTLRasterizationRateLayerDescriptor>>;

        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            layer: Option<&MTLRasterizationRateLayerDescriptor>,
            layer_index: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLRasterizationRateLayerArray {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRasterizationRateMapDescriptor;

    unsafe impl ClassType for MTLRasterizationRateMapDescriptor {
        type Super = NSObject;
    }
);

unsafe impl NSCopying for MTLRasterizationRateMapDescriptor {}

unsafe impl CopyingHelper for MTLRasterizationRateMapDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLRasterizationRateMapDescriptor {}

extern_methods!(
    unsafe impl MTLRasterizationRateMapDescriptor {
        #[cfg(feature = "MTLTypes")]
        #[method_id(@__retain_semantics Other rasterizationRateMapDescriptorWithScreenSize:)]
        pub unsafe fn rasterizationRateMapDescriptorWithScreenSize(
            screen_size: MTLSize,
        ) -> Retained<MTLRasterizationRateMapDescriptor>;

        #[cfg(feature = "MTLTypes")]
        #[method_id(@__retain_semantics Other rasterizationRateMapDescriptorWithScreenSize:layer:)]
        pub unsafe fn rasterizationRateMapDescriptorWithScreenSize_layer(
            screen_size: MTLSize,
            layer: &MTLRasterizationRateLayerDescriptor,
        ) -> Retained<MTLRasterizationRateMapDescriptor>;

        #[cfg(feature = "MTLTypes")]
        #[method_id(@__retain_semantics Other rasterizationRateMapDescriptorWithScreenSize:layerCount:layers:)]
        pub unsafe fn rasterizationRateMapDescriptorWithScreenSize_layerCount_layers(
            screen_size: MTLSize,
            layer_count: NSUInteger,
            layers: &mut Retained<MTLRasterizationRateLayerDescriptor>,
        ) -> Retained<MTLRasterizationRateMapDescriptor>;

        #[method_id(@__retain_semantics Other layerAtIndex:)]
        pub unsafe fn layerAtIndex(
            &self,
            layer_index: NSUInteger,
        ) -> Option<Retained<MTLRasterizationRateLayerDescriptor>>;

        #[method(setLayer:atIndex:)]
        pub unsafe fn setLayer_atIndex(
            &self,
            layer: Option<&MTLRasterizationRateLayerDescriptor>,
            layer_index: NSUInteger,
        );

        #[method_id(@__retain_semantics Other layers)]
        pub unsafe fn layers(&self) -> Retained<MTLRasterizationRateLayerArray>;

        #[cfg(feature = "MTLTypes")]
        #[method(screenSize)]
        pub unsafe fn screenSize(&self) -> MTLSize;

        #[cfg(feature = "MTLTypes")]
        #[method(setScreenSize:)]
        pub unsafe fn setScreenSize(&self, screen_size: MTLSize);

        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Option<Retained<NSString>>;

        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: Option<&NSString>);

        #[method(layerCount)]
        pub unsafe fn layerCount(&self) -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLRasterizationRateMapDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait MTLRasterizationRateMap: NSObjectProtocol {
        #[cfg(feature = "MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        unsafe fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        #[method_id(@__retain_semantics Other label)]
        unsafe fn label(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "MTLTypes")]
        #[method(screenSize)]
        unsafe fn screenSize(&self) -> MTLSize;

        #[cfg(feature = "MTLTypes")]
        #[method(physicalGranularity)]
        unsafe fn physicalGranularity(&self) -> MTLSize;

        #[method(layerCount)]
        unsafe fn layerCount(&self) -> NSUInteger;

        #[cfg(feature = "MTLDevice")]
        #[method(parameterBufferSizeAndAlign)]
        unsafe fn parameterBufferSizeAndAlign(&self) -> MTLSizeAndAlign;

        #[cfg(all(
            feature = "MTLAllocation",
            feature = "MTLBuffer",
            feature = "MTLResource"
        ))]
        #[method(copyParameterDataToBuffer:offset:)]
        unsafe fn copyParameterDataToBuffer_offset(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
        );

        #[cfg(feature = "MTLTypes")]
        #[method(physicalSizeForLayer:)]
        unsafe fn physicalSizeForLayer(&self, layer_index: NSUInteger) -> MTLSize;

        #[cfg(feature = "MTLTypes")]
        #[method(mapScreenToPhysicalCoordinates:forLayer:)]
        unsafe fn mapScreenToPhysicalCoordinates_forLayer(
            &self,
            screen_coordinates: MTLCoordinate2D,
            layer_index: NSUInteger,
        ) -> MTLCoordinate2D;

        #[cfg(feature = "MTLTypes")]
        #[method(mapPhysicalToScreenCoordinates:forLayer:)]
        unsafe fn mapPhysicalToScreenCoordinates_forLayer(
            &self,
            physical_coordinates: MTLCoordinate2D,
            layer_index: NSUInteger,
        ) -> MTLCoordinate2D;
    }

    unsafe impl ProtocolType for dyn MTLRasterizationRateMap {}
);
