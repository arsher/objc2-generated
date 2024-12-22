//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
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
        /// The database for this sync engine to sync with.
        ///
        /// You can have multiple instances of `CKSyncEngine` in the same process, each targeting a different database.
        /// For example, you might have one for your private database and one for your shared database.
        ///
        /// It's also technically possible to have multiple instances of `CKSyncEngine` for the same `CKDatabase`.
        /// This isn't recommended for production code, but it can be helpful for testing your `CKSyncEngine` integration.
        /// For example, you might make multiple `CKSyncEngine` instances to simulate multiple devices syncing back and forth.
        #[method_id(@__retain_semantics Other database)]
        pub unsafe fn database(&self) -> Retained<CKDatabase>;

        #[cfg(feature = "CKDatabase")]
        /// Setter for [`database`][Self::database].
        #[method(setDatabase:)]
        pub unsafe fn setDatabase(&self, database: &CKDatabase);

        #[cfg(feature = "CKSyncEngineState")]
        /// The state serialization you last received in a `CKSyncEngineStateUpdateEvent`.
        ///
        /// If this is the first time ever initializing your `CKSyncEngine`, you can provide `nil`.
        #[method_id(@__retain_semantics Other stateSerialization)]
        pub unsafe fn stateSerialization(&self)
            -> Option<Retained<CKSyncEngineStateSerialization>>;

        #[cfg(feature = "CKSyncEngineState")]
        /// Setter for [`stateSerialization`][Self::stateSerialization].
        #[method(setStateSerialization:)]
        pub unsafe fn setStateSerialization(
            &self,
            state_serialization: Option<&CKSyncEngineStateSerialization>,
        );

        #[cfg(feature = "CKSyncEngine")]
        /// Your implementation of `CKSyncEngineDelegate`.
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self)
            -> Option<Retained<ProtocolObject<dyn CKSyncEngineDelegate>>>;

        #[cfg(feature = "CKSyncEngine")]
        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn CKSyncEngineDelegate>>,
        );

        /// Whether or not the sync engine should automatically sync on your behalf.
        ///
        /// If true, then the sync engine will automatically sync using the system scheduler. This is the default value.
        /// When you add pending changes to the state, the sync engine will automatically schedule a sync task to send changes.
        /// When it receives a notification about new changes on the server, it will automatically schedule a sync task to fetch changes.
        /// It will also automatically re-schedule sync tasks for retryable errors such as network failures or server throttles.
        ///
        /// If `automaticallySync` is off, then the sync engine will not perform any operations unless you tell it to do so via `fetchChanges` or `sendChanges`.
        ///
        /// Most applications likely want to enable automatic syncing during normal use.
        /// However, you might want to disable it if you have specific requirements for when you want to sync.
        /// For example, if you want to sync only once per day, you can turn off automatic sync and manually call `fetchChanges` and `sendChanges` once per day.
        ///
        /// You might also disable automatic sync when writing automated tests for your integration with `CKSyncEngine`.
        /// This way, you can have fine grained control over exactly when the sync engine fetches or sends changes.
        /// This allows you to simulate edge cases and deterministically test your logic around scenarios like conflict resolution and error handling.
        #[method(automaticallySync)]
        pub unsafe fn automaticallySync(&self) -> bool;

        /// Setter for [`automaticallySync`][Self::automaticallySync].
        #[method(setAutomaticallySync:)]
        pub unsafe fn setAutomaticallySync(&self, automatically_sync: bool);

        #[cfg(feature = "CKSubscription")]
        /// An optional override for the sync engine's default database subscription ID.
        /// Use this for backward compatibility with a previous CloudKit sync implementation.
        ///
        /// By default, `CKSyncEngine` will create its own `CKDatabaseSubscription` with its own subscription ID.
        /// If you're migrating to `CKSyncEngine` from a custom CloudKit sync implementation, you can specify your previous subscription ID here.
        /// This allows your `CKSyncEngine` integration to be backward compatible with previous versions of your app.
        ///
        /// >Note: `CKSyncEngine` will automatically attempt to discover any previous database subscriptions,
        /// but you can be more explicit by giving the subscription ID through this configuration option.
        #[method_id(@__retain_semantics Other subscriptionID)]
        pub unsafe fn subscriptionID(&self) -> Option<Retained<CKSubscriptionID>>;

        #[cfg(feature = "CKSubscription")]
        /// Setter for [`subscriptionID`][Self::subscriptionID].
        #[method(setSubscriptionID:)]
        pub unsafe fn setSubscriptionID(&self, subscription_id: Option<&CKSubscriptionID>);
    }
);
