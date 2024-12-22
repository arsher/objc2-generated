//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// The level of the log entry.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlloglevel?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLLogLevel(pub NSInteger);
impl MTLLogLevel {
    #[doc(alias = "MTLLogLevelUndefined")]
    pub const Undefined: Self = Self(0);
    #[doc(alias = "MTLLogLevelDebug")]
    pub const Debug: Self = Self(1);
    #[doc(alias = "MTLLogLevelInfo")]
    pub const Info: Self = Self(2);
    #[doc(alias = "MTLLogLevelNotice")]
    pub const Notice: Self = Self(3);
    #[doc(alias = "MTLLogLevelError")]
    pub const Error: Self = Self(4);
    #[doc(alias = "MTLLogLevelFault")]
    pub const Fault: Self = Self(5);
}

unsafe impl Encode for MTLLogLevel {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTLLogLevel {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtllogstate?language=objc)
    pub unsafe trait MTLLogState: NSObjectProtocol {
        #[cfg(feature = "block2")]
        /// Add a function block to handle log message output.
        /// In the absence of any handlers, log messages go through the default handler.
        #[method(addLogHandler:)]
        unsafe fn addLogHandler(
            &self,
            block: &block2::Block<
                dyn Fn(*mut NSString, *mut NSString, MTLLogLevel, NonNull<NSString>),
            >,
        );
    }

    unsafe impl ProtocolType for dyn MTLLogState {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtllogstatedescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLLogStateDescriptor;
);

unsafe impl NSCopying for MTLLogStateDescriptor {}

unsafe impl CopyingHelper for MTLLogStateDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLLogStateDescriptor {}

extern_methods!(
    unsafe impl MTLLogStateDescriptor {
        /// level indicates the minimum level of the logs that will be printed.
        /// All the logs with level less than given level will be skipped on the GPU Side.
        #[method(level)]
        pub unsafe fn level(&self) -> MTLLogLevel;

        /// Setter for [`level`][Self::level].
        #[method(setLevel:)]
        pub unsafe fn setLevel(&self, level: MTLLogLevel);

        /// bufferSize indicates the size of the buffer where GPU will store the logging content from shaders. Minimum value is 1KB
        #[method(bufferSize)]
        pub unsafe fn bufferSize(&self) -> NSInteger;

        /// Setter for [`bufferSize`][Self::bufferSize].
        #[method(setBufferSize:)]
        pub unsafe fn setBufferSize(&self, buffer_size: NSInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLLogStateDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtllogstateerrordomain?language=objc)
    pub static MTLLogStateErrorDomain: &'static NSErrorDomain;
}

/// NSErrors raised when creating a logstate.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtllogstateerror?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLLogStateError(pub NSUInteger);
impl MTLLogStateError {
    #[doc(alias = "MTLLogStateErrorInvalidSize")]
    pub const InvalidSize: Self = Self(1);
    #[doc(alias = "MTLLogStateErrorInvalid")]
    pub const Invalid: Self = Self(2);
}

unsafe impl Encode for MTLLogStateError {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTLLogStateError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
