//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::Metal::*;
use crate::MetalKit::*;

typed_enum!(
    pub type MTKModelError = NSString;
);

extern_static!(MTKModelErrorDomain: &'static MTKModelError);

extern_static!(MTKModelErrorKey: &'static MTKModelError);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetalKit_MTKMeshBufferAllocator")]
    pub struct MTKMeshBufferAllocator;

    #[cfg(feature = "MetalKit_MTKMeshBufferAllocator")]
    unsafe impl ClassType for MTKMeshBufferAllocator {
        type Super = NSObject;
    }
);

#[cfg(feature = "MetalKit_MTKMeshBufferAllocator")]
unsafe impl MDLMeshBufferAllocator for MTKMeshBufferAllocator {}

#[cfg(feature = "MetalKit_MTKMeshBufferAllocator")]
unsafe impl NSObjectProtocol for MTKMeshBufferAllocator {}

extern_methods!(
    #[cfg(feature = "MetalKit_MTKMeshBufferAllocator")]
    unsafe impl MTKMeshBufferAllocator {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithDevice:)]
        pub unsafe fn initWithDevice(
            this: Option<Allocated<Self>>,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetalKit_MTKMeshBuffer")]
    pub struct MTKMeshBuffer;

    #[cfg(feature = "MetalKit_MTKMeshBuffer")]
    unsafe impl ClassType for MTKMeshBuffer {
        type Super = NSObject;
    }
);

#[cfg(feature = "MetalKit_MTKMeshBuffer")]
unsafe impl MDLMeshBuffer for MTKMeshBuffer {}

#[cfg(feature = "MetalKit_MTKMeshBuffer")]
unsafe impl MDLNamed for MTKMeshBuffer {}

#[cfg(feature = "MetalKit_MTKMeshBuffer")]
unsafe impl NSObjectProtocol for MTKMeshBuffer {}

extern_methods!(
    #[cfg(feature = "MetalKit_MTKMeshBuffer")]
    unsafe impl MTKMeshBuffer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method(length)]
        pub unsafe fn length(&self) -> NSUInteger;

        #[cfg(feature = "MetalKit_MTKMeshBufferAllocator")]
        #[method_id(@__retain_semantics Other allocator)]
        pub unsafe fn allocator(&self) -> Id<MTKMeshBufferAllocator, Shared>;

        #[method_id(@__retain_semantics Other buffer)]
        pub unsafe fn buffer(&self) -> Id<ProtocolObject<dyn MTLBuffer>, Shared>;

        #[method(offset)]
        pub unsafe fn offset(&self) -> NSUInteger;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetalKit_MTKSubmesh")]
    pub struct MTKSubmesh;

    #[cfg(feature = "MetalKit_MTKSubmesh")]
    unsafe impl ClassType for MTKSubmesh {
        type Super = NSObject;
    }
);

#[cfg(feature = "MetalKit_MTKSubmesh")]
unsafe impl NSObjectProtocol for MTKSubmesh {}

extern_methods!(
    #[cfg(feature = "MetalKit_MTKSubmesh")]
    unsafe impl MTKSubmesh {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method(primitiveType)]
        pub unsafe fn primitiveType(&self) -> MTLPrimitiveType;

        #[method(indexType)]
        pub unsafe fn indexType(&self) -> MTLIndexType;

        #[cfg(feature = "MetalKit_MTKMeshBuffer")]
        #[method_id(@__retain_semantics Other indexBuffer)]
        pub unsafe fn indexBuffer(&self) -> Id<MTKMeshBuffer, Shared>;

        #[method(indexCount)]
        pub unsafe fn indexCount(&self) -> NSUInteger;

        #[cfg(feature = "MetalKit_MTKMesh")]
        #[method_id(@__retain_semantics Other mesh)]
        pub unsafe fn mesh(&self) -> Option<Id<MTKMesh, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetalKit_MTKMesh")]
    pub struct MTKMesh;

    #[cfg(feature = "MetalKit_MTKMesh")]
    unsafe impl ClassType for MTKMesh {
        type Super = NSObject;
    }
);

#[cfg(feature = "MetalKit_MTKMesh")]
unsafe impl NSObjectProtocol for MTKMesh {}

extern_methods!(
    #[cfg(feature = "MetalKit_MTKMesh")]
    unsafe impl MTKMesh {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(all(feature = "Foundation_NSError", feature = "ModelIO_MDLMesh"))]
        #[method_id(@__retain_semantics Init initWithMesh:device:error:_)]
        pub unsafe fn initWithMesh_device_error(
            this: Option<Allocated<Self>>,
            mesh: &MDLMesh,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "ModelIO_MDLAsset",
            feature = "ModelIO_MDLMesh"
        ))]
        #[method_id(@__retain_semantics New newMeshesFromAsset:device:sourceMeshes:error:_)]
        pub unsafe fn newMeshesFromAsset_device_sourceMeshes_error(
            asset: &MDLAsset,
            device: &ProtocolObject<dyn MTLDevice>,
            source_meshes: *mut *mut NSArray<MDLMesh>,
        ) -> Result<Id<NSArray<MTKMesh>, Shared>, Id<NSError, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MetalKit_MTKMeshBuffer"))]
        #[method_id(@__retain_semantics Other vertexBuffers)]
        pub unsafe fn vertexBuffers(&self) -> Id<NSArray<MTKMeshBuffer>, Shared>;

        #[cfg(feature = "ModelIO_MDLVertexDescriptor")]
        #[method_id(@__retain_semantics Other vertexDescriptor)]
        pub unsafe fn vertexDescriptor(&self) -> Id<MDLVertexDescriptor, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MetalKit_MTKSubmesh"))]
        #[method_id(@__retain_semantics Other submeshes)]
        pub unsafe fn submeshes(&self) -> Id<NSArray<MTKSubmesh>, Shared>;

        #[method(vertexCount)]
        pub unsafe fn vertexCount(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);
    }
);

extern_fn!(
    #[cfg(all(
        feature = "Metal_MTLVertexDescriptor",
        feature = "ModelIO_MDLVertexDescriptor"
    ))]
    pub unsafe fn MTKModelIOVertexDescriptorFromMetal(
        metal_descriptor: &MTLVertexDescriptor,
    ) -> NonNull<MDLVertexDescriptor>;
);

extern_fn!(
    #[cfg(all(
        feature = "Foundation_NSError",
        feature = "Metal_MTLVertexDescriptor",
        feature = "ModelIO_MDLVertexDescriptor"
    ))]
    pub unsafe fn MTKModelIOVertexDescriptorFromMetalWithError(
        metal_descriptor: &MTLVertexDescriptor,
        error: *mut *mut NSError,
    ) -> NonNull<MDLVertexDescriptor>;
);

extern_fn!(
    #[cfg(all(
        feature = "Metal_MTLVertexDescriptor",
        feature = "ModelIO_MDLVertexDescriptor"
    ))]
    pub unsafe fn MTKMetalVertexDescriptorFromModelIO(
        model_io_descriptor: &MDLVertexDescriptor,
    ) -> *mut MTLVertexDescriptor;
);

extern_fn!(
    #[cfg(all(
        feature = "Foundation_NSError",
        feature = "Metal_MTLVertexDescriptor",
        feature = "ModelIO_MDLVertexDescriptor"
    ))]
    pub unsafe fn MTKMetalVertexDescriptorFromModelIOWithError(
        model_io_descriptor: &MDLVertexDescriptor,
        error: *mut *mut NSError,
    ) -> *mut MTLVertexDescriptor;
);
