//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsscriptcoercionhandler?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSScriptCoercionHandler;
);

unsafe impl NSObjectProtocol for NSScriptCoercionHandler {}

extern_methods!(
    unsafe impl NSScriptCoercionHandler {
        #[method_id(@__retain_semantics Other sharedCoercionHandler)]
        pub unsafe fn sharedCoercionHandler() -> Retained<NSScriptCoercionHandler>;

        #[method_id(@__retain_semantics Other coerceValue:toClass:)]
        pub unsafe fn coerceValue_toClass(
            &self,
            value: &AnyObject,
            to_class: &AnyClass,
        ) -> Option<Retained<AnyObject>>;

        #[method(registerCoercer:selector:toConvertFromClass:toClass:)]
        pub unsafe fn registerCoercer_selector_toConvertFromClass_toClass(
            &self,
            coercer: &AnyObject,
            selector: Sel,
            from_class: &AnyClass,
            to_class: &AnyClass,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSScriptCoercionHandler {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
