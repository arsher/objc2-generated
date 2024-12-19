//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_category!(
    /// Category "NSAccessibility" on [`NSObject`].
    #[doc(alias = "NSAccessibility")]
    pub unsafe trait NSObjectNSAccessibility {
        #[cfg(feature = "NSAccessibilityConstants")]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[method_id(@__retain_semantics Other accessibilityAttributeNames)]
        unsafe fn accessibilityAttributeNames(
            &self,
        ) -> Retained<NSArray<NSAccessibilityAttributeName>>;

        #[cfg(feature = "NSAccessibilityConstants")]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[method_id(@__retain_semantics Other accessibilityAttributeValue:)]
        unsafe fn accessibilityAttributeValue(
            &self,
            attribute: &NSAccessibilityAttributeName,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSAccessibilityConstants")]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[method(accessibilityIsAttributeSettable:)]
        unsafe fn accessibilityIsAttributeSettable(
            &self,
            attribute: &NSAccessibilityAttributeName,
        ) -> bool;

        #[cfg(feature = "NSAccessibilityConstants")]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[method(accessibilitySetValue:forAttribute:)]
        unsafe fn accessibilitySetValue_forAttribute(
            &self,
            value: Option<&AnyObject>,
            attribute: &NSAccessibilityAttributeName,
        );

        #[cfg(feature = "NSAccessibilityConstants")]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[method_id(@__retain_semantics Other accessibilityParameterizedAttributeNames)]
        unsafe fn accessibilityParameterizedAttributeNames(
            &self,
        ) -> Retained<NSArray<NSAccessibilityParameterizedAttributeName>>;

        #[cfg(feature = "NSAccessibilityConstants")]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[method_id(@__retain_semantics Other accessibilityAttributeValue:forParameter:)]
        unsafe fn accessibilityAttributeValue_forParameter(
            &self,
            attribute: &NSAccessibilityParameterizedAttributeName,
            parameter: Option<&AnyObject>,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSAccessibilityConstants")]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[method_id(@__retain_semantics Other accessibilityActionNames)]
        unsafe fn accessibilityActionNames(&self) -> Retained<NSArray<NSAccessibilityActionName>>;

        #[cfg(feature = "NSAccessibilityConstants")]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[method_id(@__retain_semantics Other accessibilityActionDescription:)]
        unsafe fn accessibilityActionDescription(
            &self,
            action: &NSAccessibilityActionName,
        ) -> Option<Retained<NSString>>;

        #[cfg(feature = "NSAccessibilityConstants")]
        #[deprecated = "Use the NSAccessibility protocol methods instead (see NSAccessibilityProtocols.h)"]
        #[method(accessibilityPerformAction:)]
        unsafe fn accessibilityPerformAction(&self, action: &NSAccessibilityActionName);

        #[deprecated = "Use isAccessibilityElement instead"]
        #[method(accessibilityIsIgnored)]
        unsafe fn accessibilityIsIgnored(&self) -> bool;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other accessibilityHitTest:)]
        unsafe fn accessibilityHitTest(&self, point: NSPoint) -> Option<Retained<AnyObject>>;

        #[method_id(@__retain_semantics Other accessibilityFocusedUIElement)]
        unsafe fn accessibilityFocusedUIElement(&self) -> Option<Retained<AnyObject>>;

        #[method(accessibilityIndexOfChild:)]
        unsafe fn accessibilityIndexOfChild(&self, child: &AnyObject) -> NSUInteger;

        #[cfg(feature = "NSAccessibilityConstants")]
        #[method(accessibilityArrayAttributeCount:)]
        unsafe fn accessibilityArrayAttributeCount(
            &self,
            attribute: &NSAccessibilityAttributeName,
        ) -> NSUInteger;

        #[cfg(feature = "NSAccessibilityConstants")]
        #[method_id(@__retain_semantics Other accessibilityArrayAttributeValues:index:maxCount:)]
        unsafe fn accessibilityArrayAttributeValues_index_maxCount(
            &self,
            attribute: &NSAccessibilityAttributeName,
            index: NSUInteger,
            max_count: NSUInteger,
        ) -> Retained<NSArray>;

        #[method(accessibilityNotifiesWhenDestroyed)]
        unsafe fn accessibilityNotifiesWhenDestroyed(&self) -> bool;
    }

    unsafe impl NSObjectNSAccessibility for NSObject {}
);

extern_methods!(
    /// NSWorkspaceAccessibilityDisplay
    #[cfg(feature = "NSWorkspace")]
    unsafe impl NSWorkspace {
        #[method(accessibilityDisplayShouldIncreaseContrast)]
        pub unsafe fn accessibilityDisplayShouldIncreaseContrast(&self) -> bool;

        #[method(accessibilityDisplayShouldDifferentiateWithoutColor)]
        pub unsafe fn accessibilityDisplayShouldDifferentiateWithoutColor(&self) -> bool;

        #[method(accessibilityDisplayShouldReduceTransparency)]
        pub unsafe fn accessibilityDisplayShouldReduceTransparency(&self) -> bool;

        #[method(accessibilityDisplayShouldReduceMotion)]
        pub unsafe fn accessibilityDisplayShouldReduceMotion(&self) -> bool;

        #[method(accessibilityDisplayShouldInvertColors)]
        pub unsafe fn accessibilityDisplayShouldInvertColors(&self) -> bool;
    }
);

extern_methods!(
    /// NSWorkspaceAccessibility
    #[cfg(feature = "NSWorkspace")]
    unsafe impl NSWorkspace {
        #[method(isVoiceOverEnabled)]
        pub unsafe fn isVoiceOverEnabled(&self) -> bool;

        #[method(isSwitchControlEnabled)]
        pub unsafe fn isSwitchControlEnabled(&self) -> bool;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsworkspaceaccessibilitydisplayoptionsdidchangenotification?language=objc)
    pub static NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification:
        &'static NSNotificationName;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "NSResponder",
        feature = "NSView",
        feature = "objc2-core-foundation"
    ))]
    pub fn NSAccessibilityFrameInView(parent_view: &NSView, frame: NSRect) -> NSRect;
}

extern "C-unwind" {
    #[cfg(all(
        feature = "NSResponder",
        feature = "NSView",
        feature = "objc2-core-foundation"
    ))]
    pub fn NSAccessibilityPointInView(parent_view: &NSView, point: NSPoint) -> NSPoint;
}

extern "C-unwind" {
    pub fn NSAccessibilitySetMayContainProtectedContent(flag: Bool) -> Bool;
}

extern "C-unwind" {
    #[cfg(feature = "NSAccessibilityConstants")]
    pub fn NSAccessibilityRoleDescription(
        role: &NSAccessibilityRole,
        subrole: Option<&NSAccessibilitySubrole>,
    ) -> *mut NSString;
}

extern "C-unwind" {
    pub fn NSAccessibilityRoleDescriptionForUIElement(element: &AnyObject) -> *mut NSString;
}

extern "C-unwind" {
    #[cfg(feature = "NSAccessibilityConstants")]
    pub fn NSAccessibilityActionDescription(action: &NSAccessibilityActionName) -> *mut NSString;
}

extern "C-unwind" {
    #[cfg(feature = "NSAccessibilityConstants")]
    #[deprecated = "Exceptions are no longer appropriate for indicating errors in accessibility API. Unexpected values should be handled through appropriate type checking."]
    pub fn NSAccessibilityRaiseBadArgumentException(
        element: Option<&AnyObject>,
        attribute: Option<&NSAccessibilityAttributeName>,
        value: Option<&AnyObject>,
    );
}

extern "C-unwind" {
    pub fn NSAccessibilityUnignoredAncestor(element: &AnyObject) -> *mut AnyObject;
}

extern "C-unwind" {
    pub fn NSAccessibilityUnignoredDescendant(element: &AnyObject) -> *mut AnyObject;
}

extern "C-unwind" {
    pub fn NSAccessibilityUnignoredChildren(original_children: &NSArray) -> NonNull<NSArray>;
}

extern "C-unwind" {
    pub fn NSAccessibilityUnignoredChildrenForOnlyChild(
        original_child: &AnyObject,
    ) -> NonNull<NSArray>;
}

extern "C-unwind" {
    #[cfg(feature = "NSAccessibilityConstants")]
    pub fn NSAccessibilityPostNotification(
        element: &AnyObject,
        notification: &NSAccessibilityNotificationName,
    );
}
