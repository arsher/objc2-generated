//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlvisiblefunctiontabledescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLVisibleFunctionTableDescriptor;
);

unsafe impl NSCopying for MTLVisibleFunctionTableDescriptor {}

unsafe impl CopyingHelper for MTLVisibleFunctionTableDescriptor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for MTLVisibleFunctionTableDescriptor {}

extern_methods!(
    unsafe impl MTLVisibleFunctionTableDescriptor {
        #[method_id(@__retain_semantics Other visibleFunctionTableDescriptor)]
        pub unsafe fn visibleFunctionTableDescriptor() -> Retained<MTLVisibleFunctionTableDescriptor>;

        #[method(functionCount)]
        pub unsafe fn functionCount(&self) -> NSUInteger;

        #[method(setFunctionCount:)]
        pub unsafe fn setFunctionCount(&self, function_count: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MTLVisibleFunctionTableDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable?language=objc)
    #[cfg(all(feature = "MTLAllocation", feature = "MTLResource"))]
    pub unsafe trait MTLVisibleFunctionTable: MTLResource {
        #[cfg(feature = "MTLTypes")]
        #[method(gpuResourceID)]
        unsafe fn gpuResourceID(&self) -> MTLResourceID;

        #[cfg(feature = "MTLFunctionHandle")]
        #[method(setFunction:atIndex:)]
        unsafe fn setFunction_atIndex(
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
    }

    #[cfg(all(feature = "MTLAllocation", feature = "MTLResource"))]
    unsafe impl ProtocolType for dyn MTLVisibleFunctionTable {}
);
