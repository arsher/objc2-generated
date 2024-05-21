//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIResponder")]
    pub struct UIAccessibilityElement;

    #[cfg(feature = "UIResponder")]
    unsafe impl ClassType for UIAccessibilityElement {
        #[inherits(NSObject)]
        type Super = UIResponder;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "UIResponder")]
unsafe impl NSObjectProtocol for UIAccessibilityElement {}

#[cfg(all(feature = "UIAccessibilityIdentification", feature = "UIResponder"))]
unsafe impl UIAccessibilityIdentification for UIAccessibilityElement {}

#[cfg(feature = "UIResponder")]
unsafe impl UIResponderStandardEditActions for UIAccessibilityElement {}

extern_methods!(
    #[cfg(feature = "UIResponder")]
    unsafe impl UIAccessibilityElement {
        #[method_id(@__retain_semantics Init initWithAccessibilityContainer:)]
        pub unsafe fn initWithAccessibilityContainer(
            this: Allocated<Self>,
            container: &AnyObject,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other accessibilityContainer)]
        pub unsafe fn accessibilityContainer(&self) -> Option<Retained<AnyObject>>;

        #[method(setAccessibilityContainer:)]
        pub unsafe fn setAccessibilityContainer(&self, accessibility_container: Option<&AnyObject>);

        #[method(isAccessibilityElement)]
        pub unsafe fn isAccessibilityElement(&self) -> bool;

        #[method(setIsAccessibilityElement:)]
        pub unsafe fn setIsAccessibilityElement(&self, is_accessibility_element: bool);

        #[method_id(@__retain_semantics Other accessibilityLabel)]
        pub unsafe fn accessibilityLabel(&self) -> Option<Retained<NSString>>;

        #[method(setAccessibilityLabel:)]
        pub unsafe fn setAccessibilityLabel(&self, accessibility_label: Option<&NSString>);

        #[method_id(@__retain_semantics Other accessibilityHint)]
        pub unsafe fn accessibilityHint(&self) -> Option<Retained<NSString>>;

        #[method(setAccessibilityHint:)]
        pub unsafe fn setAccessibilityHint(&self, accessibility_hint: Option<&NSString>);

        #[method_id(@__retain_semantics Other accessibilityValue)]
        pub unsafe fn accessibilityValue(&self) -> Option<Retained<NSString>>;

        #[method(setAccessibilityValue:)]
        pub unsafe fn setAccessibilityValue(&self, accessibility_value: Option<&NSString>);

        #[method(accessibilityFrame)]
        pub unsafe fn accessibilityFrame(&self) -> CGRect;

        #[method(setAccessibilityFrame:)]
        pub unsafe fn setAccessibilityFrame(&self, accessibility_frame: CGRect);

        #[cfg(feature = "UIAccessibilityConstants")]
        #[method(accessibilityTraits)]
        pub unsafe fn accessibilityTraits(&self) -> UIAccessibilityTraits;

        #[cfg(feature = "UIAccessibilityConstants")]
        #[method(setAccessibilityTraits:)]
        pub unsafe fn setAccessibilityTraits(&self, accessibility_traits: UIAccessibilityTraits);

        #[method(accessibilityFrameInContainerSpace)]
        pub unsafe fn accessibilityFrameInContainerSpace(&self) -> CGRect;

        #[method(setAccessibilityFrameInContainerSpace:)]
        pub unsafe fn setAccessibilityFrameInContainerSpace(
            &self,
            accessibility_frame_in_container_space: CGRect,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIResponder")]
    unsafe impl UIAccessibilityElement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
