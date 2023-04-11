//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSDistributedLock")]
    pub struct NSDistributedLock;

    #[cfg(feature = "Foundation_NSDistributedLock")]
    unsafe impl ClassType for NSDistributedLock {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSDistributedLock")]
unsafe impl NSObjectProtocol for NSDistributedLock {}

extern_methods!(
    #[cfg(feature = "Foundation_NSDistributedLock")]
    unsafe impl NSDistributedLock {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other lockWithPath:)]
        pub unsafe fn lockWithPath(path: &NSString) -> Option<Id<NSDistributedLock>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithPath:)]
        pub unsafe fn initWithPath(
            this: Option<Allocated<Self>>,
            path: &NSString,
        ) -> Option<Id<Self>>;

        #[method(tryLock)]
        pub unsafe fn tryLock(&self) -> bool;

        #[method(unlock)]
        pub unsafe fn unlock(&self);

        #[method(breakLock)]
        pub unsafe fn breakLock(&self);

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other lockDate)]
        pub unsafe fn lockDate(&self) -> Id<NSDate>;
    }
);
