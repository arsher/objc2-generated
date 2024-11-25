//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlfunctionlogtype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLFunctionLogType(pub NSUInteger);
impl MTLFunctionLogType {
    #[doc(alias = "MTLFunctionLogTypeValidation")]
    pub const Validation: Self = Self(0);
}

unsafe impl Encode for MTLFunctionLogType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLFunctionLogType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtllogcontainer?language=objc)
    pub unsafe trait MTLLogContainer: NSFastEnumeration + NSObjectProtocol {}

    unsafe impl ProtocolType for dyn MTLLogContainer {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlfunctionlogdebuglocation?language=objc)
    pub unsafe trait MTLFunctionLogDebugLocation: NSObjectProtocol {
        #[method_id(@__retain_semantics Other functionName)]
        unsafe fn functionName(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other URL)]
        unsafe fn URL(&self) -> Option<Retained<NSURL>>;

        #[method(line)]
        unsafe fn line(&self) -> NSUInteger;

        #[method(column)]
        unsafe fn column(&self) -> NSUInteger;
    }

    unsafe impl ProtocolType for dyn MTLFunctionLogDebugLocation {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlfunctionlog?language=objc)
    pub unsafe trait MTLFunctionLog: NSObjectProtocol {
        #[method(type)]
        unsafe fn r#type(&self) -> MTLFunctionLogType;

        #[method_id(@__retain_semantics Other encoderLabel)]
        unsafe fn encoderLabel(&self) -> Option<Retained<NSString>>;

        #[cfg(feature = "MTLLibrary")]
        #[method_id(@__retain_semantics Other function)]
        unsafe fn function(&self) -> Option<Retained<ProtocolObject<dyn MTLFunction>>>;

        #[method_id(@__retain_semantics Other debugLocation)]
        unsafe fn debugLocation(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLFunctionLogDebugLocation>>>;
    }

    unsafe impl ProtocolType for dyn MTLFunctionLog {}
);
