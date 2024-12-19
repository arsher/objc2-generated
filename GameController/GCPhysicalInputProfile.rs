//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamecontroller/gcphysicalinputprofile?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GCPhysicalInputProfile;
);

unsafe impl NSObjectProtocol for GCPhysicalInputProfile {}

extern_methods!(
    unsafe impl GCPhysicalInputProfile {
        #[cfg(feature = "GCDevice")]
        #[method_id(@__retain_semantics Other device)]
        pub unsafe fn device(&self) -> Option<Retained<ProtocolObject<dyn GCDevice>>>;

        #[method(lastEventTimestamp)]
        pub unsafe fn lastEventTimestamp(&self) -> NSTimeInterval;

        #[method(hasRemappedElements)]
        pub unsafe fn hasRemappedElements(&self) -> bool;

        #[cfg(all(feature = "GCControllerElement", feature = "block2"))]
        #[method(valueDidChangeHandler)]
        pub unsafe fn valueDidChangeHandler(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<GCPhysicalInputProfile>, NonNull<GCControllerElement>)>;

        #[cfg(all(feature = "GCControllerElement", feature = "block2"))]
        #[method(setValueDidChangeHandler:)]
        pub unsafe fn setValueDidChangeHandler(
            &self,
            value_did_change_handler: Option<
                &block2::Block<
                    dyn Fn(NonNull<GCPhysicalInputProfile>, NonNull<GCControllerElement>),
                >,
            >,
        );

        #[cfg(feature = "GCControllerElement")]
        #[method_id(@__retain_semantics Other elements)]
        pub unsafe fn elements(&self) -> Retained<NSDictionary<NSString, GCControllerElement>>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        #[method_id(@__retain_semantics Other buttons)]
        pub unsafe fn buttons(&self) -> Retained<NSDictionary<NSString, GCControllerButtonInput>>;

        #[cfg(all(feature = "GCControllerAxisInput", feature = "GCControllerElement"))]
        #[method_id(@__retain_semantics Other axes)]
        pub unsafe fn axes(&self) -> Retained<NSDictionary<NSString, GCControllerAxisInput>>;

        #[cfg(all(feature = "GCControllerDirectionPad", feature = "GCControllerElement"))]
        #[method_id(@__retain_semantics Other dpads)]
        pub unsafe fn dpads(&self) -> Retained<NSDictionary<NSString, GCControllerDirectionPad>>;

        #[cfg(all(feature = "GCControllerElement", feature = "GCControllerTouchpad"))]
        #[method_id(@__retain_semantics Other touchpads)]
        pub unsafe fn touchpads(&self) -> Retained<NSDictionary<NSString, GCControllerTouchpad>>;

        #[cfg(feature = "GCControllerElement")]
        #[method_id(@__retain_semantics Other allElements)]
        pub unsafe fn allElements(&self) -> Retained<NSSet<GCControllerElement>>;

        #[cfg(all(feature = "GCControllerButtonInput", feature = "GCControllerElement"))]
        #[method_id(@__retain_semantics Other allButtons)]
        pub unsafe fn allButtons(&self) -> Retained<NSSet<GCControllerButtonInput>>;

        #[cfg(all(feature = "GCControllerAxisInput", feature = "GCControllerElement"))]
        #[method_id(@__retain_semantics Other allAxes)]
        pub unsafe fn allAxes(&self) -> Retained<NSSet<GCControllerAxisInput>>;

        #[cfg(all(feature = "GCControllerDirectionPad", feature = "GCControllerElement"))]
        #[method_id(@__retain_semantics Other allDpads)]
        pub unsafe fn allDpads(&self) -> Retained<NSSet<GCControllerDirectionPad>>;

        #[cfg(all(feature = "GCControllerElement", feature = "GCControllerTouchpad"))]
        #[method_id(@__retain_semantics Other allTouchpads)]
        pub unsafe fn allTouchpads(&self) -> Retained<NSSet<GCControllerTouchpad>>;

        #[cfg(feature = "GCControllerElement")]
        #[method_id(@__retain_semantics Other objectForKeyedSubscript:)]
        pub unsafe fn objectForKeyedSubscript(
            &self,
            key: &NSString,
        ) -> Option<Retained<GCControllerElement>>;

        #[method_id(@__retain_semantics Other capture)]
        pub unsafe fn capture(&self) -> Retained<Self>;

        #[method(setStateFromPhysicalInput:)]
        pub unsafe fn setStateFromPhysicalInput(&self, physical_input: &GCPhysicalInputProfile);

        #[method_id(@__retain_semantics Other mappedElementAliasForPhysicalInputName:)]
        pub unsafe fn mappedElementAliasForPhysicalInputName(
            &self,
            input_name: &NSString,
        ) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other mappedPhysicalInputNamesForElementAlias:)]
        pub unsafe fn mappedPhysicalInputNamesForElementAlias(
            &self,
            element_alias: &NSString,
        ) -> Retained<NSSet<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GCPhysicalInputProfile {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
