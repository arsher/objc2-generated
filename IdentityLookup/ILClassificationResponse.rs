//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/identitylookup/ilclassificationresponse?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ILClassificationResponse;
);

unsafe impl NSCoding for ILClassificationResponse {}

unsafe impl NSObjectProtocol for ILClassificationResponse {}

unsafe impl NSSecureCoding for ILClassificationResponse {}

extern_methods!(
    unsafe impl ILClassificationResponse {
        #[cfg(feature = "ILClassificationActions")]
        #[method(action)]
        pub unsafe fn action(&self) -> ILClassificationAction;

        #[method_id(@__retain_semantics Other userString)]
        pub unsafe fn userString(&self) -> Option<Retained<NSString>>;

        #[method(setUserString:)]
        pub unsafe fn setUserString(&self, user_string: Option<&NSString>);

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;

        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&NSDictionary<NSString, AnyObject>>);

        #[cfg(feature = "ILClassificationActions")]
        #[method_id(@__retain_semantics Init initWithClassificationAction:)]
        pub unsafe fn initWithClassificationAction(
            this: Allocated<Self>,
            action: ILClassificationAction,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ILClassificationResponse {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
