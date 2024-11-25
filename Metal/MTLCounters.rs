//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncounter?language=objc)
// NS_TYPED_ENUM
pub type MTLCommonCounter = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncountertimestamp?language=objc)
    pub static MTLCommonCounterTimestamp: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncountertessellationinputpatches?language=objc)
    pub static MTLCommonCounterTessellationInputPatches: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncountervertexinvocations?language=objc)
    pub static MTLCommonCounterVertexInvocations: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncounterposttessellationvertexinvocations?language=objc)
    pub static MTLCommonCounterPostTessellationVertexInvocations: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncounterclipperinvocations?language=objc)
    pub static MTLCommonCounterClipperInvocations: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncounterclipperprimitivesout?language=objc)
    pub static MTLCommonCounterClipperPrimitivesOut: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncounterfragmentinvocations?language=objc)
    pub static MTLCommonCounterFragmentInvocations: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncounterfragmentspassed?language=objc)
    pub static MTLCommonCounterFragmentsPassed: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncountercomputekernelinvocations?language=objc)
    pub static MTLCommonCounterComputeKernelInvocations: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncountertotalcycles?language=objc)
    pub static MTLCommonCounterTotalCycles: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncountervertexcycles?language=objc)
    pub static MTLCommonCounterVertexCycles: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncountertessellationcycles?language=objc)
    pub static MTLCommonCounterTessellationCycles: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncounterposttessellationvertexcycles?language=objc)
    pub static MTLCommonCounterPostTessellationVertexCycles: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncounterfragmentcycles?language=objc)
    pub static MTLCommonCounterFragmentCycles: &'static MTLCommonCounter;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncounterrendertargetwritecycles?language=objc)
    pub static MTLCommonCounterRenderTargetWriteCycles: &'static MTLCommonCounter;
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncounterset?language=objc)
// NS_TYPED_ENUM
pub type MTLCommonCounterSet = NSString;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncountersettimestamp?language=objc)
    pub static MTLCommonCounterSetTimestamp: &'static MTLCommonCounterSet;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncountersetstageutilization?language=objc)
    pub static MTLCommonCounterSetStageUtilization: &'static MTLCommonCounterSet;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcommoncountersetstatistic?language=objc)
    pub static MTLCommonCounterSetStatistic: &'static MTLCommonCounterSet;
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcounterresulttimestamp?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLCounterResultTimestamp {
    pub timestamp: u64,
}

unsafe impl Encode for MTLCounterResultTimestamp {
    const ENCODING: Encoding = Encoding::Struct("?", &[<u64>::ENCODING]);
}

unsafe impl RefEncode for MTLCounterResultTimestamp {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcounterresultstageutilization?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLCounterResultStageUtilization {
    pub totalCycles: u64,
    pub vertexCycles: u64,
    pub tessellationCycles: u64,
    pub postTessellationVertexCycles: u64,
    pub fragmentCycles: u64,
    pub renderTargetCycles: u64,
}

unsafe impl Encode for MTLCounterResultStageUtilization {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLCounterResultStageUtilization {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcounterresultstatistic?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLCounterResultStatistic {
    pub tessellationInputPatches: u64,
    pub vertexInvocations: u64,
    pub postTessellationVertexInvocations: u64,
    pub clipperInvocations: u64,
    pub clipperPrimitivesOut: u64,
    pub fragmentInvocations: u64,
    pub fragmentsPassed: u64,
    pub computeKernelInvocations: u64,
}

unsafe impl Encode for MTLCounterResultStatistic {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
            <u64>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLCounterResultStatistic {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcounter?language=objc)
    pub unsafe trait MTLCounter: NSObjectProtocol {
        #[method_id(@__retain_semantics Other name)]
        unsafe fn name(&self) -> Retained<NSString>;
    }

    unsafe impl ProtocolType for dyn MTLCounter {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcounterset?language=objc)
    pub unsafe trait MTLCounterSet: NSObjectProtocol {
        #[method_id(@__retain_semantics Other name)]
        unsafe fn name(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other counters)]
        unsafe fn counters(&self) -> Retained<NSArray<ProtocolObject<dyn MTLCounter>>>;
    }

    unsafe impl ProtocolType for dyn MTLCounterSet {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcountersamplebufferdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLCounterSampleBufferDescriptor;
);

unsafe impl NSCopying for MTLCounterSampleBufferDescriptor {}

unsafe impl CopyingHelper for MTLCounterSampleBufferDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLCounterSampleBufferDescriptor {}

extern_methods!(
    unsafe impl MTLCounterSampleBufferDescriptor {
        #[method_id(@__retain_semantics Other counterSet)]
        pub unsafe fn counterSet(&self) -> Option<Retained<ProtocolObject<dyn MTLCounterSet>>>;

        #[method(setCounterSet:)]
        pub unsafe fn setCounterSet(&self, counter_set: Option<&ProtocolObject<dyn MTLCounterSet>>);

        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Retained<NSString>;

        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: &NSString);

        #[cfg(feature = "MTLResource")]
        #[method(storageMode)]
        pub unsafe fn storageMode(&self) -> MTLStorageMode;

        #[cfg(feature = "MTLResource")]
        #[method(setStorageMode:)]
        pub unsafe fn setStorageMode(&self, storage_mode: MTLStorageMode);

        #[method(sampleCount)]
        pub unsafe fn sampleCount(&self) -> NSUInteger;

        #[method(setSampleCount:)]
        pub unsafe fn setSampleCount(&self, sample_count: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLCounterSampleBufferDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcountersamplebuffer?language=objc)
    pub unsafe trait MTLCounterSampleBuffer: NSObjectProtocol {
        #[cfg(feature = "MTLDevice")]
        #[method_id(@__retain_semantics Other device)]
        unsafe fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        #[method_id(@__retain_semantics Other label)]
        unsafe fn label(&self) -> Retained<NSString>;

        #[method(sampleCount)]
        unsafe fn sampleCount(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other resolveCounterRange:)]
        unsafe fn resolveCounterRange(&self, range: NSRange) -> Option<Retained<NSData>>;
    }

    unsafe impl ProtocolType for dyn MTLCounterSampleBuffer {}
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcountererrordomain?language=objc)
    pub static MTLCounterErrorDomain: &'static NSErrorDomain;
}

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlcountersamplebuffererror?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLCounterSampleBufferError(pub NSInteger);
impl MTLCounterSampleBufferError {
    #[doc(alias = "MTLCounterSampleBufferErrorOutOfMemory")]
    pub const OutOfMemory: Self = Self(0);
    #[doc(alias = "MTLCounterSampleBufferErrorInvalid")]
    pub const Invalid: Self = Self(1);
    #[doc(alias = "MTLCounterSampleBufferErrorInternal")]
    pub const Internal: Self = Self(2);
}

unsafe impl Encode for MTLCounterSampleBufferError {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLCounterSampleBufferError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
