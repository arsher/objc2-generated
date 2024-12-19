//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpspolygonbuffer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct MPSPolygonBuffer;
);

unsafe impl NSCoding for MPSPolygonBuffer {}

unsafe impl NSCopying for MPSPolygonBuffer {}

unsafe impl CopyingHelper for MPSPolygonBuffer {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MPSPolygonBuffer {}

unsafe impl NSSecureCoding for MPSPolygonBuffer {}

extern_methods!(
    unsafe impl MPSPolygonBuffer {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other polygonBuffer)]
        pub unsafe fn polygonBuffer() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Copy copyWithZone:)]
        pub unsafe fn copyWithZone(&self, zone: *mut NSZone) -> Retained<Self>;

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
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPSPolygonBuffer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
