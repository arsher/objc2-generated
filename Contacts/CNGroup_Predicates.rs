//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// Predicates
    /// The predicates to match groups against.
    ///
    ///
    /// Can only use these predicates with CNContactStore.
    #[cfg(feature = "CNGroup")]
    unsafe impl CNGroup {
        #[method_id(@__retain_semantics Other predicateForGroupsWithIdentifiers:)]
        pub unsafe fn predicateForGroupsWithIdentifiers(
            identifiers: &NSArray<NSString>,
        ) -> Retained<NSPredicate>;

        #[method_id(@__retain_semantics Other predicateForSubgroupsInGroupWithIdentifier:)]
        pub unsafe fn predicateForSubgroupsInGroupWithIdentifier(
            parent_group_identifier: &NSString,
        ) -> Retained<NSPredicate>;

        #[method_id(@__retain_semantics Other predicateForGroupsInContainerWithIdentifier:)]
        pub unsafe fn predicateForGroupsInContainerWithIdentifier(
            container_identifier: &NSString,
        ) -> Retained<NSPredicate>;
    }
);
