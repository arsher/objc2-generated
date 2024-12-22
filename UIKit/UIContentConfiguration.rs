//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicontentconfiguration?language=objc)
    pub unsafe trait UIContentConfiguration:
        NSCopying + NSObjectProtocol + MainThreadOnly
    {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        /// Initializes and returns a new instance of the content view using this configuration.
        #[method_id(@__retain_semantics Other makeContentView)]
        unsafe fn makeContentView(&self) -> Retained<UIView>;

        #[cfg(feature = "UIConfigurationState")]
        /// Returns a copy of the configuration updated for the specified state, by applying the configuration's default values for that state to any properties that have not been customized.
        #[method_id(@__retain_semantics Other updatedConfigurationForState:)]
        unsafe fn updatedConfigurationForState(
            &self,
            state: &ProtocolObject<dyn UIConfigurationState>,
        ) -> Retained<Self>;
    }

    unsafe impl ProtocolType for dyn UIContentConfiguration {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicontentview?language=objc)
    pub unsafe trait UIContentView: NSObjectProtocol + MainThreadOnly {
        /// Returns the current configuration of the view. Setting this property applies the new configuration to the view.
        #[method_id(@__retain_semantics Other configuration)]
        unsafe fn configuration(&self) -> Retained<ProtocolObject<dyn UIContentConfiguration>>;

        /// Setter for [`configuration`][Self::configuration].
        #[method(setConfiguration:)]
        unsafe fn setConfiguration(
            &self,
            configuration: &ProtocolObject<dyn UIContentConfiguration>,
        );

        /// Whether this view is compatible with the provided configuration, meaning the view supports
        /// it being set to the `configuration` property and is capable of updating itself for the
        /// configuration. If not implemented, the view is assumed to be compatible with configuration
        /// classes that match the class of the view's existing configuration.
        #[optional]
        #[method(supportsConfiguration:)]
        unsafe fn supportsConfiguration(
            &self,
            configuration: &ProtocolObject<dyn UIContentConfiguration>,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn UIContentView {}
);
