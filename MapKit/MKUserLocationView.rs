//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    pub struct MKUserLocationView;

    #[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl ClassType for MKUserLocationView {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = MKAnnotationView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSAccessibility for MKUserLocationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSAccessibilityElementProtocol for MKUserLocationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSAnimatablePropertyContainer for MKUserLocationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSAppearanceCustomization for MKUserLocationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for MKUserLocationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSDraggingDestination for MKUserLocationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for MKUserLocationView {}

#[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification for MKUserLocationView {}

extern_methods!(
    #[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl MKUserLocationView {}
);

extern_methods!(
    /// Methods declared on superclass `MKAnnotationView`
    #[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl MKUserLocationView {
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
    unsafe impl MKUserLocationView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl MKUserLocationView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "MKAnnotationView", feature = "objc2-app-kit"))]
    #[cfg(target_os = "macos")]
    unsafe impl MKUserLocationView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
