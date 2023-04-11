//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "IdentityLookup_ILNetworkResponse")]
    pub struct ILNetworkResponse;

    #[cfg(feature = "IdentityLookup_ILNetworkResponse")]
    unsafe impl ClassType for ILNetworkResponse {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "IdentityLookup_ILNetworkResponse")]
unsafe impl NSCoding for ILNetworkResponse {}

#[cfg(feature = "IdentityLookup_ILNetworkResponse")]
unsafe impl NSObjectProtocol for ILNetworkResponse {}

#[cfg(feature = "IdentityLookup_ILNetworkResponse")]
unsafe impl NSSecureCoding for ILNetworkResponse {}

extern_methods!(
    #[cfg(feature = "IdentityLookup_ILNetworkResponse")]
    unsafe impl ILNetworkResponse {
        #[cfg(feature = "Foundation_NSHTTPURLResponse")]
        #[method_id(@__retain_semantics Other urlResponse)]
        pub unsafe fn urlResponse(&self) -> Id<NSHTTPURLResponse>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Id<NSData>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
