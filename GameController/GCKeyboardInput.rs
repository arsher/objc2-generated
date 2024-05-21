//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

#[cfg(all(
    feature = "GCControllerButtonInput",
    feature = "GCControllerElement",
    feature = "GCKeyCodes",
    feature = "GCPhysicalInputProfile",
    feature = "block2"
))]
pub type GCKeyboardValueChangedHandler = *mut block2::Block<
    dyn Fn(NonNull<GCKeyboardInput>, NonNull<GCControllerButtonInput>, GCKeyCode, Bool),
>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GCPhysicalInputProfile")]
    pub struct GCKeyboardInput;

    #[cfg(feature = "GCPhysicalInputProfile")]
    unsafe impl ClassType for GCKeyboardInput {
        #[inherits(NSObject)]
        type Super = GCPhysicalInputProfile;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GCPhysicalInputProfile")]
unsafe impl NSObjectProtocol for GCKeyboardInput {}

extern_methods!(
    #[cfg(feature = "GCPhysicalInputProfile")]
    unsafe impl GCKeyboardInput {
        #[cfg(all(
            feature = "GCControllerButtonInput",
            feature = "GCControllerElement",
            feature = "GCKeyCodes",
            feature = "block2"
        ))]
        #[method(keyChangedHandler)]
        pub unsafe fn keyChangedHandler(&self) -> GCKeyboardValueChangedHandler;

        #[cfg(all(
            feature = "GCControllerButtonInput",
            feature = "GCControllerElement",
            feature = "GCKeyCodes",
            feature = "block2"
        ))]
        #[method(setKeyChangedHandler:)]
        pub unsafe fn setKeyChangedHandler(
            &self,
            key_changed_handler: GCKeyboardValueChangedHandler,
        );

        #[method(isAnyKeyPressed)]
        pub unsafe fn isAnyKeyPressed(&self) -> bool;

        #[cfg(all(
            feature = "GCControllerButtonInput",
            feature = "GCControllerElement",
            feature = "GCKeyCodes"
        ))]
        #[method_id(@__retain_semantics Other buttonForKeyCode:)]
        pub unsafe fn buttonForKeyCode(
            &self,
            code: GCKeyCode,
        ) -> Option<Retained<GCControllerButtonInput>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GCPhysicalInputProfile")]
    unsafe impl GCKeyboardInput {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
