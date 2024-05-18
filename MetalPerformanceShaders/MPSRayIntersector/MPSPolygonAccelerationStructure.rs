//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpspolygontype?language=objc)
// NS_ENUM
#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSPolygonType(pub NSUInteger);
impl MPSPolygonType {
    #[deprecated]
    #[doc(alias = "MPSPolygonTypeTriangle")]
    pub const Triangle: Self = Self(0);
    #[deprecated]
    #[doc(alias = "MPSPolygonTypeQuadrilateral")]
    pub const Quadrilateral: Self = Self(1);
}

unsafe impl Encode for MPSPolygonType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MPSPolygonType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpspolygonaccelerationstructure?language=objc)
    #[unsafe(super(MPSAccelerationStructure, MPSKernel, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MPSAccelerationStructure", feature = "MPSKernel"))]
    #[deprecated]
    pub struct MPSPolygonAccelerationStructure;
);

#[cfg(all(feature = "MPSAccelerationStructure", feature = "MPSKernel"))]
unsafe impl NSCoding for MPSPolygonAccelerationStructure {}

#[cfg(all(feature = "MPSAccelerationStructure", feature = "MPSKernel"))]
unsafe impl NSCopying for MPSPolygonAccelerationStructure {}

#[cfg(all(feature = "MPSAccelerationStructure", feature = "MPSKernel"))]
unsafe impl CopyingHelper for MPSPolygonAccelerationStructure {
    type Result = Self;
}

#[cfg(all(feature = "MPSAccelerationStructure", feature = "MPSKernel"))]
unsafe impl NSObjectProtocol for MPSPolygonAccelerationStructure {}

#[cfg(all(feature = "MPSAccelerationStructure", feature = "MPSKernel"))]
unsafe impl NSSecureCoding for MPSPolygonAccelerationStructure {}

extern_methods!(
    #[cfg(all(feature = "MPSAccelerationStructure", feature = "MPSKernel"))]
    unsafe impl MPSPolygonAccelerationStructure {
        #[deprecated]
        #[method(polygonType)]
        pub unsafe fn polygonType(&self) -> MPSPolygonType;

        #[deprecated]
        #[method(setPolygonType:)]
        pub unsafe fn setPolygonType(&self, polygon_type: MPSPolygonType);

        #[deprecated]
        #[method(vertexStride)]
        pub unsafe fn vertexStride(&self) -> NSUInteger;

        #[deprecated]
        #[method(setVertexStride:)]
        pub unsafe fn setVertexStride(&self, vertex_stride: NSUInteger);

        #[cfg(feature = "MPSCoreTypes")]
        #[deprecated]
        #[method(indexType)]
        pub unsafe fn indexType(&self) -> MPSDataType;

        #[cfg(feature = "MPSCoreTypes")]
        #[deprecated]
        #[method(setIndexType:)]
        pub unsafe fn setIndexType(&self, index_type: MPSDataType);

        #[deprecated]
        #[method_id(@__retain_semantics Other vertexBuffer)]
        pub unsafe fn vertexBuffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        #[deprecated]
        #[method(setVertexBuffer:)]
        pub unsafe fn setVertexBuffer(&self, vertex_buffer: Option<&ProtocolObject<dyn MTLBuffer>>);

        #[deprecated]
        #[method(vertexBufferOffset)]
        pub unsafe fn vertexBufferOffset(&self) -> NSUInteger;

        #[deprecated]
        #[method(setVertexBufferOffset:)]
        pub unsafe fn setVertexBufferOffset(&self, vertex_buffer_offset: NSUInteger);

        #[deprecated]
        #[method_id(@__retain_semantics Other indexBuffer)]
        pub unsafe fn indexBuffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        #[deprecated]
        #[method(setIndexBuffer:)]
        pub unsafe fn setIndexBuffer(&self, index_buffer: Option<&ProtocolObject<dyn MTLBuffer>>);

        #[deprecated]
        #[method(indexBufferOffset)]
        pub unsafe fn indexBufferOffset(&self) -> NSUInteger;

        #[deprecated]
        #[method(setIndexBufferOffset:)]
        pub unsafe fn setIndexBufferOffset(&self, index_buffer_offset: NSUInteger);

        #[deprecated]
        #[method_id(@__retain_semantics Other maskBuffer)]
        pub unsafe fn maskBuffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        #[deprecated]
        #[method(setMaskBuffer:)]
        pub unsafe fn setMaskBuffer(&self, mask_buffer: Option<&ProtocolObject<dyn MTLBuffer>>);

        #[deprecated]
        #[method(maskBufferOffset)]
        pub unsafe fn maskBufferOffset(&self) -> NSUInteger;

        #[deprecated]
        #[method(setMaskBufferOffset:)]
        pub unsafe fn setMaskBufferOffset(&self, mask_buffer_offset: NSUInteger);

        #[deprecated]
        #[method(polygonCount)]
        pub unsafe fn polygonCount(&self) -> NSUInteger;

        #[deprecated]
        #[method(setPolygonCount:)]
        pub unsafe fn setPolygonCount(&self, polygon_count: NSUInteger);

        #[cfg(feature = "MPSPolygonBuffer")]
        #[deprecated]
        #[method_id(@__retain_semantics Other polygonBuffers)]
        pub unsafe fn polygonBuffers(&self) -> Option<Retained<NSArray<MPSPolygonBuffer>>>;

        #[cfg(feature = "MPSPolygonBuffer")]
        #[deprecated]
        #[method(setPolygonBuffers:)]
        pub unsafe fn setPolygonBuffers(&self, polygon_buffers: Option<&NSArray<MPSPolygonBuffer>>);
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSAccelerationStructure`
    #[cfg(all(feature = "MPSAccelerationStructure", feature = "MPSKernel"))]
    unsafe impl MPSPolygonAccelerationStructure {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MPSAccelerationStructureGroup")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithGroup:)]
        pub unsafe fn initWithGroup(
            this: Allocated<Self>,
            group: &MPSAccelerationStructureGroup,
        ) -> Retained<Self>;

        #[cfg(feature = "MPSAccelerationStructureGroup")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithCoder:group:)]
        pub unsafe fn initWithCoder_group(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            group: &MPSAccelerationStructureGroup,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `MPSKernel`
    #[cfg(all(feature = "MPSAccelerationStructure", feature = "MPSKernel"))]
    unsafe impl MPSPolygonAccelerationStructure {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MPSAccelerationStructure", feature = "MPSKernel"))]
    unsafe impl MPSPolygonAccelerationStructure {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
