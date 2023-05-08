//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::OSAKit::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum OSALanguageFeatures {
        OSASupportsCompiling = 0x0002,
        OSASupportsGetSource = 0x0004,
        OSASupportsAECoercion = 0x0008,
        OSASupportsAESending = 0x0010,
        OSASupportsRecording = 0x0020,
        OSASupportsConvenience = 0x0040,
        OSASupportsDialects = 0x0080,
        OSASupportsEventHandling = 0x0100,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "OSAKit_OSALanguage")]
    pub struct OSALanguage;

    #[cfg(feature = "OSAKit_OSALanguage")]
    unsafe impl ClassType for OSALanguage {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "OSAKit_OSALanguage")]
unsafe impl NSObjectProtocol for OSALanguage {}

extern_methods!(
    #[cfg(feature = "OSAKit_OSALanguage")]
    unsafe impl OSALanguage {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other availableLanguages)]
        pub unsafe fn availableLanguages() -> Id<NSArray<OSALanguage>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other languageForName:)]
        pub unsafe fn languageForName(name: &NSString) -> Option<Id<OSALanguage>>;

        #[cfg(feature = "Foundation_NSAppleEventDescriptor")]
        #[method_id(@__retain_semantics Other languageForScriptDataDescriptor:)]
        pub unsafe fn languageForScriptDataDescriptor(
            descriptor: &NSAppleEventDescriptor,
        ) -> Option<Id<OSALanguage>>;

        #[method_id(@__retain_semantics Other defaultLanguage)]
        pub unsafe fn defaultLanguage() -> Option<Id<OSALanguage>>;

        #[method(setDefaultLanguage:)]
        pub unsafe fn setDefaultLanguage(default_language: &OSALanguage);

        #[cfg(feature = "OSAKit_OSALanguageInstance")]
        #[method_id(@__retain_semantics Other sharedLanguageInstance)]
        pub unsafe fn sharedLanguageInstance(&self) -> Id<OSALanguageInstance>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other info)]
        pub unsafe fn info(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other version)]
        pub unsafe fn version(&self) -> Option<Id<NSString>>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> OSType;

        #[method(subType)]
        pub unsafe fn subType(&self) -> OSType;

        #[method(manufacturer)]
        pub unsafe fn manufacturer(&self) -> OSType;

        #[method(features)]
        pub unsafe fn features(&self) -> OSALanguageFeatures;

        #[method(isThreadSafe)]
        pub unsafe fn isThreadSafe(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "OSAKit_OSALanguage")]
    unsafe impl OSALanguage {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
