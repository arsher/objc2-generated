//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
use objc2_metal::*;

use crate::*;

/// MTKModelErrors
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metalkit/mtkmodelerror?language=objc)
// NS_TYPED_ENUM
pub type MTKModelError = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metalkit/mtkmodelerrordomain?language=objc)
    pub static MTKModelErrorDomain: &'static MTKModelError;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metalkit/mtkmodelerrorkey?language=objc)
    pub static MTKModelErrorKey: &'static MTKModelError;
}

extern_class!(
    /// Allocator passed to MDLAsset init method to load vertex and index data directly into Metal buffers.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalkit/mtkmeshbufferallocator?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTKMeshBufferAllocator;
);

unsafe impl NSObjectProtocol for MTKMeshBufferAllocator {}

extern_methods!(
    unsafe impl MTKMeshBufferAllocator {
        /// Must initialize with device
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Initialize the allocator with a device to be used to create buffers.
        ///
        /// The designated initializer for this class.
        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Allocated<Self>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<Self>;

        /// Device used to create buffers.
        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTKMeshBufferAllocator {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// Mesh buffer created by MTKMeshBufferAllocator when Model I/O needs to memory for vertex or index data backing.
    ///
    /// Memory backing these buffer are Metal buffers.  Model I/O will load index and vertex data from from a model asset directly in to the Metal buffer.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalkit/mtkmeshbuffer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTKMeshBuffer;
);

unsafe impl NSCopying for MTKMeshBuffer {}

unsafe impl CopyingHelper for MTKMeshBuffer {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTKMeshBuffer {}

extern_methods!(
    unsafe impl MTKMeshBuffer {
        /// Only an MTKMeshBufferAllocator object can initilize a MTKMeshBuffer object
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Size in bytes of the buffer allocation.
        #[method(length)]
        pub unsafe fn length(&self) -> NSUInteger;

        /// Allocator object used to create this buffer.
        ///
        /// This allcoator is stored so that it can be used by Model I/O for copy and relayout operations (such as when a new vertex descriptor is applied to a vertex buffer).
        #[method_id(@__retain_semantics Other allocator)]
        pub unsafe fn allocator(&self) -> Retained<MTKMeshBufferAllocator>;

        /// Metal Buffer backing vertex/index data.
        ///
        /// Many MTKMeshBuffers may reference the same buffer, but each with it's own offset.  (i.e. Many MTKMeshBuffers may be suballocated from a single buffer)
        #[method_id(@__retain_semantics Other buffer)]
        pub unsafe fn buffer(&self) -> Retained<ProtocolObject<dyn MTLBuffer>>;

        /// Byte offset of the data within the metal buffer.
        #[method(offset)]
        pub unsafe fn offset(&self) -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTKMeshBuffer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// A segment of a mesh and properties to render the segement.
    ///
    /// Container for data that can be rendered in a single draw call. 1:1 mapping to MDLSubmesh.  Each submesh contains an index Buffer with which the parents mesh data can be rendered.  Actual vertex data resides in the submesh's parent MTKMesh object.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalkit/mtksubmesh?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTKSubmesh;
);

unsafe impl NSObjectProtocol for MTKSubmesh {}

extern_methods!(
    unsafe impl MTKSubmesh {
        /// Applicatiohs must not explicity allocate or initialize.  Must initialize as part of MTKMesh object.
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Metal primitive type with which to draw this object.
        ///
        /// Value to use for primitiveType parameter in a [MTLRenderCommandEncoder drawIndexedPrimitives] call.
        #[method(primitiveType)]
        pub unsafe fn primitiveType(&self) -> MTLPrimitiveType;

        /// Metal index type of data in indexBuffer.
        ///
        /// Value to use for indexType parameter in a [MTLRenderCommandEncoder drawIndexedPrimitives] call.
        #[method(indexType)]
        pub unsafe fn indexType(&self) -> MTLIndexType;

        /// IndexBuffer (including indexCount) to render the object.
        ///
        /// The MTLBuffer to use for indexBuffer parameter in a [MTLRenderCommandEncoder drawIndexedPrimitives] call.
        #[method_id(@__retain_semantics Other indexBuffer)]
        pub unsafe fn indexBuffer(&self) -> Retained<MTKMeshBuffer>;

        /// Number of indicies in indexBuffer.
        ///
        /// Value to use for indexCount parameter in a [MTLRenderCommandEncoder drawIndexedPrimitives] call.
        #[method(indexCount)]
        pub unsafe fn indexCount(&self) -> NSUInteger;

        /// Parent MTKMesh object containing vertex data of this object.
        ///
        /// The buffer of this parent mesh should be set in the encoder before a drawIndexedPrimitives call is made.
        #[method_id(@__retain_semantics Other mesh)]
        pub unsafe fn mesh(&self) -> Option<Retained<MTKMesh>>;

        /// Name from the original MDLSubmesh object.
        ///
        /// Although not directly used by this object, the application may use this to identify the submesh in the renderer/scene/world.
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        /// Setter for [`name`][Self::name].
        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTKSubmesh {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// Container for vertex data of a mesh and submeshes to render it.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metalkit/mtkmesh?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTKMesh;
);

unsafe impl NSObjectProtocol for MTKMesh {}

extern_methods!(
    unsafe impl MTKMesh {
        /// Cannot use default init.  Must initialize with mesh and metal device.
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Array of buffers in which mesh vertex data resides.
        ///
        /// This is filled with mesh buffer objects using the layout described by the vertexDescriptor property.  Elements in this array can be [NSNull null] if the vertexDescriptor does not specify elements for buffer for the given index
        #[method_id(@__retain_semantics Other vertexBuffers)]
        pub unsafe fn vertexBuffers(&self) -> Retained<NSArray<MTKMeshBuffer>>;

        /// Submeshes containing index buffers to rendering mesh vertices.
        #[method_id(@__retain_semantics Other submeshes)]
        pub unsafe fn submeshes(&self) -> Retained<NSArray<MTKSubmesh>>;

        /// Number of vertices in the vertexBuffers.
        #[method(vertexCount)]
        pub unsafe fn vertexCount(&self) -> NSUInteger;

        /// Name of the mesh copies from the originating Model I/O mesh.
        ///
        /// Can be used by the app to identify the mesh in its scene/world/renderer etc.
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        /// Setter for [`name`][Self::name].
        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTKMesh {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
