//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkpinannotationcolor?language=objc)
// NS_ENUM
#[deprecated = "Use MKPinAnnotationView's pinTintColor instead"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MKPinAnnotationColor(pub NSUInteger);
impl MKPinAnnotationColor {
    #[deprecated = "Use MKPinAnnotationView's pinTintColor instead"]
    #[doc(alias = "MKPinAnnotationColorRed")]
    pub const Red: Self = Self(0);
    #[deprecated = "Use MKPinAnnotationView's pinTintColor instead"]
    #[doc(alias = "MKPinAnnotationColorGreen")]
    pub const Green: Self = Self(1);
    #[deprecated = "Use MKPinAnnotationView's pinTintColor instead"]
    #[doc(alias = "MKPinAnnotationColorPurple")]
    pub const Purple: Self = Self(2);
}

unsafe impl Encode for MKPinAnnotationColor {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MKPinAnnotationColor {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mapkit/mkpinannotationview?language=objc)
    #[unsafe(super(MKAnnotationView, NSView, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    #[deprecated]
    pub struct MKPinAnnotationView;
);

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSAccessibility for MKPinAnnotationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSAccessibilityElementProtocol for MKPinAnnotationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSAnimatablePropertyContainer for MKPinAnnotationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSAppearanceCustomization for MKPinAnnotationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for MKPinAnnotationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSDraggingDestination for MKPinAnnotationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for MKPinAnnotationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification for MKPinAnnotationView {}

extern_methods!(
    #[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl MKPinAnnotationView {
        #[method_id(@__retain_semantics Other redPinColor)]
        pub unsafe fn redPinColor(mtm: MainThreadMarker) -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other greenPinColor)]
        pub unsafe fn greenPinColor(mtm: MainThreadMarker) -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other purplePinColor)]
        pub unsafe fn purplePinColor(mtm: MainThreadMarker) -> Retained<NSColor>;

        #[method_id(@__retain_semantics Other pinTintColor)]
        pub unsafe fn pinTintColor(&self) -> Option<Retained<NSColor>>;

        #[method(setPinTintColor:)]
        pub unsafe fn setPinTintColor(&self, pin_tint_color: Option<&NSColor>);

        #[deprecated]
        #[method(animatesDrop)]
        pub unsafe fn animatesDrop(&self) -> bool;

        #[deprecated]
        #[method(setAnimatesDrop:)]
        pub unsafe fn setAnimatesDrop(&self, animates_drop: bool);

        #[deprecated = "Use pinTintColor instead"]
        #[method(pinColor)]
        pub unsafe fn pinColor(&self) -> MKPinAnnotationColor;

        #[deprecated = "Use pinTintColor instead"]
        #[method(setPinColor:)]
        pub unsafe fn setPinColor(&self, pin_color: MKPinAnnotationColor);
    }
);

extern_methods!(
    /// Methods declared on superclass `MKAnnotationView`
    #[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl MKPinAnnotationView {
        #[cfg(feature = "MKAnnotation")]
        #[method_id(@__retain_semantics Init initWithAnnotation:reuseIdentifier:)]
        pub unsafe fn initWithAnnotation_reuseIdentifier(
            this: Allocated<Self>,
            annotation: Option<&ProtocolObject<dyn MKAnnotation>>,
            reuse_identifier: Option<&NSString>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            a_decoder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl MKPinAnnotationView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl MKPinAnnotationView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl MKPinAnnotationView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
