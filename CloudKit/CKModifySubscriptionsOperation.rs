//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKModifySubscriptionsOperation")]
    pub struct CKModifySubscriptionsOperation;

    #[cfg(feature = "CloudKit_CKModifySubscriptionsOperation")]
    unsafe impl ClassType for CKModifySubscriptionsOperation {
        #[inherits(CKOperation, NSOperation, NSObject)]
        type Super = CKDatabaseOperation;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKModifySubscriptionsOperation")]
unsafe impl NSObjectProtocol for CKModifySubscriptionsOperation {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKModifySubscriptionsOperation")]
    unsafe impl CKModifySubscriptionsOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(all(feature = "CloudKit_CKSubscription", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Init initWithSubscriptionsToSave:subscriptionIDsToDelete:)]
        pub unsafe fn initWithSubscriptionsToSave_subscriptionIDsToDelete(
            this: Option<Allocated<Self>>,
            subscriptions_to_save: Option<&NSArray<CKSubscription>>,
            subscription_i_ds_to_delete: Option<&NSArray<CKSubscriptionID>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "CloudKit_CKSubscription", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other subscriptionsToSave)]
        pub unsafe fn subscriptionsToSave(&self) -> Option<Id<NSArray<CKSubscription>>>;

        #[cfg(all(feature = "CloudKit_CKSubscription", feature = "Foundation_NSArray"))]
        #[method(setSubscriptionsToSave:)]
        pub unsafe fn setSubscriptionsToSave(
            &self,
            subscriptions_to_save: Option<&NSArray<CKSubscription>>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other subscriptionIDsToDelete)]
        pub unsafe fn subscriptionIDsToDelete(&self) -> Option<Id<NSArray<CKSubscriptionID>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setSubscriptionIDsToDelete:)]
        pub unsafe fn setSubscriptionIDsToDelete(
            &self,
            subscription_i_ds_to_delete: Option<&NSArray<CKSubscriptionID>>,
        );

        #[cfg(all(feature = "CloudKit_CKSubscription", feature = "Foundation_NSError"))]
        #[method(perSubscriptionSaveBlock)]
        pub unsafe fn perSubscriptionSaveBlock(
            &self,
        ) -> *mut Block<(NonNull<CKSubscriptionID>, *mut CKSubscription, *mut NSError), ()>;

        #[cfg(all(feature = "CloudKit_CKSubscription", feature = "Foundation_NSError"))]
        #[method(setPerSubscriptionSaveBlock:)]
        pub unsafe fn setPerSubscriptionSaveBlock(
            &self,
            per_subscription_save_block: Option<
                &Block<(NonNull<CKSubscriptionID>, *mut CKSubscription, *mut NSError), ()>,
            >,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(perSubscriptionDeleteBlock)]
        pub unsafe fn perSubscriptionDeleteBlock(
            &self,
        ) -> *mut Block<(NonNull<CKSubscriptionID>, *mut NSError), ()>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(setPerSubscriptionDeleteBlock:)]
        pub unsafe fn setPerSubscriptionDeleteBlock(
            &self,
            per_subscription_delete_block: Option<
                &Block<(NonNull<CKSubscriptionID>, *mut NSError), ()>,
            >,
        );

        #[cfg(all(
            feature = "CloudKit_CKSubscription",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(modifySubscriptionsCompletionBlock)]
        pub unsafe fn modifySubscriptionsCompletionBlock(
            &self,
        ) -> *mut Block<
            (
                *mut NSArray<CKSubscription>,
                *mut NSArray<CKSubscriptionID>,
                *mut NSError,
            ),
            (),
        >;

        #[cfg(all(
            feature = "CloudKit_CKSubscription",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(setModifySubscriptionsCompletionBlock:)]
        pub unsafe fn setModifySubscriptionsCompletionBlock(
            &self,
            modify_subscriptions_completion_block: Option<
                &Block<
                    (
                        *mut NSArray<CKSubscription>,
                        *mut NSArray<CKSubscriptionID>,
                        *mut NSError,
                    ),
                    (),
                >,
            >,
        );
    }
);
