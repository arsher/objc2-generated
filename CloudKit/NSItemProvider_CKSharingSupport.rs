//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

pub type CKSharePreparationCompletionHandler = *mut Block<(*mut CKShare, *mut NSError), ()>;

pub type CKSharePreparationHandler = *mut Block<(CKSharePreparationCompletionHandler,), ()>;

extern_methods!(
    /// CKSharingSupport
    #[cfg(feature = "Foundation_NSItemProvider")]
    unsafe impl NSItemProvider {
        #[cfg(all(
            feature = "CloudKit_CKAllowedSharingOptions",
            feature = "CloudKit_CKContainer"
        ))]
        #[method(registerCKShareWithContainer:allowedSharingOptions:preparationHandler:)]
        pub unsafe fn registerCKShareWithContainer_allowedSharingOptions_preparationHandler(
            &self,
            container: &CKContainer,
            allowed_options: &CKAllowedSharingOptions,
            preparation_handler: CKSharePreparationHandler,
        );

        #[cfg(all(
            feature = "CloudKit_CKAllowedSharingOptions",
            feature = "CloudKit_CKContainer",
            feature = "CloudKit_CKShare"
        ))]
        #[method(registerCKShare:container:allowedSharingOptions:)]
        pub unsafe fn registerCKShare_container_allowedSharingOptions(
            &self,
            share: &CKShare,
            container: &CKContainer,
            allowed_options: &CKAllowedSharingOptions,
        );
    }
);