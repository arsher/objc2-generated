//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimagedynamicrange?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSImageDynamicRange(pub NSInteger);
impl NSImageDynamicRange {
    #[doc(alias = "NSImageDynamicRangeUnspecified")]
    pub const Unspecified: Self = Self(-1);
    #[doc(alias = "NSImageDynamicRangeStandard")]
    pub const Standard: Self = Self(0);
    #[doc(alias = "NSImageDynamicRangeConstrainedHigh")]
    pub const ConstrainedHigh: Self = Self(1);
    #[doc(alias = "NSImageDynamicRangeHigh")]
    pub const High: Self = Self(2);
}

unsafe impl Encode for NSImageDynamicRange {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSImageDynamicRange {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsimageview?language=objc)
    #[unsafe(super(NSControl, NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    pub struct NSImageView;
);

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibility for NSImageView {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityElementProtocol for NSImageView {}

#[cfg(all(
    feature = "NSAccessibilityProtocols",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAccessibilityImage for NSImageView {}

#[cfg(all(
    feature = "NSAnimation",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAnimatablePropertyContainer for NSImageView {}

#[cfg(all(
    feature = "NSAppearance",
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSAppearanceCustomization for NSImageView {}

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSCoding for NSImageView {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSDragging",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSDraggingDestination for NSImageView {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSMenu",
    feature = "NSResponder",
    feature = "NSView"
))]
unsafe impl NSMenuItemValidation for NSImageView {}

#[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
unsafe impl NSObjectProtocol for NSImageView {}

#[cfg(all(
    feature = "NSControl",
    feature = "NSResponder",
    feature = "NSUserInterfaceItemIdentification",
    feature = "NSView"
))]
unsafe impl NSUserInterfaceItemIdentification for NSImageView {}

extern_methods!(
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSImageView {
        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other imageViewWithImage:)]
        pub unsafe fn imageViewWithImage(image: &NSImage, mtm: MainThreadMarker) -> Retained<Self>;

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Retained<NSImage>>;

        #[cfg(feature = "NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[cfg(feature = "NSImageCell")]
        #[method(imageAlignment)]
        pub unsafe fn imageAlignment(&self) -> NSImageAlignment;

        #[cfg(feature = "NSImageCell")]
        #[method(setImageAlignment:)]
        pub unsafe fn setImageAlignment(&self, image_alignment: NSImageAlignment);

        #[cfg(feature = "NSCell")]
        #[method(imageScaling)]
        pub unsafe fn imageScaling(&self) -> NSImageScaling;

        #[cfg(feature = "NSCell")]
        #[method(setImageScaling:)]
        pub unsafe fn setImageScaling(&self, image_scaling: NSImageScaling);

        #[cfg(feature = "NSImageCell")]
        #[method(imageFrameStyle)]
        pub unsafe fn imageFrameStyle(&self) -> NSImageFrameStyle;

        #[cfg(feature = "NSImageCell")]
        #[method(setImageFrameStyle:)]
        pub unsafe fn setImageFrameStyle(&self, image_frame_style: NSImageFrameStyle);

        #[cfg(feature = "NSImage")]
        #[method_id(@__retain_semantics Other symbolConfiguration)]
        pub unsafe fn symbolConfiguration(&self) -> Option<Retained<NSImageSymbolConfiguration>>;

        #[cfg(feature = "NSImage")]
        #[method(setSymbolConfiguration:)]
        pub unsafe fn setSymbolConfiguration(
            &self,
            symbol_configuration: Option<&NSImageSymbolConfiguration>,
        );

        #[cfg(feature = "NSColor")]
        #[method_id(@__retain_semantics Other contentTintColor)]
        pub unsafe fn contentTintColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "NSColor")]
        #[method(setContentTintColor:)]
        pub unsafe fn setContentTintColor(&self, content_tint_color: Option<&NSColor>);

        #[method(animates)]
        pub unsafe fn animates(&self) -> bool;

        #[method(setAnimates:)]
        pub unsafe fn setAnimates(&self, animates: bool);

        #[method(allowsCutCopyPaste)]
        pub unsafe fn allowsCutCopyPaste(&self) -> bool;

        #[method(setAllowsCutCopyPaste:)]
        pub unsafe fn setAllowsCutCopyPaste(&self, allows_cut_copy_paste: bool);

        #[method(defaultPreferredImageDynamicRange)]
        pub unsafe fn defaultPreferredImageDynamicRange(
            mtm: MainThreadMarker,
        ) -> NSImageDynamicRange;

        #[method(setDefaultPreferredImageDynamicRange:)]
        pub unsafe fn setDefaultPreferredImageDynamicRange(
            default_preferred_image_dynamic_range: NSImageDynamicRange,
            mtm: MainThreadMarker,
        );

        #[method(preferredImageDynamicRange)]
        pub unsafe fn preferredImageDynamicRange(&self) -> NSImageDynamicRange;

        #[method(setPreferredImageDynamicRange:)]
        pub unsafe fn setPreferredImageDynamicRange(
            &self,
            preferred_image_dynamic_range: NSImageDynamicRange,
        );

        #[method(imageDynamicRange)]
        pub unsafe fn imageDynamicRange(&self) -> NSImageDynamicRange;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSImageView {
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
    unsafe impl NSImageView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "NSControl", feature = "NSResponder", feature = "NSView"))]
    unsafe impl NSImageView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
