//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CKOperation")]
    #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
    pub struct CKDiscoverAllUserIdentitiesOperation;

    #[cfg(feature = "CKOperation")]
    unsafe impl ClassType for CKDiscoverAllUserIdentitiesOperation {
        #[inherits(NSOperation, NSObject)]
        type Super = CKOperation;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CKOperation")]
unsafe impl NSObjectProtocol for CKDiscoverAllUserIdentitiesOperation {}

extern_methods!(
    #[cfg(feature = "CKOperation")]
    unsafe impl CKDiscoverAllUserIdentitiesOperation {
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(feature = "CKUserIdentity", feature = "block2"))]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(userIdentityDiscoveredBlock)]
        pub unsafe fn userIdentityDiscoveredBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<CKUserIdentity>)>;

        #[cfg(all(feature = "CKUserIdentity", feature = "block2"))]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(setUserIdentityDiscoveredBlock:)]
        pub unsafe fn setUserIdentityDiscoveredBlock(
            &self,
            user_identity_discovered_block: Option<&block2::Block<dyn Fn(NonNull<CKUserIdentity>)>>,
        );

        #[cfg(feature = "block2")]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(discoverAllUserIdentitiesCompletionBlock)]
        pub unsafe fn discoverAllUserIdentitiesCompletionBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(*mut NSError)>;

        #[cfg(feature = "block2")]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(setDiscoverAllUserIdentitiesCompletionBlock:)]
        pub unsafe fn setDiscoverAllUserIdentitiesCompletionBlock(
            &self,
            discover_all_user_identities_completion_block: Option<
                &block2::Block<dyn Fn(*mut NSError)>,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CKOperation")]
    unsafe impl CKDiscoverAllUserIdentitiesOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
