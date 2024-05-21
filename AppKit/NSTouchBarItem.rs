//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSTouchBarItemIdentifier = NSString;

// NS_TYPED_EXTENSIBLE_ENUM
pub type NSTouchBarItemPriority = c_float;

pub static NSTouchBarItemPriorityHigh: NSTouchBarItemPriority = 1000 as _;

pub static NSTouchBarItemPriorityNormal: NSTouchBarItemPriority = 0 as _;

pub static NSTouchBarItemPriorityLow: NSTouchBarItemPriority = -1000 as _;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTouchBarItem;

    unsafe impl ClassType for NSTouchBarItem {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSCoding for NSTouchBarItem {}

unsafe impl NSObjectProtocol for NSTouchBarItem {}

extern_methods!(
    unsafe impl NSTouchBarItem {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Allocated<Self>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSTouchBarItemIdentifier>;

        #[method(visibilityPriority)]
        pub unsafe fn visibilityPriority(&self) -> NSTouchBarItemPriority;

        #[method(setVisibilityPriority:)]
        pub unsafe fn setVisibilityPriority(&self, visibility_priority: NSTouchBarItemPriority);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Retained<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSViewController"))]
        #[method_id(@__retain_semantics Other viewController)]
        pub unsafe fn viewController(&self) -> Option<Retained<NSViewController>>;

        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Retained<NSString>;

        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTouchBarItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern "C" {
    pub static NSTouchBarItemIdentifierFixedSpaceSmall: &'static NSTouchBarItemIdentifier;
}

extern "C" {
    pub static NSTouchBarItemIdentifierFixedSpaceLarge: &'static NSTouchBarItemIdentifier;
}

extern "C" {
    pub static NSTouchBarItemIdentifierFlexibleSpace: &'static NSTouchBarItemIdentifier;
}

extern "C" {
    pub static NSTouchBarItemIdentifierOtherItemsProxy: &'static NSTouchBarItemIdentifier;
}
