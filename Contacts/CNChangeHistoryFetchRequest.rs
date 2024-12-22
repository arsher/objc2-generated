//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// Specifies the criteria to fetch change history.
    ///
    ///
    /// Changes to contacts are always returned.
    /// All changes are coalesced to remove redundant adds, updates and deletes.
    /// This request is used with [CNContactStore enumeratorForChangeHistoryFetchRequest:error:].
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/contacts/cnchangehistoryfetchrequest?language=objc)
    #[unsafe(super(CNFetchRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CNFetchRequest")]
    pub struct CNChangeHistoryFetchRequest;
);

#[cfg(feature = "CNFetchRequest")]
unsafe impl NSCoding for CNChangeHistoryFetchRequest {}

#[cfg(feature = "CNFetchRequest")]
unsafe impl NSObjectProtocol for CNChangeHistoryFetchRequest {}

#[cfg(feature = "CNFetchRequest")]
unsafe impl NSSecureCoding for CNChangeHistoryFetchRequest {}

extern_methods!(
    #[cfg(feature = "CNFetchRequest")]
    unsafe impl CNChangeHistoryFetchRequest {
        /// Request changes made after a certain point.
        ///
        ///
        /// If non-nil, only changes made after this point in history will be returned.
        ///
        /// If nil, a
        /// `CNChangeHistoryDropEverythingEvent`will be returned, followed by an add event
        /// for every contact and group currently in the contacts database.
        #[method_id(@__retain_semantics Other startingToken)]
        pub unsafe fn startingToken(&self) -> Option<Retained<NSData>>;

        /// Setter for [`startingToken`][Self::startingToken].
        #[method(setStartingToken:)]
        pub unsafe fn setStartingToken(&self, starting_token: Option<&NSData>);

        #[cfg(feature = "CNContact")]
        /// Additional keys to include in the fetched contacts.
        ///
        ///
        /// By default, only
        /// `CNContactIdentifierKey`will be fetched. If you
        /// would like to include additional key descriptors to process the contacts,
        /// include the key descriptors you need.
        ///
        /// `CNContactIdentifierKey`will always be fetched, whether you
        /// request it or not.
        #[method_id(@__retain_semantics Other additionalContactKeyDescriptors)]
        pub unsafe fn additionalContactKeyDescriptors(
            &self,
        ) -> Option<Retained<NSArray<ProtocolObject<dyn CNKeyDescriptor>>>>;

        #[cfg(feature = "CNContact")]
        /// Setter for [`additionalContactKeyDescriptors`][Self::additionalContactKeyDescriptors].
        #[method(setAdditionalContactKeyDescriptors:)]
        pub unsafe fn setAdditionalContactKeyDescriptors(
            &self,
            additional_contact_key_descriptors: Option<
                &NSArray<ProtocolObject<dyn CNKeyDescriptor>>,
            >,
        );

        /// Returns contact changes as unified contacts.
        ///
        ///
        /// If
        /// `YES,`returns unified contact history. Otherwise returns individual contact history. Default is
        /// `YES.`
        ///
        /// Note: A unified contact is the aggregation of properties from a set of linked individual contacts.
        /// If an individual contact is not linked then the unified contact is simply that individual contact.
        #[method(shouldUnifyResults)]
        pub unsafe fn shouldUnifyResults(&self) -> bool;

        /// Setter for [`shouldUnifyResults`][Self::shouldUnifyResults].
        #[method(setShouldUnifyResults:)]
        pub unsafe fn setShouldUnifyResults(&self, should_unify_results: bool);

        /// To return mutable contacts and groups.
        ///
        ///
        /// If
        /// `YES`returns mutable contacts and groups. Default is
        /// `NO.`
        #[method(mutableObjects)]
        pub unsafe fn mutableObjects(&self) -> bool;

        /// Setter for [`mutableObjects`][Self::mutableObjects].
        #[method(setMutableObjects:)]
        pub unsafe fn setMutableObjects(&self, mutable_objects: bool);

        /// Set to
        /// `YES`to also fetch group changes. Default is
        /// `NO.`
        #[method(includeGroupChanges)]
        pub unsafe fn includeGroupChanges(&self) -> bool;

        /// Setter for [`includeGroupChanges`][Self::includeGroupChanges].
        #[method(setIncludeGroupChanges:)]
        pub unsafe fn setIncludeGroupChanges(&self, include_group_changes: bool);

        /// Exclude changes made by certain authors.
        ///
        ///
        /// If set, transactions made by the specified authors will be excluded
        /// from the results. Use this, in conjunction with
        /// `CNSaveRequest.transactionAuthor,`to suppress processing of changes you already know about.
        #[method_id(@__retain_semantics Other excludedTransactionAuthors)]
        pub unsafe fn excludedTransactionAuthors(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`excludedTransactionAuthors`][Self::excludedTransactionAuthors].
        #[method(setExcludedTransactionAuthors:)]
        pub unsafe fn setExcludedTransactionAuthors(
            &self,
            excluded_transaction_authors: Option<&NSArray<NSString>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CNFetchRequest")]
    unsafe impl CNChangeHistoryFetchRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
