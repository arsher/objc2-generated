//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/healthkit/hkanchoredobjectquery?language=objc)
    #[unsafe(super(HKQuery, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HKQuery")]
    pub struct HKAnchoredObjectQuery;
);

#[cfg(feature = "HKQuery")]
unsafe impl Send for HKAnchoredObjectQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl Sync for HKAnchoredObjectQuery {}

#[cfg(feature = "HKQuery")]
unsafe impl NSObjectProtocol for HKAnchoredObjectQuery {}

extern_methods!(
    #[cfg(feature = "HKQuery")]
    unsafe impl HKAnchoredObjectQuery {
        #[cfg(all(
            feature = "HKDeletedObject",
            feature = "HKObject",
            feature = "HKQueryAnchor",
            feature = "HKSample",
            feature = "block2"
        ))]
        #[method(updateHandler)]
        pub unsafe fn updateHandler(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(
                NonNull<HKAnchoredObjectQuery>,
                *mut NSArray<HKSample>,
                *mut NSArray<HKDeletedObject>,
                *mut HKQueryAnchor,
                *mut NSError,
            ),
        >;

        #[cfg(all(
            feature = "HKDeletedObject",
            feature = "HKObject",
            feature = "HKQueryAnchor",
            feature = "HKSample",
            feature = "block2"
        ))]
        #[method(setUpdateHandler:)]
        pub unsafe fn setUpdateHandler(
            &self,
            update_handler: Option<
                &block2::Block<
                    dyn Fn(
                        NonNull<HKAnchoredObjectQuery>,
                        *mut NSArray<HKSample>,
                        *mut NSArray<HKDeletedObject>,
                        *mut HKQueryAnchor,
                        *mut NSError,
                    ),
                >,
            >,
        );

        #[cfg(all(
            feature = "HKDeletedObject",
            feature = "HKObject",
            feature = "HKObjectType",
            feature = "HKQueryAnchor",
            feature = "HKSample",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithType:predicate:anchor:limit:resultsHandler:)]
        pub unsafe fn initWithType_predicate_anchor_limit_resultsHandler(
            this: Allocated<Self>,
            r#type: &HKSampleType,
            predicate: Option<&NSPredicate>,
            anchor: Option<&HKQueryAnchor>,
            limit: NSUInteger,
            handler: &block2::Block<
                dyn Fn(
                    NonNull<HKAnchoredObjectQuery>,
                    *mut NSArray<HKSample>,
                    *mut NSArray<HKDeletedObject>,
                    *mut HKQueryAnchor,
                    *mut NSError,
                ),
            >,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "HKObject",
            feature = "HKObjectType",
            feature = "HKSample",
            feature = "block2"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithType:predicate:anchor:limit:completionHandler:)]
        pub unsafe fn initWithType_predicate_anchor_limit_completionHandler(
            this: Allocated<Self>,
            r#type: &HKSampleType,
            predicate: Option<&NSPredicate>,
            anchor: NSUInteger,
            limit: NSUInteger,
            handler: &block2::Block<
                dyn Fn(
                    NonNull<HKAnchoredObjectQuery>,
                    *mut NSArray<HKSample>,
                    NSUInteger,
                    *mut NSError,
                ),
            >,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "HKDeletedObject",
            feature = "HKObject",
            feature = "HKQueryAnchor",
            feature = "HKQueryDescriptor",
            feature = "HKSample",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithQueryDescriptors:anchor:limit:resultsHandler:)]
        pub unsafe fn initWithQueryDescriptors_anchor_limit_resultsHandler(
            this: Allocated<Self>,
            query_descriptors: &NSArray<HKQueryDescriptor>,
            anchor: Option<&HKQueryAnchor>,
            limit: NSInteger,
            handler: &block2::Block<
                dyn Fn(
                    NonNull<HKAnchoredObjectQuery>,
                    *mut NSArray<HKSample>,
                    *mut NSArray<HKDeletedObject>,
                    *mut HKQueryAnchor,
                    *mut NSError,
                ),
            >,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuery`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKAnchoredObjectQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HKQuery")]
    unsafe impl HKAnchoredObjectQuery {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
