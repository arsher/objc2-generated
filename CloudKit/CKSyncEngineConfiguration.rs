//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksyncengineconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSyncEngineConfiguration;
);

unsafe impl Send for CKSyncEngineConfiguration {}

unsafe impl Sync for CKSyncEngineConfiguration {}

unsafe impl NSObjectProtocol for CKSyncEngineConfiguration {}

extern_methods!(
    unsafe impl CKSyncEngineConfiguration {
        #[cfg(all(
            feature = "CKDatabase",
            feature = "CKSyncEngine",
            feature = "CKSyncEngineState"
        ))]
        #[method_id(@__retain_semantics Init initWithDatabase:stateSerialization:delegate:)]
        pub unsafe fn initWithDatabase_stateSerialization_delegate(
            this: Allocated<Self>,
            database: &CKDatabase,
            state_serialization: Option<&CKSyncEngineStateSerialization>,
            delegate: &ProtocolObject<dyn CKSyncEngineDelegate>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "CKDatabase")]
        #[method_id(@__retain_semantics Other database)]
        pub unsafe fn database(&self) -> Retained<CKDatabase>;

        #[cfg(feature = "CKDatabase")]
        #[method(setDatabase:)]
        pub unsafe fn setDatabase(&self, database: &CKDatabase);

        #[cfg(feature = "CKSyncEngineState")]
        #[method_id(@__retain_semantics Other stateSerialization)]
        pub unsafe fn stateSerialization(&self)
            -> Option<Retained<CKSyncEngineStateSerialization>>;

        #[cfg(feature = "CKSyncEngineState")]
        #[method(setStateSerialization:)]
        pub unsafe fn setStateSerialization(
            &self,
            state_serialization: Option<&CKSyncEngineStateSerialization>,
        );

        #[cfg(feature = "CKSyncEngine")]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self)
            -> Option<Retained<ProtocolObject<dyn CKSyncEngineDelegate>>>;

        #[cfg(feature = "CKSyncEngine")]
        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn CKSyncEngineDelegate>>,
        );

        #[method(automaticallySync)]
        pub unsafe fn automaticallySync(&self) -> bool;

        #[method(setAutomaticallySync:)]
        pub unsafe fn setAutomaticallySync(&self, automatically_sync: bool);

        #[cfg(feature = "CKSubscription")]
        #[method_id(@__retain_semantics Other subscriptionID)]
        pub unsafe fn subscriptionID(&self) -> Option<Retained<CKSubscriptionID>>;

        #[cfg(feature = "CKSubscription")]
        #[method(setSubscriptionID:)]
        pub unsafe fn setSubscriptionID(&self, subscription_id: Option<&CKSubscriptionID>);
    }
);
