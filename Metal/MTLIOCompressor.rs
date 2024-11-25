//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtliocompressionstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLIOCompressionStatus(pub NSInteger);
impl MTLIOCompressionStatus {
    #[doc(alias = "MTLIOCompressionStatusComplete")]
    pub const Complete: Self = Self(0);
    #[doc(alias = "MTLIOCompressionStatusError")]
    pub const Error: Self = Self(1);
}

unsafe impl Encode for MTLIOCompressionStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLIOCompressionStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtliocompressioncontext?language=objc)
pub type MTLIOCompressionContext = *mut c_void;

extern "C-unwind" {
    pub fn MTLIOCompressionContextDefaultChunkSize() -> usize;
}

extern "C-unwind" {
    #[cfg(feature = "MTLDevice")]
    pub fn MTLIOCreateCompressionContext(
        path: NonNull<c_char>,
        r#type: MTLIOCompressionMethod,
        chunk_size: usize,
    ) -> MTLIOCompressionContext;
}

extern "C-unwind" {
    pub fn MTLIOCompressionContextAppendData(
        context: MTLIOCompressionContext,
        data: NonNull<c_void>,
        size: usize,
    );
}

extern "C-unwind" {
    pub fn MTLIOFlushAndDestroyCompressionContext(
        context: MTLIOCompressionContext,
    ) -> MTLIOCompressionStatus;
}
