//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uizoomtransitionoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIZoomTransitionOptions;
);

unsafe impl NSCopying for UIZoomTransitionOptions {}

unsafe impl CopyingHelper for UIZoomTransitionOptions {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UIZoomTransitionOptions {}

extern_methods!(
    unsafe impl UIZoomTransitionOptions {
        #[cfg(feature = "block2")]
        #[method(interactiveDismissShouldBegin)]
        pub unsafe fn interactiveDismissShouldBegin(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<UIZoomTransitionInteractionContext>) -> Bool>;

        #[cfg(feature = "block2")]
        #[method(setInteractiveDismissShouldBegin:)]
        pub unsafe fn setInteractiveDismissShouldBegin(
            &self,
            interactive_dismiss_should_begin: Option<
                &block2::Block<dyn Fn(NonNull<UIZoomTransitionInteractionContext>) -> Bool>,
            >,
        );

        #[cfg(all(feature = "block2", feature = "objc2-core-foundation"))]
        #[method(alignmentRectProvider)]
        pub unsafe fn alignmentRectProvider(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<UIZoomTransitionAlignmentRectContext>) -> CGRect>;

        #[cfg(all(feature = "block2", feature = "objc2-core-foundation"))]
        #[method(setAlignmentRectProvider:)]
        pub unsafe fn setAlignmentRectProvider(
            &self,
            alignment_rect_provider: Option<
                &block2::Block<dyn Fn(NonNull<UIZoomTransitionAlignmentRectContext>) -> CGRect>,
            >,
        );

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other dimmingColor)]
        pub unsafe fn dimmingColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setDimmingColor:)]
        pub unsafe fn setDimmingColor(&self, dimming_color: Option<&UIColor>);

        #[cfg(all(feature = "UIBlurEffect", feature = "UIVisualEffect"))]
        #[method_id(@__retain_semantics Other dimmingVisualEffect)]
        pub unsafe fn dimmingVisualEffect(
            &self,
            mtm: MainThreadMarker,
        ) -> Option<Retained<UIBlurEffect>>;

        #[cfg(all(feature = "UIBlurEffect", feature = "UIVisualEffect"))]
        #[method(setDimmingVisualEffect:)]
        pub unsafe fn setDimmingVisualEffect(&self, dimming_visual_effect: Option<&UIBlurEffect>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIZoomTransitionOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uizoomtransitioninteractioncontext?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIZoomTransitionInteractionContext;
);

unsafe impl NSObjectProtocol for UIZoomTransitionInteractionContext {}

extern_methods!(
    unsafe impl UIZoomTransitionInteractionContext {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(location)]
        pub unsafe fn location(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(velocity)]
        pub unsafe fn velocity(&self) -> CGVector;

        #[method(willBegin)]
        pub unsafe fn willBegin(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIZoomTransitionInteractionContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uizoomtransitionalignmentrectcontext?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIZoomTransitionAlignmentRectContext;
);

unsafe impl NSObjectProtocol for UIZoomTransitionAlignmentRectContext {}

extern_methods!(
    unsafe impl UIZoomTransitionAlignmentRectContext {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other sourceView)]
        pub unsafe fn sourceView(&self, mtm: MainThreadMarker) -> Retained<UIView>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[method_id(@__retain_semantics Other zoomedViewController)]
        pub unsafe fn zoomedViewController(
            &self,
            mtm: MainThreadMarker,
        ) -> Retained<UIViewController>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIZoomTransitionAlignmentRectContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
