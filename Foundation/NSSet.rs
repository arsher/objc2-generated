//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + NSCoding> NSCoding for NSSet<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + IsIdCloneable> NSCopying for NSSet<ObjectType> {}

#[cfg(feature = "NSEnumerator")]
unsafe impl<ObjectType: ?Sized> NSFastEnumeration for NSSet<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + IsIdCloneable> NSMutableCopying for NSSet<ObjectType> {}

unsafe impl<ObjectType: ?Sized> NSObjectProtocol for NSSet<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + NSSecureCoding> NSSecureCoding for NSSet<ObjectType> {}

extern_methods!(
    unsafe impl<ObjectType: Message> NSSet<ObjectType> {
        #[method(count)]
        pub fn count(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other member:)]
        pub unsafe fn member(&self, object: &ObjectType) -> Option<Retained<ObjectType>>;

        #[cfg(feature = "NSEnumerator")]
        #[method_id(@__retain_semantics Other objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Retained<NSEnumerator<ObjectType>>;

        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithObjects:count:)]
        pub unsafe fn initWithObjects_count(
            this: Allocated<Self>,
            objects: *mut NonNull<ObjectType>,
            cnt: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<ObjectType: Message> NSSet<ObjectType> {
        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl<ObjectType: Message> DefaultRetained for NSSet<ObjectType> {
    #[inline]
    fn default_id() -> Retained<Self> {
        Self::new()
    }
}

extern_methods!(
    /// NSExtendedSet
    unsafe impl<ObjectType: Message> NSSet<ObjectType> {
        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other allObjects)]
        pub unsafe fn allObjects(&self) -> Retained<NSArray<ObjectType>>;

        #[method_id(@__retain_semantics Other anyObject)]
        pub unsafe fn anyObject(&self) -> Option<Retained<ObjectType>>;

        #[method(containsObject:)]
        pub fn containsObject(&self, an_object: &ObjectType) -> bool;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other description)]
        pub unsafe fn description(&self) -> Retained<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(
            &self,
            locale: Option<&AnyObject>,
        ) -> Retained<NSString>;

        #[method(intersectsSet:)]
        pub fn intersectsSet(&self, other_set: &NSSet<ObjectType>) -> bool;

        #[method(isEqualToSet:)]
        pub unsafe fn isEqualToSet(&self, other_set: &NSSet<ObjectType>) -> bool;

        #[method(isSubsetOfSet:)]
        pub fn isSubsetOfSet(&self, other_set: &NSSet<ObjectType>) -> bool;

        #[method(makeObjectsPerformSelector:)]
        pub unsafe fn makeObjectsPerformSelector(&self, a_selector: Sel);

        #[method(makeObjectsPerformSelector:withObject:)]
        pub unsafe fn makeObjectsPerformSelector_withObject(
            &self,
            a_selector: Sel,
            argument: Option<&AnyObject>,
        );

        #[method_id(@__retain_semantics Other setByAddingObject:)]
        pub unsafe fn setByAddingObject(
            &self,
            an_object: &ObjectType,
        ) -> Retained<NSSet<ObjectType>>;

        #[method_id(@__retain_semantics Other setByAddingObjectsFromSet:)]
        pub unsafe fn setByAddingObjectsFromSet(
            &self,
            other: &NSSet<ObjectType>,
        ) -> Retained<NSSet<ObjectType>>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other setByAddingObjectsFromArray:)]
        pub unsafe fn setByAddingObjectsFromArray(
            &self,
            other: &NSArray<ObjectType>,
        ) -> Retained<NSSet<ObjectType>>;

        #[cfg(feature = "block2")]
        #[method(enumerateObjectsUsingBlock:)]
        pub unsafe fn enumerateObjectsUsingBlock(
            &self,
            block: &block2::Block<dyn Fn(NonNull<ObjectType>, NonNull<Bool>) + '_>,
        );

        #[cfg(all(feature = "NSObjCRuntime", feature = "block2"))]
        #[method(enumerateObjectsWithOptions:usingBlock:)]
        pub unsafe fn enumerateObjectsWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: &block2::Block<dyn Fn(NonNull<ObjectType>, NonNull<Bool>) + '_>,
        );

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other objectsPassingTest:)]
        pub unsafe fn objectsPassingTest(
            &self,
            predicate: &block2::Block<dyn Fn(NonNull<ObjectType>, NonNull<Bool>) -> Bool + '_>,
        ) -> Retained<NSSet<ObjectType>>;

        #[cfg(all(feature = "NSObjCRuntime", feature = "block2"))]
        #[method_id(@__retain_semantics Other objectsWithOptions:passingTest:)]
        pub unsafe fn objectsWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: &block2::Block<dyn Fn(NonNull<ObjectType>, NonNull<Bool>) -> Bool + '_>,
        ) -> Retained<NSSet<ObjectType>>;
    }
);

extern_methods!(
    /// NSSetCreation
    unsafe impl<ObjectType: Message> NSSet<ObjectType> {
        #[method_id(@__retain_semantics Other set)]
        pub unsafe fn set() -> Retained<Self>;

        #[method_id(@__retain_semantics Other setWithObject:)]
        pub unsafe fn setWithObject(object: &ObjectType) -> Retained<Self>;

        #[method_id(@__retain_semantics Other setWithObjects:count:)]
        pub unsafe fn setWithObjects_count(
            objects: NonNull<NonNull<ObjectType>>,
            cnt: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other setWithSet:)]
        pub unsafe fn setWithSet(set: &NSSet<ObjectType>) -> Retained<Self>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other setWithArray:)]
        pub unsafe fn setWithArray(array: &NSArray<ObjectType>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithSet:)]
        pub unsafe fn initWithSet(this: Allocated<Self>, set: &NSSet<ObjectType>)
            -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithSet:copyItems:)]
        pub unsafe fn initWithSet_copyItems(
            this: Allocated<Self>,
            set: &NSSet<ObjectType>,
            flag: bool,
        ) -> Retained<Self>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Init initWithArray:)]
        pub unsafe fn initWithArray(
            this: Allocated<Self>,
            array: &NSArray<ObjectType>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSSet`
    ///
    /// NSSetCreation
    unsafe impl<ObjectType: Message> NSMutableSet<ObjectType> {
        #[method_id(@__retain_semantics Other set)]
        pub unsafe fn set() -> Retained<Self>;

        #[method_id(@__retain_semantics Other setWithObject:)]
        pub unsafe fn setWithObject(object: &ObjectType) -> Retained<Self>;

        #[method_id(@__retain_semantics Other setWithObjects:count:)]
        pub unsafe fn setWithObjects_count(
            objects: NonNull<NonNull<ObjectType>>,
            cnt: NSUInteger,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other setWithSet:)]
        pub unsafe fn setWithSet(set: &NSSet<ObjectType>) -> Retained<Self>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other setWithArray:)]
        pub unsafe fn setWithArray(array: &NSArray<ObjectType>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithSet:)]
        pub unsafe fn initWithSet(this: Allocated<Self>, set: &NSSet<ObjectType>)
            -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithSet:copyItems:)]
        pub unsafe fn initWithSet_copyItems(
            this: Allocated<Self>,
            set: &NSSet<ObjectType>,
            flag: bool,
        ) -> Retained<Self>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Init initWithArray:)]
        pub unsafe fn initWithArray(
            this: Allocated<Self>,
            array: &NSArray<ObjectType>,
        ) -> Retained<Self>;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + NSCoding> NSCoding for NSMutableSet<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + IsIdCloneable> NSCopying for NSMutableSet<ObjectType> {}

#[cfg(feature = "NSEnumerator")]
unsafe impl<ObjectType: ?Sized> NSFastEnumeration for NSMutableSet<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + IsIdCloneable> NSMutableCopying for NSMutableSet<ObjectType> {}

unsafe impl<ObjectType: ?Sized> NSObjectProtocol for NSMutableSet<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + NSSecureCoding> NSSecureCoding for NSMutableSet<ObjectType> {}

extern_methods!(
    unsafe impl<ObjectType: Message> NSMutableSet<ObjectType> {
        #[method(addObject:)]
        pub unsafe fn addObject(&mut self, object: &ObjectType);

        #[method(removeObject:)]
        pub unsafe fn removeObject(&mut self, object: &ObjectType);

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCapacity:)]
        pub unsafe fn initWithCapacity(
            this: Allocated<Self>,
            num_items: NSUInteger,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSSet`
    unsafe impl<ObjectType: Message> NSMutableSet<ObjectType> {
        #[method_id(@__retain_semantics Init initWithObjects:count:)]
        pub unsafe fn initWithObjects_count(
            this: Allocated<Self>,
            objects: *mut NonNull<ObjectType>,
            cnt: NSUInteger,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<ObjectType: Message> NSMutableSet<ObjectType> {
        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Retained<Self>;
    }
);

impl<ObjectType: Message> DefaultRetained for NSMutableSet<ObjectType> {
    #[inline]
    fn default_id() -> Retained<Self> {
        Self::new()
    }
}

extern_methods!(
    /// NSExtendedMutableSet
    unsafe impl<ObjectType: Message> NSMutableSet<ObjectType> {
        #[cfg(feature = "NSArray")]
        #[method(addObjectsFromArray:)]
        pub unsafe fn addObjectsFromArray(&mut self, array: &NSArray<ObjectType>);

        #[method(intersectSet:)]
        pub unsafe fn intersectSet(&mut self, other_set: &NSSet<ObjectType>);

        #[method(minusSet:)]
        pub unsafe fn minusSet(&mut self, other_set: &NSSet<ObjectType>);

        #[method(removeAllObjects)]
        pub fn removeAllObjects(&mut self);

        #[method(unionSet:)]
        pub unsafe fn unionSet(&mut self, other_set: &NSSet<ObjectType>);

        #[method(setSet:)]
        pub unsafe fn setSet(&mut self, other_set: &NSSet<ObjectType>);
    }
);

extern_methods!(
    /// NSMutableSetCreation
    unsafe impl<ObjectType: Message> NSMutableSet<ObjectType> {
        #[method_id(@__retain_semantics Other setWithCapacity:)]
        pub unsafe fn setWithCapacity(num_items: NSUInteger) -> Retained<Self>;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + NSCoding> NSCoding for NSCountedSet<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + IsIdCloneable> NSCopying for NSCountedSet<ObjectType> {}

#[cfg(feature = "NSEnumerator")]
unsafe impl<ObjectType: ?Sized> NSFastEnumeration for NSCountedSet<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + IsIdCloneable> NSMutableCopying for NSCountedSet<ObjectType> {}

unsafe impl<ObjectType: ?Sized> NSObjectProtocol for NSCountedSet<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + NSSecureCoding> NSSecureCoding for NSCountedSet<ObjectType> {}

extern_methods!(
    unsafe impl<ObjectType: Message> NSCountedSet<ObjectType> {
        #[method_id(@__retain_semantics Init initWithCapacity:)]
        pub unsafe fn initWithCapacity(
            this: Allocated<Self>,
            num_items: NSUInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Init initWithArray:)]
        pub unsafe fn initWithArray(
            this: Allocated<Self>,
            array: &NSArray<ObjectType>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithSet:)]
        pub unsafe fn initWithSet(this: Allocated<Self>, set: &NSSet<ObjectType>)
            -> Retained<Self>;

        #[method(countForObject:)]
        pub unsafe fn countForObject(&self, object: &ObjectType) -> NSUInteger;

        #[cfg(feature = "NSEnumerator")]
        #[method_id(@__retain_semantics Other objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Retained<NSEnumerator<ObjectType>>;

        #[method(addObject:)]
        pub unsafe fn addObject(&self, object: &ObjectType);

        #[method(removeObject:)]
        pub unsafe fn removeObject(&self, object: &ObjectType);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSMutableSet`
    unsafe impl<ObjectType: Message> NSCountedSet<ObjectType> {
        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSSet`
    unsafe impl<ObjectType: Message> NSCountedSet<ObjectType> {
        #[method_id(@__retain_semantics Init initWithObjects:count:)]
        pub unsafe fn initWithObjects_count(
            this: Allocated<Self>,
            objects: *mut NonNull<ObjectType>,
            cnt: NSUInteger,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<ObjectType: Message> NSCountedSet<ObjectType> {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
