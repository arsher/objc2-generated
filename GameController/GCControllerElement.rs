//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GCSystemGestureState {
        GCSystemGestureStateEnabled = 0,
        GCSystemGestureStateAlwaysReceive = 1,
        GCSystemGestureStateDisabled = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCControllerElement")]
    pub struct GCControllerElement;

    #[cfg(feature = "GameController_GCControllerElement")]
    unsafe impl ClassType for GCControllerElement {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameController_GCControllerElement")]
unsafe impl NSObjectProtocol for GCControllerElement {}

extern_methods!(
    #[cfg(feature = "GameController_GCControllerElement")]
    unsafe impl GCControllerElement {
        #[method_id(@__retain_semantics Other collection)]
        pub unsafe fn collection(&self) -> Option<Id<GCControllerElement, Shared>>;

        #[method(isAnalog)]
        pub unsafe fn isAnalog(&self) -> bool;

        #[method(isBoundToSystemGesture)]
        pub unsafe fn isBoundToSystemGesture(&self) -> bool;

        #[method(preferredSystemGestureState)]
        pub unsafe fn preferredSystemGestureState(&self) -> GCSystemGestureState;

        #[method(setPreferredSystemGestureState:)]
        pub unsafe fn setPreferredSystemGestureState(
            &self,
            preferred_system_gesture_state: GCSystemGestureState,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other sfSymbolsName)]
        pub unsafe fn sfSymbolsName(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSfSymbolsName:)]
        pub unsafe fn setSfSymbolsName(&self, sf_symbols_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedName)]
        pub unsafe fn localizedName(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLocalizedName:)]
        pub unsafe fn setLocalizedName(&self, localized_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other unmappedSfSymbolsName)]
        pub unsafe fn unmappedSfSymbolsName(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setUnmappedSfSymbolsName:)]
        pub unsafe fn setUnmappedSfSymbolsName(&self, unmapped_sf_symbols_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other unmappedLocalizedName)]
        pub unsafe fn unmappedLocalizedName(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setUnmappedLocalizedName:)]
        pub unsafe fn setUnmappedLocalizedName(&self, unmapped_localized_name: Option<&NSString>);

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other aliases)]
        pub unsafe fn aliases(&self) -> Id<NSSet<NSString>, Shared>;
    }
);
