//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocomponentflags?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AudioComponentFlags(pub u32);
bitflags::bitflags! {
    impl AudioComponentFlags: u32 {
        const kAudioComponentFlag_Unsearchable = 1;
        const kAudioComponentFlag_SandboxSafe = 2;
        const kAudioComponentFlag_IsV3AudioUnit = 4;
        const kAudioComponentFlag_RequiresAsyncInstantiation = 8;
        const kAudioComponentFlag_CanLoadInProcess = 0x10;
    }
}

unsafe impl Encode for AudioComponentFlags {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for AudioComponentFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocomponentinstantiationoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AudioComponentInstantiationOptions(pub u32);
bitflags::bitflags! {
    impl AudioComponentInstantiationOptions: u32 {
        const kAudioComponentInstantiation_LoadOutOfProcess = 1;
        const kAudioComponentInstantiation_LoadInProcess = 2;
        const kAudioComponentInstantiation_LoadedRemotely = 1<<31;
    }
}

unsafe impl Encode for AudioComponentInstantiationOptions {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for AudioComponentInstantiationOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocomponentdescription?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioComponentDescription {
    pub componentType: OSType,
    pub componentSubType: OSType,
    pub componentManufacturer: OSType,
    pub componentFlags: u32,
    pub componentFlagsMask: u32,
}

unsafe impl Encode for AudioComponentDescription {
    const ENCODING: Encoding = Encoding::Struct(
        "AudioComponentDescription",
        &[
            <OSType>::ENCODING,
            <OSType>::ENCODING,
            <OSType>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for AudioComponentDescription {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocomponent?language=objc)
pub type AudioComponent = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocomponentinstance?language=objc)
pub type AudioComponentInstance = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocomponentmethod?language=objc)
pub type AudioComponentMethod =
    Option<unsafe extern "C-unwind" fn(NonNull<c_void>, ...) -> OSStatus>;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocomponentplugininterface?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioComponentPlugInInterface {
    pub Open: unsafe extern "C-unwind" fn(NonNull<c_void>, AudioComponentInstance) -> OSStatus,
    pub Close: unsafe extern "C-unwind" fn(NonNull<c_void>) -> OSStatus,
    pub Lookup: unsafe extern "C-unwind" fn(i16) -> AudioComponentMethod,
    pub reserved: *mut c_void,
}

unsafe impl Encode for AudioComponentPlugInInterface {
    const ENCODING: Encoding = Encoding::Struct("AudioComponentPlugInInterface", &[<unsafe extern "C-unwind" fn(NonNull<c_void>,AudioComponentInstance,) -> OSStatus>::ENCODING,<unsafe extern "C-unwind" fn(NonNull<c_void>,) -> OSStatus>::ENCODING,<unsafe extern "C-unwind" fn(i16,) -> AudioComponentMethod>::ENCODING,<*mut c_void>::ENCODING,]);
}

unsafe impl RefEncode for AudioComponentPlugInInterface {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocomponentfactoryfunction?language=objc)
pub type AudioComponentFactoryFunction = Option<
    unsafe extern "C-unwind" fn(
        NonNull<AudioComponentDescription>,
    ) -> *mut AudioComponentPlugInInterface,
>;

extern "C-unwind" {
    pub fn AudioComponentFindNext(
        in_component: AudioComponent,
        in_desc: NonNull<AudioComponentDescription>,
    ) -> AudioComponent;
}

extern "C-unwind" {
    pub fn AudioComponentCount(in_desc: NonNull<AudioComponentDescription>) -> u32;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn AudioComponentCopyName(
        in_component: AudioComponent,
        out_name: NonNull<CFStringRef>,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn AudioComponentGetDescription(
        in_component: AudioComponent,
        out_desc: NonNull<AudioComponentDescription>,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn AudioComponentGetVersion(
        in_component: AudioComponent,
        out_version: NonNull<u32>,
    ) -> OSStatus;
}

extern "C-unwind" {
    pub fn AudioComponentInstanceNew(
        in_component: AudioComponent,
        out_instance: NonNull<AudioComponentInstance>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "block2")]
    pub fn AudioComponentInstantiate(
        in_component: AudioComponent,
        in_options: AudioComponentInstantiationOptions,
        in_completion_handler: &block2::Block<dyn Fn(AudioComponentInstance, OSStatus)>,
    );
}

extern "C-unwind" {
    pub fn AudioComponentInstanceDispose(in_instance: AudioComponentInstance) -> OSStatus;
}

extern "C-unwind" {
    pub fn AudioComponentInstanceGetComponent(
        in_instance: AudioComponentInstance,
    ) -> AudioComponent;
}

extern "C-unwind" {
    pub fn AudioComponentInstanceCanDo(
        in_instance: AudioComponentInstance,
        in_selector_id: i16,
    ) -> Boolean;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn AudioComponentRegister(
        in_desc: NonNull<AudioComponentDescription>,
        in_name: CFStringRef,
        in_version: u32,
        in_factory: AudioComponentFactoryFunction,
    ) -> AudioComponent;
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn AudioComponentCopyConfigurationInfo(
        in_component: AudioComponent,
        out_configuration_info: NonNull<CFDictionaryRef>,
    ) -> OSStatus;
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocomponentvalidationresult?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AudioComponentValidationResult(pub u32);
impl AudioComponentValidationResult {
    pub const kAudioComponentValidationResult_Unknown: Self = Self(0);
    pub const kAudioComponentValidationResult_Passed: Self = Self(1);
    pub const kAudioComponentValidationResult_Failed: Self = Self(2);
    pub const kAudioComponentValidationResult_TimedOut: Self = Self(3);
    pub const kAudioComponentValidationResult_UnauthorizedError_Open: Self = Self(4);
    pub const kAudioComponentValidationResult_UnauthorizedError_Init: Self = Self(5);
}

unsafe impl Encode for AudioComponentValidationResult {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for AudioComponentValidationResult {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(feature = "objc2-core-foundation")]
    pub fn AudioComponentValidate(
        in_component: AudioComponent,
        in_validation_parameters: CFDictionaryRef,
        out_validation_result: NonNull<AudioComponentValidationResult>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(all(feature = "block2", feature = "objc2-core-foundation"))]
    pub fn AudioComponentValidateWithResults(
        in_component: AudioComponent,
        in_validation_parameters: CFDictionaryRef,
        in_completion_handler: &block2::Block<
            dyn Fn(AudioComponentValidationResult, CFDictionaryRef),
        >,
    ) -> OSStatus;
}
