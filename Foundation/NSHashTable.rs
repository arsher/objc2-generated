//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nshashtablestrongmemory?language=objc)
#[cfg(feature = "NSPointerFunctions")]
pub static NSHashTableStrongMemory: NSPointerFunctionsOptions =
    NSPointerFunctionsOptions(NSPointerFunctionsOptions::NSPointerFunctionsStrongMemory.0);

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nshashtablezeroingweakmemory?language=objc)
#[cfg(feature = "NSPointerFunctions")]
pub static NSHashTableZeroingWeakMemory: NSPointerFunctionsOptions =
    NSPointerFunctionsOptions(NSPointerFunctionsOptions::NSPointerFunctionsZeroingWeakMemory.0);

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nshashtablecopyin?language=objc)
#[cfg(feature = "NSPointerFunctions")]
pub static NSHashTableCopyIn: NSPointerFunctionsOptions =
    NSPointerFunctionsOptions(NSPointerFunctionsOptions::NSPointerFunctionsCopyIn.0);

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nshashtableobjectpointerpersonality?language=objc)
#[cfg(feature = "NSPointerFunctions")]
pub static NSHashTableObjectPointerPersonality: NSPointerFunctionsOptions =
    NSPointerFunctionsOptions(
        NSPointerFunctionsOptions::NSPointerFunctionsObjectPointerPersonality.0,
    );

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nshashtableweakmemory?language=objc)
#[cfg(feature = "NSPointerFunctions")]
pub static NSHashTableWeakMemory: NSPointerFunctionsOptions =
    NSPointerFunctionsOptions(NSPointerFunctionsOptions::NSPointerFunctionsWeakMemory.0);

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nshashtableoptions?language=objc)
pub type NSHashTableOptions = NSUInteger;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nshashtable?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSHashTable<ObjectType: ?Sized = AnyObject>;
);

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + NSCoding> NSCoding for NSHashTable<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized> NSCopying for NSHashTable<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + Message> CopyingHelper for NSHashTable<ObjectType> {
    type Result = Self;
}

#[cfg(feature = "NSEnumerator")]
unsafe impl<ObjectType: ?Sized> NSFastEnumeration for NSHashTable<ObjectType> {}

unsafe impl<ObjectType: ?Sized> NSObjectProtocol for NSHashTable<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + NSSecureCoding> NSSecureCoding for NSHashTable<ObjectType> {}

extern_methods!(
    unsafe impl<ObjectType: Message> NSHashTable<ObjectType> {
        #[cfg(feature = "NSPointerFunctions")]
        #[method_id(@__retain_semantics Init initWithOptions:capacity:)]
        pub unsafe fn initWithOptions_capacity(
            this: Allocated<Self>,
            options: NSPointerFunctionsOptions,
            initial_capacity: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "NSPointerFunctions")]
        #[method_id(@__retain_semantics Init initWithPointerFunctions:capacity:)]
        pub unsafe fn initWithPointerFunctions_capacity(
            this: Allocated<Self>,
            functions: &NSPointerFunctions,
            initial_capacity: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "NSPointerFunctions")]
        #[method_id(@__retain_semantics Other hashTableWithOptions:)]
        pub unsafe fn hashTableWithOptions(
            options: NSPointerFunctionsOptions,
        ) -> Retained<NSHashTable<ObjectType>>;

        #[deprecated = "GC no longer supported"]
        #[method_id(@__retain_semantics Other hashTableWithWeakObjects)]
        pub unsafe fn hashTableWithWeakObjects() -> Retained<AnyObject>;

        #[method_id(@__retain_semantics Other weakObjectsHashTable)]
        pub unsafe fn weakObjectsHashTable() -> Retained<NSHashTable<ObjectType>>;

        #[cfg(feature = "NSPointerFunctions")]
        #[method_id(@__retain_semantics Other pointerFunctions)]
        pub unsafe fn pointerFunctions(&self) -> Retained<NSPointerFunctions>;

        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other member:)]
        pub unsafe fn member(&self, object: Option<&ObjectType>) -> Option<Retained<ObjectType>>;

        #[cfg(feature = "NSEnumerator")]
        #[method_id(@__retain_semantics Other objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Retained<NSEnumerator<ObjectType>>;

        #[method(addObject:)]
        pub unsafe fn addObject(&self, object: Option<&ObjectType>);

        #[method(removeObject:)]
        pub unsafe fn removeObject(&self, object: Option<&ObjectType>);

        #[method(removeAllObjects)]
        pub unsafe fn removeAllObjects(&self);

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other allObjects)]
        pub unsafe fn allObjects(&self) -> Retained<NSArray<ObjectType>>;

        #[method_id(@__retain_semantics Other anyObject)]
        pub unsafe fn anyObject(&self) -> Option<Retained<ObjectType>>;

        #[method(containsObject:)]
        pub unsafe fn containsObject(&self, an_object: Option<&ObjectType>) -> bool;

        #[method(intersectsHashTable:)]
        pub unsafe fn intersectsHashTable(&self, other: &NSHashTable<ObjectType>) -> bool;

        #[method(isEqualToHashTable:)]
        pub unsafe fn isEqualToHashTable(&self, other: &NSHashTable<ObjectType>) -> bool;

        #[method(isSubsetOfHashTable:)]
        pub unsafe fn isSubsetOfHashTable(&self, other: &NSHashTable<ObjectType>) -> bool;

        #[method(intersectHashTable:)]
        pub unsafe fn intersectHashTable(&self, other: &NSHashTable<ObjectType>);

        #[method(unionHashTable:)]
        pub unsafe fn unionHashTable(&self, other: &NSHashTable<ObjectType>);

        #[method(minusHashTable:)]
        pub unsafe fn minusHashTable(&self, other: &NSHashTable<ObjectType>);

        #[cfg(feature = "NSSet")]
        #[method_id(@__retain_semantics Other setRepresentation)]
        pub unsafe fn setRepresentation(&self) -> Retained<NSSet<ObjectType>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<ObjectType: Message> NSHashTable<ObjectType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nshashenumerator?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NSHashEnumerator {
    pub(crate) _pi: NSUInteger,
    pub(crate) _si: NSUInteger,
    pub(crate) _bs: *mut c_void,
}

unsafe impl Encode for NSHashEnumerator {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <NSUInteger>::ENCODING,
            <NSUInteger>::ENCODING,
            <*mut c_void>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for NSHashEnumerator {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    pub fn NSFreeHashTable(table: &NSHashTable);
}

extern "C-unwind" {
    pub fn NSResetHashTable(table: &NSHashTable);
}

#[inline]
pub unsafe extern "C-unwind" fn NSCompareHashTables(
    table1: &NSHashTable,
    table2: &NSHashTable,
) -> bool {
    extern "C-unwind" {
        fn NSCompareHashTables(table1: &NSHashTable, table2: &NSHashTable) -> Bool;
    }
    unsafe { NSCompareHashTables(table1, table2) }.as_bool()
}

#[cfg(feature = "NSZone")]
#[inline]
pub unsafe extern "C-unwind" fn NSCopyHashTableWithZone(
    table: &NSHashTable,
    zone: *mut NSZone,
) -> Retained<NSHashTable> {
    extern "C-unwind" {
        fn NSCopyHashTableWithZone(table: &NSHashTable, zone: *mut NSZone) -> NonNull<NSHashTable>;
    }
    let ret = unsafe { NSCopyHashTableWithZone(table, zone) };
    unsafe { Retained::from_raw(ret.as_ptr()) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}

extern "C-unwind" {
    pub fn NSHashGet(table: &NSHashTable, pointer: *const c_void) -> NonNull<c_void>;
}

extern "C-unwind" {
    pub fn NSHashInsert(table: &NSHashTable, pointer: *const c_void);
}

extern "C-unwind" {
    pub fn NSHashInsertKnownAbsent(table: &NSHashTable, pointer: *const c_void);
}

extern "C-unwind" {
    pub fn NSHashInsertIfAbsent(table: &NSHashTable, pointer: *const c_void) -> *mut c_void;
}

extern "C-unwind" {
    pub fn NSHashRemove(table: &NSHashTable, pointer: *const c_void);
}

extern "C-unwind" {
    pub fn NSEnumerateHashTable(table: &NSHashTable) -> NSHashEnumerator;
}

extern "C-unwind" {
    pub fn NSNextHashEnumeratorItem(enumerator: NonNull<NSHashEnumerator>) -> *mut c_void;
}

extern "C-unwind" {
    pub fn NSEndHashTableEnumeration(enumerator: NonNull<NSHashEnumerator>);
}

extern "C-unwind" {
    pub fn NSCountHashTable(table: &NSHashTable) -> NSUInteger;
}

#[cfg(feature = "NSString")]
#[inline]
pub unsafe extern "C-unwind" fn NSStringFromHashTable(table: &NSHashTable) -> Retained<NSString> {
    extern "C-unwind" {
        fn NSStringFromHashTable(table: &NSHashTable) -> NonNull<NSString>;
    }
    let ret = unsafe { NSStringFromHashTable(table) };
    unsafe { Retained::retain_autoreleased(ret.as_ptr()) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}

#[cfg(feature = "NSArray")]
#[inline]
pub unsafe extern "C-unwind" fn NSAllHashTableObjects(table: &NSHashTable) -> Retained<NSArray> {
    extern "C-unwind" {
        fn NSAllHashTableObjects(table: &NSHashTable) -> NonNull<NSArray>;
    }
    let ret = unsafe { NSAllHashTableObjects(table) };
    unsafe { Retained::retain_autoreleased(ret.as_ptr()) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}

/// [Apple's documentation](https://developer.apple.com/documentation/foundation/nshashtablecallbacks?language=objc)
#[cfg(feature = "NSString")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NSHashTableCallBacks {
    pub hash:
        Option<unsafe extern "C-unwind" fn(NonNull<NSHashTable>, NonNull<c_void>) -> NSUInteger>,
    pub isEqual: Option<
        unsafe extern "C-unwind" fn(NonNull<NSHashTable>, NonNull<c_void>, NonNull<c_void>) -> Bool,
    >,
    pub retain: Option<unsafe extern "C-unwind" fn(NonNull<NSHashTable>, NonNull<c_void>)>,
    pub release: Option<unsafe extern "C-unwind" fn(NonNull<NSHashTable>, NonNull<c_void>)>,
    pub describe:
        Option<unsafe extern "C-unwind" fn(NonNull<NSHashTable>, NonNull<c_void>) -> *mut NSString>,
}

#[cfg(feature = "NSString")]
unsafe impl Encode for NSHashTableCallBacks {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <Option<
                unsafe extern "C-unwind" fn(NonNull<NSHashTable>, NonNull<c_void>) -> NSUInteger,
            >>::ENCODING,
            <Option<
                unsafe extern "C-unwind" fn(
                    NonNull<NSHashTable>,
                    NonNull<c_void>,
                    NonNull<c_void>,
                ) -> Bool,
            >>::ENCODING,
            <Option<unsafe extern "C-unwind" fn(NonNull<NSHashTable>, NonNull<c_void>)>>::ENCODING,
            <Option<unsafe extern "C-unwind" fn(NonNull<NSHashTable>, NonNull<c_void>)>>::ENCODING,
            <Option<
                unsafe extern "C-unwind" fn(NonNull<NSHashTable>, NonNull<c_void>) -> *mut NSString,
            >>::ENCODING,
        ],
    );
}

#[cfg(feature = "NSString")]
unsafe impl RefEncode for NSHashTableCallBacks {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(all(feature = "NSString", feature = "NSZone"))]
#[inline]
pub unsafe extern "C-unwind" fn NSCreateHashTableWithZone(
    call_backs: NSHashTableCallBacks,
    capacity: NSUInteger,
    zone: *mut NSZone,
) -> Retained<NSHashTable> {
    extern "C-unwind" {
        fn NSCreateHashTableWithZone(
            call_backs: NSHashTableCallBacks,
            capacity: NSUInteger,
            zone: *mut NSZone,
        ) -> NonNull<NSHashTable>;
    }
    let ret = unsafe { NSCreateHashTableWithZone(call_backs, capacity, zone) };
    unsafe { Retained::from_raw(ret.as_ptr()) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}

#[cfg(feature = "NSString")]
#[inline]
pub unsafe extern "C-unwind" fn NSCreateHashTable(
    call_backs: NSHashTableCallBacks,
    capacity: NSUInteger,
) -> Retained<NSHashTable> {
    extern "C-unwind" {
        fn NSCreateHashTable(
            call_backs: NSHashTableCallBacks,
            capacity: NSUInteger,
        ) -> NonNull<NSHashTable>;
    }
    let ret = unsafe { NSCreateHashTable(call_backs, capacity) };
    unsafe { Retained::from_raw(ret.as_ptr()) }
        .expect("function was marked as returning non-null, but actually returned NULL")
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsintegerhashcallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSIntegerHashCallBacks: NSHashTableCallBacks;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnonownedpointerhashcallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSNonOwnedPointerHashCallBacks: NSHashTableCallBacks;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsnonretainedobjecthashcallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSNonRetainedObjectHashCallBacks: NSHashTableCallBacks;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsobjecthashcallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSObjectHashCallBacks: NSHashTableCallBacks;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsownedobjectidentityhashcallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSOwnedObjectIdentityHashCallBacks: NSHashTableCallBacks;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsownedpointerhashcallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSOwnedPointerHashCallBacks: NSHashTableCallBacks;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nspointertostructhashcallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSPointerToStructHashCallBacks: NSHashTableCallBacks;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsinthashcallbacks?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSIntHashCallBacks: NSHashTableCallBacks;
}
