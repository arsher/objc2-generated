//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_category!(
    /// Category "WebPlugInContainer" on [`NSObject`].
    #[doc(alias = "WebPlugInContainer")]
    /// This informal protocol enables a plug-in to request that its containing application
    /// perform certain operations.
    pub unsafe trait NSObjectWebPlugInContainer {
        /// Tell the application to show a URL in a target frame
        ///
        /// Parameter `request`: The request to be loaded.
        ///
        /// Parameter `target`: The target frame. If the frame with the specified target is not
        /// found, a new window is opened and the main frame of the new window is named
        /// with the specified target.  If nil is specified, the frame that contains
        /// the applet is targeted.
        #[method(webPlugInContainerLoadRequest:inFrame:)]
        unsafe fn webPlugInContainerLoadRequest_inFrame(
            &self,
            request: Option<&NSURLRequest>,
            target: Option<&NSString>,
        );

        /// Tell the application to show the specified status message.
        ///
        /// Parameter `message`: The string to be shown.
        #[method(webPlugInContainerShowStatus:)]
        unsafe fn webPlugInContainerShowStatus(&self, message: Option<&NSString>);

        #[cfg(feature = "objc2-app-kit")]
        #[cfg(target_os = "macos")]
        /// The color that should be used for any special drawing when
        /// plug-in is selected.
        #[method_id(@__retain_semantics Other webPlugInContainerSelectionColor)]
        unsafe fn webPlugInContainerSelectionColor(&self) -> Option<Retained<NSColor>>;

        #[cfg(feature = "WebFrame")]
        /// Allows the plug-in to access the WebFrame that
        /// contains the plug-in.  This method will not be implemented by containers that
        /// are not WebKit based.
        #[method_id(@__retain_semantics Other webFrame)]
        unsafe fn webFrame(&self) -> Option<Retained<WebFrame>>;
    }

    unsafe impl NSObjectWebPlugInContainer for NSObject {}
);
