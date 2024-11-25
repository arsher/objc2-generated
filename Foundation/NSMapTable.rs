//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsmaptablestrongmemory?language=objc)
#[cfg(feature = "NSPointerFunctions")]
pub static NSMapTableStrongMemory: NSPointerFunctionsOptions =
    NSPointerFunctionsOptions(NSPointerFunctionsOptions::NSPointerFunctionsStrongMemory.0);

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsmaptablezeroingweakmemory?language=objc)
#[cfg(feature = "NSPointerFunctions")]
pub static NSMapTableZeroingWeakMemory: NSPointerFunctionsOptions =
    NSPointerFunctionsOptions(NSPointerFunctionsOptions::NSPointerFunctionsZeroingWeakMemory.0);

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsmaptablecopyin?language=objc)
#[cfg(feature = "NSPointerFunctions")]
pub static NSMapTableCopyIn: NSPointerFunctionsOptions =
    NSPointerFunctionsOptions(NSPointerFunctionsOptions::NSPointerFunctionsCopyIn.0);

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsmaptableobjectpointerpersonality?language=objc)
#[cfg(feature = "NSPointerFunctions")]
pub static NSMapTableObjectPointerPersonality: NSPointerFunctionsOptions =
    NSPointerFunctionsOptions(
        NSPointerFunctionsOptions::NSPointerFunctionsObjectPointerPersonality.0,
    );

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsmaptableweakmemory?language=objc)
#[cfg(feature = "NSPointerFunctions")]
pub static NSMapTableWeakMemory: NSPointerFunctionsOptions =
    NSPointerFunctionsOptions(NSPointerFunctionsOptions::NSPointerFunctionsWeakMemory.0);

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsmaptableoptions?language=objc)
pub type NSMapTableOptions = NSUInteger;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsmaptable?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMapTable<KeyType: ?Sized = AnyObject, ObjectType: ?Sized = AnyObject>;
);

#[cfg(feature = "NSObject")]
unsafe impl<KeyType: ?Sized + NSCoding, ObjectType: ?Sized + NSCoding> NSCoding
    for NSMapTable<KeyType, ObjectType>
{
}

#[cfg(feature = "NSObject")]
unsafe impl<KeyType: ?Sized, ObjectType: ?Sized> NSCopying for NSMapTable<KeyType, ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<KeyType: ?Sized + Message, ObjectType: ?Sized + Message> CopyingHelper
    for NSMapTable<KeyType, ObjectType>
{
    type Result = Self;
}

#[cfg(feature = "NSEnumerator")]
unsafe impl<KeyType: ?Sized, ObjectType: ?Sized> NSFastEnumeration
    for NSMapTable<KeyType, ObjectType>
{
}

unsafe impl<KeyType: ?Sized, ObjectType: ?Sized> NSObjectProtocol
    for NSMapTable<KeyType, ObjectType>
{
}

#[cfg(feature = "NSObject")]
unsafe impl<KeyType: ?Sized + NSSecureCoding, ObjectType: ?Sized + NSSecureCoding> NSSecureCoding
    for NSMapTable<KeyType, ObjectType>
{
}

extern_methods!(
    unsafe impl<KeyType: Message, ObjectType: Message> NSMapTable<KeyType, ObjectType> {
        #[cfg(feature = "NSPointerFunctions")]
        #[method_id(@__retain_semantics Init initWithKeyOptions:valueOptions:capacity:)]
        pub unsafe fn initWithKeyOptions_valueOptions_capacity(
            this: Allocated<Self>,
            key_options: NSPointerFunctionsOptions,
            value_options: NSPointerFunctionsOptions,
            initial_capacity: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "NSPointerFunctions")]
        #[method_id(@__retain_semantics Init initWithKeyPointerFunctions:valuePointerFunctions:capacity:)]
        pub unsafe fn initWithKeyPointerFunctions_valuePointerFunctions_capacity(
            this: Allocated<Self>,
            key_functions: &NSPointerFunctions,
            value_functions: &NSPointerFunctions,
            initial_capacity: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "NSPointerFunctions")]
        #[method_id(@__retain_semantics Other mapTableWithKeyOptions:valueOptions:)]
        pub unsafe fn mapTableWithKeyOptions_valueOptions(
            key_options: NSPointerFunctionsOptions,
            value_options: NSPointerFunctionsOptions,
        ) -> Retained<NSMapTable<KeyType, ObjectType>>;

        #[deprecated = "GC no longer supported"]
        #[method_id(@__retain_semantics Other mapTableWithStrongToStrongObjects)]
        pub unsafe fn mapTableWithStrongToStrongObjects() -> Retained<AnyObject>;

        #[deprecated = "GC no longer supported"]
        #[method_id(@__retain_semantics Other mapTableWithWeakToStrongObjects)]
        pub unsafe fn mapTableWithWeakToStrongObjects() -> Retained<AnyObject>;

        #[deprecated = "GC no longer supported"]
        #[method_id(@__retain_semantics Other mapTableWithStrongToWeakObjects)]
        pub unsafe fn mapTableWithStrongToWeakObjects() -> Retained<AnyObject>;

        #[deprecated = "GC no longer supported"]
        #[method_id(@__retain_semantics Other mapTableWithWeakToWeakObjects)]
        pub unsafe fn mapTableWithWeakToWeakObjects() -> Retained<AnyObject>;

        #[method_id(@__retain_semantics Other strongToStrongObjectsMapTable)]
        pub unsafe fn strongToStrongObjectsMapTable() -> Retained<NSMapTable<KeyType, ObjectType>>;

        #[method_id(@__retain_semantics Other weakToStrongObjectsMapTable)]
        pub unsafe fn weakToStrongObjectsMapTable() -> Retained<NSMapTable<KeyType, ObjectType>>;

        #[method_id(@__retain_semantics Other strongToWeakObjectsMapTable)]
        pub unsafe fn strongToWeakObjectsMapTable() -> Retained<NSMapTable<KeyType, ObjectType>>;

        #[method_id(@__retain_semantics Other weakToWeakObjectsMapTable)]
        pub unsafe fn weakToWeakObjectsMapTable() -> Retained<NSMapTable<KeyType, ObjectType>>;

        #[cfg(feature = "NSPointerFunctions")]
        #[method_id(@__retain_semantics Other keyPointerFunctions)]
        pub unsafe fn keyPointerFunctions(&self) -> Retained<NSPointerFunctions>;

        #[cfg(feature = "NSPointerFunctions")]
        #[method_id(@__retain_semantics Other valuePointerFunctions)]
        pub unsafe fn valuePointerFunctions(&self) -> Retained<NSPointerFunctions>;

        #[method_id(@__retain_semantics Other objectForKey:)]
        pub unsafe fn objectForKey(&self, a_key: Option<&KeyType>) -> Option<Retained<ObjectType>>;

        #[method(removeObjectForKey:)]
        pub unsafe fn removeObjectForKey(&self, a_key: Option<&KeyType>);

        #[method(setObject:forKey:)]
        pub unsafe fn setObject_forKey(
            &self,
            an_object: Option<&ObjectType>,
            a_key: Option<&KeyType>,
        );

        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[cfg(feature = "NSEnumerator")]
        #[method_id(@__retain_semantics Other keyEnumerator)]
        pub unsafe fn keyEnumerator(&self) -> Retained<NSEnumerator<KeyType>>;

        #[cfg(feature = "NSEnumerator")]
        #[method_id(@__retain_semantics Other objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Option<Retained<NSEnumerator<ObjectType>>>;

        #[method(removeAllObjects)]
        pub unsafe fn removeAllObjects(&self);

        #[cfg(feature = "NSDictionary")]
        #[method_id(@__retain_semantics Other dictionaryRepresentation)]
        pub unsafe fn dictionaryRepresentation(
            &self,
        ) -> Retained<NSDictionary<KeyType, ObjectType>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<KeyType: Message, ObjectType: Message> NSMapTable<KeyType, ObjectType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsmapenumerator?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NSMapEnumerator {
    pub(crate) _pi: NSUInteger,
    pub(crate) _si: NSUInteger,
    pub(crate) _bs: *mut c_void,
}

unsafe impl Encode for NSMapEnumerator {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <NSUInteger>::ENCODING,
            <NSUInteger>::ENCODING,
            <*mut c_void>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for NSMapEnumerator {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    pub fn NSResetMapTable(table: &NSMapTable);
}

extern "C-unwind" {
    pub fn NSCompareMapTables(table1: &NSMapTable, table2: &NSMapTable) -> Bool;
}

extern "C-unwind" {
    #[cfg(feature = "NSZone")]
    pub fn NSCopyMapTableWithZone(table: &NSMapTable, zone: *mut NSZone) -> NonNull<NSMapTable>;
}

extern "C-unwind" {
    pub fn NSMapMember(
        table: &NSMapTable,
        key: NonNull<c_void>,
        original_key: *mut *mut c_void,
        value: *mut *mut c_void,
    ) -> Bool;
}

extern "C-unwind" {
    pub fn NSMapGet(table: &NSMapTable, key: *mut c_void) -> *mut c_void;
}

extern "C-unwind" {
    pub fn NSMapInsert(table: &NSMapTable, key: *mut c_void, value: *mut c_void);
}

extern "C-unwind" {
    pub fn NSMapInsertKnownAbsent(table: &NSMapTable, key: *mut c_void, value: *mut c_void);
}

extern "C-unwind" {
    pub fn NSMapInsertIfAbsent(
        table: &NSMapTable,
        key: *mut c_void,
        value: *mut c_void,
    ) -> *mut c_void;
}

extern "C-unwind" {
    pub fn NSMapRemove(table: &NSMapTable, key: *mut c_void);
}

extern "C-unwind" {
    pub fn NSEnumerateMapTable(table: &NSMapTable) -> NSMapEnumerator;
}

extern "C-unwind" {
    pub fn NSNextMapEnumeratorPair(
        enumerator: NonNull<NSMapEnumerator>,
        key: *mut *mut c_void,
        value: *mut *mut c_void,
    ) -> Bool;
}

extern "C-unwind" {
    pub fn NSEndMapTableEnumeration(enumerator: NonNull<NSMapEnumerator>);
}

extern "C-unwind" {
    pub fn NSCountMapTable(table: &NSMapTable) -> NSUInteger;
}

extern "C-unwind" {
    #[cfg(feature = "NSString")]
    pub fn NSStringFromMapTable(table: &NSMapTable) -> NonNull<NSString>;
}

extern "C-unwind" {
    #[cfg(feature = "NSArray")]
    pub fn NSAllMapTableKeys(table: &NSMapTable) -> NonNull<NSArray>;
}

extern "C-unwind" {
    #[cfg(feature = "NSArray")]
    pub fn NSAllMapTableValues(table: &NSMapTable) -> NonNull<NSArray>;
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsmaptablekeycallbacks?language=objc)
#[cfg(feature = "NSString")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NSMapTableKeyCallBacks {
    pub hash:
        Option<unsafe extern "C-unwind" fn(NonNull<NSMapTable>, NonNull<c_void>) -> NSUInteger>,
    pub isEqual: Option<
        unsafe extern "C-unwind" fn(NonNull<NSMapTable>, NonNull<c_void>, NonNull<c_void>) -> Bool,
    >,
    pub retain: Option<unsafe extern "C-unwind" fn(NonNull<NSMapTable>, NonNull<c_void>)>,
    pub release: Option<unsafe extern "C-unwind" fn(NonNull<NSMapTable>, NonNull<c_void>)>,
    pub describe:
        Option<unsafe extern "C-unwind" fn(NonNull<NSMapTable>, NonNull<c_void>) -> *mut NSString>,
    pub notAKeyMarker: *mut c_void,
}

#[cfg(feature = "NSString")]
unsafe impl Encode for NSMapTableKeyCallBacks {
    const ENCODING: Encoding = Encoding::Struct("?", &[<Option<unsafe extern "C-unwind" fn(NonNull<NSMapTable>,NonNull<c_void>,) -> NSUInteger>>::ENCODING,<Option<unsafe extern "C-unwind" fn(NonNull<NSMapTable>,NonNull<c_void>,NonNull<c_void>,) -> Bool>>::ENCODING,<Option<unsafe extern "C-unwind" fn(NonNull<NSMapTable>,NonNull<c_void>,)>>::ENCODING,<Option<unsafe extern "C-unwind" fn(NonNull<NSMapTable>,NonNull<c_void>,)>>::ENCODING,<Option<unsafe extern "C-unwind" fn(NonNull<NSMapTable>,NonNull<c_void>,) -> *mut NSString>>::ENCODING,<*mut c_void>::ENCODING,]);
}

#[cfg(feature = "NSString")]
unsafe impl RefEncode for NSMapTableKeyCallBacks {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsmaptablevaluecallbacks?language=objc)
#[cfg(feature = "NSString")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NSMapTableValueCallBacks {
    pub retain: Option<unsafe extern "C-unwind" fn(NonNull<NSMapTable>, NonNull<c_void>)>,
    pub release: Option<unsafe extern "C-unwind" fn(NonNull<NSMapTable>, NonNull<c_void>)>,
    pub describe:
        Option<unsafe extern "C-unwind" fn(NonNull<NSMapTable>, NonNull<c_void>) -> *mut NSString>,
}

#[cfg(feature = "NSString")]
unsafe impl Encode for NSMapTableValueCallBacks {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <Option<unsafe extern "C-unwind" fn(NonNull<NSMapTable>, NonNull<c_void>)>>::ENCODING,
            <Option<unsafe extern "C-unwind" fn(NonNull<NSMapTable>, NonNull<c_void>)>>::ENCODING,
            <Option<
                unsafe extern "C-unwind" fn(NonNull<NSMapTable>, NonNull<c_void>) -> *mut NSString,
            >>::ENCODING,
        ],
    );
}

#[cfg(feature = "NSString")]
unsafe impl RefEncode for NSMapTableValueCallBacks {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    #[cfg(all(feature = "NSString", feature = "NSZone"))]
    pub fn NSCreateMapTableWithZone(
        key_call_backs: NSMapTableKeyCallBacks,
        value_call_backs: NSMapTableValueCallBacks,
        capacity: NSUInteger,
        zone: *mut NSZone,
    ) -> NonNull<NSMapTable>;
}

extern "C-unwind" {
    #[cfg(feature = "NSString")]
    pub fn NSCreateMapTable(
        key_call_backs: NSMapTableKeyCallBacks,
        value_call_backs: NSMapTableValueCallBacks,
        capacity: NSUInteger,
    ) -> NonNull<NSMapTable>;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsintegermapkeycallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSIntegerMapKeyCallBacks: NSMapTableKeyCallBacks;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnonownedpointermapkeycallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSNonOwnedPointerMapKeyCallBacks: NSMapTableKeyCallBacks;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnonownedpointerornullmapkeycallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSNonOwnedPointerOrNullMapKeyCallBacks: NSMapTableKeyCallBacks;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnonretainedobjectmapkeycallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSNonRetainedObjectMapKeyCallBacks: NSMapTableKeyCallBacks;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsobjectmapkeycallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSObjectMapKeyCallBacks: NSMapTableKeyCallBacks;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsownedpointermapkeycallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSOwnedPointerMapKeyCallBacks: NSMapTableKeyCallBacks;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsintmapkeycallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSIntMapKeyCallBacks: NSMapTableKeyCallBacks;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsintegermapvaluecallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSIntegerMapValueCallBacks: NSMapTableValueCallBacks;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnonownedpointermapvaluecallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSNonOwnedPointerMapValueCallBacks: NSMapTableValueCallBacks;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsobjectmapvaluecallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSObjectMapValueCallBacks: NSMapTableValueCallBacks;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnonretainedobjectmapvaluecallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSNonRetainedObjectMapValueCallBacks: NSMapTableValueCallBacks;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsownedpointermapvaluecallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSOwnedPointerMapValueCallBacks: NSMapTableValueCallBacks;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsintmapvaluecallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSIntMapValueCallBacks: NSMapTableValueCallBacks;
}
