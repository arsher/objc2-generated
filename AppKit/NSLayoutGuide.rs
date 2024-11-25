//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nslayoutguide?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSLayoutGuide;
);

unsafe impl NSCoding for NSLayoutGuide {}

unsafe impl NSObjectProtocol for NSLayoutGuide {}

#[cfg(feature = "NSUserInterfaceItemIdentification")]
unsafe impl NSUserInterfaceItemIdentification for NSLayoutGuide {}

extern_methods!(
    unsafe impl NSLayoutGuide {
        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other owningView)]
        pub unsafe fn owningView(&self, mtm: MainThreadMarker) -> Option<Retained<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setOwningView:)]
        pub unsafe fn setOwningView(&self, owning_view: Option<&NSView>);

        #[cfg(feature = "NSUserInterfaceItemIdentification")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSUserInterfaceItemIdentifier>;

        #[cfg(feature = "NSUserInterfaceItemIdentification")]
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: &NSUserInterfaceItemIdentifier);

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

        #[method(hasAmbiguousLayout)]
        pub unsafe fn hasAmbiguousLayout(&self) -> bool;

        #[cfg(feature = "NSLayoutConstraint")]
        #[method_id(@__retain_semantics Other constraintsAffectingLayoutForOrientation:)]
        pub unsafe fn constraintsAffectingLayoutForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> Retained<NSArray<NSLayoutConstraint>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSLayoutGuide {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSLayoutGuideSupport
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSView {
        #[method(addLayoutGuide:)]
        pub unsafe fn addLayoutGuide(&self, guide: &NSLayoutGuide);

        #[method(removeLayoutGuide:)]
        pub unsafe fn removeLayoutGuide(&self, guide: &NSLayoutGuide);

        #[method_id(@__retain_semantics Other layoutGuides)]
        pub unsafe fn layoutGuides(&self) -> Retained<NSArray<NSLayoutGuide>>;
    }
);
