//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipointerinteraction?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPointerInteraction;
);

unsafe impl NSObjectProtocol for UIPointerInteraction {}

#[cfg(feature = "UIInteraction")]
unsafe impl UIInteraction for UIPointerInteraction {}

extern_methods!(
    unsafe impl UIPointerInteraction {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIPointerInteractionDelegate>>>;

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method_id(@__retain_semantics Init initWithDelegate:)]
        pub unsafe fn initWithDelegate(
            this: Allocated<Self>,
            delegate: Option<&ProtocolObject<dyn UIPointerInteractionDelegate>>,
        ) -> Retained<Self>;

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIPointerInteraction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipointerinteractiondelegate?language=objc)
    pub unsafe trait UIPointerInteractionDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(feature = "UIPointerRegion")]
        #[optional]
        #[method_id(@__retain_semantics Other pointerInteraction:regionForRequest:defaultRegion:)]
        unsafe fn pointerInteraction_regionForRequest_defaultRegion(
            &self,
            interaction: &UIPointerInteraction,
            request: &UIPointerRegionRequest,
            default_region: &UIPointerRegion,
        ) -> Option<Retained<UIPointerRegion>>;

        #[cfg(all(
            feature = "UIHoverStyle",
            feature = "UIPointerRegion",
            feature = "UIPointerStyle"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other pointerInteraction:styleForRegion:)]
        unsafe fn pointerInteraction_styleForRegion(
            &self,
            interaction: &UIPointerInteraction,
            region: &UIPointerRegion,
        ) -> Option<Retained<UIPointerStyle>>;

        #[cfg(feature = "UIPointerRegion")]
        #[optional]
        #[method(pointerInteraction:willEnterRegion:animator:)]
        unsafe fn pointerInteraction_willEnterRegion_animator(
            &self,
            interaction: &UIPointerInteraction,
            region: &UIPointerRegion,
            animator: &ProtocolObject<dyn UIPointerInteractionAnimating>,
        );

        #[cfg(feature = "UIPointerRegion")]
        #[optional]
        #[method(pointerInteraction:willExitRegion:animator:)]
        unsafe fn pointerInteraction_willExitRegion_animator(
            &self,
            interaction: &UIPointerInteraction,
            region: &UIPointerRegion,
            animator: &ProtocolObject<dyn UIPointerInteractionAnimating>,
        );
    }

    unsafe impl ProtocolType for dyn UIPointerInteractionDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipointerregionrequest?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPointerRegionRequest;
);

unsafe impl NSObjectProtocol for UIPointerRegionRequest {}

extern_methods!(
    unsafe impl UIPointerRegionRequest {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(location)]
        pub unsafe fn location(&self) -> CGPoint;

        #[cfg(feature = "UICommand")]
        #[method(modifiers)]
        pub unsafe fn modifiers(&self) -> UIKeyModifierFlags;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIPointerRegionRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipointerinteractionanimating?language=objc)
    pub unsafe trait UIPointerInteractionAnimating:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(feature = "block2")]
        #[method(addAnimations:)]
        unsafe fn addAnimations(&self, animations: &block2::Block<dyn Fn()>);

        #[cfg(feature = "block2")]
        #[method(addCompletion:)]
        unsafe fn addCompletion(&self, completion: &block2::Block<dyn Fn(Bool)>);
    }

    unsafe impl ProtocolType for dyn UIPointerInteractionAnimating {}
);
