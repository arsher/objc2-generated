//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiconfigurationstatecustomkey?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type UIConfigurationStateCustomKey = NSString;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiconfigurationstate?language=objc)
    pub unsafe trait UIConfigurationState:
        NSCopying + NSObjectProtocol + NSSecureCoding + MainThreadOnly
    {
        #[cfg(feature = "UITraitCollection")]
        #[method_id(@__retain_semantics Init initWithTraitCollection:)]
        unsafe fn initWithTraitCollection(
            this: Allocated<Self>,
            trait_collection: &UITraitCollection,
        ) -> Retained<Self>;

        #[cfg(feature = "UITraitCollection")]
        #[method_id(@__retain_semantics Other traitCollection)]
        unsafe fn traitCollection(&self) -> Retained<UITraitCollection>;

        #[cfg(feature = "UITraitCollection")]
        #[method(setTraitCollection:)]
        unsafe fn setTraitCollection(&self, trait_collection: &UITraitCollection);

        #[method_id(@__retain_semantics Other customStateForKey:)]
        unsafe fn customStateForKey(
            &self,
            key: &UIConfigurationStateCustomKey,
        ) -> Option<Retained<AnyObject>>;

        #[method(setCustomState:forKey:)]
        unsafe fn setCustomState_forKey(
            &self,
            custom_state: Option<&AnyObject>,
            key: &UIConfigurationStateCustomKey,
        );

        #[method_id(@__retain_semantics Other objectForKeyedSubscript:)]
        unsafe fn objectForKeyedSubscript(
            &self,
            key: &UIConfigurationStateCustomKey,
        ) -> Option<Retained<AnyObject>>;

        #[method(setObject:forKeyedSubscript:)]
        unsafe fn setObject_forKeyedSubscript(
            &self,
            obj: Option<&AnyObject>,
            key: &UIConfigurationStateCustomKey,
        );
    }

    unsafe impl ProtocolType for dyn UIConfigurationState {}
);
