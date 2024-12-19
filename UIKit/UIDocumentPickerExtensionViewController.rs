//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidocumentpickerextensionviewcontroller?language=objc)
    #[unsafe(super(UIViewController, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    #[deprecated = "Use enumeration based NSFileProviderExtension instead"]
    pub struct UIDocumentPickerExtensionViewController;
);

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSCoding for UIDocumentPickerExtensionViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSObjectProtocol for UIDocumentPickerExtensionViewController {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIAppearanceContainer for UIDocumentPickerExtensionViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIContentContainer for UIDocumentPickerExtensionViewController {}

#[cfg(all(
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIFocusEnvironment for UIDocumentPickerExtensionViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIResponderStandardEditActions for UIDocumentPickerExtensionViewController {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIViewController"
))]
unsafe impl UITraitEnvironment for UIDocumentPickerExtensionViewController {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIDocumentPickerExtensionViewController {
        #[deprecated = "Use enumeration based NSFileProviderExtension instead"]
        #[method(dismissGrantingAccessToURL:)]
        pub unsafe fn dismissGrantingAccessToURL(&self, url: Option<&NSURL>);

        #[cfg(feature = "UIDocumentPickerViewController")]
        #[deprecated = "Use enumeration based NSFileProviderExtension instead"]
        #[method(prepareForPresentationInMode:)]
        pub unsafe fn prepareForPresentationInMode(&self, mode: UIDocumentPickerMode);

        #[cfg(feature = "UIDocumentPickerViewController")]
        #[deprecated = "Use enumeration based NSFileProviderExtension instead"]
        #[method(documentPickerMode)]
        pub unsafe fn documentPickerMode(&self) -> UIDocumentPickerMode;

        #[deprecated = "Use enumeration based NSFileProviderExtension instead"]
        #[method_id(@__retain_semantics Other originalURL)]
        pub unsafe fn originalURL(&self) -> Option<Retained<NSURL>>;

        #[deprecated = "Use enumeration based NSFileProviderExtension instead"]
        #[method_id(@__retain_semantics Other validTypes)]
        pub unsafe fn validTypes(&self) -> Option<Retained<NSArray<NSString>>>;

        #[deprecated = "Use enumeration based NSFileProviderExtension instead"]
        #[method_id(@__retain_semantics Other providerIdentifier)]
        pub unsafe fn providerIdentifier(&self) -> Retained<NSString>;

        #[deprecated = "Use enumeration based NSFileProviderExtension instead"]
        #[method_id(@__retain_semantics Other documentStorageURL)]
        pub unsafe fn documentStorageURL(&self) -> Option<Retained<NSURL>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIViewController`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIDocumentPickerExtensionViewController {
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSString>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIDocumentPickerExtensionViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
