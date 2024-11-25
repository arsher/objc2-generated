//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckshareparticipantacceptancestatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CKShareParticipantAcceptanceStatus(pub NSInteger);
impl CKShareParticipantAcceptanceStatus {
    #[doc(alias = "CKShareParticipantAcceptanceStatusUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "CKShareParticipantAcceptanceStatusPending")]
    pub const Pending: Self = Self(1);
    #[doc(alias = "CKShareParticipantAcceptanceStatusAccepted")]
    pub const Accepted: Self = Self(2);
    #[doc(alias = "CKShareParticipantAcceptanceStatusRemoved")]
    pub const Removed: Self = Self(3);
}

unsafe impl Encode for CKShareParticipantAcceptanceStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CKShareParticipantAcceptanceStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckshareparticipantpermission?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CKShareParticipantPermission(pub NSInteger);
impl CKShareParticipantPermission {
    #[doc(alias = "CKShareParticipantPermissionUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "CKShareParticipantPermissionNone")]
    pub const None: Self = Self(1);
    #[doc(alias = "CKShareParticipantPermissionReadOnly")]
    pub const ReadOnly: Self = Self(2);
    #[doc(alias = "CKShareParticipantPermissionReadWrite")]
    pub const ReadWrite: Self = Self(3);
}

unsafe impl Encode for CKShareParticipantPermission {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CKShareParticipantPermission {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckshareparticipantrole?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CKShareParticipantRole(pub NSInteger);
impl CKShareParticipantRole {
    #[doc(alias = "CKShareParticipantRoleUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "CKShareParticipantRoleOwner")]
    pub const Owner: Self = Self(1);
    #[doc(alias = "CKShareParticipantRolePrivateUser")]
    pub const PrivateUser: Self = Self(3);
    #[doc(alias = "CKShareParticipantRolePublicUser")]
    pub const PublicUser: Self = Self(4);
}

unsafe impl Encode for CKShareParticipantRole {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CKShareParticipantRole {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckshareparticipanttype?language=objc)
// NS_ENUM
#[deprecated]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CKShareParticipantType(pub NSInteger);
impl CKShareParticipantType {
    #[deprecated]
    #[doc(alias = "CKShareParticipantTypeUnknown")]
    pub const Unknown: Self = Self(0);
    #[deprecated]
    #[doc(alias = "CKShareParticipantTypeOwner")]
    pub const Owner: Self = Self(1);
    #[deprecated]
    #[doc(alias = "CKShareParticipantTypePrivateUser")]
    pub const PrivateUser: Self = Self(3);
    #[deprecated]
    #[doc(alias = "CKShareParticipantTypePublicUser")]
    pub const PublicUser: Self = Self(4);
}

unsafe impl Encode for CKShareParticipantType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CKShareParticipantType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckshareparticipant?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CKShareParticipant;
);

unsafe impl NSCoding for CKShareParticipant {}

unsafe impl NSCopying for CKShareParticipant {}

unsafe impl CopyingHelper for CKShareParticipant {
    type Result = Self;
}

unsafe impl NSObjectProtocol for CKShareParticipant {}

unsafe impl NSSecureCoding for CKShareParticipant {}

extern_methods!(
    unsafe impl CKShareParticipant {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "CKUserIdentity")]
        #[method_id(@__retain_semantics Other userIdentity)]
        pub unsafe fn userIdentity(&self) -> Retained<CKUserIdentity>;

        #[method(role)]
        pub unsafe fn role(&self) -> CKShareParticipantRole;

        #[method(setRole:)]
        pub unsafe fn setRole(&self, role: CKShareParticipantRole);

        #[deprecated]
        #[method(type)]
        pub unsafe fn r#type(&self) -> CKShareParticipantType;

        #[deprecated]
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: CKShareParticipantType);

        #[method(acceptanceStatus)]
        pub unsafe fn acceptanceStatus(&self) -> CKShareParticipantAcceptanceStatus;

        #[method(permission)]
        pub unsafe fn permission(&self) -> CKShareParticipantPermission;

        #[method(setPermission:)]
        pub unsafe fn setPermission(&self, permission: CKShareParticipantPermission);

        #[method_id(@__retain_semantics Other participantID)]
        pub unsafe fn participantID(&self) -> Retained<NSString>;
    }
);
