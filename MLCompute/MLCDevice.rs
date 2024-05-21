//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-metal")]
use objc2_metal::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct MLCDevice;

    unsafe impl ClassType for MLCDevice {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSCopying for MLCDevice {}

unsafe impl NSObjectProtocol for MLCDevice {}

extern_methods!(
    unsafe impl MLCDevice {
        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method(type)]
        pub unsafe fn r#type(&self) -> MLCDeviceType;

        #[cfg(feature = "MLCTypes")]
        #[method(actualDeviceType)]
        pub unsafe fn actualDeviceType(&self) -> MLCDeviceType;

        #[cfg(feature = "objc2-metal")]
        #[deprecated]
        #[method_id(@__retain_semantics Other gpuDevices)]
        pub unsafe fn gpuDevices(&self) -> Retained<NSArray<ProtocolObject<dyn MTLDevice>>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other cpuDevice)]
        pub unsafe fn cpuDevice() -> Retained<Self>;

        #[deprecated]
        #[method_id(@__retain_semantics Other gpuDevice)]
        pub unsafe fn gpuDevice() -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other aneDevice)]
        pub unsafe fn aneDevice() -> Option<Retained<Self>>;

        #[cfg(feature = "MLCTypes")]
        #[deprecated]
        #[method_id(@__retain_semantics Other deviceWithType:)]
        pub unsafe fn deviceWithType(r#type: MLCDeviceType) -> Option<Retained<Self>>;

        #[cfg(feature = "MLCTypes")]
        #[method_id(@__retain_semantics Other deviceWithType:selectsMultipleComputeDevices:)]
        pub unsafe fn deviceWithType_selectsMultipleComputeDevices(
            r#type: MLCDeviceType,
            selects_multiple_compute_devices: bool,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "objc2-metal")]
        #[deprecated]
        #[method_id(@__retain_semantics Other deviceWithGPUDevices:)]
        pub unsafe fn deviceWithGPUDevices(
            gpus: &NSArray<ProtocolObject<dyn MTLDevice>>,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLCDevice {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
