//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/cloudkit/ckacceptsharesoperation?language=objc)
    #[unsafe(super(CKOperation, NSOperation, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CKOperation")]
    pub struct CKAcceptSharesOperation;
);

#[cfg(feature = "CKOperation")]
unsafe impl NSObjectProtocol for CKAcceptSharesOperation {}

extern_methods!(
    #[cfg(feature = "CKOperation")]
    unsafe impl CKAcceptSharesOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "CKShareMetadata")]
        #[method_id(@__retain_semantics Init initWithShareMetadatas:)]
        pub unsafe fn initWithShareMetadatas(
            this: Allocated<Self>,
            share_metadatas: &NSArray<CKShareMetadata>,
        ) -> Retained<Self>;

        #[cfg(feature = "CKShareMetadata")]
        #[method_id(@__retain_semantics Other shareMetadatas)]
        pub unsafe fn shareMetadatas(&self) -> Option<Retained<NSArray<CKShareMetadata>>>;

        #[cfg(feature = "CKShareMetadata")]
        /// Setter for [`shareMetadatas`][Self::shareMetadatas].
        #[method(setShareMetadatas:)]
        pub unsafe fn setShareMetadatas(&self, share_metadatas: Option<&NSArray<CKShareMetadata>>);

        #[cfg(all(
            feature = "CKRecord",
            feature = "CKShare",
            feature = "CKShareMetadata",
            feature = "block2"
        ))]
        /// Called once for each share metadata that the server processed
        ///
        ///
        /// If error is nil then the share was successfully accepted.
        /// Each
        /// `CKOperation`instance has a private serial queue. This queue is used for all callback block invocations.
        /// This block may share mutable state with other blocks assigned to this operation, but any such mutable state
        /// should not be concurrently used outside of blocks assigned to this operation.
        #[method(perShareCompletionBlock)]
        pub unsafe fn perShareCompletionBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<CKShareMetadata>, *mut CKShare, *mut NSError)>;

        #[cfg(all(
            feature = "CKRecord",
            feature = "CKShare",
            feature = "CKShareMetadata",
            feature = "block2"
        ))]
        /// Setter for [`perShareCompletionBlock`][Self::perShareCompletionBlock].
        #[method(setPerShareCompletionBlock:)]
        pub unsafe fn setPerShareCompletionBlock(
            &self,
            per_share_completion_block: Option<
                &block2::Block<dyn Fn(NonNull<CKShareMetadata>, *mut CKShare, *mut NSError)>,
            >,
        );

        #[cfg(feature = "block2")]
        /// This block is called when the operation completes.
        ///
        ///
        /// The
        ///
        /// ```text
        ///  -[NSOperation completionBlock]
        /// ```
        ///
        /// will also be called if both are set.
        /// If the error is
        /// `CKErrorPartialFailure,`the error's userInfo dictionary contains a dictionary of shareURLs to errors keyed off of
        /// `CKPartialErrorsByItemIDKey.`These errors are repeats of those sent back in previous
        /// `perShareCompletionBlock`invocations
        /// Each
        /// `CKOperation`instance has a private serial queue. This queue is used for all callback block invocations.
        /// This block may share mutable state with other blocks assigned to this operation, but any such mutable state
        /// should not be concurrently used outside of blocks assigned to this operation.
        #[method(acceptSharesCompletionBlock)]
        pub unsafe fn acceptSharesCompletionBlock(
            &self,
        ) -> *mut block2::Block<dyn Fn(*mut NSError)>;

        #[cfg(feature = "block2")]
        /// Setter for [`acceptSharesCompletionBlock`][Self::acceptSharesCompletionBlock].
        #[method(setAcceptSharesCompletionBlock:)]
        pub unsafe fn setAcceptSharesCompletionBlock(
            &self,
            accept_shares_completion_block: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CKOperation")]
    unsafe impl CKAcceptSharesOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
