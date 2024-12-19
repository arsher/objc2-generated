//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileprovidererrordomain?language=objc)
    pub static NSFileProviderErrorDomain: &'static NSErrorDomain;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileprovidererrorcollidingitemkey?language=objc)
    pub static NSFileProviderErrorCollidingItemKey: &'static NSErrorUserInfoKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileprovidererroritemkey?language=objc)
    pub static NSFileProviderErrorItemKey: &'static NSErrorUserInfoKey;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileprovidererrornonexistentitemidentifierkey?language=objc)
    pub static NSFileProviderErrorNonExistentItemIdentifierKey: &'static NSErrorUserInfoKey;
}

/// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileprovidererrorcode?language=objc)
// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFileProviderErrorCode(pub NSInteger);
impl NSFileProviderErrorCode {
    pub const NSFileProviderErrorNotAuthenticated: Self = Self(-1000);
    pub const NSFileProviderErrorFilenameCollision: Self = Self(-1001);
    pub const NSFileProviderErrorSyncAnchorExpired: Self = Self(-1002);
    pub const NSFileProviderErrorPageExpired: Self =
        Self(NSFileProviderErrorCode::NSFileProviderErrorSyncAnchorExpired.0);
    pub const NSFileProviderErrorInsufficientQuota: Self = Self(-1003);
    pub const NSFileProviderErrorServerUnreachable: Self = Self(-1004);
    pub const NSFileProviderErrorNoSuchItem: Self = Self(-1005);
    pub const NSFileProviderErrorDeletionRejected: Self = Self(-1006);
    pub const NSFileProviderErrorDirectoryNotEmpty: Self = Self(-1007);
    pub const NSFileProviderErrorProviderNotFound: Self = Self(-2001);
    pub const NSFileProviderErrorProviderTranslocated: Self = Self(-2002);
    pub const NSFileProviderErrorOlderExtensionVersionRunning: Self = Self(-2003);
    pub const NSFileProviderErrorNewerExtensionVersionFound: Self = Self(-2004);
    pub const NSFileProviderErrorCannotSynchronize: Self = Self(-2005);
    pub const NSFileProviderErrorNonEvictableChildren: Self = Self(-2006);
    pub const NSFileProviderErrorUnsyncedEdits: Self = Self(-2007);
    pub const NSFileProviderErrorNonEvictable: Self = Self(-2008);
    pub const NSFileProviderErrorVersionNoLongerAvailable: Self = Self(-2009);
    pub const NSFileProviderErrorExcludedFromSync: Self = Self(-2010);
    pub const NSFileProviderErrorDomainDisabled: Self = Self(-2011);
    pub const NSFileProviderErrorProviderDomainTemporarilyUnavailable: Self = Self(-2012);
    pub const NSFileProviderErrorProviderDomainNotFound: Self = Self(-2013);
    pub const NSFileProviderErrorApplicationExtensionNotFound: Self = Self(-2014);
}

unsafe impl Encode for NSFileProviderErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSFileProviderErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_category!(
    /// Category "NSFileProviderError" on [`NSError`].
    #[doc(alias = "NSFileProviderError")]
    pub unsafe trait NSErrorNSFileProviderError {
        #[cfg(feature = "NSFileProviderItem")]
        #[method_id(@__retain_semantics Other fileProviderErrorForCollisionWithItem:)]
        unsafe fn fileProviderErrorForCollisionWithItem(
            existing_item: &NSFileProviderItem,
        ) -> Retained<Self>;

        #[cfg(feature = "NSFileProviderItem")]
        #[method_id(@__retain_semantics Other fileProviderErrorForNonExistentItemWithIdentifier:)]
        unsafe fn fileProviderErrorForNonExistentItemWithIdentifier(
            item_identifier: &NSFileProviderItemIdentifier,
        ) -> Retained<Self>;

        #[cfg(feature = "NSFileProviderItem")]
        #[method_id(@__retain_semantics Other fileProviderErrorForRejectedDeletionOfItem:)]
        unsafe fn fileProviderErrorForRejectedDeletionOfItem(
            updated_version: &NSFileProviderItem,
        ) -> Retained<Self>;
    }

    unsafe impl NSErrorNSFileProviderError for NSError {}
);
