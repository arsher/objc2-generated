//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiscreenedgepangesturerecognizer?language=objc)
    #[unsafe(super(UIPanGestureRecognizer, UIGestureRecognizer, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIGestureRecognizer", feature = "UIPanGestureRecognizer"))]
    pub struct UIScreenEdgePanGestureRecognizer;
);

#[cfg(all(feature = "UIGestureRecognizer", feature = "UIPanGestureRecognizer"))]
unsafe impl NSObjectProtocol for UIScreenEdgePanGestureRecognizer {}

extern_methods!(
    #[cfg(all(feature = "UIGestureRecognizer", feature = "UIPanGestureRecognizer"))]
    unsafe impl UIScreenEdgePanGestureRecognizer {
        #[cfg(feature = "UIGeometry")]
        #[method(edges)]
        pub unsafe fn edges(&self) -> UIRectEdge;

        #[cfg(feature = "UIGeometry")]
        #[method(setEdges:)]
        pub unsafe fn setEdges(&self, edges: UIRectEdge);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIGestureRecognizer`
    #[cfg(all(feature = "UIGestureRecognizer", feature = "UIPanGestureRecognizer"))]
    unsafe impl UIScreenEdgePanGestureRecognizer {
        #[method_id(@__retain_semantics Init initWithTarget:action:)]
        pub unsafe fn initWithTarget_action(
            this: Allocated<Self>,
            target: Option<&AnyObject>,
            action: Option<Sel>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIGestureRecognizer", feature = "UIPanGestureRecognizer"))]
    unsafe impl UIScreenEdgePanGestureRecognizer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
