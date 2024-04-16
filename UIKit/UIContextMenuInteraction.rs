//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIContextMenuInteractionCommitStyle(pub NSInteger);
impl UIContextMenuInteractionCommitStyle {
    #[doc(alias = "UIContextMenuInteractionCommitStyleDismiss")]
    pub const Dismiss: Self = Self(0);
    #[doc(alias = "UIContextMenuInteractionCommitStylePop")]
    pub const Pop: Self = Self(1);
}

unsafe impl Encode for UIContextMenuInteractionCommitStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIContextMenuInteractionCommitStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIContextMenuInteractionAppearance(pub NSInteger);
impl UIContextMenuInteractionAppearance {
    #[doc(alias = "UIContextMenuInteractionAppearanceUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "UIContextMenuInteractionAppearanceRich")]
    pub const Rich: Self = Self(1);
    #[doc(alias = "UIContextMenuInteractionAppearanceCompact")]
    pub const Compact: Self = Self(2);
}

unsafe impl Encode for UIContextMenuInteractionAppearance {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIContextMenuInteractionAppearance {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIContextMenuInteraction;

    unsafe impl ClassType for UIContextMenuInteraction {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIContextMenuInteraction {}

#[cfg(feature = "UIInteraction")]
unsafe impl UIInteraction for UIContextMenuInteraction {}

extern_methods!(
    unsafe impl UIContextMenuInteraction {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn UIContextMenuInteractionDelegate>>>;

        #[method(menuAppearance)]
        pub unsafe fn menuAppearance(&self) -> UIContextMenuInteractionAppearance;

        #[method_id(@__retain_semantics Init initWithDelegate:)]
        pub unsafe fn initWithDelegate(
            this: Allocated<Self>,
            delegate: &ProtocolObject<dyn UIContextMenuInteractionDelegate>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(locationInView:)]
        pub unsafe fn locationInView(&self, view: Option<&UIView>) -> CGPoint;

        #[cfg(all(feature = "UIMenu", feature = "UIMenuElement", feature = "block2"))]
        #[method(updateVisibleMenuWithBlock:)]
        pub unsafe fn updateVisibleMenuWithBlock(
            &self,
            block: &Block<dyn Fn(NonNull<UIMenu>) -> NonNull<UIMenu> + '_>,
        );

        #[method(dismissMenu)]
        pub unsafe fn dismissMenu(&self);
    }
);

extern_protocol!(
    pub unsafe trait UIContextMenuInteractionAnimating:
        NSObjectProtocol + IsMainThreadOnly
    {
        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[method_id(@__retain_semantics Other previewViewController)]
        unsafe fn previewViewController(&self) -> Option<Id<UIViewController>>;

        #[cfg(feature = "block2")]
        #[method(addAnimations:)]
        unsafe fn addAnimations(&self, animations: &Block<dyn Fn()>);

        #[cfg(feature = "block2")]
        #[method(addCompletion:)]
        unsafe fn addCompletion(&self, completion: &Block<dyn Fn()>);
    }

    unsafe impl ProtocolType for dyn UIContextMenuInteractionAnimating {}
);

extern_protocol!(
    pub unsafe trait UIContextMenuInteractionCommitAnimating:
        UIContextMenuInteractionAnimating + IsMainThreadOnly
    {
        #[method(preferredCommitStyle)]
        unsafe fn preferredCommitStyle(&self) -> UIContextMenuInteractionCommitStyle;

        #[method(setPreferredCommitStyle:)]
        unsafe fn setPreferredCommitStyle(
            &self,
            preferred_commit_style: UIContextMenuInteractionCommitStyle,
        );
    }

    unsafe impl ProtocolType for dyn UIContextMenuInteractionCommitAnimating {}
);

extern_protocol!(
    pub unsafe trait UIContextMenuInteractionDelegate:
        NSObjectProtocol + IsMainThreadOnly
    {
        #[cfg(feature = "UIContextMenuConfiguration")]
        #[method_id(@__retain_semantics Other contextMenuInteraction:configurationForMenuAtLocation:)]
        unsafe fn contextMenuInteraction_configurationForMenuAtLocation(
            &self,
            interaction: &UIContextMenuInteraction,
            location: CGPoint,
        ) -> Option<Id<UIContextMenuConfiguration>>;

        #[cfg(all(feature = "UIContextMenuConfiguration", feature = "UITargetedPreview"))]
        #[optional]
        #[method_id(@__retain_semantics Other contextMenuInteraction:configuration:highlightPreviewForItemWithIdentifier:)]
        unsafe fn contextMenuInteraction_configuration_highlightPreviewForItemWithIdentifier(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
            identifier: &ProtocolObject<dyn NSCopying>,
        ) -> Option<Id<UITargetedPreview>>;

        #[cfg(all(feature = "UIContextMenuConfiguration", feature = "UITargetedPreview"))]
        #[optional]
        #[method_id(@__retain_semantics Other contextMenuInteraction:configuration:dismissalPreviewForItemWithIdentifier:)]
        unsafe fn contextMenuInteraction_configuration_dismissalPreviewForItemWithIdentifier(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
            identifier: &ProtocolObject<dyn NSCopying>,
        ) -> Option<Id<UITargetedPreview>>;

        #[cfg(feature = "UIContextMenuConfiguration")]
        #[optional]
        #[method(contextMenuInteraction:willPerformPreviewActionForMenuWithConfiguration:animator:)]
        unsafe fn contextMenuInteraction_willPerformPreviewActionForMenuWithConfiguration_animator(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
            animator: &ProtocolObject<dyn UIContextMenuInteractionCommitAnimating>,
        );

        #[cfg(feature = "UIContextMenuConfiguration")]
        #[optional]
        #[method(contextMenuInteraction:willDisplayMenuForConfiguration:animator:)]
        unsafe fn contextMenuInteraction_willDisplayMenuForConfiguration_animator(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
            animator: Option<&ProtocolObject<dyn UIContextMenuInteractionAnimating>>,
        );

        #[cfg(feature = "UIContextMenuConfiguration")]
        #[optional]
        #[method(contextMenuInteraction:willEndForConfiguration:animator:)]
        unsafe fn contextMenuInteraction_willEndForConfiguration_animator(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
            animator: Option<&ProtocolObject<dyn UIContextMenuInteractionAnimating>>,
        );

        #[cfg(all(feature = "UIContextMenuConfiguration", feature = "UITargetedPreview"))]
        #[deprecated]
        #[optional]
        #[method_id(@__retain_semantics Other contextMenuInteraction:previewForHighlightingMenuWithConfiguration:)]
        unsafe fn contextMenuInteraction_previewForHighlightingMenuWithConfiguration(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
        ) -> Option<Id<UITargetedPreview>>;

        #[cfg(all(feature = "UIContextMenuConfiguration", feature = "UITargetedPreview"))]
        #[deprecated]
        #[optional]
        #[method_id(@__retain_semantics Other contextMenuInteraction:previewForDismissingMenuWithConfiguration:)]
        unsafe fn contextMenuInteraction_previewForDismissingMenuWithConfiguration(
            &self,
            interaction: &UIContextMenuInteraction,
            configuration: &UIContextMenuConfiguration,
        ) -> Option<Id<UITargetedPreview>>;
    }

    unsafe impl ProtocolType for dyn UIContextMenuInteractionDelegate {}
);
