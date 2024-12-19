//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkobserverquerycompletionhandler?language=objc)
#[cfg(feature = "block2")]
pub type HKObserverQueryCompletionHandler = *mut block2::Block<dyn Fn()>;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkobserverquery?language=objc)
    #[unsafe(super(HKQuery, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HKQuery")]
    pub struct HKObserverQuery;
);

#[cfg(feature = "HKQuery")]
unsafe impl Send for HKObserverQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl Sync for HKObserverQuery {}

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
        ) -> Retained<Self>;

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
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuery`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKObserverQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKObserverQuery {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
