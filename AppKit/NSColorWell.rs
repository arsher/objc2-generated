//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscolorwellstyle?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSColorWellStyle(pub NSInteger);
impl NSColorWellStyle {
    #[doc(alias = "NSColorWellStyleDefault")]
    pub const Default: Self = Self(0);
    #[doc(alias = "NSColorWellStyleMinimal")]
    pub const Minimal: Self = Self(1);
    #[doc(alias = "NSColorWellStyleExpanded")]
    pub const Expanded: Self = Self(2);
}

unsafe impl Encode for NSColorWellStyle {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSColorWellStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscolorwell?language=objc)
    #[unsafe(super(NSControl, NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    pub struct NSColorWell;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSColorWell {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSColorWell {}

#[cfg(all(
    feature = "NSAnimation",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSColorWell {}

#[cfg(all(
    feature = "NSAppearance",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAppearanceCustomization for NSColorWell {}

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSColorWell {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSDragging",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSDraggingDestination for NSColorWell {}

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSColorWell {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSColorWell {}

extern_methods!(
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSColorWell {
        #[method_id(@__retain_semantics Other colorWellWithStyle:)]
        pub unsafe fn colorWellWithStyle(
            style: NSColorWellStyle,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method(deactivate)]
        pub unsafe fn deactivate(&self);

        #[method(activate:)]
        pub unsafe fn activate(&self, exclusive: bool);

        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(drawWellInside:)]
        pub unsafe fn drawWellInside(&self, inside_rect: NSRect);

        #[deprecated = "This property will be deprecated in a future release."]
        #[method(isBordered)]
        pub unsafe fn isBordered(&self) -> bool;

        #[deprecated = "This property will be deprecated in a future release."]
        #[method(setBordered:)]
        pub unsafe fn setBordered(&self, bordered: bool);

        #[method(takeColorFrom:)]
        pub unsafe fn takeColorFrom(&self, sender: Option<&AnyObject>);

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Retained<NSColor>;

        #[cfg(feature = "NSColor")]
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: &NSColor);

        #[method(colorWellStyle)]
        pub unsafe fn colorWellStyle(&self) -> NSColorWellStyle;

        #[method(setColorWellStyle:)]
        pub unsafe fn setColorWellStyle(&self, color_well_style: NSColorWellStyle);

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[method_id(@__retain_semantics Other pulldownTarget)]
        pub unsafe fn pulldownTarget(&self) -> Option<Retained<AnyObject>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setPulldownTarget:)]
        pub unsafe fn setPulldownTarget(&self, pulldown_target: Option<&AnyObject>);

        #[method(pulldownAction)]
        pub unsafe fn pulldownAction(&self) -> Option<Sel>;

        #[method(setPulldownAction:)]
        pub unsafe fn setPulldownAction(&self, pulldown_action: Option<Sel>);

        #[method(supportsAlpha)]
        pub unsafe fn supportsAlpha(&self) -> bool;

        #[method(setSupportsAlpha:)]
        pub unsafe fn setSupportsAlpha(&self, supports_alpha: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSColorWell {
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
    unsafe impl NSColorWell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSColorWell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
