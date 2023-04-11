//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSCandidateListTouchBarItem")]
    pub struct NSCandidateListTouchBarItem<CandidateType: Message = Object> {
        __superclass: NSTouchBarItem,
        _inner0: PhantomData<*mut CandidateType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "AppKit_NSCandidateListTouchBarItem")]
    unsafe impl<CandidateType: Message> ClassType for NSCandidateListTouchBarItem<CandidateType> {
        #[inherits(NSObject)]
        type Super = NSTouchBarItem;
        type Mutability = InteriorMutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

#[cfg(feature = "AppKit_NSCandidateListTouchBarItem")]
unsafe impl<CandidateType: Message + NSCoding> NSCoding
    for NSCandidateListTouchBarItem<CandidateType>
{
}

#[cfg(feature = "AppKit_NSCandidateListTouchBarItem")]
unsafe impl<CandidateType: Message> NSObjectProtocol
    for NSCandidateListTouchBarItem<CandidateType>
{
}

extern_methods!(
    #[cfg(feature = "AppKit_NSCandidateListTouchBarItem")]
    unsafe impl<CandidateType: Message> NSCandidateListTouchBarItem<CandidateType> {
        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other client)]
        pub unsafe fn client(&self) -> Option<Id<NSView>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setClient:)]
        pub unsafe fn setClient(&self, client: Option<&NSView>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSCandidateListTouchBarItemDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSCandidateListTouchBarItemDelegate>>,
        );

        #[method(isCollapsed)]
        pub unsafe fn isCollapsed(&self) -> bool;

        #[method(setCollapsed:)]
        pub unsafe fn setCollapsed(&self, collapsed: bool);

        #[method(allowsCollapsing)]
        pub unsafe fn allowsCollapsing(&self) -> bool;

        #[method(setAllowsCollapsing:)]
        pub unsafe fn setAllowsCollapsing(&self, allows_collapsing: bool);

        #[method(isCandidateListVisible)]
        pub unsafe fn isCandidateListVisible(&self) -> bool;

        #[method(updateWithInsertionPointVisibility:)]
        pub unsafe fn updateWithInsertionPointVisibility(&self, is_visible: bool);

        #[method(allowsTextInputContextCandidates)]
        pub unsafe fn allowsTextInputContextCandidates(&self) -> bool;

        #[method(setAllowsTextInputContextCandidates:)]
        pub unsafe fn setAllowsTextInputContextCandidates(
            &self,
            allows_text_input_context_candidates: bool,
        );

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(attributedStringForCandidate)]
        pub unsafe fn attributedStringForCandidate(
            &self,
        ) -> *mut Block<(NonNull<CandidateType>, NSInteger), NonNull<NSAttributedString>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedStringForCandidate:)]
        pub unsafe fn setAttributedStringForCandidate(
            &self,
            attributed_string_for_candidate: Option<
                &Block<(NonNull<CandidateType>, NSInteger), NonNull<NSAttributedString>>,
            >,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other candidates)]
        pub unsafe fn candidates(&self) -> Id<NSArray<CandidateType>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setCandidates:forSelectedRange:inString:)]
        pub unsafe fn setCandidates_forSelectedRange_inString(
            &self,
            candidates: &NSArray<CandidateType>,
            selected_range: NSRange,
            original_string: Option<&NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customization_label: Option<&NSString>);
    }
);

extern_protocol!(
    pub unsafe trait NSCandidateListTouchBarItemDelegate: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSCandidateListTouchBarItem")]
        #[optional]
        #[method(candidateListTouchBarItem:beginSelectingCandidateAtIndex:)]
        unsafe fn candidateListTouchBarItem_beginSelectingCandidateAtIndex(
            &self,
            an_item: &NSCandidateListTouchBarItem,
            index: NSInteger,
        );

        #[cfg(feature = "AppKit_NSCandidateListTouchBarItem")]
        #[optional]
        #[method(candidateListTouchBarItem:changeSelectionFromCandidateAtIndex:toIndex:)]
        unsafe fn candidateListTouchBarItem_changeSelectionFromCandidateAtIndex_toIndex(
            &self,
            an_item: &NSCandidateListTouchBarItem,
            previous_index: NSInteger,
            index: NSInteger,
        );

        #[cfg(feature = "AppKit_NSCandidateListTouchBarItem")]
        #[optional]
        #[method(candidateListTouchBarItem:endSelectingCandidateAtIndex:)]
        unsafe fn candidateListTouchBarItem_endSelectingCandidateAtIndex(
            &self,
            an_item: &NSCandidateListTouchBarItem,
            index: NSInteger,
        );

        #[cfg(feature = "AppKit_NSCandidateListTouchBarItem")]
        #[optional]
        #[method(candidateListTouchBarItem:changedCandidateListVisibility:)]
        unsafe fn candidateListTouchBarItem_changedCandidateListVisibility(
            &self,
            an_item: &NSCandidateListTouchBarItem,
            is_visible: bool,
        );
    }

    unsafe impl ProtocolType for dyn NSCandidateListTouchBarItemDelegate {}
);

extern_methods!(
    /// NSCandidateListTouchBarItem
    #[cfg(feature = "AppKit_NSView")]
    unsafe impl NSView {
        #[cfg(feature = "AppKit_NSCandidateListTouchBarItem")]
        #[method_id(@__retain_semantics Other candidateListTouchBarItem)]
        pub unsafe fn candidateListTouchBarItem(&self) -> Option<Id<NSCandidateListTouchBarItem>>;
    }
);

extern_static!(NSTouchBarItemIdentifierCandidateList: &'static NSTouchBarItemIdentifier);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    #[cfg(feature = "AppKit_NSCandidateListTouchBarItem")]
    unsafe impl<CandidateType: Message> NSCandidateListTouchBarItem<CandidateType> {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self>;
    }
);
