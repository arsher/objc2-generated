//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

#[cfg(feature = "block2")]
pub type HKObserverQueryCompletionHandler = *mut block2::Block<dyn Fn()>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HKQuery")]
    pub struct HKObserverQuery;

    #[cfg(feature = "HKQuery")]
    unsafe impl ClassType for HKObserverQuery {
        #[inherits(NSObject)]
        type Super = HKQuery;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HKQuery")]
unsafe impl NSObjectProtocol for HKObserverQuery {}

extern_methods!(
    #[cfg(feature = "HKQuery")]
    unsafe impl HKObserverQuery {
        #[cfg(all(feature = "HKObjectType", feature = "block2"))]
        #[method_id(@__retain_semantics Init initWithSampleType:predicate:updateHandler:)]
        pub unsafe fn initWithSampleType_predicate_updateHandler(
            this: Allocated<Self>,
            sample_type: &HKSampleType,
            predicate: Option<&NSPredicate>,
            update_handler: &block2::Block<
                dyn Fn(NonNull<HKObserverQuery>, HKObserverQueryCompletionHandler, *mut NSError),
            >,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "HKObjectType",
            feature = "HKQueryDescriptor",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithQueryDescriptors:updateHandler:)]
        pub unsafe fn initWithQueryDescriptors_updateHandler(
            this: Allocated<Self>,
            query_descriptors: &NSArray<HKQueryDescriptor>,
            update_handler: &block2::Block<
                dyn Fn(
                    NonNull<HKObserverQuery>,
                    *mut NSSet<HKSampleType>,
                    HKObserverQueryCompletionHandler,
                    *mut NSError,
                ),
            >,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuery`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKObserverQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKObserverQuery {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
