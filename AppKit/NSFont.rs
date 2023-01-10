//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit;
use crate::CoreData;
use crate::Foundation;

extern_static!(NSFontIdentityMatrix: NonNull<CGFloat>);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSFont;

    unsafe impl ClassType for NSFont {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSFont")]
    unsafe impl NSFont {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fontWithName:size:)]
        pub unsafe fn fontWithName_size(
            fontName: &Foundation::NSString,
            fontSize: CGFloat,
        ) -> Option<Id<AppKit::NSFont, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fontWithName:matrix:)]
        pub unsafe fn fontWithName_matrix(
            fontName: &Foundation::NSString,
            fontMatrix: NonNull<CGFloat>,
        ) -> Option<Id<AppKit::NSFont, Shared>>;

        #[cfg(feature = "AppKit_NSFontDescriptor")]
        #[method_id(@__retain_semantics Other fontWithDescriptor:size:)]
        pub unsafe fn fontWithDescriptor_size(
            fontDescriptor: &AppKit::NSFontDescriptor,
            fontSize: CGFloat,
        ) -> Option<Id<AppKit::NSFont, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSFontDescriptor",
            feature = "Foundation_NSAffineTransform"
        ))]
        #[method_id(@__retain_semantics Other fontWithDescriptor:textTransform:)]
        pub unsafe fn fontWithDescriptor_textTransform(
            fontDescriptor: &AppKit::NSFontDescriptor,
            textTransform: Option<&Foundation::NSAffineTransform>,
        ) -> Option<Id<AppKit::NSFont, Shared>>;

        #[method_id(@__retain_semantics Other userFontOfSize:)]
        pub unsafe fn userFontOfSize(fontSize: CGFloat) -> Option<Id<AppKit::NSFont, Shared>>;

        #[method_id(@__retain_semantics Other userFixedPitchFontOfSize:)]
        pub unsafe fn userFixedPitchFontOfSize(
            fontSize: CGFloat,
        ) -> Option<Id<AppKit::NSFont, Shared>>;

        #[method(setUserFont:)]
        pub unsafe fn setUserFont(font: Option<&AppKit::NSFont>);

        #[method(setUserFixedPitchFont:)]
        pub unsafe fn setUserFixedPitchFont(font: Option<&AppKit::NSFont>);

        #[method_id(@__retain_semantics Other systemFontOfSize:)]
        pub unsafe fn systemFontOfSize(fontSize: CGFloat) -> Id<AppKit::NSFont, Shared>;

        #[method_id(@__retain_semantics Other boldSystemFontOfSize:)]
        pub unsafe fn boldSystemFontOfSize(fontSize: CGFloat) -> Id<AppKit::NSFont, Shared>;

        #[method_id(@__retain_semantics Other labelFontOfSize:)]
        pub unsafe fn labelFontOfSize(fontSize: CGFloat) -> Id<AppKit::NSFont, Shared>;

        #[method_id(@__retain_semantics Other titleBarFontOfSize:)]
        pub unsafe fn titleBarFontOfSize(fontSize: CGFloat) -> Id<AppKit::NSFont, Shared>;

        #[method_id(@__retain_semantics Other menuFontOfSize:)]
        pub unsafe fn menuFontOfSize(fontSize: CGFloat) -> Id<AppKit::NSFont, Shared>;

        #[method_id(@__retain_semantics Other menuBarFontOfSize:)]
        pub unsafe fn menuBarFontOfSize(fontSize: CGFloat) -> Id<AppKit::NSFont, Shared>;

        #[method_id(@__retain_semantics Other messageFontOfSize:)]
        pub unsafe fn messageFontOfSize(fontSize: CGFloat) -> Id<AppKit::NSFont, Shared>;

        #[method_id(@__retain_semantics Other paletteFontOfSize:)]
        pub unsafe fn paletteFontOfSize(fontSize: CGFloat) -> Id<AppKit::NSFont, Shared>;

        #[method_id(@__retain_semantics Other toolTipsFontOfSize:)]
        pub unsafe fn toolTipsFontOfSize(fontSize: CGFloat) -> Id<AppKit::NSFont, Shared>;

        #[method_id(@__retain_semantics Other controlContentFontOfSize:)]
        pub unsafe fn controlContentFontOfSize(fontSize: CGFloat) -> Id<AppKit::NSFont, Shared>;

        #[method_id(@__retain_semantics Other systemFontOfSize:weight:)]
        pub unsafe fn systemFontOfSize_weight(
            fontSize: CGFloat,
            weight: NSFontWeight,
        ) -> Id<AppKit::NSFont, Shared>;

        #[method_id(@__retain_semantics Other monospacedDigitSystemFontOfSize:weight:)]
        pub unsafe fn monospacedDigitSystemFontOfSize_weight(
            fontSize: CGFloat,
            weight: NSFontWeight,
        ) -> Id<AppKit::NSFont, Shared>;

        #[method_id(@__retain_semantics Other monospacedSystemFontOfSize:weight:)]
        pub unsafe fn monospacedSystemFontOfSize_weight(
            fontSize: CGFloat,
            weight: NSFontWeight,
        ) -> Id<AppKit::NSFont, Shared>;

        #[method_id(@__retain_semantics Other fontWithSize:)]
        pub unsafe fn fontWithSize(&self, fontSize: CGFloat) -> Id<AppKit::NSFont, Shared>;

        #[method(systemFontSize)]
        pub unsafe fn systemFontSize() -> CGFloat;

        #[method(smallSystemFontSize)]
        pub unsafe fn smallSystemFontSize() -> CGFloat;

        #[method(labelFontSize)]
        pub unsafe fn labelFontSize() -> CGFloat;

        #[method(systemFontSizeForControlSize:)]
        pub unsafe fn systemFontSizeForControlSize(controlSize: NSControlSize) -> CGFloat;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fontName)]
        pub unsafe fn fontName(&self) -> Id<Foundation::NSString, Shared>;

        #[method(pointSize)]
        pub unsafe fn pointSize(&self) -> CGFloat;

        #[method(matrix)]
        pub unsafe fn matrix(&self) -> NonNull<CGFloat>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other familyName)]
        pub unsafe fn familyName(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Option<Id<Foundation::NSString, Shared>>;

        #[cfg(feature = "AppKit_NSFontDescriptor")]
        #[method_id(@__retain_semantics Other fontDescriptor)]
        pub unsafe fn fontDescriptor(&self) -> Id<AppKit::NSFontDescriptor, Shared>;

        #[cfg(feature = "Foundation_NSAffineTransform")]
        #[method_id(@__retain_semantics Other textTransform)]
        pub unsafe fn textTransform(&self) -> Id<Foundation::NSAffineTransform, Shared>;

        #[method(numberOfGlyphs)]
        pub unsafe fn numberOfGlyphs(&self) -> NSUInteger;

        #[method(mostCompatibleStringEncoding)]
        pub unsafe fn mostCompatibleStringEncoding(&self) -> NSStringEncoding;

        #[cfg(feature = "Foundation_NSCharacterSet")]
        #[method_id(@__retain_semantics Other coveredCharacterSet)]
        pub unsafe fn coveredCharacterSet(&self) -> Id<Foundation::NSCharacterSet, Shared>;

        #[method(boundingRectForFont)]
        pub unsafe fn boundingRectForFont(&self) -> NSRect;

        #[method(maximumAdvancement)]
        pub unsafe fn maximumAdvancement(&self) -> NSSize;

        #[method(ascender)]
        pub unsafe fn ascender(&self) -> CGFloat;

        #[method(descender)]
        pub unsafe fn descender(&self) -> CGFloat;

        #[method(leading)]
        pub unsafe fn leading(&self) -> CGFloat;

        #[method(underlinePosition)]
        pub unsafe fn underlinePosition(&self) -> CGFloat;

        #[method(underlineThickness)]
        pub unsafe fn underlineThickness(&self) -> CGFloat;

        #[method(italicAngle)]
        pub unsafe fn italicAngle(&self) -> CGFloat;

        #[method(capHeight)]
        pub unsafe fn capHeight(&self) -> CGFloat;

        #[method(xHeight)]
        pub unsafe fn xHeight(&self) -> CGFloat;

        #[method(isFixedPitch)]
        pub unsafe fn isFixedPitch(&self) -> bool;

        #[method(set)]
        pub unsafe fn set(&self);

        #[cfg(feature = "AppKit_NSGraphicsContext")]
        #[method(setInContext:)]
        pub unsafe fn setInContext(&self, graphicsContext: &AppKit::NSGraphicsContext);

        #[method_id(@__retain_semantics Other verticalFont)]
        pub unsafe fn verticalFont(&self) -> Id<AppKit::NSFont, Shared>;

        #[method(isVertical)]
        pub unsafe fn isVertical(&self) -> bool;
    }
);

extern_static!(NSAntialiasThresholdChangedNotification: &'static Foundation::NSNotificationName);

extern_static!(NSFontSetChangedNotification: &'static Foundation::NSNotificationName);

pub type NSGlyph = c_uint;

extern_enum!(
    #[underlying(c_uint)]
    pub enum {
        NSControlGlyph = 0x00FFFFFF,
        NSNullGlyph = 0x0,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSFontRenderingMode {
        NSFontDefaultRenderingMode = 0,
        NSFontAntialiasedRenderingMode = 1,
        NSFontIntegerAdvancementsRenderingMode = 2,
        NSFontAntialiasedIntegerAdvancementsRenderingMode = 3,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSMultibyteGlyphPacking {
        NSNativeShortGlyphPacking = 5,
    }
);

extern_fn!(
    pub unsafe fn NSConvertGlyphsToPackedGlyphs(
        glBuf: NonNull<NSGlyph>,
        count: NSInteger,
        packing: NSMultibyteGlyphPacking,
        packedGlyphs: NonNull<c_char>,
    ) -> NSInteger;
);

extern_methods!(
    /// NSFont_Deprecated
    #[cfg(feature = "AppKit_NSFont")]
    unsafe impl NSFont {
        #[cfg(feature = "Foundation_NSString")]
        #[method(glyphWithName:)]
        pub unsafe fn glyphWithName(&self, name: &Foundation::NSString) -> NSGlyph;

        #[method(boundingRectForGlyph:)]
        pub unsafe fn boundingRectForGlyph(&self, glyph: NSGlyph) -> NSRect;

        #[method(advancementForGlyph:)]
        pub unsafe fn advancementForGlyph(&self, glyph: NSGlyph) -> NSSize;

        #[method(getBoundingRects:forGlyphs:count:)]
        pub unsafe fn getBoundingRects_forGlyphs_count(
            &self,
            bounds: NSRectArray,
            glyphs: NonNull<NSGlyph>,
            glyphCount: NSUInteger,
        );

        #[method(getAdvancements:forGlyphs:count:)]
        pub unsafe fn getAdvancements_forGlyphs_count(
            &self,
            advancements: NSSizeArray,
            glyphs: NonNull<NSGlyph>,
            glyphCount: NSUInteger,
        );

        #[method(getAdvancements:forPackedGlyphs:length:)]
        pub unsafe fn getAdvancements_forPackedGlyphs_length(
            &self,
            advancements: NSSizeArray,
            packedGlyphs: NonNull<c_void>,
            length: NSUInteger,
        );

        #[method_id(@__retain_semantics Other printerFont)]
        pub unsafe fn printerFont(&self) -> Id<AppKit::NSFont, Shared>;

        #[method_id(@__retain_semantics Other screenFont)]
        pub unsafe fn screenFont(&self) -> Id<AppKit::NSFont, Shared>;

        #[method_id(@__retain_semantics Other screenFontWithRenderingMode:)]
        pub unsafe fn screenFontWithRenderingMode(
            &self,
            renderingMode: NSFontRenderingMode,
        ) -> Id<AppKit::NSFont, Shared>;

        #[method(renderingMode)]
        pub unsafe fn renderingMode(&self) -> NSFontRenderingMode;
    }
);

extern_methods!(
    /// NSFont_TextStyles
    #[cfg(feature = "AppKit_NSFont")]
    unsafe impl NSFont {
        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other preferredFontForTextStyle:options:)]
        pub unsafe fn preferredFontForTextStyle_options(
            style: &AppKit::NSFontTextStyle,
            options: &Foundation::NSDictionary<AppKit::NSFontTextStyleOptionKey, Object>,
        ) -> Id<AppKit::NSFont, Shared>;
    }
);
