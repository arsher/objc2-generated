//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
use objc2_core_graphics::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/ctrundelegateref?language=objc)
pub type CTRunDelegateRef = *mut c_void;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTRunDelegateGetTypeID() -> CFTypeID;
}

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/ctrundelegatedeallocatecallback?language=objc)
pub type CTRunDelegateDeallocateCallback = Option<unsafe extern "C-unwind" fn(NonNull<c_void>)>;

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/ctrundelegategetascentcallback?language=objc)
#[cfg(feature = "objc2-core-foundation")]
pub type CTRunDelegateGetAscentCallback =
    Option<unsafe extern "C-unwind" fn(NonNull<c_void>) -> CGFloat>;

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/ctrundelegategetdescentcallback?language=objc)
#[cfg(feature = "objc2-core-foundation")]
pub type CTRunDelegateGetDescentCallback =
    Option<unsafe extern "C-unwind" fn(NonNull<c_void>) -> CGFloat>;

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/ctrundelegategetwidthcallback?language=objc)
#[cfg(feature = "objc2-core-foundation")]
pub type CTRunDelegateGetWidthCallback =
    Option<unsafe extern "C-unwind" fn(NonNull<c_void>) -> CGFloat>;

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/ctrundelegatecallbacks?language=objc)
#[cfg(feature = "objc2-core-foundation")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CTRunDelegateCallbacks {
    pub version: CFIndex,
    pub dealloc: CTRunDelegateDeallocateCallback,
    pub getAscent: CTRunDelegateGetAscentCallback,
    pub getDescent: CTRunDelegateGetDescentCallback,
    pub getWidth: CTRunDelegateGetWidthCallback,
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl Encode for CTRunDelegateCallbacks {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <CFIndex>::ENCODING,
            <CTRunDelegateDeallocateCallback>::ENCODING,
            <CTRunDelegateGetAscentCallback>::ENCODING,
            <CTRunDelegateGetDescentCallback>::ENCODING,
            <CTRunDelegateGetWidthCallback>::ENCODING,
        ],
    );
}

#[cfg(feature = "objc2-core-foundation")]
unsafe impl RefEncode for CTRunDelegateCallbacks {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctrundelegateversion1?language=objc)
pub const kCTRunDelegateVersion1: c_uint = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/coretext/kctrundelegatecurrentversion?language=objc)
pub const kCTRunDelegateCurrentVersion: c_uint = kCTRunDelegateVersion1;

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn CTRunDelegateCreate(
        callbacks: NonNull<CTRunDelegateCallbacks>,
        ref_con: *mut c_void,
    ) -> CTRunDelegateRef;
}

extern "C-unwind" {
    pub fn CTRunDelegateGetRefCon(run_delegate: CTRunDelegateRef) -> NonNull<c_void>;
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/coretext/ctadaptiveimageproviding?language=objc)
    pub unsafe trait CTAdaptiveImageProviding {
        #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-core-graphics"))]
        #[method(imageForProposedSize:scaleFactor:imageOffset:imageSize:)]
        unsafe fn imageForProposedSize_scaleFactor_imageOffset_imageSize(
            &self,
            proposed_size: CGSize,
            scale_factor: CGFloat,
            out_image_offset: NonNull<CGPoint>,
            out_image_size: NonNull<CGSize>,
        ) -> CGImageRef;
    }

    unsafe impl ProtocolType for dyn CTAdaptiveImageProviding {}
);
