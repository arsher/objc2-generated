//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNChangeHistoryEvent")]
    pub struct CNChangeHistoryEvent;

    #[cfg(feature = "Contacts_CNChangeHistoryEvent")]
    unsafe impl ClassType for CNChangeHistoryEvent {
        type Super = NSObject;
    }
);

#[cfg(feature = "Contacts_CNChangeHistoryEvent")]
unsafe impl NSCoding for CNChangeHistoryEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryEvent")]
unsafe impl NSObjectProtocol for CNChangeHistoryEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryEvent")]
unsafe impl NSSecureCoding for CNChangeHistoryEvent {}

extern_methods!(
    #[cfg(feature = "Contacts_CNChangeHistoryEvent")]
    unsafe impl CNChangeHistoryEvent {
        #[method(acceptEventVisitor:)]
        pub unsafe fn acceptEventVisitor(
            &self,
            visitor: &ProtocolObject<dyn CNChangeHistoryEventVisitor>,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNChangeHistoryDropEverythingEvent")]
    pub struct CNChangeHistoryDropEverythingEvent;

    #[cfg(feature = "Contacts_CNChangeHistoryDropEverythingEvent")]
    unsafe impl ClassType for CNChangeHistoryDropEverythingEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

#[cfg(feature = "Contacts_CNChangeHistoryDropEverythingEvent")]
unsafe impl NSCoding for CNChangeHistoryDropEverythingEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryDropEverythingEvent")]
unsafe impl NSObjectProtocol for CNChangeHistoryDropEverythingEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryDropEverythingEvent")]
unsafe impl NSSecureCoding for CNChangeHistoryDropEverythingEvent {}

extern_methods!(
    #[cfg(feature = "Contacts_CNChangeHistoryDropEverythingEvent")]
    unsafe impl CNChangeHistoryDropEverythingEvent {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNChangeHistoryAddContactEvent")]
    pub struct CNChangeHistoryAddContactEvent;

    #[cfg(feature = "Contacts_CNChangeHistoryAddContactEvent")]
    unsafe impl ClassType for CNChangeHistoryAddContactEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

#[cfg(feature = "Contacts_CNChangeHistoryAddContactEvent")]
unsafe impl NSCoding for CNChangeHistoryAddContactEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryAddContactEvent")]
unsafe impl NSObjectProtocol for CNChangeHistoryAddContactEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryAddContactEvent")]
unsafe impl NSSecureCoding for CNChangeHistoryAddContactEvent {}

extern_methods!(
    #[cfg(feature = "Contacts_CNChangeHistoryAddContactEvent")]
    unsafe impl CNChangeHistoryAddContactEvent {
        #[cfg(feature = "Contacts_CNContact")]
        #[method_id(@__retain_semantics Other contact)]
        pub unsafe fn contact(&self) -> Id<CNContact, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other containerIdentifier)]
        pub unsafe fn containerIdentifier(&self) -> Option<Id<NSString, Shared>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNChangeHistoryUpdateContactEvent")]
    pub struct CNChangeHistoryUpdateContactEvent;

    #[cfg(feature = "Contacts_CNChangeHistoryUpdateContactEvent")]
    unsafe impl ClassType for CNChangeHistoryUpdateContactEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

#[cfg(feature = "Contacts_CNChangeHistoryUpdateContactEvent")]
unsafe impl NSCoding for CNChangeHistoryUpdateContactEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryUpdateContactEvent")]
unsafe impl NSObjectProtocol for CNChangeHistoryUpdateContactEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryUpdateContactEvent")]
unsafe impl NSSecureCoding for CNChangeHistoryUpdateContactEvent {}

extern_methods!(
    #[cfg(feature = "Contacts_CNChangeHistoryUpdateContactEvent")]
    unsafe impl CNChangeHistoryUpdateContactEvent {
        #[cfg(feature = "Contacts_CNContact")]
        #[method_id(@__retain_semantics Other contact)]
        pub unsafe fn contact(&self) -> Id<CNContact, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNChangeHistoryDeleteContactEvent")]
    pub struct CNChangeHistoryDeleteContactEvent;

    #[cfg(feature = "Contacts_CNChangeHistoryDeleteContactEvent")]
    unsafe impl ClassType for CNChangeHistoryDeleteContactEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

#[cfg(feature = "Contacts_CNChangeHistoryDeleteContactEvent")]
unsafe impl NSCoding for CNChangeHistoryDeleteContactEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryDeleteContactEvent")]
unsafe impl NSObjectProtocol for CNChangeHistoryDeleteContactEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryDeleteContactEvent")]
unsafe impl NSSecureCoding for CNChangeHistoryDeleteContactEvent {}

extern_methods!(
    #[cfg(feature = "Contacts_CNChangeHistoryDeleteContactEvent")]
    unsafe impl CNChangeHistoryDeleteContactEvent {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other contactIdentifier)]
        pub unsafe fn contactIdentifier(&self) -> Id<NSString, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNChangeHistoryAddGroupEvent")]
    pub struct CNChangeHistoryAddGroupEvent;

    #[cfg(feature = "Contacts_CNChangeHistoryAddGroupEvent")]
    unsafe impl ClassType for CNChangeHistoryAddGroupEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

#[cfg(feature = "Contacts_CNChangeHistoryAddGroupEvent")]
unsafe impl NSCoding for CNChangeHistoryAddGroupEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryAddGroupEvent")]
unsafe impl NSObjectProtocol for CNChangeHistoryAddGroupEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryAddGroupEvent")]
unsafe impl NSSecureCoding for CNChangeHistoryAddGroupEvent {}

extern_methods!(
    #[cfg(feature = "Contacts_CNChangeHistoryAddGroupEvent")]
    unsafe impl CNChangeHistoryAddGroupEvent {
        #[cfg(feature = "Contacts_CNGroup")]
        #[method_id(@__retain_semantics Other group)]
        pub unsafe fn group(&self) -> Id<CNGroup, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other containerIdentifier)]
        pub unsafe fn containerIdentifier(&self) -> Id<NSString, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNChangeHistoryUpdateGroupEvent")]
    pub struct CNChangeHistoryUpdateGroupEvent;

    #[cfg(feature = "Contacts_CNChangeHistoryUpdateGroupEvent")]
    unsafe impl ClassType for CNChangeHistoryUpdateGroupEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

#[cfg(feature = "Contacts_CNChangeHistoryUpdateGroupEvent")]
unsafe impl NSCoding for CNChangeHistoryUpdateGroupEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryUpdateGroupEvent")]
unsafe impl NSObjectProtocol for CNChangeHistoryUpdateGroupEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryUpdateGroupEvent")]
unsafe impl NSSecureCoding for CNChangeHistoryUpdateGroupEvent {}

extern_methods!(
    #[cfg(feature = "Contacts_CNChangeHistoryUpdateGroupEvent")]
    unsafe impl CNChangeHistoryUpdateGroupEvent {
        #[cfg(feature = "Contacts_CNGroup")]
        #[method_id(@__retain_semantics Other group)]
        pub unsafe fn group(&self) -> Id<CNGroup, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNChangeHistoryDeleteGroupEvent")]
    pub struct CNChangeHistoryDeleteGroupEvent;

    #[cfg(feature = "Contacts_CNChangeHistoryDeleteGroupEvent")]
    unsafe impl ClassType for CNChangeHistoryDeleteGroupEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

#[cfg(feature = "Contacts_CNChangeHistoryDeleteGroupEvent")]
unsafe impl NSCoding for CNChangeHistoryDeleteGroupEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryDeleteGroupEvent")]
unsafe impl NSObjectProtocol for CNChangeHistoryDeleteGroupEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryDeleteGroupEvent")]
unsafe impl NSSecureCoding for CNChangeHistoryDeleteGroupEvent {}

extern_methods!(
    #[cfg(feature = "Contacts_CNChangeHistoryDeleteGroupEvent")]
    unsafe impl CNChangeHistoryDeleteGroupEvent {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other groupIdentifier)]
        pub unsafe fn groupIdentifier(&self) -> Id<NSString, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNChangeHistoryAddMemberToGroupEvent")]
    pub struct CNChangeHistoryAddMemberToGroupEvent;

    #[cfg(feature = "Contacts_CNChangeHistoryAddMemberToGroupEvent")]
    unsafe impl ClassType for CNChangeHistoryAddMemberToGroupEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

#[cfg(feature = "Contacts_CNChangeHistoryAddMemberToGroupEvent")]
unsafe impl NSCoding for CNChangeHistoryAddMemberToGroupEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryAddMemberToGroupEvent")]
unsafe impl NSObjectProtocol for CNChangeHistoryAddMemberToGroupEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryAddMemberToGroupEvent")]
unsafe impl NSSecureCoding for CNChangeHistoryAddMemberToGroupEvent {}

extern_methods!(
    #[cfg(feature = "Contacts_CNChangeHistoryAddMemberToGroupEvent")]
    unsafe impl CNChangeHistoryAddMemberToGroupEvent {
        #[cfg(feature = "Contacts_CNContact")]
        #[method_id(@__retain_semantics Other member)]
        pub unsafe fn member(&self) -> Id<CNContact, Shared>;

        #[cfg(feature = "Contacts_CNGroup")]
        #[method_id(@__retain_semantics Other group)]
        pub unsafe fn group(&self) -> Id<CNGroup, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNChangeHistoryRemoveMemberFromGroupEvent")]
    pub struct CNChangeHistoryRemoveMemberFromGroupEvent;

    #[cfg(feature = "Contacts_CNChangeHistoryRemoveMemberFromGroupEvent")]
    unsafe impl ClassType for CNChangeHistoryRemoveMemberFromGroupEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

#[cfg(feature = "Contacts_CNChangeHistoryRemoveMemberFromGroupEvent")]
unsafe impl NSCoding for CNChangeHistoryRemoveMemberFromGroupEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryRemoveMemberFromGroupEvent")]
unsafe impl NSObjectProtocol for CNChangeHistoryRemoveMemberFromGroupEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryRemoveMemberFromGroupEvent")]
unsafe impl NSSecureCoding for CNChangeHistoryRemoveMemberFromGroupEvent {}

extern_methods!(
    #[cfg(feature = "Contacts_CNChangeHistoryRemoveMemberFromGroupEvent")]
    unsafe impl CNChangeHistoryRemoveMemberFromGroupEvent {
        #[cfg(feature = "Contacts_CNContact")]
        #[method_id(@__retain_semantics Other member)]
        pub unsafe fn member(&self) -> Id<CNContact, Shared>;

        #[cfg(feature = "Contacts_CNGroup")]
        #[method_id(@__retain_semantics Other group)]
        pub unsafe fn group(&self) -> Id<CNGroup, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNChangeHistoryAddSubgroupToGroupEvent")]
    pub struct CNChangeHistoryAddSubgroupToGroupEvent;

    #[cfg(feature = "Contacts_CNChangeHistoryAddSubgroupToGroupEvent")]
    unsafe impl ClassType for CNChangeHistoryAddSubgroupToGroupEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

#[cfg(feature = "Contacts_CNChangeHistoryAddSubgroupToGroupEvent")]
unsafe impl NSCoding for CNChangeHistoryAddSubgroupToGroupEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryAddSubgroupToGroupEvent")]
unsafe impl NSObjectProtocol for CNChangeHistoryAddSubgroupToGroupEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryAddSubgroupToGroupEvent")]
unsafe impl NSSecureCoding for CNChangeHistoryAddSubgroupToGroupEvent {}

extern_methods!(
    #[cfg(feature = "Contacts_CNChangeHistoryAddSubgroupToGroupEvent")]
    unsafe impl CNChangeHistoryAddSubgroupToGroupEvent {
        #[cfg(feature = "Contacts_CNGroup")]
        #[method_id(@__retain_semantics Other subgroup)]
        pub unsafe fn subgroup(&self) -> Id<CNGroup, Shared>;

        #[cfg(feature = "Contacts_CNGroup")]
        #[method_id(@__retain_semantics Other group)]
        pub unsafe fn group(&self) -> Id<CNGroup, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNChangeHistoryRemoveSubgroupFromGroupEvent")]
    pub struct CNChangeHistoryRemoveSubgroupFromGroupEvent;

    #[cfg(feature = "Contacts_CNChangeHistoryRemoveSubgroupFromGroupEvent")]
    unsafe impl ClassType for CNChangeHistoryRemoveSubgroupFromGroupEvent {
        #[inherits(NSObject)]
        type Super = CNChangeHistoryEvent;
    }
);

#[cfg(feature = "Contacts_CNChangeHistoryRemoveSubgroupFromGroupEvent")]
unsafe impl NSCoding for CNChangeHistoryRemoveSubgroupFromGroupEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryRemoveSubgroupFromGroupEvent")]
unsafe impl NSObjectProtocol for CNChangeHistoryRemoveSubgroupFromGroupEvent {}

#[cfg(feature = "Contacts_CNChangeHistoryRemoveSubgroupFromGroupEvent")]
unsafe impl NSSecureCoding for CNChangeHistoryRemoveSubgroupFromGroupEvent {}

extern_methods!(
    #[cfg(feature = "Contacts_CNChangeHistoryRemoveSubgroupFromGroupEvent")]
    unsafe impl CNChangeHistoryRemoveSubgroupFromGroupEvent {
        #[cfg(feature = "Contacts_CNGroup")]
        #[method_id(@__retain_semantics Other subgroup)]
        pub unsafe fn subgroup(&self) -> Id<CNGroup, Shared>;

        #[cfg(feature = "Contacts_CNGroup")]
        #[method_id(@__retain_semantics Other group)]
        pub unsafe fn group(&self) -> Id<CNGroup, Shared>;
    }
);

extern_protocol!(
    pub unsafe trait CNChangeHistoryEventVisitor: NSObjectProtocol {
        #[cfg(feature = "Contacts_CNChangeHistoryDropEverythingEvent")]
        #[method(visitDropEverythingEvent:)]
        unsafe fn visitDropEverythingEvent(&self, event: &CNChangeHistoryDropEverythingEvent);

        #[cfg(feature = "Contacts_CNChangeHistoryAddContactEvent")]
        #[method(visitAddContactEvent:)]
        unsafe fn visitAddContactEvent(&self, event: &CNChangeHistoryAddContactEvent);

        #[cfg(feature = "Contacts_CNChangeHistoryUpdateContactEvent")]
        #[method(visitUpdateContactEvent:)]
        unsafe fn visitUpdateContactEvent(&self, event: &CNChangeHistoryUpdateContactEvent);

        #[cfg(feature = "Contacts_CNChangeHistoryDeleteContactEvent")]
        #[method(visitDeleteContactEvent:)]
        unsafe fn visitDeleteContactEvent(&self, event: &CNChangeHistoryDeleteContactEvent);

        #[cfg(feature = "Contacts_CNChangeHistoryAddGroupEvent")]
        #[optional]
        #[method(visitAddGroupEvent:)]
        unsafe fn visitAddGroupEvent(&self, event: &CNChangeHistoryAddGroupEvent);

        #[cfg(feature = "Contacts_CNChangeHistoryUpdateGroupEvent")]
        #[optional]
        #[method(visitUpdateGroupEvent:)]
        unsafe fn visitUpdateGroupEvent(&self, event: &CNChangeHistoryUpdateGroupEvent);

        #[cfg(feature = "Contacts_CNChangeHistoryDeleteGroupEvent")]
        #[optional]
        #[method(visitDeleteGroupEvent:)]
        unsafe fn visitDeleteGroupEvent(&self, event: &CNChangeHistoryDeleteGroupEvent);

        #[cfg(feature = "Contacts_CNChangeHistoryAddMemberToGroupEvent")]
        #[optional]
        #[method(visitAddMemberToGroupEvent:)]
        unsafe fn visitAddMemberToGroupEvent(&self, event: &CNChangeHistoryAddMemberToGroupEvent);

        #[cfg(feature = "Contacts_CNChangeHistoryRemoveMemberFromGroupEvent")]
        #[optional]
        #[method(visitRemoveMemberFromGroupEvent:)]
        unsafe fn visitRemoveMemberFromGroupEvent(
            &self,
            event: &CNChangeHistoryRemoveMemberFromGroupEvent,
        );

        #[cfg(feature = "Contacts_CNChangeHistoryAddSubgroupToGroupEvent")]
        #[optional]
        #[method(visitAddSubgroupToGroupEvent:)]
        unsafe fn visitAddSubgroupToGroupEvent(
            &self,
            event: &CNChangeHistoryAddSubgroupToGroupEvent,
        );

        #[cfg(feature = "Contacts_CNChangeHistoryRemoveSubgroupFromGroupEvent")]
        #[optional]
        #[method(visitRemoveSubgroupFromGroupEvent:)]
        unsafe fn visitRemoveSubgroupFromGroupEvent(
            &self,
            event: &CNChangeHistoryRemoveSubgroupFromGroupEvent,
        );
    }

    unsafe impl ProtocolType for dyn CNChangeHistoryEventVisitor {}
);
