//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Contains security information about a decoded message
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/mailkit/memessagesecurityinformation?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MEMessageSecurityInformation;
);

unsafe impl NSCoding for MEMessageSecurityInformation {}

unsafe impl NSObjectProtocol for MEMessageSecurityInformation {}

unsafe impl NSSecureCoding for MEMessageSecurityInformation {}

extern_methods!(
    unsafe impl MEMessageSecurityInformation {
        #[cfg(feature = "MEMessageSigner")]
        /// The signers of the message
        #[method_id(@__retain_semantics Other signers)]
        pub unsafe fn signers(&self) -> Retained<NSArray<MEMessageSigner>>;

        /// Whether or not the message was encrypted.
        #[method(isEncrypted)]
        pub unsafe fn isEncrypted(&self) -> bool;

        /// Any signing error that occured when decoding the message.
        #[method_id(@__retain_semantics Other signingError)]
        pub unsafe fn signingError(&self) -> Option<Retained<NSError>>;

        /// Any encryption error that occured when decoding the message.
        #[method_id(@__retain_semantics Other encryptionError)]
        pub unsafe fn encryptionError(&self) -> Option<Retained<NSError>>;

        /// Whether or not Mail should block loading remote content for the message by default. The user will have the option to load remote content manually.
        #[method(shouldBlockRemoteContent)]
        pub unsafe fn shouldBlockRemoteContent(&self) -> bool;

        /// A localized string containing the reason for blocking remote content.
        #[method_id(@__retain_semantics Other localizedRemoteContentBlockingReason)]
        pub unsafe fn localizedRemoteContentBlockingReason(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "MEMessageSigner")]
        #[method_id(@__retain_semantics Init initWithSigners:isEncrypted:signingError:encryptionError:)]
        pub unsafe fn initWithSigners_isEncrypted_signingError_encryptionError(
            this: Allocated<Self>,
            signers: &NSArray<MEMessageSigner>,
            is_encrypted: bool,
            signing_error: Option<&NSError>,
            encryption_error: Option<&NSError>,
        ) -> Retained<Self>;

        #[cfg(feature = "MEMessageSigner")]
        #[method_id(@__retain_semantics Init initWithSigners:isEncrypted:signingError:encryptionError:shouldBlockRemoteContent:localizedRemoteContentBlockingReason:)]
        pub unsafe fn initWithSigners_isEncrypted_signingError_encryptionError_shouldBlockRemoteContent_localizedRemoteContentBlockingReason(
            this: Allocated<Self>,
            signers: &NSArray<MEMessageSigner>,
            is_encrypted: bool,
            signing_error: Option<&NSError>,
            encryption_error: Option<&NSError>,
            should_block_remote_content: bool,
            localized_remote_content_blocking_reason: Option<&NSString>,
        ) -> Retained<Self>;
    }
);
