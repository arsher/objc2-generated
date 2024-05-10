//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + NSCoding> NSCoding for NSOrderedSet<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + IsIdCloneable> NSCopying for NSOrderedSet<ObjectType> {}

#[cfg(feature = "NSEnumerator")]
unsafe impl<ObjectType: ?Sized> NSFastEnumeration for NSOrderedSet<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + IsIdCloneable> NSMutableCopying for NSOrderedSet<ObjectType> {}

unsafe impl<ObjectType: ?Sized> NSObjectProtocol for NSOrderedSet<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + NSSecureCoding> NSSecureCoding for NSOrderedSet<ObjectType> {}

extern_methods!(
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other objectAtIndex:)]
        pub unsafe fn objectAtIndex(&self, idx: NSUInteger) -> Id<ObjectType>;

        #[method(indexOfObject:)]
        pub unsafe fn indexOfObject(&self, object: &ObjectType) -> NSUInteger;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithObjects:count:)]
        pub unsafe fn initWithObjects_count(
            this: Allocated<Self>,
            objects: *mut NonNull<ObjectType>,
            cnt: NSUInteger,
        ) -> Id<Self>;

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSExtendedOrderedSet
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[cfg(feature = "NSRange")]
        #[method(getObjects:range:)]
        pub unsafe fn getObjects_range(&self, objects: *mut NonNull<ObjectType>, range: NSRange);

        #[cfg(all(feature = "NSArray", feature = "NSIndexSet"))]
        #[method_id(@__retain_semantics Other objectsAtIndexes:)]
        pub unsafe fn objectsAtIndexes(&self, indexes: &NSIndexSet) -> Id<NSArray<ObjectType>>;

        #[method_id(@__retain_semantics Other firstObject)]
        pub unsafe fn firstObject(&self) -> Option<Id<ObjectType>>;

        #[method_id(@__retain_semantics Other lastObject)]
        pub unsafe fn lastObject(&self) -> Option<Id<ObjectType>>;

        #[method(isEqualToOrderedSet:)]
        pub unsafe fn isEqualToOrderedSet(&self, other: &NSOrderedSet<ObjectType>) -> bool;

        #[method(containsObject:)]
        pub unsafe fn containsObject(&self, object: &ObjectType) -> bool;

        #[method(intersectsOrderedSet:)]
        pub unsafe fn intersectsOrderedSet(&self, other: &NSOrderedSet<ObjectType>) -> bool;

        #[cfg(feature = "NSSet")]
        #[method(intersectsSet:)]
        pub unsafe fn intersectsSet(&self, set: &NSSet<ObjectType>) -> bool;

        #[method(isSubsetOfOrderedSet:)]
        pub unsafe fn isSubsetOfOrderedSet(&self, other: &NSOrderedSet<ObjectType>) -> bool;

        #[cfg(feature = "NSSet")]
        #[method(isSubsetOfSet:)]
        pub unsafe fn isSubsetOfSet(&self, set: &NSSet<ObjectType>) -> bool;

        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(&self, idx: NSUInteger) -> Id<ObjectType>;

        #[cfg(feature = "NSEnumerator")]
        #[method_id(@__retain_semantics Other objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Id<NSEnumerator<ObjectType>>;

        #[cfg(feature = "NSEnumerator")]
        #[method_id(@__retain_semantics Other reverseObjectEnumerator)]
        pub unsafe fn reverseObjectEnumerator(&self) -> Id<NSEnumerator<ObjectType>>;

        #[method_id(@__retain_semantics Other reversedOrderedSet)]
        pub unsafe fn reversedOrderedSet(&self) -> Id<NSOrderedSet<ObjectType>>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other array)]
        pub unsafe fn array(&self) -> Id<NSArray<ObjectType>>;

        #[cfg(feature = "NSSet")]
        #[method_id(@__retain_semantics Other set)]
        pub unsafe fn set(&self) -> Id<NSSet<ObjectType>>;

        #[cfg(feature = "block2")]
        #[method(enumerateObjectsUsingBlock:)]
        pub unsafe fn enumerateObjectsUsingBlock(
            &self,
            block: &block2::Block<dyn Fn(NonNull<ObjectType>, NSUInteger, NonNull<Bool>) + '_>,
        );

        #[cfg(all(feature = "NSObjCRuntime", feature = "block2"))]
        #[method(enumerateObjectsWithOptions:usingBlock:)]
        pub unsafe fn enumerateObjectsWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: &block2::Block<dyn Fn(NonNull<ObjectType>, NSUInteger, NonNull<Bool>) + '_>,
        );

        #[cfg(all(feature = "NSIndexSet", feature = "NSObjCRuntime", feature = "block2"))]
        #[method(enumerateObjectsAtIndexes:options:usingBlock:)]
        pub unsafe fn enumerateObjectsAtIndexes_options_usingBlock(
            &self,
            s: &NSIndexSet,
            opts: NSEnumerationOptions,
            block: &block2::Block<dyn Fn(NonNull<ObjectType>, NSUInteger, NonNull<Bool>) + '_>,
        );

        #[cfg(feature = "block2")]
        #[method(indexOfObjectPassingTest:)]
        pub unsafe fn indexOfObjectPassingTest(
            &self,
            predicate: &block2::Block<
                dyn Fn(NonNull<ObjectType>, NSUInteger, NonNull<Bool>) -> Bool + '_,
            >,
        ) -> NSUInteger;

        #[cfg(all(feature = "NSObjCRuntime", feature = "block2"))]
        #[method(indexOfObjectWithOptions:passingTest:)]
        pub unsafe fn indexOfObjectWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: &block2::Block<
                dyn Fn(NonNull<ObjectType>, NSUInteger, NonNull<Bool>) -> Bool + '_,
            >,
        ) -> NSUInteger;

        #[cfg(all(feature = "NSIndexSet", feature = "NSObjCRuntime", feature = "block2"))]
        #[method(indexOfObjectAtIndexes:options:passingTest:)]
        pub unsafe fn indexOfObjectAtIndexes_options_passingTest(
            &self,
            s: &NSIndexSet,
            opts: NSEnumerationOptions,
            predicate: &block2::Block<
                dyn Fn(NonNull<ObjectType>, NSUInteger, NonNull<Bool>) -> Bool + '_,
            >,
        ) -> NSUInteger;

        #[cfg(all(feature = "NSIndexSet", feature = "block2"))]
        #[method_id(@__retain_semantics Other indexesOfObjectsPassingTest:)]
        pub unsafe fn indexesOfObjectsPassingTest(
            &self,
            predicate: &block2::Block<
                dyn Fn(NonNull<ObjectType>, NSUInteger, NonNull<Bool>) -> Bool + '_,
            >,
        ) -> Id<NSIndexSet>;

        #[cfg(all(feature = "NSIndexSet", feature = "NSObjCRuntime", feature = "block2"))]
        #[method_id(@__retain_semantics Other indexesOfObjectsWithOptions:passingTest:)]
        pub unsafe fn indexesOfObjectsWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: &block2::Block<
                dyn Fn(NonNull<ObjectType>, NSUInteger, NonNull<Bool>) -> Bool + '_,
            >,
        ) -> Id<NSIndexSet>;

        #[cfg(all(feature = "NSIndexSet", feature = "NSObjCRuntime", feature = "block2"))]
        #[method_id(@__retain_semantics Other indexesOfObjectsAtIndexes:options:passingTest:)]
        pub unsafe fn indexesOfObjectsAtIndexes_options_passingTest(
            &self,
            s: &NSIndexSet,
            opts: NSEnumerationOptions,
            predicate: &block2::Block<
                dyn Fn(NonNull<ObjectType>, NSUInteger, NonNull<Bool>) -> Bool + '_,
            >,
        ) -> Id<NSIndexSet>;

        #[cfg(all(
            feature = "NSArray",
            feature = "NSObjCRuntime",
            feature = "NSRange",
            feature = "block2"
        ))]
        #[method(indexOfObject:inSortedRange:options:usingComparator:)]
        pub unsafe fn indexOfObject_inSortedRange_options_usingComparator(
            &self,
            object: &ObjectType,
            range: NSRange,
            opts: NSBinarySearchingOptions,
            cmp: NSComparator,
        ) -> NSUInteger;

        #[cfg(all(feature = "NSArray", feature = "NSObjCRuntime", feature = "block2"))]
        #[method_id(@__retain_semantics Other sortedArrayUsingComparator:)]
        pub unsafe fn sortedArrayUsingComparator(
            &self,
            cmptr: NSComparator,
        ) -> Id<NSArray<ObjectType>>;

        #[cfg(all(feature = "NSArray", feature = "NSObjCRuntime", feature = "block2"))]
        #[method_id(@__retain_semantics Other sortedArrayWithOptions:usingComparator:)]
        pub unsafe fn sortedArrayWithOptions_usingComparator(
            &self,
            opts: NSSortOptions,
            cmptr: NSComparator,
        ) -> Id<NSArray<ObjectType>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other description)]
        pub unsafe fn description(&self) -> Id<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(&self, locale: Option<&AnyObject>) -> Id<NSString>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other descriptionWithLocale:indent:)]
        pub unsafe fn descriptionWithLocale_indent(
            &self,
            locale: Option<&AnyObject>,
            level: NSUInteger,
        ) -> Id<NSString>;
    }
);

extern_methods!(
    /// NSOrderedSetCreation
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[method_id(@__retain_semantics Other orderedSet)]
        pub unsafe fn orderedSet() -> Id<Self>;

        #[method_id(@__retain_semantics Other orderedSetWithObject:)]
        pub unsafe fn orderedSetWithObject(object: &ObjectType) -> Id<Self>;

        #[method_id(@__retain_semantics Other orderedSetWithObjects:count:)]
        pub unsafe fn orderedSetWithObjects_count(
            objects: NonNull<NonNull<ObjectType>>,
            cnt: NSUInteger,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other orderedSetWithOrderedSet:)]
        pub unsafe fn orderedSetWithOrderedSet(set: &NSOrderedSet<ObjectType>) -> Id<Self>;

        #[cfg(feature = "NSRange")]
        #[method_id(@__retain_semantics Other orderedSetWithOrderedSet:range:copyItems:)]
        pub unsafe fn orderedSetWithOrderedSet_range_copyItems(
            set: &NSOrderedSet<ObjectType>,
            range: NSRange,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other orderedSetWithArray:)]
        pub unsafe fn orderedSetWithArray(array: &NSArray<ObjectType>) -> Id<Self>;

        #[cfg(all(feature = "NSArray", feature = "NSRange"))]
        #[method_id(@__retain_semantics Other orderedSetWithArray:range:copyItems:)]
        pub unsafe fn orderedSetWithArray_range_copyItems(
            array: &NSArray<ObjectType>,
            range: NSRange,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "NSSet")]
        #[method_id(@__retain_semantics Other orderedSetWithSet:)]
        pub unsafe fn orderedSetWithSet(set: &NSSet<ObjectType>) -> Id<Self>;

        #[cfg(feature = "NSSet")]
        #[method_id(@__retain_semantics Other orderedSetWithSet:copyItems:)]
        pub unsafe fn orderedSetWithSet_copyItems(set: &NSSet<ObjectType>, flag: bool) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithObject:)]
        pub unsafe fn initWithObject(this: Allocated<Self>, object: &ObjectType) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithOrderedSet:)]
        pub unsafe fn initWithOrderedSet(
            this: Allocated<Self>,
            set: &NSOrderedSet<ObjectType>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithOrderedSet:copyItems:)]
        pub unsafe fn initWithOrderedSet_copyItems(
            this: Allocated<Self>,
            set: &NSOrderedSet<ObjectType>,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "NSRange")]
        #[method_id(@__retain_semantics Init initWithOrderedSet:range:copyItems:)]
        pub unsafe fn initWithOrderedSet_range_copyItems(
            this: Allocated<Self>,
            set: &NSOrderedSet<ObjectType>,
            range: NSRange,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Init initWithArray:)]
        pub unsafe fn initWithArray(this: Allocated<Self>, array: &NSArray<ObjectType>)
            -> Id<Self>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Init initWithArray:copyItems:)]
        pub unsafe fn initWithArray_copyItems(
            this: Allocated<Self>,
            set: &NSArray<ObjectType>,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(all(feature = "NSArray", feature = "NSRange"))]
        #[method_id(@__retain_semantics Init initWithArray:range:copyItems:)]
        pub unsafe fn initWithArray_range_copyItems(
            this: Allocated<Self>,
            set: &NSArray<ObjectType>,
            range: NSRange,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "NSSet")]
        #[method_id(@__retain_semantics Init initWithSet:)]
        pub unsafe fn initWithSet(this: Allocated<Self>, set: &NSSet<ObjectType>) -> Id<Self>;

        #[cfg(feature = "NSSet")]
        #[method_id(@__retain_semantics Init initWithSet:copyItems:)]
        pub unsafe fn initWithSet_copyItems(
            this: Allocated<Self>,
            set: &NSSet<ObjectType>,
            flag: bool,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSOrderedSet`
    ///
    /// NSOrderedSetCreation
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[method_id(@__retain_semantics Other orderedSet)]
        pub unsafe fn orderedSet() -> Id<Self>;

        #[method_id(@__retain_semantics Other orderedSetWithObject:)]
        pub unsafe fn orderedSetWithObject(object: &ObjectType) -> Id<Self>;

        #[method_id(@__retain_semantics Other orderedSetWithObjects:count:)]
        pub unsafe fn orderedSetWithObjects_count(
            objects: NonNull<NonNull<ObjectType>>,
            cnt: NSUInteger,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other orderedSetWithOrderedSet:)]
        pub unsafe fn orderedSetWithOrderedSet(set: &NSOrderedSet<ObjectType>) -> Id<Self>;

        #[cfg(feature = "NSRange")]
        #[method_id(@__retain_semantics Other orderedSetWithOrderedSet:range:copyItems:)]
        pub unsafe fn orderedSetWithOrderedSet_range_copyItems(
            set: &NSOrderedSet<ObjectType>,
            range: NSRange,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Other orderedSetWithArray:)]
        pub unsafe fn orderedSetWithArray(array: &NSArray<ObjectType>) -> Id<Self>;

        #[cfg(all(feature = "NSArray", feature = "NSRange"))]
        #[method_id(@__retain_semantics Other orderedSetWithArray:range:copyItems:)]
        pub unsafe fn orderedSetWithArray_range_copyItems(
            array: &NSArray<ObjectType>,
            range: NSRange,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "NSSet")]
        #[method_id(@__retain_semantics Other orderedSetWithSet:)]
        pub unsafe fn orderedSetWithSet(set: &NSSet<ObjectType>) -> Id<Self>;

        #[cfg(feature = "NSSet")]
        #[method_id(@__retain_semantics Other orderedSetWithSet:copyItems:)]
        pub unsafe fn orderedSetWithSet_copyItems(set: &NSSet<ObjectType>, flag: bool) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithObject:)]
        pub unsafe fn initWithObject(this: Allocated<Self>, object: &ObjectType) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithOrderedSet:)]
        pub unsafe fn initWithOrderedSet(
            this: Allocated<Self>,
            set: &NSOrderedSet<ObjectType>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithOrderedSet:copyItems:)]
        pub unsafe fn initWithOrderedSet_copyItems(
            this: Allocated<Self>,
            set: &NSOrderedSet<ObjectType>,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "NSRange")]
        #[method_id(@__retain_semantics Init initWithOrderedSet:range:copyItems:)]
        pub unsafe fn initWithOrderedSet_range_copyItems(
            this: Allocated<Self>,
            set: &NSOrderedSet<ObjectType>,
            range: NSRange,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Init initWithArray:)]
        pub unsafe fn initWithArray(this: Allocated<Self>, array: &NSArray<ObjectType>)
            -> Id<Self>;

        #[cfg(feature = "NSArray")]
        #[method_id(@__retain_semantics Init initWithArray:copyItems:)]
        pub unsafe fn initWithArray_copyItems(
            this: Allocated<Self>,
            set: &NSArray<ObjectType>,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(all(feature = "NSArray", feature = "NSRange"))]
        #[method_id(@__retain_semantics Init initWithArray:range:copyItems:)]
        pub unsafe fn initWithArray_range_copyItems(
            this: Allocated<Self>,
            set: &NSArray<ObjectType>,
            range: NSRange,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "NSSet")]
        #[method_id(@__retain_semantics Init initWithSet:)]
        pub unsafe fn initWithSet(this: Allocated<Self>, set: &NSSet<ObjectType>) -> Id<Self>;

        #[cfg(feature = "NSSet")]
        #[method_id(@__retain_semantics Init initWithSet:copyItems:)]
        pub unsafe fn initWithSet_copyItems(
            this: Allocated<Self>,
            set: &NSSet<ObjectType>,
            flag: bool,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// NSOrderedSetDiffing
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[cfg(all(feature = "NSOrderedCollectionDifference", feature = "block2"))]
        #[method_id(@__retain_semantics Other differenceFromOrderedSet:withOptions:usingEquivalenceTest:)]
        pub unsafe fn differenceFromOrderedSet_withOptions_usingEquivalenceTest(
            &self,
            other: &NSOrderedSet<ObjectType>,
            options: NSOrderedCollectionDifferenceCalculationOptions,
            block: &block2::Block<dyn Fn(NonNull<ObjectType>, NonNull<ObjectType>) -> Bool + '_>,
        ) -> Id<NSOrderedCollectionDifference<ObjectType>>;

        #[cfg(feature = "NSOrderedCollectionDifference")]
        #[method_id(@__retain_semantics Other differenceFromOrderedSet:withOptions:)]
        pub unsafe fn differenceFromOrderedSet_withOptions(
            &self,
            other: &NSOrderedSet<ObjectType>,
            options: NSOrderedCollectionDifferenceCalculationOptions,
        ) -> Id<NSOrderedCollectionDifference<ObjectType>>;

        #[cfg(feature = "NSOrderedCollectionDifference")]
        #[method_id(@__retain_semantics Other differenceFromOrderedSet:)]
        pub unsafe fn differenceFromOrderedSet(
            &self,
            other: &NSOrderedSet<ObjectType>,
        ) -> Id<NSOrderedCollectionDifference<ObjectType>>;

        #[cfg(feature = "NSOrderedCollectionDifference")]
        #[method_id(@__retain_semantics Other orderedSetByApplyingDifference:)]
        pub unsafe fn orderedSetByApplyingDifference(
            &self,
            difference: &NSOrderedCollectionDifference<ObjectType>,
        ) -> Option<Id<NSOrderedSet<ObjectType>>>;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + NSCoding> NSCoding for NSMutableOrderedSet<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + IsIdCloneable> NSCopying for NSMutableOrderedSet<ObjectType> {}

#[cfg(feature = "NSEnumerator")]
unsafe impl<ObjectType: ?Sized> NSFastEnumeration for NSMutableOrderedSet<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + IsIdCloneable> NSMutableCopying
    for NSMutableOrderedSet<ObjectType>
{
}

unsafe impl<ObjectType: ?Sized> NSObjectProtocol for NSMutableOrderedSet<ObjectType> {}

#[cfg(feature = "NSObject")]
unsafe impl<ObjectType: ?Sized + NSSecureCoding> NSSecureCoding
    for NSMutableOrderedSet<ObjectType>
{
}

extern_methods!(
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[method(insertObject:atIndex:)]
        pub unsafe fn insertObject_atIndex(&mut self, object: &ObjectType, idx: NSUInteger);

        #[method(removeObjectAtIndex:)]
        pub unsafe fn removeObjectAtIndex(&mut self, idx: NSUInteger);

        #[method(replaceObjectAtIndex:withObject:)]
        pub unsafe fn replaceObjectAtIndex_withObject(
            &mut self,
            idx: NSUInteger,
            object: &ObjectType,
        );

        #[cfg(feature = "NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCapacity:)]
        pub unsafe fn initWithCapacity(this: Allocated<Self>, num_items: NSUInteger) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSOrderedSet`
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[method_id(@__retain_semantics Init initWithObjects:count:)]
        pub unsafe fn initWithObjects_count(
            this: Allocated<Self>,
            objects: *mut NonNull<ObjectType>,
            cnt: NSUInteger,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSExtendedMutableOrderedSet
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[method(addObject:)]
        pub unsafe fn addObject(&mut self, object: &ObjectType);

        #[method(addObjects:count:)]
        pub unsafe fn addObjects_count(
            &mut self,
            objects: *mut NonNull<ObjectType>,
            count: NSUInteger,
        );

        #[cfg(feature = "NSArray")]
        #[method(addObjectsFromArray:)]
        pub unsafe fn addObjectsFromArray(&mut self, array: &NSArray<ObjectType>);

        #[method(exchangeObjectAtIndex:withObjectAtIndex:)]
        pub unsafe fn exchangeObjectAtIndex_withObjectAtIndex(
            &mut self,
            idx1: NSUInteger,
            idx2: NSUInteger,
        );

        #[cfg(feature = "NSIndexSet")]
        #[method(moveObjectsAtIndexes:toIndex:)]
        pub unsafe fn moveObjectsAtIndexes_toIndex(
            &mut self,
            indexes: &NSIndexSet,
            idx: NSUInteger,
        );

        #[cfg(all(feature = "NSArray", feature = "NSIndexSet"))]
        #[method(insertObjects:atIndexes:)]
        pub unsafe fn insertObjects_atIndexes(
            &mut self,
            objects: &NSArray<ObjectType>,
            indexes: &NSIndexSet,
        );

        #[method(setObject:atIndex:)]
        pub unsafe fn setObject_atIndex(&mut self, obj: &ObjectType, idx: NSUInteger);

        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(&mut self, obj: &ObjectType, idx: NSUInteger);

        #[cfg(feature = "NSRange")]
        #[method(replaceObjectsInRange:withObjects:count:)]
        pub unsafe fn replaceObjectsInRange_withObjects_count(
            &mut self,
            range: NSRange,
            objects: *mut NonNull<ObjectType>,
            count: NSUInteger,
        );

        #[cfg(all(feature = "NSArray", feature = "NSIndexSet"))]
        #[method(replaceObjectsAtIndexes:withObjects:)]
        pub unsafe fn replaceObjectsAtIndexes_withObjects(
            &mut self,
            indexes: &NSIndexSet,
            objects: &NSArray<ObjectType>,
        );

        #[cfg(feature = "NSRange")]
        #[method(removeObjectsInRange:)]
        pub unsafe fn removeObjectsInRange(&mut self, range: NSRange);

        #[cfg(feature = "NSIndexSet")]
        #[method(removeObjectsAtIndexes:)]
        pub unsafe fn removeObjectsAtIndexes(&mut self, indexes: &NSIndexSet);

        #[method(removeAllObjects)]
        pub unsafe fn removeAllObjects(&mut self);

        #[method(removeObject:)]
        pub unsafe fn removeObject(&mut self, object: &ObjectType);

        #[cfg(feature = "NSArray")]
        #[method(removeObjectsInArray:)]
        pub unsafe fn removeObjectsInArray(&mut self, array: &NSArray<ObjectType>);

        #[method(intersectOrderedSet:)]
        pub unsafe fn intersectOrderedSet(&mut self, other: &NSOrderedSet<ObjectType>);

        #[method(minusOrderedSet:)]
        pub unsafe fn minusOrderedSet(&mut self, other: &NSOrderedSet<ObjectType>);

        #[method(unionOrderedSet:)]
        pub unsafe fn unionOrderedSet(&mut self, other: &NSOrderedSet<ObjectType>);

        #[cfg(feature = "NSSet")]
        #[method(intersectSet:)]
        pub unsafe fn intersectSet(&mut self, other: &NSSet<ObjectType>);

        #[cfg(feature = "NSSet")]
        #[method(minusSet:)]
        pub unsafe fn minusSet(&mut self, other: &NSSet<ObjectType>);

        #[cfg(feature = "NSSet")]
        #[method(unionSet:)]
        pub unsafe fn unionSet(&mut self, other: &NSSet<ObjectType>);

        #[cfg(all(feature = "NSObjCRuntime", feature = "block2"))]
        #[method(sortUsingComparator:)]
        pub unsafe fn sortUsingComparator(&mut self, cmptr: NSComparator);

        #[cfg(all(feature = "NSObjCRuntime", feature = "block2"))]
        #[method(sortWithOptions:usingComparator:)]
        pub unsafe fn sortWithOptions_usingComparator(
            &mut self,
            opts: NSSortOptions,
            cmptr: NSComparator,
        );

        #[cfg(all(feature = "NSObjCRuntime", feature = "NSRange", feature = "block2"))]
        #[method(sortRange:options:usingComparator:)]
        pub unsafe fn sortRange_options_usingComparator(
            &mut self,
            range: NSRange,
            opts: NSSortOptions,
            cmptr: NSComparator,
        );
    }
);

extern_methods!(
    /// NSMutableOrderedSetCreation
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[method_id(@__retain_semantics Other orderedSetWithCapacity:)]
        pub unsafe fn orderedSetWithCapacity(num_items: NSUInteger) -> Id<Self>;
    }
);

extern_methods!(
    /// NSMutableOrderedSetDiffing
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[cfg(feature = "NSOrderedCollectionDifference")]
        #[method(applyDifference:)]
        pub unsafe fn applyDifference(
            &mut self,
            difference: &NSOrderedCollectionDifference<ObjectType>,
        );
    }
);
