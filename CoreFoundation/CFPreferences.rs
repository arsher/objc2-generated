//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfpreferencesanyapplication?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFPreferencesAnyApplication: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfpreferencescurrentapplication?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFPreferencesCurrentApplication: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfpreferencesanyhost?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFPreferencesAnyHost: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfpreferencescurrenthost?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFPreferencesCurrentHost: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfpreferencesanyuser?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFPreferencesAnyUser: CFStringRef;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/corefoundation/kcfpreferencescurrentuser?language=objc)
    #[cfg(feature = "CFBase")]
    pub static kCFPreferencesCurrentUser: CFStringRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFPreferencesCopyAppValue(
        key: CFStringRef,
        application_id: CFStringRef,
    ) -> CFPropertyListRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFPreferencesGetAppBooleanValue(
        key: CFStringRef,
        application_id: CFStringRef,
        key_exists_and_has_valid_format: *mut Boolean,
    ) -> Boolean;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFPreferencesGetAppIntegerValue(
        key: CFStringRef,
        application_id: CFStringRef,
        key_exists_and_has_valid_format: *mut Boolean,
    ) -> CFIndex;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFPreferencesSetAppValue(
        key: CFStringRef,
        value: CFPropertyListRef,
        application_id: CFStringRef,
    );
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFPreferencesAddSuitePreferencesToApp(
        application_id: CFStringRef,
        suite_id: CFStringRef,
    );
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFPreferencesRemoveSuitePreferencesFromApp(
        application_id: CFStringRef,
        suite_id: CFStringRef,
    );
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFPreferencesAppSynchronize(application_id: CFStringRef) -> Boolean;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFPreferencesCopyValue(
        key: CFStringRef,
        application_id: CFStringRef,
        user_name: CFStringRef,
        host_name: CFStringRef,
    ) -> CFPropertyListRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFArray", feature = "CFBase", feature = "CFDictionary"))]
    pub fn CFPreferencesCopyMultiple(
        keys_to_fetch: CFArrayRef,
        application_id: CFStringRef,
        user_name: CFStringRef,
        host_name: CFStringRef,
    ) -> CFDictionaryRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFPreferencesSetValue(
        key: CFStringRef,
        value: CFPropertyListRef,
        application_id: CFStringRef,
        user_name: CFStringRef,
        host_name: CFStringRef,
    );
}

extern "C-unwind" {
    #[cfg(all(feature = "CFArray", feature = "CFBase", feature = "CFDictionary"))]
    pub fn CFPreferencesSetMultiple(
        keys_to_set: CFDictionaryRef,
        keys_to_remove: CFArrayRef,
        application_id: CFStringRef,
        user_name: CFStringRef,
        host_name: CFStringRef,
    );
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFPreferencesSynchronize(
        application_id: CFStringRef,
        user_name: CFStringRef,
        host_name: CFStringRef,
    ) -> Boolean;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFArray", feature = "CFBase"))]
    #[deprecated = "Unsupported API"]
    pub fn CFPreferencesCopyApplicationList(
        user_name: CFStringRef,
        host_name: CFStringRef,
    ) -> CFArrayRef;
}

extern "C-unwind" {
    #[cfg(all(feature = "CFArray", feature = "CFBase"))]
    pub fn CFPreferencesCopyKeyList(
        application_id: CFStringRef,
        user_name: CFStringRef,
        host_name: CFStringRef,
    ) -> CFArrayRef;
}

extern "C-unwind" {
    #[cfg(feature = "CFBase")]
    pub fn CFPreferencesAppValueIsForced(key: CFStringRef, application_id: CFStringRef) -> Boolean;
}