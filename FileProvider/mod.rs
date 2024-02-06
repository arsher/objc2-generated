// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `FileProvider` framework
#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "FileProvider", kind = "framework")]
extern "C" {}

#[path = "NSFileProviderActions.rs"]
mod __NSFileProviderActions;
#[path = "NSFileProviderDefines.rs"]
mod __NSFileProviderDefines;
#[path = "NSFileProviderDomain.rs"]
mod __NSFileProviderDomain;
#[path = "NSFileProviderEnumerating.rs"]
mod __NSFileProviderEnumerating;
#[path = "NSFileProviderError.rs"]
mod __NSFileProviderError;
#[path = "NSFileProviderExtension.rs"]
mod __NSFileProviderExtension;
#[path = "NSFileProviderItem.rs"]
mod __NSFileProviderItem;
#[path = "NSFileProviderItemDecoration.rs"]
mod __NSFileProviderItemDecoration;
#[path = "NSFileProviderManager.rs"]
mod __NSFileProviderManager;
#[path = "NSFileProviderModifyItemOptions.rs"]
mod __NSFileProviderModifyItemOptions;
#[path = "NSFileProviderReplicatedExtension.rs"]
mod __NSFileProviderReplicatedExtension;
#[path = "NSFileProviderRequest.rs"]
mod __NSFileProviderRequest;
#[path = "NSFileProviderService.rs"]
mod __NSFileProviderService;
#[path = "NSFileProviderTesting.rs"]
mod __NSFileProviderTesting;
#[path = "NSFileProviderThumbnailing.rs"]
mod __NSFileProviderThumbnailing;

pub use self::__NSFileProviderActions::NSFileProviderExtensionActionIdentifier;
#[cfg(feature = "FileProvider_NSFileProviderDomain")]
pub use self::__NSFileProviderDomain::NSFileProviderDomain;
pub use self::__NSFileProviderDomain::NSFileProviderDomainDidChange;
pub use self::__NSFileProviderDomain::NSFileProviderDomainIdentifier;
pub use self::__NSFileProviderDomain::NSFileProviderDomainTestingModes;
#[cfg(feature = "FileProvider_NSFileProviderDomainVersion")]
pub use self::__NSFileProviderDomain::NSFileProviderDomainVersion;
pub use self::__NSFileProviderDomain::{
    NSFileProviderDomainTestingModeAlwaysEnabled, NSFileProviderDomainTestingModeInteractive,
};
pub use self::__NSFileProviderEnumerating::NSFileProviderChangeObserver;
pub use self::__NSFileProviderEnumerating::NSFileProviderEnumerationObserver;
pub use self::__NSFileProviderEnumerating::NSFileProviderEnumerator;
pub use self::__NSFileProviderEnumerating::NSFileProviderInitialPageSortedByDate;
pub use self::__NSFileProviderEnumerating::NSFileProviderInitialPageSortedByName;
pub use self::__NSFileProviderEnumerating::NSFileProviderPage;
pub use self::__NSFileProviderEnumerating::NSFileProviderSyncAnchor;
#[cfg(feature = "Foundation_NSError")]
pub use self::__NSFileProviderError::NSErrorNSFileProviderError;
pub use self::__NSFileProviderError::NSFileProviderErrorCode;
pub use self::__NSFileProviderError::NSFileProviderErrorCollidingItemKey;
pub use self::__NSFileProviderError::NSFileProviderErrorDomain;
pub use self::__NSFileProviderError::NSFileProviderErrorItemKey;
pub use self::__NSFileProviderError::NSFileProviderErrorNonExistentItemIdentifierKey;
pub use self::__NSFileProviderError::{
    NSFileProviderErrorCannotSynchronize, NSFileProviderErrorDeletionRejected,
    NSFileProviderErrorDirectoryNotEmpty, NSFileProviderErrorDomainDisabled,
    NSFileProviderErrorExcludedFromSync, NSFileProviderErrorFilenameCollision,
    NSFileProviderErrorInsufficientQuota, NSFileProviderErrorNewerExtensionVersionFound,
    NSFileProviderErrorNoSuchItem, NSFileProviderErrorNonEvictable,
    NSFileProviderErrorNonEvictableChildren, NSFileProviderErrorNotAuthenticated,
    NSFileProviderErrorOlderExtensionVersionRunning, NSFileProviderErrorPageExpired,
    NSFileProviderErrorProviderNotFound, NSFileProviderErrorProviderTranslocated,
    NSFileProviderErrorServerUnreachable, NSFileProviderErrorSyncAnchorExpired,
    NSFileProviderErrorUnsyncedEdits, NSFileProviderErrorVersionNoLongerAvailable,
};
#[cfg(feature = "FileProvider_NSFileProviderExtension")]
pub use self::__NSFileProviderExtension::NSFileProviderExtension;
pub use self::__NSFileProviderItem::NSFileProviderContentPolicy;
pub use self::__NSFileProviderItem::NSFileProviderFavoriteRankUnranked;
pub use self::__NSFileProviderItem::NSFileProviderFileSystemFlags;
pub use self::__NSFileProviderItem::NSFileProviderItem;
pub use self::__NSFileProviderItem::NSFileProviderItemCapabilities;
pub use self::__NSFileProviderItem::NSFileProviderItemFields;
pub use self::__NSFileProviderItem::NSFileProviderItemIdentifier;
pub use self::__NSFileProviderItem::NSFileProviderItemProtocol;
#[cfg(feature = "FileProvider_NSFileProviderItemVersion")]
pub use self::__NSFileProviderItem::NSFileProviderItemVersion;
pub use self::__NSFileProviderItem::NSFileProviderRootContainerItemIdentifier;
pub use self::__NSFileProviderItem::NSFileProviderTrashContainerItemIdentifier;
pub use self::__NSFileProviderItem::NSFileProviderTypeAndCreator;
pub use self::__NSFileProviderItem::NSFileProviderWorkingSetContainerItemIdentifier;
pub use self::__NSFileProviderItem::{
    NSFileProviderContentPolicyDownloadEagerlyAndKeepDownloaded,
    NSFileProviderContentPolicyDownloadLazily,
    NSFileProviderContentPolicyDownloadLazilyAndEvictOnRemoteUpdate,
    NSFileProviderContentPolicyInherited,
};
pub use self::__NSFileProviderItem::{
    NSFileProviderFileSystemHidden, NSFileProviderFileSystemPathExtensionHidden,
    NSFileProviderFileSystemUserExecutable, NSFileProviderFileSystemUserReadable,
    NSFileProviderFileSystemUserWritable,
};
pub use self::__NSFileProviderItem::{
    NSFileProviderItemCapabilitiesAllowsAddingSubItems, NSFileProviderItemCapabilitiesAllowsAll,
    NSFileProviderItemCapabilitiesAllowsContentEnumerating,
    NSFileProviderItemCapabilitiesAllowsDeleting, NSFileProviderItemCapabilitiesAllowsEvicting,
    NSFileProviderItemCapabilitiesAllowsExcludingFromSync,
    NSFileProviderItemCapabilitiesAllowsReading, NSFileProviderItemCapabilitiesAllowsRenaming,
    NSFileProviderItemCapabilitiesAllowsReparenting, NSFileProviderItemCapabilitiesAllowsTrashing,
    NSFileProviderItemCapabilitiesAllowsWriting,
};
pub use self::__NSFileProviderItem::{
    NSFileProviderItemContentModificationDate, NSFileProviderItemContents,
    NSFileProviderItemCreationDate, NSFileProviderItemExtendedAttributes,
    NSFileProviderItemFavoriteRank, NSFileProviderItemFileSystemFlags, NSFileProviderItemFilename,
    NSFileProviderItemLastUsedDate, NSFileProviderItemParentItemIdentifier,
    NSFileProviderItemTagData, NSFileProviderItemTypeAndCreator,
};
pub use self::__NSFileProviderItemDecoration::NSFileProviderItemDecorating;
pub use self::__NSFileProviderItemDecoration::NSFileProviderItemDecorationIdentifier;
pub use self::__NSFileProviderManager::NSFileProviderDomainRemovalMode;
#[cfg(feature = "FileProvider_NSFileProviderManager")]
pub use self::__NSFileProviderManager::NSFileProviderManager;
pub use self::__NSFileProviderManager::NSFileProviderManagerDisconnectionOptions;
pub use self::__NSFileProviderManager::NSFileProviderManagerDisconnectionOptionsTemporary;
pub use self::__NSFileProviderManager::NSFileProviderMaterializedSetDidChange;
pub use self::__NSFileProviderManager::NSFileProviderPendingSetDidChange;
pub use self::__NSFileProviderManager::NSFileProviderPendingSetEnumerator;
pub use self::__NSFileProviderManager::{
    NSFileProviderDomainRemovalModePreserveDirtyUserData,
    NSFileProviderDomainRemovalModePreserveDownloadedUserData,
    NSFileProviderDomainRemovalModeRemoveAll,
};
pub use self::__NSFileProviderModifyItemOptions::NSFileProviderModifyItemMayAlreadyExist;
pub use self::__NSFileProviderModifyItemOptions::NSFileProviderModifyItemOptions;
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderCreateItemOptions;
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderCustomAction;
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderDeleteItemOptions;
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderDeleteItemRecursive;
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderDomainState;
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderEnumerating;
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderFetchContentsOptions;
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderFetchContentsOptionsStrictVersioning;
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderIncrementalContentFetching;
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderMaterializationFlags;
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderMaterializationFlagsKnownSparseRanges;
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderPartialContentFetching;
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderReplicatedExtension;
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderServicing;
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderThumbnailing;
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderUserInteractionSuppressing;
pub use self::__NSFileProviderReplicatedExtension::{
    NSFileProviderCreateItemDeletionConflicted, NSFileProviderCreateItemMayAlreadyExist,
};
#[cfg(feature = "FileProvider_NSFileProviderRequest")]
pub use self::__NSFileProviderRequest::NSFileProviderRequest;
pub use self::__NSFileProviderService::NSFileProviderServiceSource;
pub use self::__NSFileProviderTesting::NSFileProviderTestingChildrenEnumeration;
pub use self::__NSFileProviderTesting::NSFileProviderTestingCollisionResolution;
pub use self::__NSFileProviderTesting::NSFileProviderTestingContentFetch;
pub use self::__NSFileProviderTesting::NSFileProviderTestingCreation;
pub use self::__NSFileProviderTesting::NSFileProviderTestingDeletion;
pub use self::__NSFileProviderTesting::NSFileProviderTestingIngestion;
pub use self::__NSFileProviderTesting::NSFileProviderTestingLookup;
pub use self::__NSFileProviderTesting::NSFileProviderTestingModification;
pub use self::__NSFileProviderTesting::NSFileProviderTestingOperation;
pub use self::__NSFileProviderTesting::NSFileProviderTestingOperationSide;
pub use self::__NSFileProviderTesting::NSFileProviderTestingOperationType;
pub use self::__NSFileProviderTesting::{
    NSFileProviderTestingOperationSideDisk, NSFileProviderTestingOperationSideFileProvider,
};
pub use self::__NSFileProviderTesting::{
    NSFileProviderTestingOperationTypeChildrenEnumeration,
    NSFileProviderTestingOperationTypeCollisionResolution,
    NSFileProviderTestingOperationTypeContentFetch, NSFileProviderTestingOperationTypeCreation,
    NSFileProviderTestingOperationTypeDeletion, NSFileProviderTestingOperationTypeIngestion,
    NSFileProviderTestingOperationTypeLookup, NSFileProviderTestingOperationTypeModification,
};
