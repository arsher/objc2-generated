//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/photos/phauthorizationstatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PHAuthorizationStatus(pub NSInteger);
impl PHAuthorizationStatus {
    #[doc(alias = "PHAuthorizationStatusNotDetermined")]
    pub const NotDetermined: Self = Self(0);
    #[doc(alias = "PHAuthorizationStatusRestricted")]
    pub const Restricted: Self = Self(1);
    #[doc(alias = "PHAuthorizationStatusDenied")]
    pub const Denied: Self = Self(2);
    #[doc(alias = "PHAuthorizationStatusAuthorized")]
    pub const Authorized: Self = Self(3);
    #[doc(alias = "PHAuthorizationStatusLimited")]
    pub const Limited: Self = Self(4);
}

unsafe impl Encode for PHAuthorizationStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for PHAuthorizationStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/photos/phaccesslevel?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PHAccessLevel(pub NSInteger);
impl PHAccessLevel {
    #[doc(alias = "PHAccessLevelAddOnly")]
    pub const AddOnly: Self = Self(1);
    #[doc(alias = "PHAccessLevelReadWrite")]
    pub const ReadWrite: Self = Self(2);
}

unsafe impl Encode for PHAccessLevel {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for PHAccessLevel {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phphotolibrarychangeobserver?language=objc)
    pub unsafe trait PHPhotoLibraryChangeObserver: NSObjectProtocol {
        #[cfg(feature = "PHChange")]
        #[method(photoLibraryDidChange:)]
        unsafe fn photoLibraryDidChange(&self, change_instance: &PHChange);
    }

    unsafe impl ProtocolType for dyn PHPhotoLibraryChangeObserver {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phphotolibraryavailabilityobserver?language=objc)
    pub unsafe trait PHPhotoLibraryAvailabilityObserver: NSObjectProtocol {
        #[method(photoLibraryDidBecomeUnavailable:)]
        unsafe fn photoLibraryDidBecomeUnavailable(&self, photo_library: &PHPhotoLibrary);
    }

    unsafe impl ProtocolType for dyn PHPhotoLibraryAvailabilityObserver {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phphotolibrary?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHPhotoLibrary;
);

unsafe impl Send for PHPhotoLibrary {}

unsafe impl Sync for PHPhotoLibrary {}

unsafe impl NSObjectProtocol for PHPhotoLibrary {}

extern_methods!(
    unsafe impl PHPhotoLibrary {
        #[method_id(@__retain_semantics Other sharedPhotoLibrary)]
        pub unsafe fn sharedPhotoLibrary() -> Retained<PHPhotoLibrary>;

        #[method(authorizationStatusForAccessLevel:)]
        pub unsafe fn authorizationStatusForAccessLevel(
            access_level: PHAccessLevel,
        ) -> PHAuthorizationStatus;

        #[cfg(feature = "block2")]
        #[method(requestAuthorizationForAccessLevel:handler:)]
        pub unsafe fn requestAuthorizationForAccessLevel_handler(
            access_level: PHAccessLevel,
            handler: &block2::Block<dyn Fn(PHAuthorizationStatus)>,
        );

        #[deprecated]
        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus() -> PHAuthorizationStatus;

        #[cfg(feature = "block2")]
        #[deprecated]
        #[method(requestAuthorization:)]
        pub unsafe fn requestAuthorization(handler: &block2::Block<dyn Fn(PHAuthorizationStatus)>);

        #[method_id(@__retain_semantics Other unavailabilityReason)]
        pub unsafe fn unavailabilityReason(&self) -> Option<Retained<NSError>>;

        #[method(registerAvailabilityObserver:)]
        pub unsafe fn registerAvailabilityObserver(
            &self,
            observer: &ProtocolObject<dyn PHPhotoLibraryAvailabilityObserver>,
        );

        #[method(unregisterAvailabilityObserver:)]
        pub unsafe fn unregisterAvailabilityObserver(
            &self,
            observer: &ProtocolObject<dyn PHPhotoLibraryAvailabilityObserver>,
        );

        #[method(registerChangeObserver:)]
        pub unsafe fn registerChangeObserver(
            &self,
            observer: &ProtocolObject<dyn PHPhotoLibraryChangeObserver>,
        );

        #[method(unregisterChangeObserver:)]
        pub unsafe fn unregisterChangeObserver(
            &self,
            observer: &ProtocolObject<dyn PHPhotoLibraryChangeObserver>,
        );

        #[cfg(all(
            feature = "PHPersistentChangeFetchResult",
            feature = "PHPersistentChangeToken"
        ))]
        #[method_id(@__retain_semantics Other fetchPersistentChangesSinceToken:error:_)]
        pub unsafe fn fetchPersistentChangesSinceToken_error(
            &self,
            token: &PHPersistentChangeToken,
        ) -> Result<Retained<PHPersistentChangeFetchResult>, Retained<NSError>>;

        #[cfg(feature = "PHPersistentChangeToken")]
        #[method_id(@__retain_semantics Other currentChangeToken)]
        pub unsafe fn currentChangeToken(&self) -> Retained<PHPersistentChangeToken>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHPhotoLibrary {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
