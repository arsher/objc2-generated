//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksystemsharinguiobserver?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKSystemSharingUIObserver;
);

unsafe impl NSObjectProtocol for CKSystemSharingUIObserver {}

extern_methods!(
    unsafe impl CKSystemSharingUIObserver {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "CKContainer")]
        #[method_id(@__retain_semantics Init initWithContainer:)]
        pub unsafe fn initWithContainer(
            this: Allocated<Self>,
            container: &CKContainer,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "CKRecord",
            feature = "CKRecordID",
            feature = "CKShare",
            feature = "block2"
        ))]
        #[method(systemSharingUIDidSaveShareBlock)]
        pub unsafe fn systemSharingUIDidSaveShareBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<CKRecordID>, *mut CKShare, *mut NSError)>;

        #[cfg(all(
            feature = "CKRecord",
            feature = "CKRecordID",
            feature = "CKShare",
            feature = "block2"
        ))]
        #[method(setSystemSharingUIDidSaveShareBlock:)]
        pub unsafe fn setSystemSharingUIDidSaveShareBlock(
            &self,
            system_sharing_ui_did_save_share_block: Option<
                &block2::Block<dyn Fn(NonNull<CKRecordID>, *mut CKShare, *mut NSError)>,
            >,
        );

        #[cfg(all(feature = "CKRecordID", feature = "block2"))]
        #[method(systemSharingUIDidStopSharingBlock)]
        pub unsafe fn systemSharingUIDidStopSharingBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<CKRecordID>, *mut NSError)>;

        #[cfg(all(feature = "CKRecordID", feature = "block2"))]
        #[method(setSystemSharingUIDidStopSharingBlock:)]
        pub unsafe fn setSystemSharingUIDidStopSharingBlock(
            &self,
            system_sharing_ui_did_stop_sharing_block: Option<
                &block2::Block<dyn Fn(NonNull<CKRecordID>, *mut NSError)>,
            >,
        );
    }
);
