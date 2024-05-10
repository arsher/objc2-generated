//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    pub struct CKFetchWebAuthTokenOperation;

    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl ClassType for CKFetchWebAuthTokenOperation {
        #[inherits(CKOperation, NSOperation, NSObject)]
        type Super = CKDatabaseOperation;
        type Mutability = InteriorMutable;
    }
);

#[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
unsafe impl NSObjectProtocol for CKFetchWebAuthTokenOperation {}

extern_methods!(
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl CKFetchWebAuthTokenOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithAPIToken:)]
        pub unsafe fn initWithAPIToken(this: Allocated<Self>, api_token: &NSString) -> Id<Self>;

        #[method_id(@__retain_semantics Other APIToken)]
        pub unsafe fn APIToken(&self) -> Option<Id<NSString>>;

        #[method(setAPIToken:)]
        pub unsafe fn setAPIToken(&self, api_token: Option<&NSString>);

        #[cfg(feature = "block2")]
        #[method(fetchWebAuthTokenCompletionBlock)]
        pub unsafe fn fetchWebAuthTokenCompletionBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(*mut NSString, *mut NSError)>;

        #[cfg(feature = "block2")]
        #[method(setFetchWebAuthTokenCompletionBlock:)]
        pub unsafe fn setFetchWebAuthTokenCompletionBlock(
            &self,
            fetch_web_auth_token_completion_block: Option<
                &block2::Block<dyn Fn(*mut NSString, *mut NSError)>,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "CKDatabaseOperation", feature = "CKOperation"))]
    unsafe impl CKFetchWebAuthTokenOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
