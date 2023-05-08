//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::OSAKit::*;

extern_static!(OSAScriptErrorMessageKey: &'static NSString);

extern_static!(OSAScriptErrorBriefMessageKey: &'static NSString);

extern_static!(OSAScriptErrorNumberKey: &'static NSString);

extern_static!(OSAScriptErrorPartialResultKey: &'static NSString);

extern_static!(OSAScriptErrorOffendingObjectKey: &'static NSString);

extern_static!(OSAScriptErrorExpectedTypeKey: &'static NSString);

extern_static!(OSAScriptErrorAppAddressKey: &'static NSString);

extern_static!(OSAScriptErrorAppNameKey: &'static NSString);

extern_static!(OSAScriptErrorRangeKey: &'static NSString);

extern_static!(OSAScriptErrorMessage: &'static NSString);

extern_static!(OSAScriptErrorNumber: &'static NSString);

extern_static!(OSAScriptErrorAppName: &'static NSString);

extern_static!(OSAScriptErrorBriefMessage: &'static NSString);

extern_static!(OSAScriptErrorRange: &'static NSString);

extern_static!(OSAStorageScriptType: &'static NSString);

extern_static!(OSAStorageScriptBundleType: &'static NSString);

extern_static!(OSAStorageApplicationType: &'static NSString);

extern_static!(OSAStorageApplicationBundleType: &'static NSString);

extern_static!(OSAStorageTextType: &'static NSString);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum OSAStorageOptions {
        OSANull = 0x00000000,
        OSAPreventGetSource = 0x00000001,
        OSACompileIntoContext = 0x00000002,
        OSADontSetScriptLocation = 0x01000000,
        OSAStayOpenApplet = 0x10000000,
        OSAShowStartupScreen = 0x20000000,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "OSAKit_OSAScript")]
    pub struct OSAScript;

    #[cfg(feature = "OSAKit_OSAScript")]
    unsafe impl ClassType for OSAScript {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "OSAKit_OSAScript")]
unsafe impl NSCopying for OSAScript {}

#[cfg(feature = "OSAKit_OSAScript")]
unsafe impl NSObjectProtocol for OSAScript {}

extern_methods!(
    #[cfg(feature = "OSAKit_OSAScript")]
    unsafe impl OSAScript {
        #[cfg(all(
            feature = "Foundation_NSAppleEventDescriptor",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other scriptDataDescriptorWithContentsOfURL:)]
        pub unsafe fn scriptDataDescriptorWithContentsOfURL(
            url: &NSURL,
        ) -> Option<Id<NSAppleEventDescriptor>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithSource:)]
        pub unsafe fn initWithSource(this: Option<Allocated<Self>>, source: &NSString) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSString", feature = "OSAKit_OSALanguage"))]
        #[method_id(@__retain_semantics Init initWithSource:language:)]
        pub unsafe fn initWithSource_language(
            this: Option<Allocated<Self>>,
            source: &NSString,
            language: Option<&OSALanguage>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL",
            feature = "OSAKit_OSALanguageInstance"
        ))]
        #[method_id(@__retain_semantics Init initWithSource:fromURL:languageInstance:usingStorageOptions:)]
        pub unsafe fn initWithSource_fromURL_languageInstance_usingStorageOptions(
            this: Option<Allocated<Self>>,
            source: &NSString,
            url: Option<&NSURL>,
            instance: Option<&OSALanguageInstance>,
            storage_options: OSAStorageOptions,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:error:)]
        pub unsafe fn initWithContentsOfURL_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            error_info: Option<&mut Option<Id<NSDictionary<NSString, Object>>>>,
        ) -> Option<Id<Self>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL",
            feature = "OSAKit_OSALanguage"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:language:error:)]
        pub unsafe fn initWithContentsOfURL_language_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            language: &OSALanguage,
            error_info: Option<&mut Option<Id<NSDictionary<NSString, Object>>>>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL",
            feature = "OSAKit_OSALanguageInstance"
        ))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:languageInstance:usingStorageOptions:error:_)]
        pub unsafe fn initWithContentsOfURL_languageInstance_usingStorageOptions_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            instance: Option<&OSALanguageInstance>,
            storage_options: OSAStorageOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithCompiledData:error:)]
        pub unsafe fn initWithCompiledData_error(
            this: Option<Allocated<Self>>,
            data: &NSData,
            error_info: Option<&mut Option<Id<NSDictionary<NSString, Object>>>>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initWithCompiledData:fromURL:usingStorageOptions:error:_)]
        pub unsafe fn initWithCompiledData_fromURL_usingStorageOptions_error(
            this: Option<Allocated<Self>>,
            data: &NSData,
            url: Option<&NSURL>,
            storage_options: OSAStorageOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSAppleEventDescriptor",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL",
            feature = "OSAKit_OSALanguageInstance"
        ))]
        #[method_id(@__retain_semantics Init initWithScriptDataDescriptor:fromURL:languageInstance:usingStorageOptions:error:_)]
        pub unsafe fn initWithScriptDataDescriptor_fromURL_languageInstance_usingStorageOptions_error(
            this: Option<Allocated<Self>>,
            data: &NSAppleEventDescriptor,
            url: Option<&NSURL>,
            instance: Option<&OSALanguageInstance>,
            storage_options: OSAStorageOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other source)]
        pub unsafe fn source(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other url)]
        pub unsafe fn url(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "OSAKit_OSALanguage")]
        #[method_id(@__retain_semantics Other language)]
        pub unsafe fn language(&self) -> Id<OSALanguage>;

        #[cfg(feature = "OSAKit_OSALanguage")]
        #[method(setLanguage:)]
        pub unsafe fn setLanguage(&self, language: &OSALanguage);

        #[cfg(feature = "OSAKit_OSALanguageInstance")]
        #[method_id(@__retain_semantics Other languageInstance)]
        pub unsafe fn languageInstance(&self) -> Id<OSALanguageInstance>;

        #[cfg(feature = "OSAKit_OSALanguageInstance")]
        #[method(setLanguageInstance:)]
        pub unsafe fn setLanguageInstance(&self, language_instance: &OSALanguageInstance);

        #[method(isCompiled)]
        pub unsafe fn isCompiled(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(compileAndReturnError:)]
        pub unsafe fn compileAndReturnError(
            &self,
            error_info: Option<&mut Option<Id<NSDictionary<NSString, Object>>>>,
        ) -> bool;

        #[cfg(all(
            feature = "Foundation_NSAppleEventDescriptor",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other executeAndReturnError:)]
        pub unsafe fn executeAndReturnError(
            &self,
            error_info: Option<&mut Option<Id<NSDictionary<NSString, Object>>>>,
        ) -> Option<Id<NSAppleEventDescriptor>>;

        #[cfg(all(
            feature = "Foundation_NSAppleEventDescriptor",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other executeAppleEvent:error:)]
        pub unsafe fn executeAppleEvent_error(
            &self,
            event: &NSAppleEventDescriptor,
            error_info: Option<&mut Option<Id<NSDictionary<NSString, Object>>>>,
        ) -> Option<Id<NSAppleEventDescriptor>>;

        #[cfg(all(
            feature = "Foundation_NSAppleEventDescriptor",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other executeAndReturnDisplayValue:error:)]
        pub unsafe fn executeAndReturnDisplayValue_error(
            &self,
            display_value: &mut Option<Id<NSAttributedString>>,
            error_info: Option<&mut Option<Id<NSDictionary<NSString, Object>>>>,
        ) -> Option<Id<NSAppleEventDescriptor>>;

        #[cfg(all(
            feature = "Foundation_NSAppleEventDescriptor",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other executeHandlerWithName:arguments:error:)]
        pub unsafe fn executeHandlerWithName_arguments_error(
            &self,
            name: &NSString,
            arguments: &NSArray,
            error_info: Option<&mut Option<Id<NSDictionary<NSString, Object>>>>,
        ) -> Option<Id<NSAppleEventDescriptor>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other richTextSource)]
        pub unsafe fn richTextSource(&self) -> Option<Id<NSAttributedString>>;

        #[cfg(all(
            feature = "Foundation_NSAppleEventDescriptor",
            feature = "Foundation_NSAttributedString"
        ))]
        #[method_id(@__retain_semantics Other richTextFromDescriptor:)]
        pub unsafe fn richTextFromDescriptor(
            &self,
            descriptor: &NSAppleEventDescriptor,
        ) -> Option<Id<NSAttributedString>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(writeToURL:ofType:error:)]
        pub unsafe fn writeToURL_ofType_error(
            &self,
            url: &NSURL,
            r#type: &NSString,
            error_info: Option<&mut Option<Id<NSDictionary<NSString, Object>>>>,
        ) -> bool;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(writeToURL:ofType:usingStorageOptions:error:)]
        pub unsafe fn writeToURL_ofType_usingStorageOptions_error(
            &self,
            url: &NSURL,
            r#type: &NSString,
            storage_options: OSAStorageOptions,
            error_info: Option<&mut Option<Id<NSDictionary<NSString, Object>>>>,
        ) -> bool;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other compiledDataForType:usingStorageOptions:error:)]
        pub unsafe fn compiledDataForType_usingStorageOptions_error(
            &self,
            r#type: &NSString,
            storage_options: OSAStorageOptions,
            error_info: Option<&mut Option<Id<NSDictionary<NSString, Object>>>>,
        ) -> Option<Id<NSData>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "OSAKit_OSAScript")]
    unsafe impl OSAScript {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
