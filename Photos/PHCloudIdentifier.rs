//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/photos/phcloudidentifier?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHCloudIdentifier;
);

unsafe impl Send for PHCloudIdentifier {}

unsafe impl Sync for PHCloudIdentifier {}

unsafe impl NSCoding for PHCloudIdentifier {}

unsafe impl NSObjectProtocol for PHCloudIdentifier {}

unsafe impl NSSecureCoding for PHCloudIdentifier {}

extern_methods!(
    unsafe impl PHCloudIdentifier {
        /// DEPRECATED: If there is a failure to determine the global identifier for a local identifier, the notFoundIdentifier is provided in that array slot.
        #[deprecated]
        #[method_id(@__retain_semantics Other notFoundIdentifier)]
        pub unsafe fn notFoundIdentifier() -> Retained<PHCloudIdentifier>;

        #[method_id(@__retain_semantics Other stringValue)]
        pub unsafe fn stringValue(&self) -> Retained<NSString>;

        /// For use in serialization
        #[method_id(@__retain_semantics Init initWithStringValue:)]
        pub unsafe fn initWithStringValue(
            this: Allocated<Self>,
            string_value: &NSString,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHCloudIdentifier {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// Contains the cloud identifier result from looking up a local identifier via
    /// `cloudIdentifierMappingsForLocalIdentifiers,`or an
    /// `error`indicating why the lookup failed
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/photos/phcloudidentifiermapping?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHCloudIdentifierMapping;
);

unsafe impl Send for PHCloudIdentifierMapping {}

unsafe impl Sync for PHCloudIdentifierMapping {}

unsafe impl NSObjectProtocol for PHCloudIdentifierMapping {}

extern_methods!(
    unsafe impl PHCloudIdentifierMapping {
        #[method_id(@__retain_semantics Other cloudIdentifier)]
        pub unsafe fn cloudIdentifier(&self) -> Option<Retained<PHCloudIdentifier>>;

        /// The cloud identifier of the resource found for this local identifier
        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Option<Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHCloudIdentifierMapping {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// Contains the local identifier result from looking up a cloud identifier via
    /// `localIdentifierMappingsForCloudIdentifiers,`or an
    /// `error`indicating why the lookup failed
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/photos/phlocalidentifiermapping?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct PHLocalIdentifierMapping;
);

unsafe impl Send for PHLocalIdentifierMapping {}

unsafe impl Sync for PHLocalIdentifierMapping {}

unsafe impl NSObjectProtocol for PHLocalIdentifierMapping {}

extern_methods!(
    unsafe impl PHLocalIdentifierMapping {
        #[method_id(@__retain_semantics Other localIdentifier)]
        pub unsafe fn localIdentifier(&self) -> Option<Retained<NSString>>;

        /// The
        /// `NSString`representing the local identifier of the resource found for this cloud identifier, or nil if the match was not found.
        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Option<Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl PHLocalIdentifierMapping {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    unsafe impl PHCloudIdentifier {}
);

unsafe impl NSCopying for PHCloudIdentifier {}

unsafe impl CopyingHelper for PHCloudIdentifier {
    type Result = Self;
}

extern_methods!(
    /// CloudIdentifiers
    #[cfg(feature = "PHPhotoLibrary")]
    unsafe impl PHPhotoLibrary {
        /// Returns a dictionary that maps each cloud identifier from the provided array to a PLLocalIdentifierMapping result containing the local identifier found for that cloud identifier.
        ///
        /// This method can be very expensive so they should be used sparingly for batch lookup of all needed identifiers. Clients should work in terms of local identifiers and call these methods only once after loading from and before saving to persistent storage.  If the attempt to lookup a local identifier for a given cloud identifier fails, the error parameter will indicate the reason.
        ///
        /// Parameter `cloudIdentifiers`: The array of
        /// `PHCloudIdentifier`instances whose local identifiers are to being requested.
        #[method_id(@__retain_semantics Other localIdentifierMappingsForCloudIdentifiers:)]
        pub unsafe fn localIdentifierMappingsForCloudIdentifiers(
            &self,
            cloud_identifiers: &NSArray<PHCloudIdentifier>,
        ) -> Retained<NSDictionary<PHCloudIdentifier, PHLocalIdentifierMapping>>;

        /// Returns a dictionary that maps each local identifier from the provided array to a PLCloudIdentifierMapping result containing the cloud identifier found for that local identifier
        ///
        /// This method can be very expensive so they should be used sparingly for batch lookup of all needed identifiers. Clients should work in terms of local identifiers and call these methods only once after loading from and before saving to persistent storage.  If the attempt to lookup a cloud identifier for a given local identifier fails, the error parameter will indicate the reason.
        ///
        /// Parameter `localIdentifiers`: The array of
        /// `NSString`instances whose cloud identifiers are to being requested.
        #[method_id(@__retain_semantics Other cloudIdentifierMappingsForLocalIdentifiers:)]
        pub unsafe fn cloudIdentifierMappingsForLocalIdentifiers(
            &self,
            local_identifiers: &NSArray<NSString>,
        ) -> Retained<NSDictionary<NSString, PHCloudIdentifierMapping>>;

        /// DEPRECATED: These two methods can be very expensive so they should be used sparingly for batch lookup of all needed identifiers. Clients should work in terms of local identifiers and call these methods only once after loading from and before saving to persistent storage.
        #[deprecated]
        #[method_id(@__retain_semantics Other localIdentifiersForCloudIdentifiers:)]
        pub unsafe fn localIdentifiersForCloudIdentifiers(
            &self,
            cloud_identifiers: &NSArray<PHCloudIdentifier>,
        ) -> Retained<NSArray<NSString>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other cloudIdentifiersForLocalIdentifiers:)]
        pub unsafe fn cloudIdentifiersForLocalIdentifiers(
            &self,
            local_identifiers: &NSArray<NSString>,
        ) -> Retained<NSArray<PHCloudIdentifier>>;
    }
);

extern "C" {
    /// DEPRECATED: If the local object cannot be resolved from a global identifier, PHLocalIdentifierNotFound is provided in that array slot.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/photos/phlocalidentifiernotfound?language=objc)
    pub static PHLocalIdentifierNotFound: &'static NSString;
}
