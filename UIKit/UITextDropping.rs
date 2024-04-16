//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    #[cfg(all(
        feature = "UIPasteConfigurationSupporting",
        feature = "UITextInput",
        feature = "UITextInputTraits",
        feature = "UITextPasteConfigurationSupporting"
    ))]
    pub unsafe trait UITextDroppable:
        UITextInput + UITextPasteConfigurationSupporting + IsMainThreadOnly
    {
        #[method_id(@__retain_semantics Other textDropDelegate)]
        unsafe fn textDropDelegate(&self) -> Option<Id<ProtocolObject<dyn UITextDropDelegate>>>;

        #[method(setTextDropDelegate:)]
        unsafe fn setTextDropDelegate(
            &self,
            text_drop_delegate: Option<&ProtocolObject<dyn UITextDropDelegate>>,
        );

        #[cfg(feature = "UIDropInteraction")]
        #[method_id(@__retain_semantics Other textDropInteraction)]
        unsafe fn textDropInteraction(&self) -> Option<Id<UIDropInteraction>>;

        #[method(isTextDropActive)]
        unsafe fn isTextDropActive(&self) -> bool;
    }

    #[cfg(all(
        feature = "UIPasteConfigurationSupporting",
        feature = "UITextInput",
        feature = "UITextInputTraits",
        feature = "UITextPasteConfigurationSupporting"
    ))]
    unsafe impl ProtocolType for dyn UITextDroppable {}
);

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITextDropEditability(pub NSUInteger);
impl UITextDropEditability {
    #[doc(alias = "UITextDropEditabilityNo")]
    pub const No: Self = Self(0);
    #[doc(alias = "UITextDropEditabilityTemporary")]
    pub const Temporary: Self = Self(1);
    #[doc(alias = "UITextDropEditabilityYes")]
    pub const Yes: Self = Self(2);
}

unsafe impl Encode for UITextDropEditability {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UITextDropEditability {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    pub unsafe trait UITextDropDelegate: NSObjectProtocol + IsMainThreadOnly {
        #[cfg(all(
            feature = "UIPasteConfigurationSupporting",
            feature = "UIResponder",
            feature = "UITextInput",
            feature = "UITextInputTraits",
            feature = "UITextPasteConfigurationSupporting",
            feature = "UIView"
        ))]
        #[optional]
        #[method(textDroppableView:willBecomeEditableForDrop:)]
        unsafe fn textDroppableView_willBecomeEditableForDrop(
            &self,
            text_droppable_view: &UIView,
            drop: &ProtocolObject<dyn UITextDropRequest>,
        ) -> UITextDropEditability;

        #[cfg(all(
            feature = "UIDropInteraction",
            feature = "UIPasteConfigurationSupporting",
            feature = "UIResponder",
            feature = "UITextDropProposal",
            feature = "UITextInput",
            feature = "UITextInputTraits",
            feature = "UITextPasteConfigurationSupporting",
            feature = "UIView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other textDroppableView:proposalForDrop:)]
        unsafe fn textDroppableView_proposalForDrop(
            &self,
            text_droppable_view: &UIView,
            drop: &ProtocolObject<dyn UITextDropRequest>,
        ) -> Id<UITextDropProposal>;

        #[cfg(all(
            feature = "UIPasteConfigurationSupporting",
            feature = "UIResponder",
            feature = "UITextInput",
            feature = "UITextInputTraits",
            feature = "UITextPasteConfigurationSupporting",
            feature = "UIView"
        ))]
        #[optional]
        #[method(textDroppableView:willPerformDrop:)]
        unsafe fn textDroppableView_willPerformDrop(
            &self,
            text_droppable_view: &UIView,
            drop: &ProtocolObject<dyn UITextDropRequest>,
        );

        #[cfg(all(
            feature = "UIPasteConfigurationSupporting",
            feature = "UIResponder",
            feature = "UITargetedDragPreview",
            feature = "UITargetedPreview",
            feature = "UITextInput",
            feature = "UITextInputTraits",
            feature = "UITextPasteConfigurationSupporting",
            feature = "UIView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other textDroppableView:previewForDroppingAllItemsWithDefault:)]
        unsafe fn textDroppableView_previewForDroppingAllItemsWithDefault(
            &self,
            text_droppable_view: &UIView,
            default_preview: &UITargetedDragPreview,
        ) -> Option<Id<UITargetedDragPreview>>;

        #[cfg(all(
            feature = "UIDragSession",
            feature = "UIPasteConfigurationSupporting",
            feature = "UIResponder",
            feature = "UITextInput",
            feature = "UITextInputTraits",
            feature = "UITextPasteConfigurationSupporting",
            feature = "UIView"
        ))]
        #[optional]
        #[method(textDroppableView:dropSessionDidEnter:)]
        unsafe fn textDroppableView_dropSessionDidEnter(
            &self,
            text_droppable_view: &UIView,
            session: &ProtocolObject<dyn UIDropSession>,
        );

        #[cfg(all(
            feature = "UIDragSession",
            feature = "UIPasteConfigurationSupporting",
            feature = "UIResponder",
            feature = "UITextInput",
            feature = "UITextInputTraits",
            feature = "UITextPasteConfigurationSupporting",
            feature = "UIView"
        ))]
        #[optional]
        #[method(textDroppableView:dropSessionDidUpdate:)]
        unsafe fn textDroppableView_dropSessionDidUpdate(
            &self,
            text_droppable_view: &UIView,
            session: &ProtocolObject<dyn UIDropSession>,
        );

        #[cfg(all(
            feature = "UIDragSession",
            feature = "UIPasteConfigurationSupporting",
            feature = "UIResponder",
            feature = "UITextInput",
            feature = "UITextInputTraits",
            feature = "UITextPasteConfigurationSupporting",
            feature = "UIView"
        ))]
        #[optional]
        #[method(textDroppableView:dropSessionDidExit:)]
        unsafe fn textDroppableView_dropSessionDidExit(
            &self,
            text_droppable_view: &UIView,
            session: &ProtocolObject<dyn UIDropSession>,
        );

        #[cfg(all(
            feature = "UIDragSession",
            feature = "UIPasteConfigurationSupporting",
            feature = "UIResponder",
            feature = "UITextInput",
            feature = "UITextInputTraits",
            feature = "UITextPasteConfigurationSupporting",
            feature = "UIView"
        ))]
        #[optional]
        #[method(textDroppableView:dropSessionDidEnd:)]
        unsafe fn textDroppableView_dropSessionDidEnd(
            &self,
            text_droppable_view: &UIView,
            session: &ProtocolObject<dyn UIDropSession>,
        );
    }

    unsafe impl ProtocolType for dyn UITextDropDelegate {}
);

extern_protocol!(
    pub unsafe trait UITextDropRequest: NSObjectProtocol + IsMainThreadOnly {
        #[cfg(feature = "UITextInput")]
        #[method_id(@__retain_semantics Other dropPosition)]
        unsafe fn dropPosition(&self) -> Id<UITextPosition>;

        #[cfg(all(feature = "UIDropInteraction", feature = "UITextDropProposal"))]
        #[method_id(@__retain_semantics Other suggestedProposal)]
        unsafe fn suggestedProposal(&self) -> Id<UITextDropProposal>;

        #[method(isSameView)]
        unsafe fn isSameView(&self) -> bool;

        #[cfg(feature = "UIDragSession")]
        #[method_id(@__retain_semantics Other dropSession)]
        unsafe fn dropSession(&self) -> Id<ProtocolObject<dyn UIDropSession>>;
    }

    unsafe impl ProtocolType for dyn UITextDropRequest {}
);
