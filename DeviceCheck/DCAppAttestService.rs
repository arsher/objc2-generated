//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::DeviceCheck::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "DeviceCheck_DCAppAttestService")]
    pub struct DCAppAttestService;

    #[cfg(feature = "DeviceCheck_DCAppAttestService")]
    unsafe impl ClassType for DCAppAttestService {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "DeviceCheck_DCAppAttestService")]
unsafe impl NSObjectProtocol for DCAppAttestService {}

extern_methods!(
    #[cfg(feature = "DeviceCheck_DCAppAttestService")]
    unsafe impl DCAppAttestService {
        #[method_id(@__retain_semantics Other sharedService)]
        pub unsafe fn sharedService() -> Id<DCAppAttestService>;

        #[method(isSupported)]
        pub unsafe fn isSupported(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(generateKeyWithCompletionHandler:)]
        pub unsafe fn generateKeyWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSString, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(attestKey:clientDataHash:completionHandler:)]
        pub unsafe fn attestKey_clientDataHash_completionHandler(
            &self,
            key_id: &NSString,
            client_data_hash: &NSData,
            completion_handler: &Block<(*mut NSData, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(generateAssertion:clientDataHash:completionHandler:)]
        pub unsafe fn generateAssertion_clientDataHash_completionHandler(
            &self,
            key_id: &NSString,
            client_data_hash: &NSData,
            completion_handler: &Block<(*mut NSData, *mut NSError), ()>,
        );
    }
);
