//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CXCallController;

    unsafe impl ClassType for CXCallController {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for CXCallController {}

extern_methods!(
    unsafe impl CXCallController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "CXCallObserver")]
        #[method_id(@__retain_semantics Other callObserver)]
        pub unsafe fn callObserver(&self) -> Retained<CXCallObserver>;

        #[cfg(all(feature = "CXTransaction", feature = "block2"))]
        #[method(requestTransaction:completion:)]
        pub unsafe fn requestTransaction_completion(
            &self,
            transaction: &CXTransaction,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "CXAction", feature = "block2"))]
        #[method(requestTransactionWithActions:completion:)]
        pub unsafe fn requestTransactionWithActions_completion(
            &self,
            actions: &NSArray<CXAction>,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(all(feature = "CXAction", feature = "block2"))]
        #[method(requestTransactionWithAction:completion:)]
        pub unsafe fn requestTransactionWithAction_completion(
            &self,
            action: &CXAction,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CXCallController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
