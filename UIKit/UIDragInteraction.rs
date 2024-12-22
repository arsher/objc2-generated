//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidraganimating?language=objc)
    pub unsafe trait UIDragAnimating: NSObjectProtocol + MainThreadOnly {
        #[cfg(feature = "block2")]
        #[method(addAnimations:)]
        unsafe fn addAnimations(&self, animations: &block2::Block<dyn Fn()>);

        #[cfg(all(feature = "UIViewAnimating", feature = "block2"))]
        #[method(addCompletion:)]
        unsafe fn addCompletion(&self, completion: &block2::Block<dyn Fn(UIViewAnimatingPosition)>);
    }

    unsafe impl ProtocolType for dyn UIDragAnimating {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidraginteraction?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIDragInteraction;
);

unsafe impl NSObjectProtocol for UIDragInteraction {}

#[cfg(feature = "UIInteraction")]
unsafe impl UIInteraction for UIDragInteraction {}

extern_methods!(
    unsafe impl UIDragInteraction {
        #[method_id(@__retain_semantics Init initWithDelegate:)]
        pub unsafe fn initWithDelegate(
            this: Allocated<Self>,
            delegate: &ProtocolObject<dyn UIDragInteractionDelegate>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIDragInteractionDelegate>>>;

        #[method(allowsSimultaneousRecognitionDuringLift)]
        pub unsafe fn allowsSimultaneousRecognitionDuringLift(&self) -> bool;

        /// Setter for [`allowsSimultaneousRecognitionDuringLift`][Self::allowsSimultaneousRecognitionDuringLift].
        #[method(setAllowsSimultaneousRecognitionDuringLift:)]
        pub unsafe fn setAllowsSimultaneousRecognitionDuringLift(
            &self,
            allows_simultaneous_recognition_during_lift: bool,
        );

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        /// Setter for [`isEnabled`][Self::isEnabled].
        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(isEnabledByDefault)]
        pub unsafe fn isEnabledByDefault(mtm: MainThreadMarker) -> bool;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidraginteractiondelegate?language=objc)
    pub unsafe trait UIDragInteractionDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(feature = "UIDragItem", feature = "UIDragSession"))]
        #[method_id(@__retain_semantics Other dragInteraction:itemsForBeginningSession:)]
        unsafe fn dragInteraction_itemsForBeginningSession(
            &self,
            interaction: &UIDragInteraction,
            session: &ProtocolObject<dyn UIDragSession>,
        ) -> Retained<NSArray<UIDragItem>>;

        #[cfg(all(
            feature = "UIDragItem",
            feature = "UIDragSession",
            feature = "UITargetedDragPreview",
            feature = "UITargetedPreview"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other dragInteraction:previewForLiftingItem:session:)]
        unsafe fn dragInteraction_previewForLiftingItem_session(
            &self,
            interaction: &UIDragInteraction,
            item: &UIDragItem,
            session: &ProtocolObject<dyn UIDragSession>,
        ) -> Option<Retained<UITargetedDragPreview>>;

        #[cfg(feature = "UIDragSession")]
        #[optional]
        #[method(dragInteraction:willAnimateLiftWithAnimator:session:)]
        unsafe fn dragInteraction_willAnimateLiftWithAnimator_session(
            &self,
            interaction: &UIDragInteraction,
            animator: &ProtocolObject<dyn UIDragAnimating>,
            session: &ProtocolObject<dyn UIDragSession>,
        );

        #[cfg(feature = "UIDragSession")]
        #[optional]
        #[method(dragInteraction:sessionWillBegin:)]
        unsafe fn dragInteraction_sessionWillBegin(
            &self,
            interaction: &UIDragInteraction,
            session: &ProtocolObject<dyn UIDragSession>,
        );

        #[cfg(feature = "UIDragSession")]
        #[optional]
        #[method(dragInteraction:sessionAllowsMoveOperation:)]
        unsafe fn dragInteraction_sessionAllowsMoveOperation(
            &self,
            interaction: &UIDragInteraction,
            session: &ProtocolObject<dyn UIDragSession>,
        ) -> bool;

        #[cfg(feature = "UIDragSession")]
        #[optional]
        #[method(dragInteraction:sessionIsRestrictedToDraggingApplication:)]
        unsafe fn dragInteraction_sessionIsRestrictedToDraggingApplication(
            &self,
            interaction: &UIDragInteraction,
            session: &ProtocolObject<dyn UIDragSession>,
        ) -> bool;

        #[cfg(feature = "UIDragSession")]
        #[optional]
        #[method(dragInteraction:prefersFullSizePreviewsForSession:)]
        unsafe fn dragInteraction_prefersFullSizePreviewsForSession(
            &self,
            interaction: &UIDragInteraction,
            session: &ProtocolObject<dyn UIDragSession>,
        ) -> bool;

        #[cfg(feature = "UIDragSession")]
        #[optional]
        #[method(dragInteraction:sessionDidMove:)]
        unsafe fn dragInteraction_sessionDidMove(
            &self,
            interaction: &UIDragInteraction,
            session: &ProtocolObject<dyn UIDragSession>,
        );

        #[cfg(all(feature = "UIDragSession", feature = "UIDropInteraction"))]
        #[optional]
        #[method(dragInteraction:session:willEndWithOperation:)]
        unsafe fn dragInteraction_session_willEndWithOperation(
            &self,
            interaction: &UIDragInteraction,
            session: &ProtocolObject<dyn UIDragSession>,
            operation: UIDropOperation,
        );

        #[cfg(all(feature = "UIDragSession", feature = "UIDropInteraction"))]
        #[optional]
        #[method(dragInteraction:session:didEndWithOperation:)]
        unsafe fn dragInteraction_session_didEndWithOperation(
            &self,
            interaction: &UIDragInteraction,
            session: &ProtocolObject<dyn UIDragSession>,
            operation: UIDropOperation,
        );

        #[cfg(feature = "UIDragSession")]
        #[optional]
        #[method(dragInteraction:sessionDidTransferItems:)]
        unsafe fn dragInteraction_sessionDidTransferItems(
            &self,
            interaction: &UIDragInteraction,
            session: &ProtocolObject<dyn UIDragSession>,
        );

        #[cfg(all(
            feature = "UIDragItem",
            feature = "UIDragSession",
            feature = "objc2-core-foundation"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other dragInteraction:itemsForAddingToSession:withTouchAtPoint:)]
        unsafe fn dragInteraction_itemsForAddingToSession_withTouchAtPoint(
            &self,
            interaction: &UIDragInteraction,
            session: &ProtocolObject<dyn UIDragSession>,
            point: CGPoint,
        ) -> Retained<NSArray<UIDragItem>>;

        #[cfg(all(feature = "UIDragSession", feature = "objc2-core-foundation"))]
        #[optional]
        #[method_id(@__retain_semantics Other dragInteraction:sessionForAddingItems:withTouchAtPoint:)]
        unsafe fn dragInteraction_sessionForAddingItems_withTouchAtPoint(
            &self,
            interaction: &UIDragInteraction,
            sessions: &NSArray<ProtocolObject<dyn UIDragSession>>,
            point: CGPoint,
        ) -> Option<Retained<ProtocolObject<dyn UIDragSession>>>;

        #[cfg(all(feature = "UIDragItem", feature = "UIDragSession"))]
        #[optional]
        #[method(dragInteraction:session:willAddItems:forInteraction:)]
        unsafe fn dragInteraction_session_willAddItems_forInteraction(
            &self,
            interaction: &UIDragInteraction,
            session: &ProtocolObject<dyn UIDragSession>,
            items: &NSArray<UIDragItem>,
            adding_interaction: &UIDragInteraction,
        );

        #[cfg(all(
            feature = "UIDragItem",
            feature = "UITargetedDragPreview",
            feature = "UITargetedPreview"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other dragInteraction:previewForCancellingItem:withDefault:)]
        unsafe fn dragInteraction_previewForCancellingItem_withDefault(
            &self,
            interaction: &UIDragInteraction,
            item: &UIDragItem,
            default_preview: &UITargetedDragPreview,
        ) -> Option<Retained<UITargetedDragPreview>>;

        #[cfg(feature = "UIDragItem")]
        #[optional]
        #[method(dragInteraction:item:willAnimateCancelWithAnimator:)]
        unsafe fn dragInteraction_item_willAnimateCancelWithAnimator(
            &self,
            interaction: &UIDragInteraction,
            item: &UIDragItem,
            animator: &ProtocolObject<dyn UIDragAnimating>,
        );
    }

    unsafe impl ProtocolType for dyn UIDragInteractionDelegate {}
);
