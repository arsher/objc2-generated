//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsmenuitem?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMenuItem;
);

#[cfg(feature = "NSAccessibilityProtocols")]
unsafe impl NSAccessibility for NSMenuItem {}

#[cfg(feature = "NSAccessibilityProtocols")]
unsafe impl NSAccessibilityElementProtocol for NSMenuItem {}

unsafe impl NSCoding for NSMenuItem {}

unsafe impl NSCopying for NSMenuItem {}

unsafe impl CopyingHelper for NSMenuItem {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSMenuItem {}

#[cfg(feature = "NSUserInterfaceItemIdentification")]
unsafe impl NSUserInterfaceItemIdentification for NSMenuItem {}

#[cfg(feature = "NSUserInterfaceValidation")]
unsafe impl NSValidatedUserInterfaceItem for NSMenuItem {}

extern_methods!(
    unsafe impl NSMenuItem {
        #[method(usesUserKeyEquivalents)]
        pub unsafe fn usesUserKeyEquivalents(mtm: MainThreadMarker) -> bool;

        /// Setter for [`usesUserKeyEquivalents`][Self::usesUserKeyEquivalents].
        #[method(setUsesUserKeyEquivalents:)]
        pub unsafe fn setUsesUserKeyEquivalents(
            uses_user_key_equivalents: bool,
            mtm: MainThreadMarker,
        );

        #[method_id(@__retain_semantics Other separatorItem)]
        pub fn separatorItem(mtm: MainThreadMarker) -> Retained<NSMenuItem>;

        /// Creates a menu item representing a section header with the provided title.
        /// Section header items are used to provide context to a grouping of menu items.
        /// Items created using this method are non-interactive and do not perform an action.
        #[method_id(@__retain_semantics Other sectionHeaderWithTitle:)]
        pub unsafe fn sectionHeaderWithTitle(
            title: &NSString,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        /// An array of standard menu items related to Writing Tools.
        /// Each call to this method returns an array of newly allocated instances of NSMenuItem.
        #[method_id(@__retain_semantics Other writingToolsItems)]
        pub unsafe fn writingToolsItems(mtm: MainThreadMarker) -> Retained<NSArray<NSMenuItem>>;

        #[method_id(@__retain_semantics Init initWithTitle:action:keyEquivalent:)]
        pub unsafe fn initWithTitle_action_keyEquivalent(
            this: Allocated<Self>,
            string: &NSString,
            selector: Option<Sel>,
            char_code: &NSString,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Retained<Self>;

        #[cfg(feature = "NSMenu")]
        /// Note: Never call the setter method directly: it is there only for subclassers.
        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Option<Retained<NSMenu>>;

        #[cfg(feature = "NSMenu")]
        /// Setter for [`menu`][Self::menu].
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);

        #[method(hasSubmenu)]
        pub unsafe fn hasSubmenu(&self) -> bool;

        #[cfg(feature = "NSMenu")]
        #[method_id(@__retain_semantics Other submenu)]
        pub unsafe fn submenu(&self) -> Option<Retained<NSMenu>>;

        #[cfg(feature = "NSMenu")]
        /// Setter for [`submenu`][Self::submenu].
        #[method(setSubmenu:)]
        pub fn setSubmenu(&self, submenu: Option<&NSMenu>);

        /// Returns: The `NSMenuItem` whose submenu contains the receiver, or nil if the receiver does not have a parent item.
        #[method_id(@__retain_semantics Other parentItem)]
        pub unsafe fn parentItem(&self) -> Option<Retained<NSMenuItem>>;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        /// Setter for [`title`][Self::title].
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[method_id(@__retain_semantics Other attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Option<Retained<NSAttributedString>>;

        /// Setter for [`attributedTitle`][Self::attributedTitle].
        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributed_title: Option<&NSAttributedString>);

        /// Used to specify a standard subtitle for the menu item.
        ///
        /// The subtitle is displayed below the standard title.
        ///
        /// Note: On macOS 14, a menu item with an attributed title does not show the subtitle. The subtitle is shown on macOS 15 and later.
        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Option<Retained<NSString>>;

        /// Setter for [`subtitle`][Self::subtitle].
        #[method(setSubtitle:)]
        pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>);

        #[method(isSeparatorItem)]
        pub unsafe fn isSeparatorItem(&self) -> bool;

        /// Indicates whether the item is a section header.
        /// Section header items are created using the `sectionHeader(title:)` class method.
        #[method(isSectionHeader)]
        pub unsafe fn isSectionHeader(&self) -> bool;

        #[method_id(@__retain_semantics Other keyEquivalent)]
        pub unsafe fn keyEquivalent(&self) -> Retained<NSString>;

        /// Setter for [`keyEquivalent`][Self::keyEquivalent].
        #[method(setKeyEquivalent:)]
        pub unsafe fn setKeyEquivalent(&self, key_equivalent: &NSString);

        #[cfg(feature = "NSEvent")]
        #[method(keyEquivalentModifierMask)]
        pub unsafe fn keyEquivalentModifierMask(&self) -> NSEventModifierFlags;

        #[cfg(feature = "NSEvent")]
        /// Setter for [`keyEquivalentModifierMask`][Self::keyEquivalentModifierMask].
        #[method(setKeyEquivalentModifierMask:)]
        pub fn setKeyEquivalentModifierMask(
            &self,
            key_equivalent_modifier_mask: NSEventModifierFlags,
        );

        #[method_id(@__retain_semantics Other userKeyEquivalent)]
        pub unsafe fn userKeyEquivalent(&self) -> Retained<NSString>;

        #[method(allowsKeyEquivalentWhenHidden)]
        pub unsafe fn allowsKeyEquivalentWhenHidden(&self) -> bool;

        /// Setter for [`allowsKeyEquivalentWhenHidden`][Self::allowsKeyEquivalentWhenHidden].
        #[method(setAllowsKeyEquivalentWhenHidden:)]
        pub unsafe fn setAllowsKeyEquivalentWhenHidden(
            &self,
            allows_key_equivalent_when_hidden: bool,
        );

        #[method(allowsAutomaticKeyEquivalentLocalization)]
        pub unsafe fn allowsAutomaticKeyEquivalentLocalization(&self) -> bool;

        /// Setter for [`allowsAutomaticKeyEquivalentLocalization`][Self::allowsAutomaticKeyEquivalentLocalization].
        #[method(setAllowsAutomaticKeyEquivalentLocalization:)]
        pub unsafe fn setAllowsAutomaticKeyEquivalentLocalization(
            &self,
            allows_automatic_key_equivalent_localization: bool,
        );

        #[method(allowsAutomaticKeyEquivalentMirroring)]
        pub unsafe fn allowsAutomaticKeyEquivalentMirroring(&self) -> bool;

        /// Setter for [`allowsAutomaticKeyEquivalentMirroring`][Self::allowsAutomaticKeyEquivalentMirroring].
        #[method(setAllowsAutomaticKeyEquivalentMirroring:)]
        pub unsafe fn setAllowsAutomaticKeyEquivalentMirroring(
            &self,
            allows_automatic_key_equivalent_mirroring: bool,
        );

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        /// Setter for [`image`][Self::image].
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[cfg(feature = "NSCell")]
        #[method(state)]
        pub unsafe fn state(&self) -> NSControlStateValue;

        #[cfg(feature = "NSCell")]
        /// Setter for [`state`][Self::state].
        #[method(setState:)]
        pub unsafe fn setState(&self, state: NSControlStateValue);

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other onStateImage)]
        pub unsafe fn onStateImage(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        /// Setter for [`onStateImage`][Self::onStateImage].
        #[method(setOnStateImage:)]
        pub unsafe fn setOnStateImage(&self, on_state_image: Option<&NSImage>);

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other offStateImage)]
        pub unsafe fn offStateImage(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        /// Setter for [`offStateImage`][Self::offStateImage].
        #[method(setOffStateImage:)]
        pub unsafe fn setOffStateImage(&self, off_state_image: Option<&NSImage>);

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other mixedStateImage)]
        pub unsafe fn mixedStateImage(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        /// Setter for [`mixedStateImage`][Self::mixedStateImage].
        #[method(setMixedStateImage:)]
        pub unsafe fn setMixedStateImage(&self, mixed_state_image: Option<&NSImage>);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        /// Setter for [`isEnabled`][Self::isEnabled].
        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(isAlternate)]
        pub unsafe fn isAlternate(&self) -> bool;

        /// Setter for [`isAlternate`][Self::isAlternate].
        #[method(setAlternate:)]
        pub unsafe fn setAlternate(&self, alternate: bool);

        #[method(indentationLevel)]
        pub unsafe fn indentationLevel(&self) -> NSInteger;

        /// Setter for [`indentationLevel`][Self::indentationLevel].
        #[method(setIndentationLevel:)]
        pub unsafe fn setIndentationLevel(&self, indentation_level: NSInteger);

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Retained<AnyObject>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`target`][Self::target].
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&AnyObject>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        /// Setter for [`action`][Self::action].
        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;

        /// Setter for [`tag`][Self::tag].
        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);

        #[method_id(@__retain_semantics Other representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Retained<AnyObject>>;

        /// Setter for [`representedObject`][Self::representedObject].
        #[method(setRepresentedObject:)]
        pub unsafe fn setRepresentedObject(&self, represented_object: Option<&AnyObject>);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Retained<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        /// Setter for [`view`][Self::view].
        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&NSView>);

        #[method(isHighlighted)]
        pub unsafe fn isHighlighted(&self) -> bool;

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        /// Setter for [`isHidden`][Self::isHidden].
        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);

        #[method(isHiddenOrHasHiddenAncestor)]
        pub unsafe fn isHiddenOrHasHiddenAncestor(&self) -> bool;

        #[method_id(@__retain_semantics Other toolTip)]
        pub unsafe fn toolTip(&self) -> Option<Retained<NSString>>;

        /// Setter for [`toolTip`][Self::toolTip].
        #[method(setToolTip:)]
        pub unsafe fn setToolTip(&self, tool_tip: Option<&NSString>);

        #[cfg(feature = "NSMenuItemBadge")]
        /// A badge used to provide additional quantitative information specific to
        /// the menu item, such as the number of available updates.
        ///
        ///
        /// Note: The default value of this property is
        /// `nil.`
        #[method_id(@__retain_semantics Other badge)]
        pub unsafe fn badge(&self) -> Option<Retained<NSMenuItemBadge>>;

        #[cfg(feature = "NSMenuItemBadge")]
        /// Setter for [`badge`][Self::badge].
        #[method(setBadge:)]
        pub unsafe fn setBadge(&self, badge: Option<&NSMenuItemBadge>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSMenuItem {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSViewEnclosingMenuItem
    #[cfg(all(feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSView {
        #[method_id(@__retain_semantics Other enclosingMenuItem)]
        pub unsafe fn enclosingMenuItem(&self) -> Option<Retained<NSMenuItem>>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsmenuitemimportfromdeviceidentifier?language=objc)
    #[cfg(feature = "NSUserInterfaceItemIdentification")]
    pub static NSMenuItemImportFromDeviceIdentifier: &'static NSUserInterfaceItemIdentifier;
}

extern_methods!(
    /// NSDeprecated
    unsafe impl NSMenuItem {
        #[deprecated]
        #[method(setMnemonicLocation:)]
        pub unsafe fn setMnemonicLocation(&self, location: NSUInteger);

        #[deprecated]
        #[method(mnemonicLocation)]
        pub unsafe fn mnemonicLocation(&self) -> NSUInteger;

        #[deprecated]
        #[method_id(@__retain_semantics Other mnemonic)]
        pub unsafe fn mnemonic(&self) -> Option<Retained<NSString>>;

        #[deprecated]
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, string_with_ampersand: &NSString);
    }
);
