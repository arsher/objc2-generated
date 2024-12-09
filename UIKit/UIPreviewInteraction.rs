//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipreviewinteraction?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPreviewInteraction;
);

unsafe impl NSObjectProtocol for UIPreviewInteraction {}

extern_methods!(
    unsafe impl UIPreviewInteraction {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Init initWithView:)]
        pub unsafe fn initWithView(this: Allocated<Self>, view: &UIView) -> Retained<Self>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Retained<UIView>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIPreviewInteractionDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UIPreviewInteractionDelegate>>,
        );

        #[cfg(all(feature = "UIView", feature = "objc2-core-foundation"))]
        #[method(locationInCoordinateSpace:)]
        pub unsafe fn locationInCoordinateSpace(
            &self,
            coordinate_space: Option<&ProtocolObject<dyn UICoordinateSpace>>,
        ) -> CGPoint;

        #[method(cancelInteraction)]
        pub unsafe fn cancelInteraction(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIPreviewInteraction {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipreviewinteractiondelegate?language=objc)
    pub unsafe trait UIPreviewInteractionDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(previewInteraction:didUpdatePreviewTransition:ended:)]
        unsafe fn previewInteraction_didUpdatePreviewTransition_ended(
            &self,
            preview_interaction: &UIPreviewInteraction,
            transition_progress: CGFloat,
            ended: bool,
        );

        #[method(previewInteractionDidCancel:)]
        unsafe fn previewInteractionDidCancel(&self, preview_interaction: &UIPreviewInteraction);

        #[optional]
        #[method(previewInteractionShouldBegin:)]
        unsafe fn previewInteractionShouldBegin(
            &self,
            preview_interaction: &UIPreviewInteraction,
        ) -> bool;

        #[cfg(feature = "objc2-core-foundation")]
        #[optional]
        #[method(previewInteraction:didUpdateCommitTransition:ended:)]
        unsafe fn previewInteraction_didUpdateCommitTransition_ended(
            &self,
            preview_interaction: &UIPreviewInteraction,
            transition_progress: CGFloat,
            ended: bool,
        );
    }

    unsafe impl ProtocolType for dyn UIPreviewInteractionDelegate {}
);
