//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::LocalAuthentication::*;
use crate::LocalAuthenticationEmbeddedUI::*;

extern_category!(
    /// Category "UI" on [`LARight`].
    #[doc(alias = "UI")]
    pub unsafe trait LARightUI {
        #[cfg(all(
            feature = "AppKit_NSWindow",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(authorizeWithLocalizedReason:inPresentationContext:completion:)]
        unsafe fn authorizeWithLocalizedReason_inPresentationContext_completion(
            &self,
            localized_reason: &NSString,
            presentation_context: &LAPresentationContext,
            handler: &Block<dyn Fn(*mut NSError)>,
        );
    }

    #[cfg(feature = "LocalAuthentication_LARight")]
    unsafe impl LARightUI for LARight {}
);
