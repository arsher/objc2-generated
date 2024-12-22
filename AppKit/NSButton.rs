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
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsbutton?language=objc)
    #[unsafe(super(NSControl, NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    pub struct NSButton;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSButton {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityButton for NSButton {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSButton {}

#[cfg(all(
    feature = "NSAnimation",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSButton {}

#[cfg(all(
    feature = "NSAppearance",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAppearanceCustomization for NSButton {}

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSButton {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSDragging",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSDraggingDestination for NSButton {}

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSButton {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSUserInterfaceCompression",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceCompression for NSButton {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSButton {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSUserInterfaceValidation",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceValidations for NSButton {}

extern_methods!(
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSButton {
        #[cfg(feature = "NSImage")]
        /// Creates a standard push button with a title and image.
        ///
        /// Parameter `title`: The localized title string that is displayed on the button.
        ///
        /// Parameter `image`: The image that is displayed alongside the title. In left-to-right localizations, the image is displayed to the left of the title. In right-to-left localizations, it is displayed to the right.
        ///
        /// Parameter `target`: The target object that receives action messages from the control.
        ///
        /// Parameter `action`: The action message sent by the control.
        ///
        /// Returns: An initialized button object.
        #[method_id(@__retain_semantics Other buttonWithTitle:image:target:action:)]
        pub unsafe fn buttonWithTitle_image_target_action(
            title: &NSString,
            image: &NSImage,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        /// Creates a standard push button with the provided title.
        ///
        /// Parameter `title`: The localized title string that is displayed on the button.
        ///
        /// Parameter `target`: The target object that receives action messages from the control.
        ///
        /// Parameter `action`: The action message sent by the control.
        ///
        /// Returns: An initialized button object.
        #[method_id(@__retain_semantics Other buttonWithTitle:target:action:)]
        pub unsafe fn buttonWithTitle_target_action(
            title: &NSString,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "NSImage")]
        /// Creates a standard push button with the provided image. Set the image's accessibilityDescription property to ensure accessibility for this control.
        ///
        /// Parameter `image`: The image to display in the body of the button.
        ///
        /// Parameter `target`: The target object that receives action messages from the control.
        ///
        /// Parameter `action`: The action message sent by the control.
        ///
        /// Returns: An initialized button object.
        #[method_id(@__retain_semantics Other buttonWithImage:target:action:)]
        pub unsafe fn buttonWithImage_target_action(
            image: &NSImage,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        /// Creates a standard checkbox with the provided title.
        ///
        /// Parameter `title`: The localized title string that is displayed alongside the checkbox.
        ///
        /// Parameter `target`: The target object that receives action messages from the control.
        ///
        /// Parameter `action`: The action message sent by the control.
        ///
        /// Returns: An initialized button object.
        #[method_id(@__retain_semantics Other checkboxWithTitle:target:action:)]
        pub unsafe fn checkboxWithTitle_target_action(
            title: &NSString,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        /// Creates a standard radio button with the provided title.
        ///
        /// Parameter `title`: The localized title string that is displayed alongside the radio button.
        ///
        /// Parameter `target`: The target object that receives action messages from the control.
        ///
        /// Parameter `action`: The action message sent by the control.
        ///
        /// Returns: An initialized button object.
        #[method_id(@__retain_semantics Other radioButtonWithTitle:target:action:)]
        pub unsafe fn radioButtonWithTitle_target_action(
            title: &NSString,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "NSButtonCell")]
        /// Sets the button’s type, which affects its user interface and behavior when clicked. See the NSButtonType enumeration for possible options and their behaviors.
        #[method(setButtonType:)]
        pub unsafe fn setButtonType(&self, r#type: NSButtonType);

        /// The title displayed on the button when it’s in an off state, or an empty string if the button does not display a title. By default, a button's title is "Button".
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Retained<NSString>;

        /// Setter for [`title`][Self::title].
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        /// The button's title, expressed as an attributed string.
        #[method_id(@__retain_semantics Other attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Retained<NSAttributedString>;

        /// Setter for [`attributedTitle`][Self::attributedTitle].
        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributed_title: &NSAttributedString);

        /// The title that the button displays when the button is in an on state, or an empty string if there is no such title. Note that some button types do not display an alternate title.
        #[method_id(@__retain_semantics Other alternateTitle)]
        pub unsafe fn alternateTitle(&self) -> Retained<NSString>;

        /// Setter for [`alternateTitle`][Self::alternateTitle].
        #[method(setAlternateTitle:)]
        pub unsafe fn setAlternateTitle(&self, alternate_title: &NSString);

        /// The alternate title, expressed as an attributed string.
        #[method_id(@__retain_semantics Other attributedAlternateTitle)]
        pub unsafe fn attributedAlternateTitle(&self) -> Retained<NSAttributedString>;

        /// Setter for [`attributedAlternateTitle`][Self::attributedAlternateTitle].
        #[method(setAttributedAlternateTitle:)]
        pub unsafe fn setAttributedAlternateTitle(
            &self,
            attributed_alternate_title: &NSAttributedString,
        );

        /// Indicates whether the button's action has a destructive effect on user data.  AppKit may guard a destructive-actioned button against accidental presses, and may give the button a special appearance in certain contexts to caution against unintentional use.  Defaults to NO.
        #[method(hasDestructiveAction)]
        pub unsafe fn hasDestructiveAction(&self) -> bool;

        /// Setter for [`hasDestructiveAction`][Self::hasDestructiveAction].
        #[method(setHasDestructiveAction:)]
        pub unsafe fn setHasDestructiveAction(&self, has_destructive_action: bool);

        #[cfg(feature = "NSSound")]
        /// The sound that plays when the user clicks the button, or nil if the button should not play a sound. The default value is nil.
        #[method_id(@__retain_semantics Other sound)]
        pub unsafe fn sound(&self) -> Option<Retained<NSSound>>;

        #[cfg(feature = "NSSound")]
        /// Setter for [`sound`][Self::sound].
        #[method(setSound:)]
        pub unsafe fn setSound(&self, sound: Option<&NSSound>);

        /// Sends action on deep-press or extended hover while dragging. Defaults to NO.
        #[method(isSpringLoaded)]
        pub unsafe fn isSpringLoaded(&self) -> bool;

        /// Setter for [`isSpringLoaded`][Self::isSpringLoaded].
        #[method(setSpringLoaded:)]
        pub unsafe fn setSpringLoaded(&self, spring_loaded: bool);

        /// Configures the maximum allowed level for an NSMultiLevelAcceleratorButton, allowed values range from [1,5]. Defaults to 2.
        #[method(maxAcceleratorLevel)]
        pub unsafe fn maxAcceleratorLevel(&self) -> NSInteger;

        /// Setter for [`maxAcceleratorLevel`][Self::maxAcceleratorLevel].
        #[method(setMaxAcceleratorLevel:)]
        pub unsafe fn setMaxAcceleratorLevel(&self, max_accelerator_level: NSInteger);

        /// Sets the initial delay and repeat interval, in seconds, for repeated action messages sent when `continuous` is YES.
        #[method(setPeriodicDelay:interval:)]
        pub unsafe fn setPeriodicDelay_interval(&self, delay: c_float, interval: c_float);

        /// Gets the initial delay and repeat interval, in seconds, for repeated action messages sent when `continuous` is YES. Both parameters to this method must not be NULL.
        #[method(getPeriodicDelay:interval:)]
        pub unsafe fn getPeriodicDelay_interval(
            &self,
            delay: NonNull<c_float>,
            interval: NonNull<c_float>,
        );

        #[cfg(feature = "NSButtonCell")]
        /// The bezel style of the button, which provides a set of bezel artwork, layout metrics, and content styling from a set of system-provided styles. See the NSBezelStyle enumeration for a list of available styles. The bezel style is not used if the `bordered` property is set to `NO`.
        #[method(bezelStyle)]
        pub unsafe fn bezelStyle(&self) -> NSBezelStyle;

        #[cfg(feature = "NSButtonCell")]
        /// Setter for [`bezelStyle`][Self::bezelStyle].
        #[method(setBezelStyle:)]
        pub unsafe fn setBezelStyle(&self, bezel_style: NSBezelStyle);

        /// A Boolean value that determines whether the button draws a border.
        #[method(isBordered)]
        pub unsafe fn isBordered(&self) -> bool;

        /// Setter for [`isBordered`][Self::isBordered].
        #[method(setBordered:)]
        pub unsafe fn setBordered(&self, bordered: bool);

        /// A Boolean value that indicates whether the button is transparent. A transparent button never draws itself, but it receives mouse events, sends its action, and tracks the mouse properly.
        #[method(isTransparent)]
        pub unsafe fn isTransparent(&self) -> bool;

        /// Setter for [`isTransparent`][Self::isTransparent].
        #[method(setTransparent:)]
        pub unsafe fn setTransparent(&self, transparent: bool);

        #[method(showsBorderOnlyWhileMouseInside)]
        pub unsafe fn showsBorderOnlyWhileMouseInside(&self) -> bool;

        /// Setter for [`showsBorderOnlyWhileMouseInside`][Self::showsBorderOnlyWhileMouseInside].
        #[method(setShowsBorderOnlyWhileMouseInside:)]
        pub unsafe fn setShowsBorderOnlyWhileMouseInside(
            &self,
            shows_border_only_while_mouse_inside: bool,
        );

        #[cfg(feature = "NSColor")]
        /// Applies a custom color to the button's bezel, in appearances that support it. A nil value indicates an unmodified button appearance. The default value is nil.
        #[method_id(@__retain_semantics Other bezelColor)]
        pub unsafe fn bezelColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColor")]
        /// Setter for [`bezelColor`][Self::bezelColor].
        #[method(setBezelColor:)]
        pub unsafe fn setBezelColor(&self, bezel_color: Option<&NSColor>);

        #[cfg(feature = "NSColor")]
        /// Applies a tint color to template image and text content, in combination with other theme-appropriate effects. Only applicable to borderless buttons. A nil value indicates the standard set of effects without color modification. The default value is nil. Non-template images and attributed string values are not affected by the contentTintColor.
        #[method_id(@__retain_semantics Other contentTintColor)]
        pub unsafe fn contentTintColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColor")]
        /// Setter for [`contentTintColor`][Self::contentTintColor].
        #[method(setContentTintColor:)]
        pub unsafe fn setContentTintColor(&self, content_tint_color: Option<&NSColor>);

        #[cfg(feature = "NSImage")]
        /// The image that appears on the button when it’s in an off state, or nil if there is no such image.
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        /// Setter for [`image`][Self::image].
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[cfg(feature = "NSImage")]
        /// An alternate image that appears on the button when the button is in an on state, or nil if there is no such image. Note that some button types do not display an alternate image.
        #[method_id(@__retain_semantics Other alternateImage)]
        pub unsafe fn alternateImage(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        /// Setter for [`alternateImage`][Self::alternateImage].
        #[method(setAlternateImage:)]
        pub unsafe fn setAlternateImage(&self, alternate_image: Option<&NSImage>);

        #[cfg(feature = "NSCell")]
        /// The position of the button's image relative to its title. See the NSCellImagePosition enumeration for possible values.
        #[method(imagePosition)]
        pub unsafe fn imagePosition(&self) -> NSCellImagePosition;

        #[cfg(feature = "NSCell")]
        /// Setter for [`imagePosition`][Self::imagePosition].
        #[method(setImagePosition:)]
        pub unsafe fn setImagePosition(&self, image_position: NSCellImagePosition);

        #[cfg(feature = "NSCell")]
        /// The scaling mode applied to make the button's image fit within its bounds.
        #[method(imageScaling)]
        pub unsafe fn imageScaling(&self) -> NSImageScaling;

        #[cfg(feature = "NSCell")]
        /// Setter for [`imageScaling`][Self::imageScaling].
        #[method(setImageScaling:)]
        pub unsafe fn setImageScaling(&self, image_scaling: NSImageScaling);

        /// A Boolean value that determines how the button's image and title are positioned together within the button bezel. If false, the image is positioned according to the imagePosition property at the edge of the button bezel, and the title is positioned within the remaining space. If true, the button’s image is positioned directly adjacent to the title based on the imagePosition property, and the image and title are positioned within the button bezel as a single unit.
        #[method(imageHugsTitle)]
        pub unsafe fn imageHugsTitle(&self) -> bool;

        /// Setter for [`imageHugsTitle`][Self::imageHugsTitle].
        #[method(setImageHugsTitle:)]
        pub unsafe fn setImageHugsTitle(&self, image_hugs_title: bool);

        #[cfg(feature = "NSImage")]
        /// Specifies a combination of point size, weight, and scale to use when sizing and displaying symbol images. If a symbol configuration isn't provided, the symbol is matched to the button's `font` property. The default value is nil.
        #[method_id(@__retain_semantics Other symbolConfiguration)]
        pub unsafe fn symbolConfiguration(&self) -> Option<Retained<NSImageSymbolConfiguration>>;

        #[cfg(feature = "NSImage")]
        /// Setter for [`symbolConfiguration`][Self::symbolConfiguration].
        #[method(setSymbolConfiguration:)]
        pub unsafe fn setSymbolConfiguration(
            &self,
            symbol_configuration: Option<&NSImageSymbolConfiguration>,
        );

        #[cfg(feature = "NSCell")]
        /// The button's state. Buttons support the off and on states, and an additional mixed state depending on the value of the `allowsMixedState` property.
        #[method(state)]
        pub unsafe fn state(&self) -> NSControlStateValue;

        #[cfg(feature = "NSCell")]
        /// Setter for [`state`][Self::state].
        #[method(setState:)]
        pub unsafe fn setState(&self, state: NSControlStateValue);

        /// A Boolean value that indicates whether the button allows a mixed state. If NO, the button has two states (on and off), and if YES, the button has three states (on, off, and mixed). The mixed state is commonly used with checkboxes and radio buttons to indicate a value which is partially on.
        #[method(allowsMixedState)]
        pub unsafe fn allowsMixedState(&self) -> bool;

        /// Setter for [`allowsMixedState`][Self::allowsMixedState].
        #[method(setAllowsMixedState:)]
        pub unsafe fn setAllowsMixedState(&self, allows_mixed_state: bool);

        /// Sets the button to its next eligible state. If the button allows mixed state, this cycles through the states in the order: on, off, mixed, on, etc. If the button does not allow mixed state, it toggles between off and on.
        #[method(setNextState)]
        pub unsafe fn setNextState(&self);

        /// Highlights, or un-highlights, the button. Highlighting makes the button appear "pressed", which may include showing an illuminated bezel, or showing the alternate image or title, depending on the type of button.
        #[method(highlight:)]
        pub unsafe fn highlight(&self, flag: bool);

        /// This property contains the button's key equivalent, or the empty string if no equivalent has been defined. Buttons don’t have a default key equivalent. Setting the key equivalent to the Return character causes it to act as the default button for its window.
        #[method_id(@__retain_semantics Other keyEquivalent)]
        pub unsafe fn keyEquivalent(&self) -> Retained<NSString>;

        /// Setter for [`keyEquivalent`][Self::keyEquivalent].
        #[method(setKeyEquivalent:)]
        pub unsafe fn setKeyEquivalent(&self, key_equivalent: &NSString);

        #[cfg(feature = "NSEvent")]
        /// A bitmask specifying the modifier keys that are applied to the button's key equivalent. Mask bits are defined by the NSEventModifierFlags option set. The only mask bits relevant in button key-equivalent modifier masks are NSEventModifierFlagControl, NSEventModifierFlagOption, and NSEventModifierFlagCommand.
        #[method(keyEquivalentModifierMask)]
        pub unsafe fn keyEquivalentModifierMask(&self) -> NSEventModifierFlags;

        #[cfg(feature = "NSEvent")]
        /// Setter for [`keyEquivalentModifierMask`][Self::keyEquivalentModifierMask].
        #[method(setKeyEquivalentModifierMask:)]
        pub unsafe fn setKeyEquivalentModifierMask(
            &self,
            key_equivalent_modifier_mask: NSEventModifierFlags,
        );

        #[cfg(feature = "NSEvent")]
        /// If the event parameter matches the button's key equivalent, the button briefly highlights and performs its action, and then returns YES. Otherwise, returns NO.
        #[method(performKeyEquivalent:)]
        pub unsafe fn performKeyEquivalent(&self, key: &NSEvent) -> bool;

        #[cfg(feature = "NSUserInterfaceCompression")]
        #[method(compressWithPrioritizedCompressionOptions:)]
        pub unsafe fn compressWithPrioritizedCompressionOptions(
            &self,
            prioritized_options: &NSArray<NSUserInterfaceCompressionOptions>,
        );

        #[cfg(all(
            feature = "NSUserInterfaceCompression",
            feature = "objc2-core-foundation"
        ))]
        #[method(minimumSizeWithPrioritizedCompressionOptions:)]
        pub unsafe fn minimumSizeWithPrioritizedCompressionOptions(
            &self,
            prioritized_options: &NSArray<NSUserInterfaceCompressionOptions>,
        ) -> NSSize;

        #[cfg(feature = "NSUserInterfaceCompression")]
        #[method_id(@__retain_semantics Other activeCompressionOptions)]
        pub unsafe fn activeCompressionOptions(
            &self,
        ) -> Retained<NSUserInterfaceCompressionOptions>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSButton {
        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSButton {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSButton {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// NSButtonDeprecated
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSButton {
        #[deprecated = "Mnemonics are not used on macOS. Set the title property directly instead."]
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, string_with_ampersand: Option<&NSString>);
    }
);
