//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSDictionaryControllerKeyValuePair")]
    pub struct NSDictionaryControllerKeyValuePair;

    #[cfg(feature = "AppKit_NSDictionaryControllerKeyValuePair")]
    unsafe impl ClassType for NSDictionaryControllerKeyValuePair {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSDictionaryControllerKeyValuePair")]
unsafe impl NSObjectProtocol for NSDictionaryControllerKeyValuePair {}

extern_methods!(
    #[cfg(feature = "AppKit_NSDictionaryControllerKeyValuePair")]
    unsafe impl NSDictionaryControllerKeyValuePair {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other key)]
        pub unsafe fn key(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setKey:)]
        pub unsafe fn setKey(&self, key: Option<&NSString>);

        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Option<Id<Object, Shared>>;

        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: Option<&Object>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedKey)]
        pub unsafe fn localizedKey(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLocalizedKey:)]
        pub unsafe fn setLocalizedKey(&self, localized_key: Option<&NSString>);

        #[method(isExplicitlyIncluded)]
        pub unsafe fn isExplicitlyIncluded(&self) -> bool;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSDictionaryController")]
    pub struct NSDictionaryController;

    #[cfg(feature = "AppKit_NSDictionaryController")]
    unsafe impl ClassType for NSDictionaryController {
        #[inherits(NSObjectController, NSController, NSObject)]
        type Super = NSArrayController;
    }
);

#[cfg(feature = "AppKit_NSDictionaryController")]
unsafe impl NSCoding for NSDictionaryController {}

#[cfg(feature = "AppKit_NSDictionaryController")]
unsafe impl NSEditor for NSDictionaryController {}

#[cfg(feature = "AppKit_NSDictionaryController")]
unsafe impl NSEditorRegistration for NSDictionaryController {}

#[cfg(feature = "AppKit_NSDictionaryController")]
unsafe impl NSObjectProtocol for NSDictionaryController {}

extern_methods!(
    #[cfg(feature = "AppKit_NSDictionaryController")]
    unsafe impl NSDictionaryController {
        #[cfg(feature = "AppKit_NSDictionaryControllerKeyValuePair")]
        #[method_id(@__retain_semantics New newObject)]
        pub unsafe fn newObject(&self) -> Id<NSDictionaryControllerKeyValuePair, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other initialKey)]
        pub unsafe fn initialKey(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setInitialKey:)]
        pub unsafe fn setInitialKey(&self, initial_key: &NSString);

        #[method_id(@__retain_semantics Other initialValue)]
        pub unsafe fn initialValue(&self) -> Id<Object, Shared>;

        #[method(setInitialValue:)]
        pub unsafe fn setInitialValue(&self, initial_value: &Object);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other includedKeys)]
        pub unsafe fn includedKeys(&self) -> Id<NSArray<NSString>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setIncludedKeys:)]
        pub unsafe fn setIncludedKeys(&self, included_keys: &NSArray<NSString>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other excludedKeys)]
        pub unsafe fn excludedKeys(&self) -> Id<NSArray<NSString>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setExcludedKeys:)]
        pub unsafe fn setExcludedKeys(&self, excluded_keys: &NSArray<NSString>);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other localizedKeyDictionary)]
        pub unsafe fn localizedKeyDictionary(&self)
            -> Id<NSDictionary<NSString, NSString>, Shared>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setLocalizedKeyDictionary:)]
        pub unsafe fn setLocalizedKeyDictionary(
            &self,
            localized_key_dictionary: &NSDictionary<NSString, NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedKeyTable)]
        pub unsafe fn localizedKeyTable(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLocalizedKeyTable:)]
        pub unsafe fn setLocalizedKeyTable(&self, localized_key_table: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObjectController`
    #[cfg(feature = "AppKit_NSDictionaryController")]
    unsafe impl NSDictionaryController {
        #[method_id(@__retain_semantics Init initWithContent:)]
        pub unsafe fn initWithContent(
            this: Option<Allocated<Self>>,
            content: Option<&Object>,
        ) -> Id<Self, Shared>;
    }
);
