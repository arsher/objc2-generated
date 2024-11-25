//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisearchcontainerviewcontroller?language=objc)
    #[unsafe(super(UIViewController, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    pub struct UISearchContainerViewController;
);

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSCoding for UISearchContainerViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl NSObjectProtocol for UISearchContainerViewController {}

#[cfg(all(
    feature = "UIAppearance",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIAppearanceContainer for UISearchContainerViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIContentContainer for UISearchContainerViewController {}

#[cfg(all(
    feature = "UIFocus",
    feature = "UIResponder",
    feature = "UIViewController"
))]
unsafe impl UIFocusEnvironment for UISearchContainerViewController {}

#[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
unsafe impl UIResponderStandardEditActions for UISearchContainerViewController {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIViewController"
))]
unsafe impl UITraitEnvironment for UISearchContainerViewController {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UISearchContainerViewController {
        #[cfg(feature = "UISearchController")]
        #[method_id(@__retain_semantics Other searchController)]
        pub unsafe fn searchController(&self) -> Retained<UISearchController>;

        #[cfg(feature = "UISearchController")]
        #[method_id(@__retain_semantics Init initWithSearchController:)]
        pub unsafe fn initWithSearchController(
            this: Allocated<Self>,
            search_controller: &UISearchController,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIViewController`
    #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
    unsafe impl UISearchContainerViewController {
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
    unsafe impl UISearchContainerViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
