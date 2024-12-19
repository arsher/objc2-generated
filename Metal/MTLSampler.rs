//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlsamplerminmagfilter?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLSamplerMinMagFilter(pub NSUInteger);
impl MTLSamplerMinMagFilter {
    #[doc(alias = "MTLSamplerMinMagFilterNearest")]
    pub const Nearest: Self = Self(0);
    #[doc(alias = "MTLSamplerMinMagFilterLinear")]
    pub const Linear: Self = Self(1);
}

unsafe impl Encode for MTLSamplerMinMagFilter {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLSamplerMinMagFilter {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlsamplermipfilter?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLSamplerMipFilter(pub NSUInteger);
impl MTLSamplerMipFilter {
    #[doc(alias = "MTLSamplerMipFilterNotMipmapped")]
    pub const NotMipmapped: Self = Self(0);
    #[doc(alias = "MTLSamplerMipFilterNearest")]
    pub const Nearest: Self = Self(1);
    #[doc(alias = "MTLSamplerMipFilterLinear")]
    pub const Linear: Self = Self(2);
}

unsafe impl Encode for MTLSamplerMipFilter {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLSamplerMipFilter {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlsampleraddressmode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLSamplerAddressMode(pub NSUInteger);
impl MTLSamplerAddressMode {
    #[doc(alias = "MTLSamplerAddressModeClampToEdge")]
    pub const ClampToEdge: Self = Self(0);
    #[doc(alias = "MTLSamplerAddressModeMirrorClampToEdge")]
    pub const MirrorClampToEdge: Self = Self(1);
    #[doc(alias = "MTLSamplerAddressModeRepeat")]
    pub const Repeat: Self = Self(2);
    #[doc(alias = "MTLSamplerAddressModeMirrorRepeat")]
    pub const MirrorRepeat: Self = Self(3);
    #[doc(alias = "MTLSamplerAddressModeClampToZero")]
    pub const ClampToZero: Self = Self(4);
    #[doc(alias = "MTLSamplerAddressModeClampToBorderColor")]
    pub const ClampToBorderColor: Self = Self(5);
}

unsafe impl Encode for MTLSamplerAddressMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLSamplerAddressMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlsamplerbordercolor?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLSamplerBorderColor(pub NSUInteger);
impl MTLSamplerBorderColor {
    #[doc(alias = "MTLSamplerBorderColorTransparentBlack")]
    pub const TransparentBlack: Self = Self(0);
    #[doc(alias = "MTLSamplerBorderColorOpaqueBlack")]
    pub const OpaqueBlack: Self = Self(1);
    #[doc(alias = "MTLSamplerBorderColorOpaqueWhite")]
    pub const OpaqueWhite: Self = Self(2);
}

unsafe impl Encode for MTLSamplerBorderColor {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLSamplerBorderColor {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLSamplerDescriptor;
);

unsafe impl NSCopying for MTLSamplerDescriptor {}

unsafe impl CopyingHelper for MTLSamplerDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLSamplerDescriptor {}

extern_methods!(
    unsafe impl MTLSamplerDescriptor {
        #[method(minFilter)]
        pub fn minFilter(&self) -> MTLSamplerMinMagFilter;

        #[method(setMinFilter:)]
        pub fn setMinFilter(&self, min_filter: MTLSamplerMinMagFilter);

        #[method(magFilter)]
        pub fn magFilter(&self) -> MTLSamplerMinMagFilter;

        #[method(setMagFilter:)]
        pub fn setMagFilter(&self, mag_filter: MTLSamplerMinMagFilter);

        #[method(mipFilter)]
        pub fn mipFilter(&self) -> MTLSamplerMipFilter;

        #[method(setMipFilter:)]
        pub fn setMipFilter(&self, mip_filter: MTLSamplerMipFilter);

        #[method(maxAnisotropy)]
        pub fn maxAnisotropy(&self) -> NSUInteger;

        #[method(setMaxAnisotropy:)]
        pub fn setMaxAnisotropy(&self, max_anisotropy: NSUInteger);

        #[method(sAddressMode)]
        pub fn sAddressMode(&self) -> MTLSamplerAddressMode;

        #[method(setSAddressMode:)]
        pub fn setSAddressMode(&self, s_address_mode: MTLSamplerAddressMode);

        #[method(tAddressMode)]
        pub fn tAddressMode(&self) -> MTLSamplerAddressMode;

        #[method(setTAddressMode:)]
        pub fn setTAddressMode(&self, t_address_mode: MTLSamplerAddressMode);

        #[method(rAddressMode)]
        pub fn rAddressMode(&self) -> MTLSamplerAddressMode;

        #[method(setRAddressMode:)]
        pub fn setRAddressMode(&self, r_address_mode: MTLSamplerAddressMode);

        #[method(borderColor)]
        pub fn borderColor(&self) -> MTLSamplerBorderColor;

        #[method(setBorderColor:)]
        pub fn setBorderColor(&self, border_color: MTLSamplerBorderColor);

        #[method(normalizedCoordinates)]
        pub fn normalizedCoordinates(&self) -> bool;

        #[method(setNormalizedCoordinates:)]
        pub fn setNormalizedCoordinates(&self, normalized_coordinates: bool);

        #[method(lodMinClamp)]
        pub fn lodMinClamp(&self) -> c_float;

        #[method(setLodMinClamp:)]
        pub fn setLodMinClamp(&self, lod_min_clamp: c_float);

        #[method(lodMaxClamp)]
        pub fn lodMaxClamp(&self) -> c_float;

        #[method(setLodMaxClamp:)]
        pub fn setLodMaxClamp(&self, lod_max_clamp: c_float);

        #[method(lodAverage)]
        pub fn lodAverage(&self) -> bool;

        #[method(setLodAverage:)]
        pub fn setLodAverage(&self, lod_average: bool);

        #[cfg(feature = "MTLDepthStencil")]
        #[method(compareFunction)]
        pub fn compareFunction(&self) -> MTLCompareFunction;

        #[cfg(feature = "MTLDepthStencil")]
        #[method(setCompareFunction:)]
        pub fn setCompareFunction(&self, compare_function: MTLCompareFunction);

        #[method(supportArgumentBuffers)]
        pub fn supportArgumentBuffers(&self) -> bool;

        #[method(setSupportArgumentBuffers:)]
        pub fn setSupportArgumentBuffers(&self, support_argument_buffers: bool);

        #[method_id(@__retain_semantics Other label)]
        pub fn label(&self) -> Option<Retained<NSString>>;

        #[method(setLabel:)]
        pub fn setLabel(&self, label: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLSamplerDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl DefaultRetained for MTLSamplerDescriptor {
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlsamplerstate?language=objc)
    pub unsafe trait MTLSamplerState: NSObjectProtocol {
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        #[cfg(feature = "MTLTypes")]
        #[method(gpuResourceID)]
        unsafe fn gpuResourceID(&self) -> MTLResourceID;
    }

    unsafe impl ProtocolType for dyn MTLSamplerState {}
);
