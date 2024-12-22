//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// Signature defining what data is provided to an intersection function. The signature
/// must match across the shading language declaration of the intersection function table,
/// intersection functions in the table, and the intersector using the table.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlintersectionfunctionsignature?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLIntersectionFunctionSignature(pub NSUInteger);
bitflags::bitflags! {
    impl MTLIntersectionFunctionSignature: NSUInteger {
/// No signature
        #[doc(alias = "MTLIntersectionFunctionSignatureNone")]
        const None = 0;
/// The intersection functions can read the built-in instance_id as described in
/// the Metal Shading Language Guide.
        #[doc(alias = "MTLIntersectionFunctionSignatureInstancing")]
        const Instancing = 1<<0;
/// The triangle intersection functions can read the built-in barycentric_coord
/// and front_facing as described in the Metal Shading Language Guide.
        #[doc(alias = "MTLIntersectionFunctionSignatureTriangleData")]
        const TriangleData = 1<<1;
/// The intersection functions can query world_space_origin and
/// world_space_direction as described in the Metal Shading Language Guide.
        #[doc(alias = "MTLIntersectionFunctionSignatureWorldSpaceData")]
        const WorldSpaceData = 1<<2;
/// The intersection functions may be called from intersectors using the
/// instance_motion intersection tag as described in the Metal Shading Language Guide.
        #[doc(alias = "MTLIntersectionFunctionSignatureInstanceMotion")]
        const InstanceMotion = 1<<3;
/// The intersection functions can query time, motion_start_time,
/// motion_end_time and key_frame_count as described in the Metal Shading Language Guide.
        #[doc(alias = "MTLIntersectionFunctionSignaturePrimitiveMotion")]
        const PrimitiveMotion = 1<<4;
/// The intersection functions may be called from intersectors using the
/// extended_limits intersection tag as described in the Metal Shading Language Guide.
        #[doc(alias = "MTLIntersectionFunctionSignatureExtendedLimits")]
        const ExtendedLimits = 1<<5;
/// The intersection functions may be called from intersectors using the
/// max_levels intersection tag as described in the Metal Shading Language Guide.
        #[doc(alias = "MTLIntersectionFunctionSignatureMaxLevels")]
        const MaxLevels = 1<<6;
/// The curve intersection functions can read the built-in curve_parameter
/// as described in the Metal Shading Language Guide.
        #[doc(alias = "MTLIntersectionFunctionSignatureCurveData")]
        const CurveData = 1<<7;
    }
}

unsafe impl Encode for MTLIntersectionFunctionSignature {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLIntersectionFunctionSignature {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontabledescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLIntersectionFunctionTableDescriptor;
);

unsafe impl NSCopying for MTLIntersectionFunctionTableDescriptor {}

unsafe impl CopyingHelper for MTLIntersectionFunctionTableDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLIntersectionFunctionTableDescriptor {}

extern_methods!(
    unsafe impl MTLIntersectionFunctionTableDescriptor {
        /// Create an autoreleased intersection function table descriptor
        #[method_id(@__retain_semantics Other intersectionFunctionTableDescriptor)]
        pub unsafe fn intersectionFunctionTableDescriptor(
        ) -> Retained<MTLIntersectionFunctionTableDescriptor>;

        /// The number of functions in the table.
        #[method(functionCount)]
        pub unsafe fn functionCount(&self) -> NSUInteger;

        /// Setter for [`functionCount`][Self::functionCount].
        #[method(setFunctionCount:)]
        pub fn setFunctionCount(&self, function_count: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLIntersectionFunctionTableDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for MTLIntersectionFunctionTableDescriptor {
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlintersectionfunctiontable?language=objc)
    #[cfg(all(feature = "MTLAllocation", feature = "MTLResource"))]
    pub unsafe trait MTLIntersectionFunctionTable: MTLResource {
        #[cfg(feature = "MTLBuffer")]
        #[method(setBuffer:offset:atIndex:)]
        unsafe fn setBuffer_offset_atIndex(
            &self,
            buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: NSUInteger,
            index: NSUInteger,
        );

        #[cfg(feature = "MTLBuffer")]
        #[method(setBuffers:offsets:withRange:)]
        unsafe fn setBuffers_offsets_withRange(
            &self,
            buffers: NonNull<*const ProtocolObject<dyn MTLBuffer>>,
            offsets: NonNull<NSUInteger>,
            range: NSRange,
        );

        #[cfg(feature = "MTLTypes")]
        /// Handle of the GPU resource suitable for storing in an Argument Buffer
        #[method(gpuResourceID)]
        unsafe fn gpuResourceID(&self) -> MTLResourceID;

        #[cfg(feature = "MTLFunctionHandle")]
        #[method(setFunction:atIndex:)]
        fn setFunction_atIndex(
            &self,
            function: Option<&ProtocolObject<dyn MTLFunctionHandle>>,
            index: NSUInteger,
        );

        #[cfg(feature = "MTLFunctionHandle")]
        #[method(setFunctions:withRange:)]
        unsafe fn setFunctions_withRange(
            &self,
            functions: NonNull<*const ProtocolObject<dyn MTLFunctionHandle>>,
            range: NSRange,
        );

        #[method(setOpaqueTriangleIntersectionFunctionWithSignature:atIndex:)]
        unsafe fn setOpaqueTriangleIntersectionFunctionWithSignature_atIndex(
            &self,
            signature: MTLIntersectionFunctionSignature,
            index: NSUInteger,
        );

        #[method(setOpaqueTriangleIntersectionFunctionWithSignature:withRange:)]
        unsafe fn setOpaqueTriangleIntersectionFunctionWithSignature_withRange(
            &self,
            signature: MTLIntersectionFunctionSignature,
            range: NSRange,
        );

        #[method(setOpaqueCurveIntersectionFunctionWithSignature:atIndex:)]
        unsafe fn setOpaqueCurveIntersectionFunctionWithSignature_atIndex(
            &self,
            signature: MTLIntersectionFunctionSignature,
            index: NSUInteger,
        );

        #[method(setOpaqueCurveIntersectionFunctionWithSignature:withRange:)]
        unsafe fn setOpaqueCurveIntersectionFunctionWithSignature_withRange(
            &self,
            signature: MTLIntersectionFunctionSignature,
            range: NSRange,
        );

        #[cfg(feature = "MTLVisibleFunctionTable")]
        #[method(setVisibleFunctionTable:atBufferIndex:)]
        unsafe fn setVisibleFunctionTable_atBufferIndex(
            &self,
            function_table: Option<&ProtocolObject<dyn MTLVisibleFunctionTable>>,
            buffer_index: NSUInteger,
        );

        #[cfg(feature = "MTLVisibleFunctionTable")]
        #[method(setVisibleFunctionTables:withBufferRange:)]
        unsafe fn setVisibleFunctionTables_withBufferRange(
            &self,
            function_tables: NonNull<*const ProtocolObject<dyn MTLVisibleFunctionTable>>,
            buffer_range: NSRange,
        );
    }

    #[cfg(all(feature = "MTLAllocation", feature = "MTLResource"))]
    unsafe impl ProtocolType for dyn MTLIntersectionFunctionTable {}
);
