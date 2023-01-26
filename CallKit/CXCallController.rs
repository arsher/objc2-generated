//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CallKit_CXCallController")]
    pub struct CXCallController;

    #[cfg(feature = "CallKit_CXCallController")]
    unsafe impl ClassType for CXCallController {
        type Super = NSObject;
    }
);

#[cfg(feature = "CallKit_CXCallController")]
unsafe impl NSObjectProtocol for CXCallController {}

extern_methods!(
    #[cfg(feature = "CallKit_CXCallController")]
    unsafe impl CXCallController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "CallKit_CXCallObserver")]
        #[method_id(@__retain_semantics Other callObserver)]
        pub unsafe fn callObserver(&self) -> Id<CXCallObserver, Shared>;

        #[cfg(all(feature = "CallKit_CXTransaction", feature = "Foundation_NSError"))]
        #[method(requestTransaction:completion:)]
        pub unsafe fn requestTransaction_completion(
            &self,
            transaction: &CXTransaction,
            completion: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(
            feature = "CallKit_CXAction",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(requestTransactionWithActions:completion:)]
        pub unsafe fn requestTransactionWithActions_completion(
            &self,
            actions: &NSArray<CXAction>,
            completion: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(feature = "CallKit_CXAction", feature = "Foundation_NSError"))]
        #[method(requestTransactionWithAction:completion:)]
        pub unsafe fn requestTransactionWithAction_completion(
            &self,
            action: &CXAction,
            completion: &Block<(*mut NSError,), ()>,
        );
    }
);
