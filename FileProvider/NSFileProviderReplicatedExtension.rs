//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFileProviderCreateItemOptions(pub NSUInteger);
impl NSFileProviderCreateItemOptions {
    pub const NSFileProviderCreateItemMayAlreadyExist: Self = Self(1 << 0);
    pub const NSFileProviderCreateItemDeletionConflicted: Self = Self(1 << 1);
}

unsafe impl Encode for NSFileProviderCreateItemOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFileProviderCreateItemOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFileProviderDeleteItemOptions(pub NSUInteger);
impl NSFileProviderDeleteItemOptions {
    pub const NSFileProviderDeleteItemRecursive: Self = Self(1 << 0);
}

unsafe impl Encode for NSFileProviderDeleteItemOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFileProviderDeleteItemOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFileProviderMaterializationFlags(pub NSUInteger);
impl NSFileProviderMaterializationFlags {
    #[doc(alias = "NSFileProviderMaterializationFlagsKnownSparseRanges")]
    pub const KnownSparseRanges: Self = Self(1 << 0);
}

unsafe impl Encode for NSFileProviderMaterializationFlags {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFileProviderMaterializationFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFileProviderFetchContentsOptions(pub NSUInteger);
impl NSFileProviderFetchContentsOptions {
    #[doc(alias = "NSFileProviderFetchContentsOptionsStrictVersioning")]
    pub const StrictVersioning: Self = Self(1 << 0);
}

unsafe impl Encode for NSFileProviderFetchContentsOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFileProviderFetchContentsOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait NSFileProviderEnumerating: NSObjectProtocol {
        #[cfg(all(
            feature = "NSFileProviderEnumerating",
            feature = "NSFileProviderItem",
            feature = "NSFileProviderRequest"
        ))]
        #[method_id(@__retain_semantics Other enumeratorForContainerItemIdentifier:request:error:_)]
        unsafe fn enumeratorForContainerItemIdentifier_request_error(
            &self,
            container_item_identifier: &NSFileProviderItemIdentifier,
            request: &NSFileProviderRequest,
        ) -> Result<Id<ProtocolObject<dyn NSFileProviderEnumerator>>, Id<NSError>>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderEnumerating {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderReplicatedExtension:
        NSFileProviderEnumerating + NSObjectProtocol
    {
        #[cfg(feature = "NSFileProviderDomain")]
        #[method_id(@__retain_semantics Init initWithDomain:)]
        unsafe fn initWithDomain(this: Allocated<Self>, domain: &NSFileProviderDomain) -> Id<Self>;

        #[method(invalidate)]
        unsafe fn invalidate(&self);

        #[cfg(all(
            feature = "NSFileProviderItem",
            feature = "NSFileProviderRequest",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other itemForIdentifier:request:completionHandler:)]
        unsafe fn itemForIdentifier_request_completionHandler(
            &self,
            identifier: &NSFileProviderItemIdentifier,
            request: &NSFileProviderRequest,
            completion_handler: &block2::Block<dyn Fn(*mut NSFileProviderItem, *mut NSError)>,
        ) -> Id<NSProgress>;

        #[cfg(all(
            feature = "NSFileProviderItem",
            feature = "NSFileProviderRequest",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other fetchContentsForItemWithIdentifier:version:request:completionHandler:)]
        unsafe fn fetchContentsForItemWithIdentifier_version_request_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            requested_version: Option<&NSFileProviderItemVersion>,
            request: &NSFileProviderRequest,
            completion_handler: &block2::Block<
                dyn Fn(*mut NSURL, *mut NSFileProviderItem, *mut NSError),
            >,
        ) -> Id<NSProgress>;

        #[cfg(all(
            feature = "NSFileProviderItem",
            feature = "NSFileProviderRequest",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other createItemBasedOnTemplate:fields:contents:options:request:completionHandler:)]
        unsafe fn createItemBasedOnTemplate_fields_contents_options_request_completionHandler(
            &self,
            item_template: &NSFileProviderItem,
            fields: NSFileProviderItemFields,
            url: Option<&NSURL>,
            options: NSFileProviderCreateItemOptions,
            request: &NSFileProviderRequest,
            completion_handler: &block2::Block<
                dyn Fn(*mut NSFileProviderItem, NSFileProviderItemFields, Bool, *mut NSError),
            >,
        ) -> Id<NSProgress>;

        #[cfg(all(
            feature = "NSFileProviderItem",
            feature = "NSFileProviderModifyItemOptions",
            feature = "NSFileProviderRequest",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other modifyItem:baseVersion:changedFields:contents:options:request:completionHandler:)]
        unsafe fn modifyItem_baseVersion_changedFields_contents_options_request_completionHandler(
            &self,
            item: &NSFileProviderItem,
            version: &NSFileProviderItemVersion,
            changed_fields: NSFileProviderItemFields,
            new_contents: Option<&NSURL>,
            options: NSFileProviderModifyItemOptions,
            request: &NSFileProviderRequest,
            completion_handler: &block2::Block<
                dyn Fn(*mut NSFileProviderItem, NSFileProviderItemFields, Bool, *mut NSError),
            >,
        ) -> Id<NSProgress>;

        #[cfg(all(
            feature = "NSFileProviderItem",
            feature = "NSFileProviderRequest",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other deleteItemWithIdentifier:baseVersion:options:request:completionHandler:)]
        unsafe fn deleteItemWithIdentifier_baseVersion_options_request_completionHandler(
            &self,
            identifier: &NSFileProviderItemIdentifier,
            version: &NSFileProviderItemVersion,
            options: NSFileProviderDeleteItemOptions,
            request: &NSFileProviderRequest,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        ) -> Id<NSProgress>;

        #[cfg(feature = "block2")]
        #[optional]
        #[method(importDidFinishWithCompletionHandler:)]
        unsafe fn importDidFinishWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn()>,
        );

        #[cfg(feature = "block2")]
        #[optional]
        #[method(materializedItemsDidChangeWithCompletionHandler:)]
        unsafe fn materializedItemsDidChangeWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn()>,
        );

        #[cfg(feature = "block2")]
        #[optional]
        #[method(pendingItemsDidChangeWithCompletionHandler:)]
        unsafe fn pendingItemsDidChangeWithCompletionHandler(
            &self,
            completion_handler: &block2::Block<dyn Fn()>,
        );
    }

    unsafe impl ProtocolType for dyn NSFileProviderReplicatedExtension {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderIncrementalContentFetching: NSObjectProtocol {
        #[cfg(all(
            feature = "NSFileProviderItem",
            feature = "NSFileProviderRequest",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other fetchContentsForItemWithIdentifier:version:usingExistingContentsAtURL:existingVersion:request:completionHandler:)]
        unsafe fn fetchContentsForItemWithIdentifier_version_usingExistingContentsAtURL_existingVersion_request_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            requested_version: Option<&NSFileProviderItemVersion>,
            existing_contents: &NSURL,
            existing_version: &NSFileProviderItemVersion,
            request: &NSFileProviderRequest,
            completion_handler: &block2::Block<
                dyn Fn(*mut NSURL, *mut NSFileProviderItem, *mut NSError),
            >,
        ) -> Id<NSProgress>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderIncrementalContentFetching {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderServicing: NSObjectProtocol {
        #[cfg(all(
            feature = "NSFileProviderItem",
            feature = "NSFileProviderService",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other supportedServiceSourcesForItemIdentifier:completionHandler:)]
        unsafe fn supportedServiceSourcesForItemIdentifier_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &block2::Block<
                dyn Fn(*mut NSArray<ProtocolObject<dyn NSFileProviderServiceSource>>, *mut NSError),
            >,
        ) -> Id<NSProgress>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderServicing {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderThumbnailing: NSObjectProtocol {
        #[cfg(all(feature = "NSFileProviderItem", feature = "block2"))]
        #[method_id(@__retain_semantics Other fetchThumbnailsForItemIdentifiers:requestedSize:perThumbnailCompletionHandler:completionHandler:)]
        unsafe fn fetchThumbnailsForItemIdentifiers_requestedSize_perThumbnailCompletionHandler_completionHandler(
            &self,
            item_identifiers: &NSArray<NSFileProviderItemIdentifier>,
            size: CGSize,
            per_thumbnail_completion_handler: &block2::Block<
                dyn Fn(NonNull<NSFileProviderItemIdentifier>, *mut NSData, *mut NSError),
            >,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        ) -> Id<NSProgress>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderThumbnailing {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderCustomAction: NSObjectProtocol {
        #[cfg(all(
            feature = "NSFileProviderActions",
            feature = "NSFileProviderItem",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other performActionWithIdentifier:onItemsWithIdentifiers:completionHandler:)]
        unsafe fn performActionWithIdentifier_onItemsWithIdentifiers_completionHandler(
            &self,
            action_identifier: &NSFileProviderExtensionActionIdentifier,
            item_identifiers: &NSArray<NSFileProviderItemIdentifier>,
            completion_handler: &block2::Block<dyn Fn(*mut NSError)>,
        ) -> Id<NSProgress>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderCustomAction {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderUserInteractionSuppressing: NSObjectProtocol {
        #[method(setInteractionSuppressed:forIdentifier:)]
        unsafe fn setInteractionSuppressed_forIdentifier(
            &self,
            suppression: bool,
            suppression_identifier: &NSString,
        );

        #[method(isInteractionSuppressedForIdentifier:)]
        unsafe fn isInteractionSuppressedForIdentifier(
            &self,
            suppression_identifier: &NSString,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn NSFileProviderUserInteractionSuppressing {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderDomainState: NSObjectProtocol {
        #[cfg(feature = "NSFileProviderDomain")]
        #[method_id(@__retain_semantics Other domainVersion)]
        unsafe fn domainVersion(&self) -> Id<NSFileProviderDomainVersion>;

        #[method_id(@__retain_semantics Other userInfo)]
        unsafe fn userInfo(&self) -> Id<NSDictionary>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderDomainState {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderPartialContentFetching: NSObjectProtocol {
        #[cfg(all(
            feature = "NSFileProviderItem",
            feature = "NSFileProviderRequest",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Other fetchPartialContentsForItemWithIdentifier:version:request:minimalRange:aligningTo:options:completionHandler:)]
        unsafe fn fetchPartialContentsForItemWithIdentifier_version_request_minimalRange_aligningTo_options_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            requested_version: &NSFileProviderItemVersion,
            request: &NSFileProviderRequest,
            requested_range: NSRange,
            alignment: NSUInteger,
            options: NSFileProviderFetchContentsOptions,
            completion_handler: &block2::Block<
                dyn Fn(
                    *mut NSURL,
                    *mut NSFileProviderItem,
                    NSRange,
                    NSFileProviderMaterializationFlags,
                    *mut NSError,
                ),
            >,
        ) -> Id<NSProgress>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderPartialContentFetching {}
);
