//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/extensionkit/exhostviewcontroller?language=objc)
    #[unsafe(super(NSViewController, NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    pub struct EXHostViewController;
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for EXHostViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSEditor for EXHostViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for EXHostViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSSeguePerforming for EXHostViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification for EXHostViewController {}

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl EXHostViewController {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn EXHostViewControllerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn EXHostViewControllerDelegate>>,
        );

        #[method_id(@__retain_semantics Other placeholderView)]
        pub unsafe fn placeholderView(&self) -> Retained<NSView>;

        #[method(setPlaceholderView:)]
        pub unsafe fn setPlaceholderView(&self, placeholder_view: &NSView);

        #[method_id(@__retain_semantics Other makeXPCConnectionWithError:_)]
        pub unsafe fn makeXPCConnectionWithError(
            &self,
        ) -> Result<Retained<NSXPCConnection>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl EXHostViewController {
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
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
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl EXHostViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl EXHostViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/extensionkit/exhostviewcontrollerdelegate?language=objc)
    pub unsafe trait EXHostViewControllerDelegate:
        NSObjectProtocol + MainThreadOnly
    {
        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(hostViewControllerDidActivate:)]
        unsafe fn hostViewControllerDidActivate(&self, view_controller: &EXHostViewController);

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        #[optional]
        #[method(hostViewControllerWillDeactivate:error:)]
        unsafe fn hostViewControllerWillDeactivate_error(
            &self,
            view_controller: &EXHostViewController,
            error: Option<&NSError>,
        );
    }

    unsafe impl ProtocolType for dyn EXHostViewControllerDelegate {}
);
