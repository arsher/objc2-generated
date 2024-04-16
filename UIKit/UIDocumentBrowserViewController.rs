//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[cfg(feature = "block2")]
use block2::*;
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-uniform-type-identifiers")]
use objc2_uniform_type_identifiers::*;

use crate::*;

extern "C" {
    pub static UIDocumentBrowserErrorDomain: &'static NSErrorDomain;
}

// NS_ERROR_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIDocumentBrowserErrorCode(pub NSInteger);
impl UIDocumentBrowserErrorCode {
    pub const UIDocumentBrowserErrorGeneric: Self = Self(1);
    pub const UIDocumentBrowserErrorNoLocationAvailable: Self = Self(2);
}

unsafe impl Encode for UIDocumentBrowserErrorCode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIDocumentBrowserErrorCode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIDocumentBrowserImportMode(pub NSUInteger);
impl UIDocumentBrowserImportMode {
    #[doc(alias = "UIDocumentBrowserImportModeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "UIDocumentBrowserImportModeCopy")]
    pub const Copy: Self = Self(1);
    #[doc(alias = "UIDocumentBrowserImportModeMove")]
    pub const Move: Self = Self(2);
}

unsafe impl Encode for UIDocumentBrowserImportMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIDocumentBrowserImportMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIDocumentBrowserUserInterfaceStyle(pub NSUInteger);
impl UIDocumentBrowserUserInterfaceStyle {
    #[doc(alias = "UIDocumentBrowserUserInterfaceStyleWhite")]
    pub const White: Self = Self(0);
    #[doc(alias = "UIDocumentBrowserUserInterfaceStyleLight")]
    pub const Light: Self = Self(1);
    #[doc(alias = "UIDocumentBrowserUserInterfaceStyleDark")]
    pub const Dark: Self = Self(2);
}

unsafe impl Encode for UIDocumentBrowserUserInterfaceStyle {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIDocumentBrowserUserInterfaceStyle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    pub struct UIDocumentBrowserViewController;

    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl ClassType for UIDocumentBrowserViewController {
        #[inherits(UIResponder, NSObject)]
        type Super = UIViewController;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSCoding for UIDocumentBrowserViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSObjectProtocol for UIDocumentBrowserViewController {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIAppearanceContainer for UIDocumentBrowserViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIContentContainer for UIDocumentBrowserViewController {}

#[cfg(all(
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIFocusEnvironment for UIDocumentBrowserViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIResponderStandardEditActions for UIDocumentBrowserViewController {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIViewController"
))]
unsafe impl UITraitEnvironment for UIDocumentBrowserViewController {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIDocumentBrowserViewController {
        #[deprecated]
        #[method_id(@__retain_semantics Init initForOpeningFilesWithContentTypes:)]
        pub unsafe fn initForOpeningFilesWithContentTypes(
            this: Allocated<Self>,
            allowed_content_types: Option<&NSArray<NSString>>,
        ) -> Id<Self>;

        #[cfg(feature = "objc2-uniform-type-identifiers")]
        #[method_id(@__retain_semantics Init initForOpeningContentTypes:)]
        pub unsafe fn initForOpeningContentTypes(
            this: Allocated<Self>,
            content_types: Option<&NSArray<UTType>>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name: Option<&NSString>,
            bundle: Option<&NSBundle>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn UIDocumentBrowserViewControllerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UIDocumentBrowserViewControllerDelegate>>,
        );

        #[method(allowsDocumentCreation)]
        pub unsafe fn allowsDocumentCreation(&self) -> bool;

        #[method(setAllowsDocumentCreation:)]
        pub unsafe fn setAllowsDocumentCreation(&self, allows_document_creation: bool);

        #[method(allowsPickingMultipleItems)]
        pub unsafe fn allowsPickingMultipleItems(&self) -> bool;

        #[method(setAllowsPickingMultipleItems:)]
        pub unsafe fn setAllowsPickingMultipleItems(&self, allows_picking_multiple_items: bool);

        #[deprecated = "allowedContentTypes is no longer supported"]
        #[method_id(@__retain_semantics Other allowedContentTypes)]
        pub unsafe fn allowedContentTypes(&self) -> Id<NSArray<NSString>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other recentDocumentsContentTypes)]
        pub unsafe fn recentDocumentsContentTypes(&self) -> Id<NSArray<NSString>>;

        #[cfg(feature = "objc2-uniform-type-identifiers")]
        #[method_id(@__retain_semantics Other contentTypesForRecentDocuments)]
        pub unsafe fn contentTypesForRecentDocuments(&self) -> Id<NSArray<UTType>>;

        #[method(shouldShowFileExtensions)]
        pub unsafe fn shouldShowFileExtensions(&self) -> bool;

        #[method(setShouldShowFileExtensions:)]
        pub unsafe fn setShouldShowFileExtensions(&self, should_show_file_extensions: bool);

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        #[method_id(@__retain_semantics Other additionalLeadingNavigationBarButtonItems)]
        pub unsafe fn additionalLeadingNavigationBarButtonItems(
            &self,
        ) -> Id<NSArray<UIBarButtonItem>>;

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        #[method(setAdditionalLeadingNavigationBarButtonItems:)]
        pub unsafe fn setAdditionalLeadingNavigationBarButtonItems(
            &self,
            additional_leading_navigation_bar_button_items: &NSArray<UIBarButtonItem>,
        );

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        #[method_id(@__retain_semantics Other additionalTrailingNavigationBarButtonItems)]
        pub unsafe fn additionalTrailingNavigationBarButtonItems(
            &self,
        ) -> Id<NSArray<UIBarButtonItem>>;

        #[cfg(all(feature = "UIBarButtonItem", feature = "UIBarItem"))]
        #[method(setAdditionalTrailingNavigationBarButtonItems:)]
        pub unsafe fn setAdditionalTrailingNavigationBarButtonItems(
            &self,
            additional_trailing_navigation_bar_button_items: &NSArray<UIBarButtonItem>,
        );

        #[cfg(feature = "block2")]
        #[method(revealDocumentAtURL:importIfNeeded:completion:)]
        pub unsafe fn revealDocumentAtURL_importIfNeeded_completion(
            &self,
            url: &NSURL,
            import_if_needed: bool,
            completion: Option<&Block<dyn Fn(*mut NSURL, *mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(importDocumentAtURL:nextToDocumentAtURL:mode:completionHandler:)]
        pub unsafe fn importDocumentAtURL_nextToDocumentAtURL_mode_completionHandler(
            &self,
            document_url: &NSURL,
            neighbour_url: &NSURL,
            import_mode: UIDocumentBrowserImportMode,
            completion: &Block<dyn Fn(*mut NSURL, *mut NSError)>,
        );

        #[cfg(feature = "block2")]
        #[method(renameDocumentAtURL:proposedName:completionHandler:)]
        pub unsafe fn renameDocumentAtURL_proposedName_completionHandler(
            &self,
            document_url: &NSURL,
            proposed_name: &NSString,
            completion_handler: &Block<dyn Fn(*mut NSURL, *mut NSError)>,
        );

        #[method_id(@__retain_semantics Other transitionControllerForDocumentAtURL:)]
        pub unsafe fn transitionControllerForDocumentAtURL(
            &self,
            document_url: &NSURL,
        ) -> Id<UIDocumentBrowserTransitionController>;

        #[deprecated]
        #[method_id(@__retain_semantics Other transitionControllerForDocumentURL:)]
        pub unsafe fn transitionControllerForDocumentURL(
            &self,
            document_url: &NSURL,
        ) -> Id<UIDocumentBrowserTransitionController>;

        #[cfg(feature = "UIDocumentBrowserAction")]
        #[method_id(@__retain_semantics Other customActions)]
        pub unsafe fn customActions(&self) -> Id<NSArray<UIDocumentBrowserAction>>;

        #[cfg(feature = "UIDocumentBrowserAction")]
        #[method(setCustomActions:)]
        pub unsafe fn setCustomActions(&self, custom_actions: &NSArray<UIDocumentBrowserAction>);

        #[method(browserUserInterfaceStyle)]
        pub unsafe fn browserUserInterfaceStyle(&self) -> UIDocumentBrowserUserInterfaceStyle;

        #[method(setBrowserUserInterfaceStyle:)]
        pub unsafe fn setBrowserUserInterfaceStyle(
            &self,
            browser_user_interface_style: UIDocumentBrowserUserInterfaceStyle,
        );

        #[method_id(@__retain_semantics Other localizedCreateDocumentActionTitle)]
        pub unsafe fn localizedCreateDocumentActionTitle(&self) -> Id<NSString>;

        #[method(setLocalizedCreateDocumentActionTitle:)]
        pub unsafe fn setLocalizedCreateDocumentActionTitle(
            &self,
            localized_create_document_action_title: &NSString,
        );

        #[method(defaultDocumentAspectRatio)]
        pub unsafe fn defaultDocumentAspectRatio(&self) -> CGFloat;

        #[method(setDefaultDocumentAspectRatio:)]
        pub unsafe fn setDefaultDocumentAspectRatio(&self, default_document_aspect_ratio: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIViewController`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIDocumentBrowserViewController {
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UIDocumentBrowserViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait UIDocumentBrowserViewControllerDelegate: NSObjectProtocol {
        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[deprecated]
        #[optional]
        #[method(documentBrowser:didPickDocumentURLs:)]
        unsafe fn documentBrowser_didPickDocumentURLs(
            &self,
            controller: &UIDocumentBrowserViewController,
            document_ur_ls: &NSArray<NSURL>,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(documentBrowser:didPickDocumentsAtURLs:)]
        unsafe fn documentBrowser_didPickDocumentsAtURLs(
            &self,
            controller: &UIDocumentBrowserViewController,
            document_ur_ls: &NSArray<NSURL>,
        );

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIViewController",
            feature = "block2"
        ))]
        #[optional]
        #[method(documentBrowser:didRequestDocumentCreationWithHandler:)]
        unsafe fn documentBrowser_didRequestDocumentCreationWithHandler(
            &self,
            controller: &UIDocumentBrowserViewController,
            import_handler: &Block<dyn Fn(*mut NSURL, UIDocumentBrowserImportMode)>,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(documentBrowser:didImportDocumentAtURL:toDestinationURL:)]
        unsafe fn documentBrowser_didImportDocumentAtURL_toDestinationURL(
            &self,
            controller: &UIDocumentBrowserViewController,
            source_url: &NSURL,
            destination_url: &NSURL,
        );

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[optional]
        #[method(documentBrowser:failedToImportDocumentAtURL:error:)]
        unsafe fn documentBrowser_failedToImportDocumentAtURL_error(
            &self,
            controller: &UIDocumentBrowserViewController,
            document_url: &NSURL,
            error: Option<&NSError>,
        );

        #[cfg(all(
            feature = "UIActivity",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other documentBrowser:applicationActivitiesForDocumentURLs:)]
        unsafe fn documentBrowser_applicationActivitiesForDocumentURLs(
            &self,
            controller: &UIDocumentBrowserViewController,
            document_ur_ls: &NSArray<NSURL>,
        ) -> Id<NSArray<UIActivity>>;

        #[cfg(all(
            feature = "UIActivityViewController",
            feature = "UIResponder",
            feature = "UIViewController"
        ))]
        #[optional]
        #[method(documentBrowser:willPresentActivityViewController:)]
        unsafe fn documentBrowser_willPresentActivityViewController(
            &self,
            controller: &UIDocumentBrowserViewController,
            activity_view_controller: &UIActivityViewController,
        );
    }

    unsafe impl ProtocolType for dyn UIDocumentBrowserViewControllerDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIDocumentBrowserTransitionController;

    unsafe impl ClassType for UIDocumentBrowserTransitionController {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIDocumentBrowserTransitionController {}

#[cfg(feature = "UIViewControllerTransitioning")]
unsafe impl UIViewControllerAnimatedTransitioning for UIDocumentBrowserTransitionController {}

extern_methods!(
    unsafe impl UIDocumentBrowserTransitionController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Other loadingProgress)]
        pub unsafe fn loadingProgress(&self) -> Option<Id<NSProgress>>;

        #[method(setLoadingProgress:)]
        pub unsafe fn setLoadingProgress(&self, loading_progress: Option<&NSProgress>);

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other targetView)]
        pub unsafe fn targetView(&self) -> Option<Id<UIView>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(setTargetView:)]
        pub unsafe fn setTargetView(&self, target_view: Option<&UIView>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIDocumentBrowserTransitionController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
