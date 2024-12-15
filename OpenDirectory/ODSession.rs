//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odsessionproxyaddress?language=objc)
    pub static ODSessionProxyAddress: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odsessionproxyport?language=objc)
    pub static ODSessionProxyPort: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odsessionproxyusername?language=objc)
    pub static ODSessionProxyUsername: Option<&'static NSString>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odsessionproxypassword?language=objc)
    pub static ODSessionProxyPassword: Option<&'static NSString>;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odsession?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ODSession;
);

unsafe impl NSObjectProtocol for ODSession {}

extern_methods!(
    unsafe impl ODSession {
        #[method_id(@__retain_semantics Other defaultSession)]
        pub unsafe fn defaultSession() -> Option<Retained<ODSession>>;

        #[method_id(@__retain_semantics Other sessionWithOptions:error:)]
        pub unsafe fn sessionWithOptions_error(
            in_options: Option<&NSDictionary>,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithOptions:error:)]
        pub unsafe fn initWithOptions_error(
            this: Allocated<Self>,
            in_options: Option<&NSDictionary>,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other nodeNamesAndReturnError:)]
        pub unsafe fn nodeNamesAndReturnError(
            &self,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<NSArray>>;

        #[method_id(@__retain_semantics Other configurationTemplateNames)]
        pub unsafe fn configurationTemplateNames(&self) -> Retained<NSArray>;

        #[method_id(@__retain_semantics Other mappingTemplateNames)]
        pub unsafe fn mappingTemplateNames(&self) -> Retained<NSArray>;

        #[cfg(feature = "ODConfiguration")]
        #[method_id(@__retain_semantics Other configurationForNodename:)]
        pub unsafe fn configurationForNodename(
            &self,
            nodename: Option<&NSString>,
        ) -> Option<Retained<ODConfiguration>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ODSession {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);