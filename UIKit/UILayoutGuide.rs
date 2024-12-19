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
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uilayoutguide?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UILayoutGuide;
);

unsafe impl NSCoding for UILayoutGuide {}

unsafe impl NSObjectProtocol for UILayoutGuide {}

extern_methods!(
    unsafe impl UILayoutGuide {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(layoutFrame)]
        pub unsafe fn layoutFrame(&self) -> CGRect;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other owningView)]
        pub unsafe fn owningView(&self) -> Option<Retained<UIView>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setOwningView:)]
        pub unsafe fn setOwningView(&self, owning_view: Option<&UIView>);

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: &NSString);

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other leadingAnchor)]
        pub unsafe fn leadingAnchor(&self) -> Retained<NSLayoutXAxisAnchor>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other trailingAnchor)]
        pub unsafe fn trailingAnchor(&self) -> Retained<NSLayoutXAxisAnchor>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other leftAnchor)]
        pub unsafe fn leftAnchor(&self) -> Retained<NSLayoutXAxisAnchor>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other rightAnchor)]
        pub unsafe fn rightAnchor(&self) -> Retained<NSLayoutXAxisAnchor>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other topAnchor)]
        pub unsafe fn topAnchor(&self) -> Retained<NSLayoutYAxisAnchor>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other bottomAnchor)]
        pub unsafe fn bottomAnchor(&self) -> Retained<NSLayoutYAxisAnchor>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other widthAnchor)]
        pub unsafe fn widthAnchor(&self) -> Retained<NSLayoutDimension>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other heightAnchor)]
        pub unsafe fn heightAnchor(&self) -> Retained<NSLayoutDimension>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other centerXAnchor)]
        pub unsafe fn centerXAnchor(&self) -> Retained<NSLayoutXAxisAnchor>;

        #[cfg(feature = "NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other centerYAnchor)]
        pub unsafe fn centerYAnchor(&self) -> Retained<NSLayoutYAxisAnchor>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UILayoutGuide {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
