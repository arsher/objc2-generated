//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait UIAppearanceContainer: NSObjectProtocol + MainThreadOnly {}

    unsafe impl ProtocolType for dyn UIAppearanceContainer {}
);

extern_protocol!(
    pub unsafe trait UIAppearance: NSObjectProtocol + MainThreadOnly {
        #[method_id(@__retain_semantics Other appearance)]
        unsafe fn appearance(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other appearanceWhenContainedInInstancesOfClasses:)]
        unsafe fn appearanceWhenContainedInInstancesOfClasses(
            container_types: &NSArray<AnyClass>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "UITraitCollection")]
        #[method_id(@__retain_semantics Other appearanceForTraitCollection:)]
        unsafe fn appearanceForTraitCollection(
            r#trait: &UITraitCollection,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[cfg(feature = "UITraitCollection")]
        #[method_id(@__retain_semantics Other appearanceForTraitCollection:whenContainedInInstancesOfClasses:)]
        unsafe fn appearanceForTraitCollection_whenContainedInInstancesOfClasses(
            r#trait: &UITraitCollection,
            container_types: &NSArray<AnyClass>,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;
    }

    unsafe impl ProtocolType for dyn UIAppearance {}
);
