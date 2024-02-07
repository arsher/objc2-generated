//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSUbiquitousKeyValueStore")]
    pub struct NSUbiquitousKeyValueStore;

    #[cfg(feature = "Foundation_NSUbiquitousKeyValueStore")]
    unsafe impl ClassType for NSUbiquitousKeyValueStore {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSUbiquitousKeyValueStore")]
unsafe impl NSObjectProtocol for NSUbiquitousKeyValueStore {}

extern_methods!(
    #[cfg(feature = "Foundation_NSUbiquitousKeyValueStore")]
    unsafe impl NSUbiquitousKeyValueStore {
        #[method_id(@__retain_semantics Other defaultStore)]
        pub unsafe fn defaultStore() -> Id<NSUbiquitousKeyValueStore>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other objectForKey:)]
        pub unsafe fn objectForKey(&self, a_key: &NSString) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setObject:forKey:)]
        pub unsafe fn setObject_forKey(&self, an_object: Option<&AnyObject>, a_key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(removeObjectForKey:)]
        pub unsafe fn removeObjectForKey(&self, a_key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringForKey:)]
        pub unsafe fn stringForKey(&self, a_key: &NSString) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other arrayForKey:)]
        pub unsafe fn arrayForKey(&self, a_key: &NSString) -> Option<Id<NSArray>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other dictionaryForKey:)]
        pub unsafe fn dictionaryForKey(
            &self,
            a_key: &NSString,
        ) -> Option<Id<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other dataForKey:)]
        pub unsafe fn dataForKey(&self, a_key: &NSString) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(longLongForKey:)]
        pub unsafe fn longLongForKey(&self, a_key: &NSString) -> c_longlong;

        #[cfg(feature = "Foundation_NSString")]
        #[method(doubleForKey:)]
        pub unsafe fn doubleForKey(&self, a_key: &NSString) -> c_double;

        #[cfg(feature = "Foundation_NSString")]
        #[method(boolForKey:)]
        pub unsafe fn boolForKey(&self, a_key: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setString:forKey:)]
        pub unsafe fn setString_forKey(&self, a_string: Option<&NSString>, a_key: &NSString);

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[method(setData:forKey:)]
        pub unsafe fn setData_forKey(&self, a_data: Option<&NSData>, a_key: &NSString);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setArray:forKey:)]
        pub unsafe fn setArray_forKey(&self, an_array: Option<&NSArray>, a_key: &NSString);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setDictionary:forKey:)]
        pub unsafe fn setDictionary_forKey(
            &self,
            a_dictionary: Option<&NSDictionary<NSString, AnyObject>>,
            a_key: &NSString,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLongLong:forKey:)]
        pub unsafe fn setLongLong_forKey(&self, value: c_longlong, a_key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDouble:forKey:)]
        pub unsafe fn setDouble_forKey(&self, value: c_double, a_key: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setBool:forKey:)]
        pub unsafe fn setBool_forKey(&self, value: bool, a_key: &NSString);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other dictionaryRepresentation)]
        pub unsafe fn dictionaryRepresentation(&self) -> Id<NSDictionary<NSString, AnyObject>>;

        #[method(synchronize)]
        pub unsafe fn synchronize(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSUbiquitousKeyValueStore")]
    unsafe impl NSUbiquitousKeyValueStore {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(NSUbiquitousKeyValueStoreDidChangeExternallyNotification: &'static NSNotificationName);

extern_static!(NSUbiquitousKeyValueStoreChangeReasonKey: &'static NSString);

extern_static!(NSUbiquitousKeyValueStoreChangedKeysKey: &'static NSString);

pub const NSUbiquitousKeyValueStoreServerChange: NSInteger = 0;
pub const NSUbiquitousKeyValueStoreInitialSyncChange: NSInteger = 1;
pub const NSUbiquitousKeyValueStoreQuotaViolationChange: NSInteger = 2;
pub const NSUbiquitousKeyValueStoreAccountChange: NSInteger = 3;
