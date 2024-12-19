//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsmethodsignature?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMethodSignature;
);

unsafe impl NSObjectProtocol for NSMethodSignature {}

extern_methods!(
    unsafe impl NSMethodSignature {
        #[method_id(@__retain_semantics Other signatureWithObjCTypes:)]
        pub unsafe fn signatureWithObjCTypes(
            types: NonNull<c_char>,
        ) -> Option<Retained<NSMethodSignature>>;

        #[method(numberOfArguments)]
        pub unsafe fn numberOfArguments(&self) -> NSUInteger;

        #[method(getArgumentTypeAtIndex:)]
        pub unsafe fn getArgumentTypeAtIndex(&self, idx: NSUInteger) -> NonNull<c_char>;

        #[method(frameLength)]
        pub unsafe fn frameLength(&self) -> NSUInteger;

        #[method(isOneway)]
        pub unsafe fn isOneway(&self) -> bool;

        #[method(methodReturnType)]
        pub unsafe fn methodReturnType(&self) -> NonNull<c_char>;

        #[method(methodReturnLength)]
        pub unsafe fn methodReturnLength(&self) -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSMethodSignature {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
