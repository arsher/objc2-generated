//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNSaveRequest")]
    pub struct CNSaveRequest;

    #[cfg(feature = "Contacts_CNSaveRequest")]
    unsafe impl ClassType for CNSaveRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Contacts_CNSaveRequest")]
unsafe impl NSObjectProtocol for CNSaveRequest {}

extern_methods!(
    #[cfg(feature = "Contacts_CNSaveRequest")]
    unsafe impl CNSaveRequest {
        #[cfg(all(feature = "Contacts_CNMutableContact", feature = "Foundation_NSString"))]
        #[method(addContact:toContainerWithIdentifier:)]
        pub unsafe fn addContact_toContainerWithIdentifier(
            &self,
            contact: &CNMutableContact,
            identifier: Option<&NSString>,
        );

        #[cfg(feature = "Contacts_CNMutableContact")]
        #[method(updateContact:)]
        pub unsafe fn updateContact(&self, contact: &CNMutableContact);

        #[cfg(feature = "Contacts_CNMutableContact")]
        #[method(deleteContact:)]
        pub unsafe fn deleteContact(&self, contact: &CNMutableContact);

        #[cfg(all(feature = "Contacts_CNMutableGroup", feature = "Foundation_NSString"))]
        #[method(addGroup:toContainerWithIdentifier:)]
        pub unsafe fn addGroup_toContainerWithIdentifier(
            &self,
            group: &CNMutableGroup,
            identifier: Option<&NSString>,
        );

        #[cfg(feature = "Contacts_CNMutableGroup")]
        #[method(updateGroup:)]
        pub unsafe fn updateGroup(&self, group: &CNMutableGroup);

        #[cfg(feature = "Contacts_CNMutableGroup")]
        #[method(deleteGroup:)]
        pub unsafe fn deleteGroup(&self, group: &CNMutableGroup);

        #[cfg(feature = "Contacts_CNGroup")]
        #[method(addSubgroup:toGroup:)]
        pub unsafe fn addSubgroup_toGroup(&self, subgroup: &CNGroup, group: &CNGroup);

        #[cfg(feature = "Contacts_CNGroup")]
        #[method(removeSubgroup:fromGroup:)]
        pub unsafe fn removeSubgroup_fromGroup(&self, subgroup: &CNGroup, group: &CNGroup);

        #[cfg(all(feature = "Contacts_CNContact", feature = "Contacts_CNGroup"))]
        #[method(addMember:toGroup:)]
        pub unsafe fn addMember_toGroup(&self, contact: &CNContact, group: &CNGroup);

        #[cfg(all(feature = "Contacts_CNContact", feature = "Contacts_CNGroup"))]
        #[method(removeMember:fromGroup:)]
        pub unsafe fn removeMember_fromGroup(&self, contact: &CNContact, group: &CNGroup);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other transactionAuthor)]
        pub unsafe fn transactionAuthor(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTransactionAuthor:)]
        pub unsafe fn setTransactionAuthor(&self, transaction_author: Option<&NSString>);

        #[method(shouldRefetchContacts)]
        pub unsafe fn shouldRefetchContacts(&self) -> bool;

        #[method(setShouldRefetchContacts:)]
        pub unsafe fn setShouldRefetchContacts(&self, should_refetch_contacts: bool);
    }
);
