//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidocumentbrowseractionavailability?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIDocumentBrowserActionAvailability(pub NSInteger);
bitflags::bitflags! {
    impl UIDocumentBrowserActionAvailability: NSInteger {
        #[doc(alias = "UIDocumentBrowserActionAvailabilityMenu")]
        const Menu = 1;
        #[doc(alias = "UIDocumentBrowserActionAvailabilityNavigationBar")]
        const NavigationBar = 1<<1;
    }
}

unsafe impl Encode for UIDocumentBrowserActionAvailability {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIDocumentBrowserActionAvailability {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidocumentbrowseraction?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIDocumentBrowserAction;
);

unsafe impl NSObjectProtocol for UIDocumentBrowserAction {}

extern_methods!(
    unsafe impl UIDocumentBrowserAction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Init initWithIdentifier:localizedTitle:availability:handler:)]
        pub unsafe fn initWithIdentifier_localizedTitle_availability_handler(
            this: Allocated<Self>,
            identifier: &NSString,
            localized_title: &NSString,
            availability: UIDocumentBrowserActionAvailability,
            handler: &block2::Block<dyn Fn(NonNull<NSArray<NSURL>>)>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other localizedTitle)]
        pub unsafe fn localizedTitle(&self) -> Retained<NSString>;

        #[method(availability)]
        pub unsafe fn availability(&self) -> UIDocumentBrowserActionAvailability;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&UIImage>);

        #[method_id(@__retain_semantics Other supportedContentTypes)]
        pub unsafe fn supportedContentTypes(&self) -> Retained<NSArray<NSString>>;

        #[method(setSupportedContentTypes:)]
        pub unsafe fn setSupportedContentTypes(&self, supported_content_types: &NSArray<NSString>);

        #[method(supportsMultipleItems)]
        pub unsafe fn supportsMultipleItems(&self) -> bool;

        #[method(setSupportsMultipleItems:)]
        pub unsafe fn setSupportsMultipleItems(&self, supports_multiple_items: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIDocumentBrowserAction {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
