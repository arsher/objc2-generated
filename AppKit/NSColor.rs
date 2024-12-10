//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
#[cfg(target_vendor = "apple")]
use objc2_core_graphics::*;
#[cfg(feature = "objc2-core-image")]
#[cfg(target_vendor = "apple")]
use objc2_core_image::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsappkitversionnumberwithpatterncolorleakfix?language=objc)
#[cfg(feature = "NSApplication")]
pub static NSAppKitVersionNumberWithPatternColorLeakFix: NSAppKitVersion = 641.0 as _;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscolortype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSColorType(pub NSInteger);
impl NSColorType {
    #[doc(alias = "NSColorTypeComponentBased")]
    pub const ComponentBased: Self = Self(0);
    #[doc(alias = "NSColorTypePattern")]
    pub const Pattern: Self = Self(1);
    #[doc(alias = "NSColorTypeCatalog")]
    pub const Catalog: Self = Self(2);
}

unsafe impl Encode for NSColorType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSColorType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscolorsystemeffect?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSColorSystemEffect(pub NSInteger);
impl NSColorSystemEffect {
    #[doc(alias = "NSColorSystemEffectNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "NSColorSystemEffectPressed")]
    pub const Pressed: Self = Self(1);
    #[doc(alias = "NSColorSystemEffectDeepPressed")]
    pub const DeepPressed: Self = Self(2);
    #[doc(alias = "NSColorSystemEffectDisabled")]
    pub const Disabled: Self = Self(3);
    #[doc(alias = "NSColorSystemEffectRollover")]
    pub const Rollover: Self = Self(4);
}

unsafe impl Encode for NSColorSystemEffect {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSColorSystemEffect {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscolor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSColor;
);

unsafe impl Send for NSColor {}

unsafe impl Sync for NSColor {}

unsafe impl NSCoding for NSColor {}

unsafe impl NSCopying for NSColor {}

unsafe impl CopyingHelper for NSColor {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSColor {}

#[cfg(feature = "NSPasteboard")]
unsafe impl NSPasteboardReading for NSColor {}

#[cfg(feature = "NSPasteboard")]
unsafe impl NSPasteboardWriting for NSColor {}

unsafe impl NSSecureCoding for NSColor {}

extern_methods!(
    unsafe impl NSColor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "NSColorSpace", feature = "objc2-core-foundation"))]
        #[method_id(@__retain_semantics Other colorWithColorSpace:components:count:)]
        pub unsafe fn colorWithColorSpace_components_count(
            space: &NSColorSpace,
            components: NonNull<CGFloat>,
            number_of_components: NSInteger,
        ) -> Retained<NSColor>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other colorWithSRGBRed:green:blue:alpha:)]
        pub unsafe fn colorWithSRGBRed_green_blue_alpha(
            red: CGFloat,
            green: CGFloat,
            blue: CGFloat,
            alpha: CGFloat,
        ) -> Retained<NSColor>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other colorWithGenericGamma22White:alpha:)]
        pub unsafe fn colorWithGenericGamma22White_alpha(
            white: CGFloat,
            alpha: CGFloat,
        ) -> Retained<NSColor>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other colorWithDisplayP3Red:green:blue:alpha:)]
        pub unsafe fn colorWithDisplayP3Red_green_blue_alpha(
            red: CGFloat,
            green: CGFloat,
            blue: CGFloat,
            alpha: CGFloat,
        ) -> Retained<NSColor>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other colorWithWhite:alpha:)]
        pub unsafe fn colorWithWhite_alpha(white: CGFloat, alpha: CGFloat) -> Retained<NSColor>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other colorWithRed:green:blue:alpha:)]
        pub unsafe fn colorWithRed_green_blue_alpha(
            red: CGFloat,
            green: CGFloat,
            blue: CGFloat,
            alpha: CGFloat,
        ) -> Retained<NSColor>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other colorWithHue:saturation:brightness:alpha:)]
        pub unsafe fn colorWithHue_saturation_brightness_alpha(
            hue: CGFloat,
            saturation: CGFloat,
            brightness: CGFloat,
            alpha: CGFloat,
        ) -> Retained<NSColor>;

        #[cfg(all(feature = "NSColorSpace", feature = "objc2-core-foundation"))]
        #[method_id(@__retain_semantics Other colorWithColorSpace:hue:saturation:brightness:alpha:)]
        pub unsafe fn colorWithColorSpace_hue_saturation_brightness_alpha(
            space: &NSColorSpace,
            hue: CGFloat,
            saturation: CGFloat,
            brightness: CGFloat,
            alpha: CGFloat,
        ) -> Retained<NSColor>;

        #[cfg(feature = "NSColorList")]
        #[method_id(@__retain_semantics Other colorWithCatalogName:colorName:)]
        pub unsafe fn colorWithCatalogName_colorName(
            list_name: &NSColorListName,
            color_name: &NSColorName,
        ) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColorList")]
        #[method_id(@__retain_semantics Other colorNamed:bundle:)]
        pub unsafe fn colorNamed_bundle(
            name: &NSColorName,
            bundle: Option<&NSBundle>,
        ) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColorList")]
        #[method_id(@__retain_semantics Other colorNamed:)]
        pub unsafe fn colorNamed(name: &NSColorName) -> Option<Retained<NSColor>>;

        #[cfg(all(feature = "NSAppearance", feature = "NSColorList", feature = "block2"))]
        #[method_id(@__retain_semantics Other colorWithName:dynamicProvider:)]
        pub unsafe fn colorWithName_dynamicProvider(
            color_name: Option<&NSColorName>,
            dynamic_provider: &block2::Block<dyn Fn(NonNull<NSAppearance>) -> NonNull<NSColor>>,
        ) -> Retained<NSColor>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other colorWithDeviceWhite:alpha:)]
        pub unsafe fn colorWithDeviceWhite_alpha(
            white: CGFloat,
            alpha: CGFloat,
        ) -> Retained<NSColor>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other colorWithDeviceRed:green:blue:alpha:)]
        pub unsafe fn colorWithDeviceRed_green_blue_alpha(
            red: CGFloat,
            green: CGFloat,
            blue: CGFloat,
            alpha: CGFloat,
        ) -> Retained<NSColor>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other colorWithDeviceHue:saturation:brightness:alpha:)]
        pub unsafe fn colorWithDeviceHue_saturation_brightness_alpha(
            hue: CGFloat,
            saturation: CGFloat,
            brightness: CGFloat,
            alpha: CGFloat,
        ) -> Retained<NSColor>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other colorWithDeviceCyan:magenta:yellow:black:alpha:)]
        pub unsafe fn colorWithDeviceCyan_magenta_yellow_black_alpha(
            cyan: CGFloat,
            magenta: CGFloat,
            yellow: CGFloat,
            black: CGFloat,
            alpha: CGFloat,
        ) -> Retained<NSColor>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other colorWithCalibratedWhite:alpha:)]
        pub unsafe fn colorWithCalibratedWhite_alpha(
            white: CGFloat,
            alpha: CGFloat,
        ) -> Retained<NSColor>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other colorWithCalibratedRed:green:blue:alpha:)]
        pub unsafe fn colorWithCalibratedRed_green_blue_alpha(
            red: CGFloat,
            green: CGFloat,
            blue: CGFloat,
            alpha: CGFloat,
        ) -> Retained<NSColor>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other colorWithCalibratedHue:saturation:brightness:alpha:)]
        pub unsafe fn colorWithCalibratedHue_saturation_brightness_alpha(
            hue: CGFloat,
            saturation: CGFloat,
            brightness: CGFloat,
            alpha: CGFloat,
        ) -> Retained<NSColor>;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other colorWithPatternImage:)]
        pub unsafe fn colorWithPatternImage(image: &NSImage) -> Retained<NSColor>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> NSColorType;

        #[method_id(@__retain_semantics Other colorUsingType:)]
        pub unsafe fn colorUsingType(&self, r#type: NSColorType) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColorSpace")]
        #[method_id(@__retain_semantics Other colorUsingColorSpace:)]
        pub unsafe fn colorUsingColorSpace(
            &self,
            space: &NSColorSpace,
        ) -> Option<Retained<NSColor>>;

        #[method_id(@__retain_semantics Other blackColor)]
        pub unsafe fn blackColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other darkGrayColor)]
        pub unsafe fn darkGrayColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other lightGrayColor)]
        pub unsafe fn lightGrayColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other whiteColor)]
        pub unsafe fn whiteColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other grayColor)]
        pub unsafe fn grayColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other redColor)]
        pub unsafe fn redColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other greenColor)]
        pub unsafe fn greenColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other blueColor)]
        pub unsafe fn blueColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other cyanColor)]
        pub unsafe fn cyanColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other yellowColor)]
        pub unsafe fn yellowColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other magentaColor)]
        pub unsafe fn magentaColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other orangeColor)]
        pub unsafe fn orangeColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other purpleColor)]
        pub unsafe fn purpleColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other brownColor)]
        pub unsafe fn brownColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other clearColor)]
        pub unsafe fn clearColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other labelColor)]
        pub unsafe fn labelColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other secondaryLabelColor)]
        pub unsafe fn secondaryLabelColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other tertiaryLabelColor)]
        pub unsafe fn tertiaryLabelColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other quaternaryLabelColor)]
        pub unsafe fn quaternaryLabelColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other quinaryLabelColor)]
        pub unsafe fn quinaryLabelColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other linkColor)]
        pub unsafe fn linkColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other placeholderTextColor)]
        pub unsafe fn placeholderTextColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other windowFrameTextColor)]
        pub unsafe fn windowFrameTextColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other selectedMenuItemTextColor)]
        pub unsafe fn selectedMenuItemTextColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other alternateSelectedControlTextColor)]
        pub unsafe fn alternateSelectedControlTextColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other headerTextColor)]
        pub unsafe fn headerTextColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other separatorColor)]
        pub unsafe fn separatorColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other gridColor)]
        pub unsafe fn gridColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other windowBackgroundColor)]
        pub unsafe fn windowBackgroundColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other underPageBackgroundColor)]
        pub unsafe fn underPageBackgroundColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other controlBackgroundColor)]
        pub unsafe fn controlBackgroundColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other selectedContentBackgroundColor)]
        pub unsafe fn selectedContentBackgroundColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other unemphasizedSelectedContentBackgroundColor)]
        pub unsafe fn unemphasizedSelectedContentBackgroundColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other alternatingContentBackgroundColors)]
        pub unsafe fn alternatingContentBackgroundColors() -> Retained<NSArray<NSColor>>;

        #[method_id(@__retain_semantics Other findHighlightColor)]
        pub unsafe fn findHighlightColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other textColor)]
        pub unsafe fn textColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other textBackgroundColor)]
        pub unsafe fn textBackgroundColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other textInsertionPointColor)]
        pub unsafe fn textInsertionPointColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other selectedTextColor)]
        pub unsafe fn selectedTextColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other selectedTextBackgroundColor)]
        pub unsafe fn selectedTextBackgroundColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other unemphasizedSelectedTextBackgroundColor)]
        pub unsafe fn unemphasizedSelectedTextBackgroundColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other unemphasizedSelectedTextColor)]
        pub unsafe fn unemphasizedSelectedTextColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other controlColor)]
        pub unsafe fn controlColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other controlTextColor)]
        pub unsafe fn controlTextColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other selectedControlColor)]
        pub unsafe fn selectedControlColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other selectedControlTextColor)]
        pub unsafe fn selectedControlTextColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other disabledControlTextColor)]
        pub unsafe fn disabledControlTextColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other keyboardFocusIndicatorColor)]
        pub unsafe fn keyboardFocusIndicatorColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other scrubberTexturedBackgroundColor)]
        pub unsafe fn scrubberTexturedBackgroundColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other systemRedColor)]
        pub unsafe fn systemRedColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other systemGreenColor)]
        pub unsafe fn systemGreenColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other systemBlueColor)]
        pub unsafe fn systemBlueColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other systemOrangeColor)]
        pub unsafe fn systemOrangeColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other systemYellowColor)]
        pub unsafe fn systemYellowColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other systemBrownColor)]
        pub unsafe fn systemBrownColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other systemPinkColor)]
        pub unsafe fn systemPinkColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other systemPurpleColor)]
        pub unsafe fn systemPurpleColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other systemGrayColor)]
        pub unsafe fn systemGrayColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other systemTealColor)]
        pub unsafe fn systemTealColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other systemIndigoColor)]
        pub unsafe fn systemIndigoColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other systemMintColor)]
        pub unsafe fn systemMintColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other systemCyanColor)]
        pub unsafe fn systemCyanColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other systemFillColor)]
        pub unsafe fn systemFillColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other secondarySystemFillColor)]
        pub unsafe fn secondarySystemFillColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other tertiarySystemFillColor)]
        pub unsafe fn tertiarySystemFillColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other quaternarySystemFillColor)]
        pub unsafe fn quaternarySystemFillColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other quinarySystemFillColor)]
        pub unsafe fn quinarySystemFillColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other controlAccentColor)]
        pub unsafe fn controlAccentColor() -> Retained<NSColor>;

        #[cfg(feature = "NSCell")]
        #[method(currentControlTint)]
        pub unsafe fn currentControlTint() -> NSControlTint;

        #[cfg(feature = "NSCell")]
        #[deprecated = "NSControlTint does not describe the full range of available control accent colors. Use +[NSColor controlAccentColor] instead."]
        #[method_id(@__retain_semantics Other colorForControlTint:)]
        pub unsafe fn colorForControlTint(control_tint: NSControlTint) -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other highlightColor)]
        pub unsafe fn highlightColor() -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other shadowColor)]
        pub unsafe fn shadowColor() -> Retained<NSColor>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other highlightWithLevel:)]
        pub unsafe fn highlightWithLevel(&self, val: CGFloat) -> Option<Retained<NSColor>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other shadowWithLevel:)]
        pub unsafe fn shadowWithLevel(&self, val: CGFloat) -> Option<Retained<NSColor>>;

        #[method_id(@__retain_semantics Other colorWithSystemEffect:)]
        pub unsafe fn colorWithSystemEffect(
            &self,
            system_effect: NSColorSystemEffect,
        ) -> Retained<NSColor>;

        #[method(set)]
        pub unsafe fn set(&self);

        #[method(setFill)]
        pub unsafe fn setFill(&self);

        #[method(setStroke)]
        pub unsafe fn setStroke(&self);

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other blendedColorWithFraction:ofColor:)]
        pub unsafe fn blendedColorWithFraction_ofColor(
            &self,
            fraction: CGFloat,
            color: &NSColor,
        ) -> Option<Retained<NSColor>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other colorWithAlphaComponent:)]
        pub unsafe fn colorWithAlphaComponent(&self, alpha: CGFloat) -> Retained<NSColor>;

        #[cfg(feature = "NSColorList")]
        #[method_id(@__retain_semantics Other catalogNameComponent)]
        pub unsafe fn catalogNameComponent(&self) -> Retained<NSColorListName>;

        #[cfg(feature = "NSColorList")]
        #[method_id(@__retain_semantics Other colorNameComponent)]
        pub unsafe fn colorNameComponent(&self) -> Retained<NSColorName>;

        #[method_id(@__retain_semantics Other localizedCatalogNameComponent)]
        pub unsafe fn localizedCatalogNameComponent(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other localizedColorNameComponent)]
        pub unsafe fn localizedColorNameComponent(&self) -> Retained<NSString>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(redComponent)]
        pub unsafe fn redComponent(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(greenComponent)]
        pub unsafe fn greenComponent(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(blueComponent)]
        pub unsafe fn blueComponent(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(getRed:green:blue:alpha:)]
        pub unsafe fn getRed_green_blue_alpha(
            &self,
            red: *mut CGFloat,
            green: *mut CGFloat,
            blue: *mut CGFloat,
            alpha: *mut CGFloat,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(hueComponent)]
        pub unsafe fn hueComponent(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(saturationComponent)]
        pub unsafe fn saturationComponent(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(brightnessComponent)]
        pub unsafe fn brightnessComponent(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(getHue:saturation:brightness:alpha:)]
        pub unsafe fn getHue_saturation_brightness_alpha(
            &self,
            hue: *mut CGFloat,
            saturation: *mut CGFloat,
            brightness: *mut CGFloat,
            alpha: *mut CGFloat,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(whiteComponent)]
        pub unsafe fn whiteComponent(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(getWhite:alpha:)]
        pub unsafe fn getWhite_alpha(&self, white: *mut CGFloat, alpha: *mut CGFloat);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(cyanComponent)]
        pub unsafe fn cyanComponent(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(magentaComponent)]
        pub unsafe fn magentaComponent(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(yellowComponent)]
        pub unsafe fn yellowComponent(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(blackComponent)]
        pub unsafe fn blackComponent(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(getCyan:magenta:yellow:black:alpha:)]
        pub unsafe fn getCyan_magenta_yellow_black_alpha(
            &self,
            cyan: *mut CGFloat,
            magenta: *mut CGFloat,
            yellow: *mut CGFloat,
            black: *mut CGFloat,
            alpha: *mut CGFloat,
        );

        #[cfg(feature = "NSColorSpace")]
        #[method_id(@__retain_semantics Other colorSpace)]
        pub unsafe fn colorSpace(&self) -> Retained<NSColorSpace>;

        #[method(numberOfComponents)]
        pub unsafe fn numberOfComponents(&self) -> NSInteger;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(getComponents:)]
        pub unsafe fn getComponents(&self, components: NonNull<CGFloat>);

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other patternImage)]
        pub unsafe fn patternImage(&self) -> Retained<NSImage>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(alphaComponent)]
        pub unsafe fn alphaComponent(&self) -> CGFloat;

        #[cfg(feature = "NSPasteboard")]
        #[method_id(@__retain_semantics Other colorFromPasteboard:)]
        pub unsafe fn colorFromPasteboard(paste_board: &NSPasteboard) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSPasteboard")]
        #[method(writeToPasteboard:)]
        pub unsafe fn writeToPasteboard(&self, paste_board: &NSPasteboard);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(drawSwatchInRect:)]
        pub unsafe fn drawSwatchInRect(&self, rect: NSRect);

        #[cfg(feature = "objc2-core-graphics")]
        #[cfg(target_vendor = "apple")]
        #[method_id(@__retain_semantics Other colorWithCGColor:)]
        pub unsafe fn colorWithCGColor(cg_color: CGColorRef) -> Option<Retained<NSColor>>;

        #[cfg(feature = "objc2-core-graphics")]
        #[cfg(target_vendor = "apple")]
        #[method(CGColor)]
        pub unsafe fn CGColor(&self) -> CGColorRef;

        #[deprecated = "Use `showsAlpha` in `NSColorPanel` and `supportsAlpha` in `NSColorWell` to control alpha behavior for individual controls."]
        #[method(ignoresAlpha)]
        pub unsafe fn ignoresAlpha(mtm: MainThreadMarker) -> bool;

        #[deprecated = "Use `showsAlpha` in `NSColorPanel` and `supportsAlpha` in `NSColorWell` to control alpha behavior for individual controls."]
        #[method(setIgnoresAlpha:)]
        pub unsafe fn setIgnoresAlpha(ignores_alpha: bool, mtm: MainThreadMarker);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSColor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSColor {
        #[deprecated = "Use a color that matches the semantics being used, such as `separatorColor`"]
        #[method_id(@__retain_semantics Other controlHighlightColor)]
        pub unsafe fn controlHighlightColor() -> Retained<NSColor>;

        #[deprecated = "Use a color that matches the semantics being used, such as `separatorColor`"]
        #[method_id(@__retain_semantics Other controlLightHighlightColor)]
        pub unsafe fn controlLightHighlightColor() -> Retained<NSColor>;

        #[deprecated = "Use a color that matches the semantics being used, such as `separatorColor`"]
        #[method_id(@__retain_semantics Other controlShadowColor)]
        pub unsafe fn controlShadowColor() -> Retained<NSColor>;

        #[deprecated = "Use a color that matches the semantics being used, such as `separatorColor`"]
        #[method_id(@__retain_semantics Other controlDarkShadowColor)]
        pub unsafe fn controlDarkShadowColor() -> Retained<NSColor>;

        #[deprecated = "Use NSScroller instead"]
        #[method_id(@__retain_semantics Other scrollBarColor)]
        pub unsafe fn scrollBarColor() -> Retained<NSColor>;

        #[deprecated = "Use NSScroller instead"]
        #[method_id(@__retain_semantics Other knobColor)]
        pub unsafe fn knobColor() -> Retained<NSColor>;

        #[deprecated = "Use NSScroller instead"]
        #[method_id(@__retain_semantics Other selectedKnobColor)]
        pub unsafe fn selectedKnobColor() -> Retained<NSColor>;

        #[deprecated = "Use NSVisualEffectMaterialTitlebar"]
        #[method_id(@__retain_semantics Other windowFrameColor)]
        pub unsafe fn windowFrameColor() -> Retained<NSColor>;

        #[deprecated = "Use NSVisualEffectMaterialSelection"]
        #[method_id(@__retain_semantics Other selectedMenuItemColor)]
        pub unsafe fn selectedMenuItemColor() -> Retained<NSColor>;

        #[deprecated = "Use NSVisualEffectMaterialHeaderView"]
        #[method_id(@__retain_semantics Other headerColor)]
        pub unsafe fn headerColor() -> Retained<NSColor>;

        #[deprecated]
        #[method_id(@__retain_semantics Other secondarySelectedControlColor)]
        pub unsafe fn secondarySelectedControlColor() -> Retained<NSColor>;

        #[deprecated]
        #[method_id(@__retain_semantics Other alternateSelectedControlColor)]
        pub unsafe fn alternateSelectedControlColor() -> Retained<NSColor>;

        #[deprecated]
        #[method_id(@__retain_semantics Other controlAlternatingRowBackgroundColors)]
        pub unsafe fn controlAlternatingRowBackgroundColors() -> Retained<NSArray<NSColor>>;

        #[cfg(feature = "NSGraphics")]
        #[deprecated = "Use -type and NSColorType instead"]
        #[method_id(@__retain_semantics Other colorSpaceName)]
        pub unsafe fn colorSpaceName(&self) -> Retained<NSColorSpaceName>;

        #[cfg(feature = "NSGraphics")]
        #[deprecated = "Use -colorUsingType: or -colorUsingColorSpace: instead"]
        #[method_id(@__retain_semantics Other colorUsingColorSpaceName:device:)]
        pub unsafe fn colorUsingColorSpaceName_device(
            &self,
            name: Option<&NSColorSpaceName>,
            device_description: Option<&NSDictionary<NSDeviceDescriptionKey, AnyObject>>,
        ) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSGraphics")]
        #[deprecated = "Use -colorUsingType: or -colorUsingColorSpace: instead"]
        #[method_id(@__retain_semantics Other colorUsingColorSpaceName:)]
        pub unsafe fn colorUsingColorSpaceName(
            &self,
            name: &NSColorSpaceName,
        ) -> Option<Retained<NSColor>>;
    }
);

extern_methods!(
    /// NSQuartzCoreAdditions
    unsafe impl NSColor {
        #[cfg(feature = "objc2-core-image")]
        #[cfg(target_vendor = "apple")]
        #[method_id(@__retain_semantics Other colorWithCIColor:)]
        pub unsafe fn colorWithCIColor(color: &CIColor) -> Retained<NSColor>;
    }
);

extern_category!(
    /// Category "NSAppKitAdditions" on [`CIColor`].
    #[doc(alias = "NSAppKitAdditions")]
    pub unsafe trait CIColorNSAppKitAdditions {
        #[method_id(@__retain_semantics Init initWithColor:)]
        unsafe fn initWithColor(this: Allocated<Self>, color: &NSColor) -> Option<Retained<Self>>;
    }

    #[cfg(feature = "objc2-core-image")]
    #[cfg(target_vendor = "apple")]
    unsafe impl CIColorNSAppKitAdditions for CIColor {}
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nssystemcolorsdidchangenotification?language=objc)
    pub static NSSystemColorsDidChangeNotification: &'static NSNotificationName;
}
