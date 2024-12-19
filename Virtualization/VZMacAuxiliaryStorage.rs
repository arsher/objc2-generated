//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzmacauxiliarystorageinitializationoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VZMacAuxiliaryStorageInitializationOptions(pub NSUInteger);
bitflags::bitflags! {
    impl VZMacAuxiliaryStorageInitializationOptions: NSUInteger {
        const VZMacAuxiliaryStorageInitializationOptionAllowOverwrite = 1<<0;
    }
}

unsafe impl Encode for VZMacAuxiliaryStorageInitializationOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for VZMacAuxiliaryStorageInitializationOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/virtualization/vzmacauxiliarystorage?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct VZMacAuxiliaryStorage;
);

unsafe impl NSObjectProtocol for VZMacAuxiliaryStorage {}

extern_methods!(
    unsafe impl VZMacAuxiliaryStorage {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithURL:)]
        pub unsafe fn initWithURL(this: Allocated<Self>, url: &NSURL) -> Retained<Self>;

        #[cfg(feature = "VZMacHardwareModel")]
        #[method_id(@__retain_semantics Init initCreatingStorageAtURL:hardwareModel:options:error:_)]
        pub unsafe fn initCreatingStorageAtURL_hardwareModel_options_error(
            this: Allocated<Self>,
            url: &NSURL,
            hardware_model: &VZMacHardwareModel,
            options: VZMacAuxiliaryStorageInitializationOptions,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Retained<NSURL>;
    }
);

extern_methods!(
    /// VZDeprecated
    unsafe impl VZMacAuxiliaryStorage {
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(this: Allocated<Self>, url: &NSURL) -> Retained<Self>;
    }
);
