//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlvertexformat?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLVertexFormat(pub NSUInteger);
impl MTLVertexFormat {
    #[doc(alias = "MTLVertexFormatInvalid")]
    pub const Invalid: Self = Self(0);
    #[doc(alias = "MTLVertexFormatUChar2")]
    pub const UChar2: Self = Self(1);
    #[doc(alias = "MTLVertexFormatUChar3")]
    pub const UChar3: Self = Self(2);
    #[doc(alias = "MTLVertexFormatUChar4")]
    pub const UChar4: Self = Self(3);
    #[doc(alias = "MTLVertexFormatChar2")]
    pub const Char2: Self = Self(4);
    #[doc(alias = "MTLVertexFormatChar3")]
    pub const Char3: Self = Self(5);
    #[doc(alias = "MTLVertexFormatChar4")]
    pub const Char4: Self = Self(6);
    #[doc(alias = "MTLVertexFormatUChar2Normalized")]
    pub const UChar2Normalized: Self = Self(7);
    #[doc(alias = "MTLVertexFormatUChar3Normalized")]
    pub const UChar3Normalized: Self = Self(8);
    #[doc(alias = "MTLVertexFormatUChar4Normalized")]
    pub const UChar4Normalized: Self = Self(9);
    #[doc(alias = "MTLVertexFormatChar2Normalized")]
    pub const Char2Normalized: Self = Self(10);
    #[doc(alias = "MTLVertexFormatChar3Normalized")]
    pub const Char3Normalized: Self = Self(11);
    #[doc(alias = "MTLVertexFormatChar4Normalized")]
    pub const Char4Normalized: Self = Self(12);
    #[doc(alias = "MTLVertexFormatUShort2")]
    pub const UShort2: Self = Self(13);
    #[doc(alias = "MTLVertexFormatUShort3")]
    pub const UShort3: Self = Self(14);
    #[doc(alias = "MTLVertexFormatUShort4")]
    pub const UShort4: Self = Self(15);
    #[doc(alias = "MTLVertexFormatShort2")]
    pub const Short2: Self = Self(16);
    #[doc(alias = "MTLVertexFormatShort3")]
    pub const Short3: Self = Self(17);
    #[doc(alias = "MTLVertexFormatShort4")]
    pub const Short4: Self = Self(18);
    #[doc(alias = "MTLVertexFormatUShort2Normalized")]
    pub const UShort2Normalized: Self = Self(19);
    #[doc(alias = "MTLVertexFormatUShort3Normalized")]
    pub const UShort3Normalized: Self = Self(20);
    #[doc(alias = "MTLVertexFormatUShort4Normalized")]
    pub const UShort4Normalized: Self = Self(21);
    #[doc(alias = "MTLVertexFormatShort2Normalized")]
    pub const Short2Normalized: Self = Self(22);
    #[doc(alias = "MTLVertexFormatShort3Normalized")]
    pub const Short3Normalized: Self = Self(23);
    #[doc(alias = "MTLVertexFormatShort4Normalized")]
    pub const Short4Normalized: Self = Self(24);
    #[doc(alias = "MTLVertexFormatHalf2")]
    pub const Half2: Self = Self(25);
    #[doc(alias = "MTLVertexFormatHalf3")]
    pub const Half3: Self = Self(26);
    #[doc(alias = "MTLVertexFormatHalf4")]
    pub const Half4: Self = Self(27);
    #[doc(alias = "MTLVertexFormatFloat")]
    pub const Float: Self = Self(28);
    #[doc(alias = "MTLVertexFormatFloat2")]
    pub const Float2: Self = Self(29);
    #[doc(alias = "MTLVertexFormatFloat3")]
    pub const Float3: Self = Self(30);
    #[doc(alias = "MTLVertexFormatFloat4")]
    pub const Float4: Self = Self(31);
    #[doc(alias = "MTLVertexFormatInt")]
    pub const Int: Self = Self(32);
    #[doc(alias = "MTLVertexFormatInt2")]
    pub const Int2: Self = Self(33);
    #[doc(alias = "MTLVertexFormatInt3")]
    pub const Int3: Self = Self(34);
    #[doc(alias = "MTLVertexFormatInt4")]
    pub const Int4: Self = Self(35);
    #[doc(alias = "MTLVertexFormatUInt")]
    pub const UInt: Self = Self(36);
    #[doc(alias = "MTLVertexFormatUInt2")]
    pub const UInt2: Self = Self(37);
    #[doc(alias = "MTLVertexFormatUInt3")]
    pub const UInt3: Self = Self(38);
    #[doc(alias = "MTLVertexFormatUInt4")]
    pub const UInt4: Self = Self(39);
    #[doc(alias = "MTLVertexFormatInt1010102Normalized")]
    pub const Int1010102Normalized: Self = Self(40);
    #[doc(alias = "MTLVertexFormatUInt1010102Normalized")]
    pub const UInt1010102Normalized: Self = Self(41);
    #[doc(alias = "MTLVertexFormatUChar4Normalized_BGRA")]
    pub const UChar4Normalized_BGRA: Self = Self(42);
    #[doc(alias = "MTLVertexFormatUChar")]
    pub const UChar: Self = Self(45);
    #[doc(alias = "MTLVertexFormatChar")]
    pub const Char: Self = Self(46);
    #[doc(alias = "MTLVertexFormatUCharNormalized")]
    pub const UCharNormalized: Self = Self(47);
    #[doc(alias = "MTLVertexFormatCharNormalized")]
    pub const CharNormalized: Self = Self(48);
    #[doc(alias = "MTLVertexFormatUShort")]
    pub const UShort: Self = Self(49);
    #[doc(alias = "MTLVertexFormatShort")]
    pub const Short: Self = Self(50);
    #[doc(alias = "MTLVertexFormatUShortNormalized")]
    pub const UShortNormalized: Self = Self(51);
    #[doc(alias = "MTLVertexFormatShortNormalized")]
    pub const ShortNormalized: Self = Self(52);
    #[doc(alias = "MTLVertexFormatHalf")]
    pub const Half: Self = Self(53);
    #[doc(alias = "MTLVertexFormatFloatRG11B10")]
    pub const FloatRG11B10: Self = Self(54);
    #[doc(alias = "MTLVertexFormatFloatRGB9E5")]
    pub const FloatRGB9E5: Self = Self(55);
}

unsafe impl Encode for MTLVertexFormat {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLVertexFormat {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlvertexstepfunction?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLVertexStepFunction(pub NSUInteger);
impl MTLVertexStepFunction {
    #[doc(alias = "MTLVertexStepFunctionConstant")]
    pub const Constant: Self = Self(0);
    #[doc(alias = "MTLVertexStepFunctionPerVertex")]
    pub const PerVertex: Self = Self(1);
    #[doc(alias = "MTLVertexStepFunctionPerInstance")]
    pub const PerInstance: Self = Self(2);
    #[doc(alias = "MTLVertexStepFunctionPerPatch")]
    pub const PerPatch: Self = Self(3);
    #[doc(alias = "MTLVertexStepFunctionPerPatchControlPoint")]
    pub const PerPatchControlPoint: Self = Self(4);
}

unsafe impl Encode for MTLVertexStepFunction {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLVertexStepFunction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlbufferlayoutstridedynamic?language=objc)
pub static MTLBufferLayoutStrideDynamic: NSUInteger = NSUIntegerMax as _;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLVertexBufferLayoutDescriptor;
);

unsafe impl NSCopying for MTLVertexBufferLayoutDescriptor {}

unsafe impl CopyingHelper for MTLVertexBufferLayoutDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLVertexBufferLayoutDescriptor {}

extern_methods!(
    unsafe impl MTLVertexBufferLayoutDescriptor {
        #[method(stride)]
        pub fn stride(&self) -> NSUInteger;

        #[method(setStride:)]
        pub unsafe fn setStride(&self, stride: NSUInteger);

        #[method(stepFunction)]
        pub fn stepFunction(&self) -> MTLVertexStepFunction;

        #[method(setStepFunction:)]
        pub unsafe fn setStepFunction(&self, step_function: MTLVertexStepFunction);

        #[method(stepRate)]
        pub fn stepRate(&self) -> NSUInteger;

        #[method(setStepRate:)]
        pub unsafe fn setStepRate(&self, step_rate: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLVertexBufferLayoutDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for MTLVertexBufferLayoutDescriptor {
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptorarray?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLVertexBufferLayoutDescriptorArray;
);

unsafe impl NSObjectProtocol for MTLVertexBufferLayoutDescriptorArray {}

extern_methods!(
    unsafe impl MTLVertexBufferLayoutDescriptorArray {
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            index: NSUInteger,
        ) -> Retained<MTLVertexBufferLayoutDescriptor>;

        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            buffer_desc: Option<&MTLVertexBufferLayoutDescriptor>,
            index: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLVertexBufferLayoutDescriptorArray {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLVertexAttributeDescriptor;
);

unsafe impl NSCopying for MTLVertexAttributeDescriptor {}

unsafe impl CopyingHelper for MTLVertexAttributeDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLVertexAttributeDescriptor {}

extern_methods!(
    unsafe impl MTLVertexAttributeDescriptor {
        #[method(format)]
        pub fn format(&self) -> MTLVertexFormat;

        #[method(setFormat:)]
        pub fn setFormat(&self, format: MTLVertexFormat);

        #[method(offset)]
        pub fn offset(&self) -> NSUInteger;

        #[method(setOffset:)]
        pub unsafe fn setOffset(&self, offset: NSUInteger);

        #[method(bufferIndex)]
        pub fn bufferIndex(&self) -> NSUInteger;

        #[method(setBufferIndex:)]
        pub unsafe fn setBufferIndex(&self, buffer_index: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLVertexAttributeDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for MTLVertexAttributeDescriptor {
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptorarray?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLVertexAttributeDescriptorArray;
);

unsafe impl NSObjectProtocol for MTLVertexAttributeDescriptorArray {}

extern_methods!(
    unsafe impl MTLVertexAttributeDescriptorArray {
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            index: NSUInteger,
        ) -> Retained<MTLVertexAttributeDescriptor>;

        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attribute_desc: Option<&MTLVertexAttributeDescriptor>,
            index: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLVertexAttributeDescriptorArray {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlvertexdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLVertexDescriptor;
);

unsafe impl NSCopying for MTLVertexDescriptor {}

unsafe impl CopyingHelper for MTLVertexDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLVertexDescriptor {}

extern_methods!(
    unsafe impl MTLVertexDescriptor {
        #[method_id(@__retain_semantics Other vertexDescriptor)]
        pub fn vertexDescriptor() -> Retained<MTLVertexDescriptor>;

        #[method_id(@__retain_semantics Other layouts)]
        pub fn layouts(&self) -> Retained<MTLVertexBufferLayoutDescriptorArray>;

        #[method_id(@__retain_semantics Other attributes)]
        pub fn attributes(&self) -> Retained<MTLVertexAttributeDescriptorArray>;

        #[method(reset)]
        pub fn reset(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLVertexDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
