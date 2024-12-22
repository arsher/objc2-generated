//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksharingparticipantaccessoption?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CKSharingParticipantAccessOption(pub NSUInteger);
bitflags::bitflags! {
    impl CKSharingParticipantAccessOption: NSUInteger {
/// If specified, the system sharing UI will allow the user to share publicly i.e. anyone with the link has access.
        #[doc(alias = "CKSharingParticipantAccessOptionAnyoneWithLink")]
        const AnyoneWithLink = 1<<0;
/// If specified, the system sharing UI will allow the user to share privately to specified recipients.
        #[doc(alias = "CKSharingParticipantAccessOptionSpecifiedRecipientsOnly")]
        const SpecifiedRecipientsOnly = 1<<1;
/// Allow the user to configure the share with either access option.
        #[doc(alias = "CKSharingParticipantAccessOptionAny")]
        const Any = CKSharingParticipantAccessOption::AnyoneWithLink.0|CKSharingParticipantAccessOption::SpecifiedRecipientsOnly.0;
    }
}

unsafe impl Encode for CKSharingParticipantAccessOption {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for CKSharingParticipantAccessOption {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/cksharingparticipantpermissionoption?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CKSharingParticipantPermissionOption(pub NSUInteger);
bitflags::bitflags! {
    impl CKSharingParticipantPermissionOption: NSUInteger {
/// If specified, the system sharing UI will allow the user to grant participants read-only permissions.
        #[doc(alias = "CKSharingParticipantPermissionOptionReadOnly")]
        const ReadOnly = 1<<0;
/// If specified, the system sharing UI will allow the user to grant participants read/write permissions.
        #[doc(alias = "CKSharingParticipantPermissionOptionReadWrite")]
        const ReadWrite = 1<<1;
/// Allow the user to configure added share participants with either permission option.
        #[doc(alias = "CKSharingParticipantPermissionOptionAny")]
        const Any = CKSharingParticipantPermissionOption::ReadOnly.0|CKSharingParticipantPermissionOption::ReadWrite.0;
    }
}

unsafe impl Encode for CKSharingParticipantPermissionOption {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for CKSharingParticipantPermissionOption {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckallowedsharingoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKAllowedSharingOptions;
);

unsafe impl NSCoding for CKAllowedSharingOptions {}

unsafe impl NSCopying for CKAllowedSharingOptions {}

unsafe impl CopyingHelper for CKAllowedSharingOptions {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CKAllowedSharingOptions {}

unsafe impl NSSecureCoding for CKAllowedSharingOptions {}

extern_methods!(
    unsafe impl CKAllowedSharingOptions {
        #[method_id(@__retain_semantics Init initWithAllowedParticipantPermissionOptions:allowedParticipantAccessOptions:)]
        pub unsafe fn initWithAllowedParticipantPermissionOptions_allowedParticipantAccessOptions(
            this: Allocated<Self>,
            allowed_participant_permission_options: CKSharingParticipantPermissionOption,
            allowed_participant_access_options: CKSharingParticipantAccessOption,
        ) -> Retained<Self>;

        #[method(allowedParticipantPermissionOptions)]
        pub unsafe fn allowedParticipantPermissionOptions(
            &self,
        ) -> CKSharingParticipantPermissionOption;

        /// Setter for [`allowedParticipantPermissionOptions`][Self::allowedParticipantPermissionOptions].
        #[method(setAllowedParticipantPermissionOptions:)]
        pub unsafe fn setAllowedParticipantPermissionOptions(
            &self,
            allowed_participant_permission_options: CKSharingParticipantPermissionOption,
        );

        #[method(allowedParticipantAccessOptions)]
        pub unsafe fn allowedParticipantAccessOptions(&self) -> CKSharingParticipantAccessOption;

        /// Setter for [`allowedParticipantAccessOptions`][Self::allowedParticipantAccessOptions].
        #[method(setAllowedParticipantAccessOptions:)]
        pub unsafe fn setAllowedParticipantAccessOptions(
            &self,
            allowed_participant_access_options: CKSharingParticipantAccessOption,
        );

        /// Standard allowed options are most permissive i.e.
        /// `allowedParticipantPermissionOptions`=
        /// `CKSharingParticipantPermissionOptionAny`and
        /// `allowedParticipantAccessOptions`=
        /// `CKSharingParticipantAccessOptionAny`
        #[method_id(@__retain_semantics Other standardOptions)]
        pub unsafe fn standardOptions() -> Retained<CKAllowedSharingOptions>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl CKAllowedSharingOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
