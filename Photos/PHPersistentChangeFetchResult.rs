//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHPersistentChangeFetchResult;

    unsafe impl ClassType for PHPersistentChangeFetchResult {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for PHPersistentChangeFetchResult {}

extern_methods!(
    unsafe impl PHPersistentChangeFetchResult {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(feature = "PHPersistentChange", feature = "block2"))]
        #[method(enumerateChangesWithBlock:)]
        pub unsafe fn enumerateChangesWithBlock(
            &self,
            block: &block2::Block<dyn Fn(NonNull<PHPersistentChange>, NonNull<Bool>) + '_>,
        );
    }
);
