//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitrackinglayoutguide?language=objc)
    #[unsafe(super(UILayoutGuide, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UILayoutGuide")]
    pub struct UITrackingLayoutGuide;
);

#[cfg(feature = "UILayoutGuide")]
unsafe impl NSCoding for UITrackingLayoutGuide {}

#[cfg(feature = "UILayoutGuide")]
unsafe impl NSObjectProtocol for UITrackingLayoutGuide {}

extern_methods!(
    #[cfg(feature = "UILayoutGuide")]
    unsafe impl UITrackingLayoutGuide {
        #[cfg(all(feature = "NSLayoutConstraint", feature = "UIGeometry"))]
        #[method(setConstraints:activeWhenNearEdge:)]
        pub unsafe fn setConstraints_activeWhenNearEdge(
            &self,
            tracking_constraints: &NSArray<NSLayoutConstraint>,
            edge: NSDirectionalRectEdge,
        );

        #[cfg(all(feature = "NSLayoutConstraint", feature = "UIGeometry"))]
        #[method_id(@__retain_semantics Other constraintsActiveWhenNearEdge:)]
        pub unsafe fn constraintsActiveWhenNearEdge(
            &self,
            edge: NSDirectionalRectEdge,
        ) -> Retained<NSArray<NSLayoutConstraint>>;

        #[cfg(all(feature = "NSLayoutConstraint", feature = "UIGeometry"))]
        #[method(setConstraints:activeWhenAwayFromEdge:)]
        pub unsafe fn setConstraints_activeWhenAwayFromEdge(
            &self,
            tracking_constraints: &NSArray<NSLayoutConstraint>,
            edge: NSDirectionalRectEdge,
        );

        #[cfg(all(feature = "NSLayoutConstraint", feature = "UIGeometry"))]
        #[method_id(@__retain_semantics Other constraintsActiveWhenAwayFromEdge:)]
        pub unsafe fn constraintsActiveWhenAwayFromEdge(
            &self,
            edge: NSDirectionalRectEdge,
        ) -> Retained<NSArray<NSLayoutConstraint>>;

        #[method(removeAllTrackedConstraints)]
        pub unsafe fn removeAllTrackedConstraints(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UILayoutGuide")]
    unsafe impl UITrackingLayoutGuide {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
