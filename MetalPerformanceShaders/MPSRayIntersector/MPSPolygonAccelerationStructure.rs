//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
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
    /// Triangles with three vertices
    #[deprecated]
    #[doc(alias = "MPSPolygonTypeTriangle")]
    pub const Triangle: Self = Self(0);
    /// Quadrilaterals with four vertices
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
    /// An acceleration structure built over polygonal shapes
    ///
    ///
    /// See MPSAccelerationStructure for more information
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpspolygonaccelerationstructure?language=objc)
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
        /// The type of polygon. Defaults to MPSPolygonTypeTriangle. Changes to this property require
        /// rebuilding the acceleration structure.
        #[deprecated]
        #[method(polygonType)]
        pub unsafe fn polygonType(&self) -> MPSPolygonType;

        /// Setter for [`polygonType`][Self::polygonType].
        #[deprecated]
        #[method(setPolygonType:)]
        pub unsafe fn setPolygonType(&self, polygon_type: MPSPolygonType);

        /// Offset, in bytes, between consecutive vertices in the vertex buffer. Defaults to 0 bytes,
        /// indicating that the vertices are packed according to the natural alignment of the vector_float3
        /// type: 16 bytes.
        ///
        ///
        /// This can be used to skip past any additional per-vertex data which may be stored
        /// alongside the position such as the vertex normal and texture coordinates. Must be a multiple of
        /// 4 bytes, and must be at least 12 bytes. Changes to this property require rebuilding the
        /// acceleration structure.
        #[deprecated]
        #[method(vertexStride)]
        pub unsafe fn vertexStride(&self) -> NSUInteger;

        /// Setter for [`vertexStride`][Self::vertexStride].
        #[deprecated]
        #[method(setVertexStride:)]
        pub unsafe fn setVertexStride(&self, vertex_stride: NSUInteger);

        #[cfg(feature = "MPSCoreTypes")]
        /// Index type. Defaults to MPSDataTypeUInt32. Only MPSDataTypeUInt16 and MPSDataTypeUInt32
        /// are supported.
        #[deprecated]
        #[method(indexType)]
        pub unsafe fn indexType(&self) -> MPSDataType;

        #[cfg(feature = "MPSCoreTypes")]
        /// Setter for [`indexType`][Self::indexType].
        #[deprecated]
        #[method(setIndexType:)]
        pub unsafe fn setIndexType(&self, index_type: MPSDataType);

        /// Vertex buffer containing vertex data encoded as three 32 bit floats per vertex. Note
        /// that by default each vertex is aligned to the alignment of the vector_float3 type: 16 bytes.
        /// This can be changed using the vertexStride property. A vertex buffer must be provided before
        /// the acceleration structure is built.
        ///
        /// When using triangle polygons, degenerate (zero or negative area) triangles are ignored
        /// during acceleration structure construction. This can be used to pad triangle indices if needed.
        ///
        /// Quadrilateral polygons are internally treated as two triangles. If the quadrilateral has
        /// vertices v0, v1, v2, and v3, the two triangles will have vertices v0, v1, v2 and v0, v2, v3.
        /// A quadrilateral may be used to represent a triangle by repeating the last vertex. If the first
        /// triangle is degenerate (zero or negative area), the entire quadrilateral will be ignored. This
        /// can be used to pad quadrilateral indices if needed. All four vertices of a quadrilateral must
        /// be coplanar and the quadrilateral must be convex.
        ///
        /// This is an alias for polygonBuffers[0].vertexBuffer. There must be exactly one polygon buffer
        /// to use this property, or the polygonBuffers property must be nil, in which case an
        /// MPSPolygonBuffer will be created automatically.
        #[deprecated]
        #[method_id(@__retain_semantics Other vertexBuffer)]
        pub unsafe fn vertexBuffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Setter for [`vertexBuffer`][Self::vertexBuffer].
        #[deprecated]
        #[method(setVertexBuffer:)]
        pub unsafe fn setVertexBuffer(&self, vertex_buffer: Option<&ProtocolObject<dyn MTLBuffer>>);

        /// Offset, in bytes, into the vertex buffer. Defaults to 0 bytes. Must be aligned to 4
        /// bytes.
        ///
        /// This is an alias for polygonBuffers[0].vertexBufferOffset. There must be exactly one polygon
        /// buffer to use this property, or the polygonBuffers property must be nil, in which case an
        /// MPSPolygonBuffer will be created automatically.
        #[deprecated]
        #[method(vertexBufferOffset)]
        pub unsafe fn vertexBufferOffset(&self) -> NSUInteger;

        /// Setter for [`vertexBufferOffset`][Self::vertexBufferOffset].
        #[deprecated]
        #[method(setVertexBufferOffset:)]
        pub unsafe fn setVertexBufferOffset(&self, vertex_buffer_offset: NSUInteger);

        /// Index buffer containing index data. Each index references a vertex in the vertex buffer.
        /// May be nil.
        ///
        /// This is an alias for polygonBuffers[0].indexBuffer. There must be exactly one polygon buffer
        /// to use this property, or the polygonBuffers property must be nil, in which case an
        /// MPSPolygonBuffer will be created automatically.
        #[deprecated]
        #[method_id(@__retain_semantics Other indexBuffer)]
        pub unsafe fn indexBuffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Setter for [`indexBuffer`][Self::indexBuffer].
        #[deprecated]
        #[method(setIndexBuffer:)]
        pub unsafe fn setIndexBuffer(&self, index_buffer: Option<&ProtocolObject<dyn MTLBuffer>>);

        /// Offset, in bytes, into the index buffer. Defaults to 0 bytes. Must be aligned to a
        /// multiple of the index type. Changes to this property require rebuilding the acceleration
        /// structure.
        ///
        /// This is an alias for polygonBuffers[0].indexBufferOffset. There must be exactly one polygon
        /// buffer to use this property, or the polygonBuffers property must be nil, in which case an
        /// MPSPolygonBuffer will be created automatically.
        #[deprecated]
        #[method(indexBufferOffset)]
        pub unsafe fn indexBufferOffset(&self) -> NSUInteger;

        /// Setter for [`indexBufferOffset`][Self::indexBufferOffset].
        #[deprecated]
        #[method(setIndexBufferOffset:)]
        pub unsafe fn setIndexBufferOffset(&self, index_buffer_offset: NSUInteger);

        /// Mask buffer containing one uint32_t mask per polygon. May be nil. Otherwise, the mask
        /// type must be specified on the MPSRayIntersector with which it is used.
        ///
        /// This is an alias for polygonBuffers[0].maskBuffer. There must be exactly one polygon buffer
        /// to use this property, or the polygonBuffers property must be nil, in which case an
        /// MPSPolygonBuffer will be created automatically.
        #[deprecated]
        #[method_id(@__retain_semantics Other maskBuffer)]
        pub unsafe fn maskBuffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Setter for [`maskBuffer`][Self::maskBuffer].
        #[deprecated]
        #[method(setMaskBuffer:)]
        pub unsafe fn setMaskBuffer(&self, mask_buffer: Option<&ProtocolObject<dyn MTLBuffer>>);

        /// Offset, in bytes, into the mask buffer. Defaults to 0 bytes. Must be aligned to 4 bytes.
        ///
        /// This is an alias for polygonBuffers[0].maskBufferOffset. There must be exactly one polygon
        /// buffer to use this property, or the polygonBuffers property must be nil, in which case an
        /// MPSPolygonBuffer will be created automatically.
        #[deprecated]
        #[method(maskBufferOffset)]
        pub unsafe fn maskBufferOffset(&self) -> NSUInteger;

        /// Setter for [`maskBufferOffset`][Self::maskBufferOffset].
        #[deprecated]
        #[method(setMaskBufferOffset:)]
        pub unsafe fn setMaskBufferOffset(&self, mask_buffer_offset: NSUInteger);

        /// Number of polygons. Changes to this property require rebuilding the acceleration
        /// structure.
        ///
        /// This is an alias for polygonBuffers[0].polygonCount. There must be exactly one polygon buffer
        /// to use this property, or the polygonBuffers property must be nil, in which case an
        /// MPSPolygonBuffer will be created automatically.
        #[deprecated]
        #[method(polygonCount)]
        pub unsafe fn polygonCount(&self) -> NSUInteger;

        /// Setter for [`polygonCount`][Self::polygonCount].
        #[deprecated]
        #[method(setPolygonCount:)]
        pub unsafe fn setPolygonCount(&self, polygon_count: NSUInteger);

        #[cfg(feature = "MPSPolygonBuffer")]
        /// Array of polygon buffers. Each buffer contains a vertex buffer and optional index and
        /// mask buffer for an array of polygons. Changing the length of this array requires rebuilding the
        /// acceleration structure.
        ///
        /// Using more than one MPSPolygonBuffer will reduce performance. It is better to concatenate
        /// these buffers into a single vertex buffer, index buffer, and mask buffer and use a single
        /// MPSPolygonBuffer if possible. This also applies when using an MPSInstanceAccelerationStructure:
        /// each instance or subclass of MPSPolygonAccelerationStructure in an instance hierarchy should use
        /// the same vertex buffer, index buffer, and mask buffer, although each acceleration structure
        /// may use different offsets into these buffers. This allows for the vertex, index, and mask
        /// buffers to be bound directly instead of indirectly through an argument buffer.
        ///
        /// There must be at least one MPSPolygonBuffer. On argument buffer tier 1 devices, there must be
        /// be exactly one MPSPolygonBuffer. Use the argumentBuffersSupport property of the MTLDevice to
        /// check for support.
        #[deprecated]
        #[method_id(@__retain_semantics Other polygonBuffers)]
        pub unsafe fn polygonBuffers(&self) -> Option<Retained<NSArray<MPSPolygonBuffer>>>;

        #[cfg(feature = "MPSPolygonBuffer")]
        /// Setter for [`polygonBuffers`][Self::polygonBuffers].
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

        /// Initialize the acceleration structure with a Metal device
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        /// Initialize the acceleration structure with an NSCoder and a Metal device. Buffer
        /// properties such as the vertex buffer, instance buffer, etc. are set to nil. Encode and decode
        /// these buffers along with the acceleration structure instead.
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithCoder:device:)]
        pub unsafe fn initWithCoder_device(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "MPSAccelerationStructureGroup")]
        /// Initialize the acceleration structure with an acceleration structure group, if the
        /// acceleration structure will be used in an instance hierarchy.
        ///
        ///
        /// The Metal device is determined from the acceleration structure group. All
        /// acceleration structures in the instance hierarchy must share the same group.
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithGroup:)]
        pub unsafe fn initWithGroup(
            this: Allocated<Self>,
            group: &MPSAccelerationStructureGroup,
        ) -> Retained<Self>;

        #[cfg(feature = "MPSAccelerationStructureGroup")]
        /// Initialize the acceleration structure with an NSCoder and an acceleration structure
        /// group, if the acceleration structure will be used in an instance hierarchy. All acceleration
        /// structures in the instance hierarchy must share the same group. Buffer properties such as the
        /// vertex buffer, instance buffer, etc. are set to nil. Encode and decode these buffers along with
        /// the acceleration structure instead.
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
        /// Called by NSCoder to decode MPSKernels
        ///
        /// This isn't the right interface to decode a MPSKernel, but
        /// it is the one that NSCoder uses. To enable your NSCoder
        /// (e.g. NSKeyedUnarchiver) to set which device to use
        /// extend the object to adopt the MPSDeviceProvider
        /// protocol. Otherwise, the Metal system default device
        /// will be used.
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
