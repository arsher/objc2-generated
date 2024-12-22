//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisnapbehavior?language=objc)
    #[unsafe(super(UIDynamicBehavior, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIDynamicBehavior")]
    pub struct UISnapBehavior;
);

#[cfg(feature = "UIDynamicBehavior")]
unsafe impl NSObjectProtocol for UISnapBehavior {}

extern_methods!(
    #[cfg(feature = "UIDynamicBehavior")]
    unsafe impl UISnapBehavior {
        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithItem:snapToPoint:)]
        pub unsafe fn initWithItem_snapToPoint(
            this: Allocated<Self>,
            item: &ProtocolObject<dyn UIDynamicItem>,
            point: CGPoint,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(snapPoint)]
        pub unsafe fn snapPoint(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`snapPoint`][Self::snapPoint].
        #[method(setSnapPoint:)]
        pub unsafe fn setSnapPoint(&self, snap_point: CGPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(damping)]
        pub unsafe fn damping(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`damping`][Self::damping].
        #[method(setDamping:)]
        pub unsafe fn setDamping(&self, damping: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIDynamicBehavior")]
    unsafe impl UISnapBehavior {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
