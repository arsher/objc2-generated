//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

#[cfg(feature = "Foundation_NSOrderedSet")]
unsafe impl<ObjectType: ?Sized + NSCoding> NSCoding for NSOrderedSet<ObjectType> {}

#[cfg(feature = "Foundation_NSOrderedSet")]
unsafe impl<ObjectType: ?Sized + IsIdCloneable> NSCopying for NSOrderedSet<ObjectType> {}

#[cfg(feature = "Foundation_NSOrderedSet")]
unsafe impl<ObjectType: ?Sized> NSFastEnumeration for NSOrderedSet<ObjectType> {}

#[cfg(feature = "Foundation_NSOrderedSet")]
unsafe impl<ObjectType: ?Sized + IsIdCloneable> NSMutableCopying for NSOrderedSet<ObjectType> {}

#[cfg(feature = "Foundation_NSOrderedSet")]
unsafe impl<ObjectType: ?Sized> NSObjectProtocol for NSOrderedSet<ObjectType> {}

#[cfg(feature = "Foundation_NSOrderedSet")]
unsafe impl<ObjectType: ?Sized + NSSecureCoding> NSSecureCoding for NSOrderedSet<ObjectType> {}

extern_methods!(
    #[cfg(feature = "Foundation_NSOrderedSet")]
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other objectAtIndex:)]
        pub unsafe fn objectAtIndex(&self, idx: NSUInteger) -> Id<ObjectType>;

        #[method(indexOfObject:)]
        pub unsafe fn indexOfObject(&self, object: &ObjectType) -> NSUInteger;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithObjects:count:)]
        pub unsafe fn initWithObjects_count(
            this: Option<Allocated<Self>>,
            objects: *mut NonNull<ObjectType>,
            cnt: NSUInteger,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSOrderedSet")]
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSExtendedOrderedSet
    #[cfg(feature = "Foundation_NSOrderedSet")]
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[method(getObjects:range:)]
        pub unsafe fn getObjects_range(&self, objects: *mut NonNull<ObjectType>, range: NSRange);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSIndexSet"))]
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

        #[cfg(feature = "Foundation_NSSet")]
        #[method(intersectsSet:)]
        pub unsafe fn intersectsSet(&self, set: &NSSet<ObjectType>) -> bool;

        #[method(isSubsetOfOrderedSet:)]
        pub unsafe fn isSubsetOfOrderedSet(&self, other: &NSOrderedSet<ObjectType>) -> bool;

        #[cfg(feature = "Foundation_NSSet")]
        #[method(isSubsetOfSet:)]
        pub unsafe fn isSubsetOfSet(&self, set: &NSSet<ObjectType>) -> bool;

        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(&self, idx: NSUInteger) -> Id<ObjectType>;

        #[cfg(feature = "Foundation_NSEnumerator")]
        #[method_id(@__retain_semantics Other objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Id<NSEnumerator<ObjectType>>;

        #[cfg(feature = "Foundation_NSEnumerator")]
        #[method_id(@__retain_semantics Other reverseObjectEnumerator)]
        pub unsafe fn reverseObjectEnumerator(&self) -> Id<NSEnumerator<ObjectType>>;

        #[method_id(@__retain_semantics Other reversedOrderedSet)]
        pub unsafe fn reversedOrderedSet(&self) -> Id<NSOrderedSet<ObjectType>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other array)]
        pub unsafe fn array(&self) -> Id<NSArray<ObjectType>>;

        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Other set)]
        pub unsafe fn set(&self) -> Id<NSSet<ObjectType>>;

        #[method(enumerateObjectsUsingBlock:)]
        pub unsafe fn enumerateObjectsUsingBlock(
            &self,
            block: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), ()>,
        );

        #[method(enumerateObjectsWithOptions:usingBlock:)]
        pub unsafe fn enumerateObjectsWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), ()>,
        );

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(enumerateObjectsAtIndexes:options:usingBlock:)]
        pub unsafe fn enumerateObjectsAtIndexes_options_usingBlock(
            &self,
            s: &NSIndexSet,
            opts: NSEnumerationOptions,
            block: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), ()>,
        );

        #[method(indexOfObjectPassingTest:)]
        pub unsafe fn indexOfObjectPassingTest(
            &self,
            predicate: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), Bool>,
        ) -> NSUInteger;

        #[method(indexOfObjectWithOptions:passingTest:)]
        pub unsafe fn indexOfObjectWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), Bool>,
        ) -> NSUInteger;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(indexOfObjectAtIndexes:options:passingTest:)]
        pub unsafe fn indexOfObjectAtIndexes_options_passingTest(
            &self,
            s: &NSIndexSet,
            opts: NSEnumerationOptions,
            predicate: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), Bool>,
        ) -> NSUInteger;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method_id(@__retain_semantics Other indexesOfObjectsPassingTest:)]
        pub unsafe fn indexesOfObjectsPassingTest(
            &self,
            predicate: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), Bool>,
        ) -> Id<NSIndexSet>;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method_id(@__retain_semantics Other indexesOfObjectsWithOptions:passingTest:)]
        pub unsafe fn indexesOfObjectsWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), Bool>,
        ) -> Id<NSIndexSet>;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method_id(@__retain_semantics Other indexesOfObjectsAtIndexes:options:passingTest:)]
        pub unsafe fn indexesOfObjectsAtIndexes_options_passingTest(
            &self,
            s: &NSIndexSet,
            opts: NSEnumerationOptions,
            predicate: &Block<(NonNull<ObjectType>, NSUInteger, NonNull<Bool>), Bool>,
        ) -> Id<NSIndexSet>;

        #[method(indexOfObject:inSortedRange:options:usingComparator:)]
        pub unsafe fn indexOfObject_inSortedRange_options_usingComparator(
            &self,
            object: &ObjectType,
            range: NSRange,
            opts: NSBinarySearchingOptions,
            cmp: NSComparator,
        ) -> NSUInteger;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other sortedArrayUsingComparator:)]
        pub unsafe fn sortedArrayUsingComparator(
            &self,
            cmptr: NSComparator,
        ) -> Id<NSArray<ObjectType>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other sortedArrayWithOptions:usingComparator:)]
        pub unsafe fn sortedArrayWithOptions_usingComparator(
            &self,
            opts: NSSortOptions,
            cmptr: NSComparator,
        ) -> Id<NSArray<ObjectType>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other description)]
        pub unsafe fn description(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(&self, locale: Option<&AnyObject>) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
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
    #[cfg(feature = "Foundation_NSOrderedSet")]
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

        #[method_id(@__retain_semantics Other orderedSetWithOrderedSet:range:copyItems:)]
        pub unsafe fn orderedSetWithOrderedSet_range_copyItems(
            set: &NSOrderedSet<ObjectType>,
            range: NSRange,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other orderedSetWithArray:)]
        pub unsafe fn orderedSetWithArray(array: &NSArray<ObjectType>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other orderedSetWithArray:range:copyItems:)]
        pub unsafe fn orderedSetWithArray_range_copyItems(
            array: &NSArray<ObjectType>,
            range: NSRange,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Other orderedSetWithSet:)]
        pub unsafe fn orderedSetWithSet(set: &NSSet<ObjectType>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Other orderedSetWithSet:copyItems:)]
        pub unsafe fn orderedSetWithSet_copyItems(set: &NSSet<ObjectType>, flag: bool) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithObject:)]
        pub unsafe fn initWithObject(
            this: Option<Allocated<Self>>,
            object: &ObjectType,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithOrderedSet:)]
        pub unsafe fn initWithOrderedSet(
            this: Option<Allocated<Self>>,
            set: &NSOrderedSet<ObjectType>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithOrderedSet:copyItems:)]
        pub unsafe fn initWithOrderedSet_copyItems(
            this: Option<Allocated<Self>>,
            set: &NSOrderedSet<ObjectType>,
            flag: bool,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithOrderedSet:range:copyItems:)]
        pub unsafe fn initWithOrderedSet_range_copyItems(
            this: Option<Allocated<Self>>,
            set: &NSOrderedSet<ObjectType>,
            range: NSRange,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Init initWithArray:)]
        pub unsafe fn initWithArray(
            this: Option<Allocated<Self>>,
            array: &NSArray<ObjectType>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Init initWithArray:copyItems:)]
        pub unsafe fn initWithArray_copyItems(
            this: Option<Allocated<Self>>,
            set: &NSArray<ObjectType>,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Init initWithArray:range:copyItems:)]
        pub unsafe fn initWithArray_range_copyItems(
            this: Option<Allocated<Self>>,
            set: &NSArray<ObjectType>,
            range: NSRange,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Init initWithSet:)]
        pub unsafe fn initWithSet(
            this: Option<Allocated<Self>>,
            set: &NSSet<ObjectType>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Init initWithSet:copyItems:)]
        pub unsafe fn initWithSet_copyItems(
            this: Option<Allocated<Self>>,
            set: &NSSet<ObjectType>,
            flag: bool,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSOrderedSet`
    ///
    /// NSOrderedSetCreation
    #[cfg(feature = "Foundation_NSMutableOrderedSet")]
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

        #[method_id(@__retain_semantics Other orderedSetWithOrderedSet:range:copyItems:)]
        pub unsafe fn orderedSetWithOrderedSet_range_copyItems(
            set: &NSOrderedSet<ObjectType>,
            range: NSRange,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other orderedSetWithArray:)]
        pub unsafe fn orderedSetWithArray(array: &NSArray<ObjectType>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other orderedSetWithArray:range:copyItems:)]
        pub unsafe fn orderedSetWithArray_range_copyItems(
            array: &NSArray<ObjectType>,
            range: NSRange,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Other orderedSetWithSet:)]
        pub unsafe fn orderedSetWithSet(set: &NSSet<ObjectType>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Other orderedSetWithSet:copyItems:)]
        pub unsafe fn orderedSetWithSet_copyItems(set: &NSSet<ObjectType>, flag: bool) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithObject:)]
        pub unsafe fn initWithObject(
            this: Option<Allocated<Self>>,
            object: &ObjectType,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithOrderedSet:)]
        pub unsafe fn initWithOrderedSet(
            this: Option<Allocated<Self>>,
            set: &NSOrderedSet<ObjectType>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithOrderedSet:copyItems:)]
        pub unsafe fn initWithOrderedSet_copyItems(
            this: Option<Allocated<Self>>,
            set: &NSOrderedSet<ObjectType>,
            flag: bool,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithOrderedSet:range:copyItems:)]
        pub unsafe fn initWithOrderedSet_range_copyItems(
            this: Option<Allocated<Self>>,
            set: &NSOrderedSet<ObjectType>,
            range: NSRange,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Init initWithArray:)]
        pub unsafe fn initWithArray(
            this: Option<Allocated<Self>>,
            array: &NSArray<ObjectType>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Init initWithArray:copyItems:)]
        pub unsafe fn initWithArray_copyItems(
            this: Option<Allocated<Self>>,
            set: &NSArray<ObjectType>,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Init initWithArray:range:copyItems:)]
        pub unsafe fn initWithArray_range_copyItems(
            this: Option<Allocated<Self>>,
            set: &NSArray<ObjectType>,
            range: NSRange,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Init initWithSet:)]
        pub unsafe fn initWithSet(
            this: Option<Allocated<Self>>,
            set: &NSSet<ObjectType>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Init initWithSet:copyItems:)]
        pub unsafe fn initWithSet_copyItems(
            this: Option<Allocated<Self>>,
            set: &NSSet<ObjectType>,
            flag: bool,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// NSOrderedSetDiffing
    #[cfg(feature = "Foundation_NSOrderedSet")]
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[cfg(feature = "Foundation_NSOrderedCollectionDifference")]
        #[method_id(@__retain_semantics Other differenceFromOrderedSet:withOptions:usingEquivalenceTest:)]
        pub unsafe fn differenceFromOrderedSet_withOptions_usingEquivalenceTest(
            &self,
            other: &NSOrderedSet<ObjectType>,
            options: NSOrderedCollectionDifferenceCalculationOptions,
            block: &Block<(NonNull<ObjectType>, NonNull<ObjectType>), Bool>,
        ) -> Id<NSOrderedCollectionDifference<ObjectType>>;

        #[cfg(feature = "Foundation_NSOrderedCollectionDifference")]
        #[method_id(@__retain_semantics Other differenceFromOrderedSet:withOptions:)]
        pub unsafe fn differenceFromOrderedSet_withOptions(
            &self,
            other: &NSOrderedSet<ObjectType>,
            options: NSOrderedCollectionDifferenceCalculationOptions,
        ) -> Id<NSOrderedCollectionDifference<ObjectType>>;

        #[cfg(feature = "Foundation_NSOrderedCollectionDifference")]
        #[method_id(@__retain_semantics Other differenceFromOrderedSet:)]
        pub unsafe fn differenceFromOrderedSet(
            &self,
            other: &NSOrderedSet<ObjectType>,
        ) -> Id<NSOrderedCollectionDifference<ObjectType>>;

        #[cfg(feature = "Foundation_NSOrderedCollectionDifference")]
        #[method_id(@__retain_semantics Other orderedSetByApplyingDifference:)]
        pub unsafe fn orderedSetByApplyingDifference(
            &self,
            difference: &NSOrderedCollectionDifference<ObjectType>,
        ) -> Option<Id<NSOrderedSet<ObjectType>>>;
    }
);

#[cfg(feature = "Foundation_NSMutableOrderedSet")]
unsafe impl<ObjectType: ?Sized + NSCoding> NSCoding for NSMutableOrderedSet<ObjectType> {}

#[cfg(feature = "Foundation_NSMutableOrderedSet")]
unsafe impl<ObjectType: ?Sized + IsIdCloneable> NSCopying for NSMutableOrderedSet<ObjectType> {}

#[cfg(feature = "Foundation_NSMutableOrderedSet")]
unsafe impl<ObjectType: ?Sized> NSFastEnumeration for NSMutableOrderedSet<ObjectType> {}

#[cfg(feature = "Foundation_NSMutableOrderedSet")]
unsafe impl<ObjectType: ?Sized + IsIdCloneable> NSMutableCopying
    for NSMutableOrderedSet<ObjectType>
{
}

#[cfg(feature = "Foundation_NSMutableOrderedSet")]
unsafe impl<ObjectType: ?Sized> NSObjectProtocol for NSMutableOrderedSet<ObjectType> {}

#[cfg(feature = "Foundation_NSMutableOrderedSet")]
unsafe impl<ObjectType: ?Sized + NSSecureCoding> NSSecureCoding
    for NSMutableOrderedSet<ObjectType>
{
}

extern_methods!(
    #[cfg(feature = "Foundation_NSMutableOrderedSet")]
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

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCapacity:)]
        pub unsafe fn initWithCapacity(
            this: Option<Allocated<Self>>,
            num_items: NSUInteger,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSOrderedSet`
    #[cfg(feature = "Foundation_NSMutableOrderedSet")]
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[method_id(@__retain_semantics Init initWithObjects:count:)]
        pub unsafe fn initWithObjects_count(
            this: Option<Allocated<Self>>,
            objects: *mut NonNull<ObjectType>,
            cnt: NSUInteger,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSMutableOrderedSet")]
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSExtendedMutableOrderedSet
    #[cfg(feature = "Foundation_NSMutableOrderedSet")]
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[method(addObject:)]
        pub unsafe fn addObject(&mut self, object: &ObjectType);

        #[method(addObjects:count:)]
        pub unsafe fn addObjects_count(
            &mut self,
            objects: *mut NonNull<ObjectType>,
            count: NSUInteger,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method(addObjectsFromArray:)]
        pub unsafe fn addObjectsFromArray(&mut self, array: &NSArray<ObjectType>);

        #[method(exchangeObjectAtIndex:withObjectAtIndex:)]
        pub unsafe fn exchangeObjectAtIndex_withObjectAtIndex(
            &mut self,
            idx1: NSUInteger,
            idx2: NSUInteger,
        );

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(moveObjectsAtIndexes:toIndex:)]
        pub unsafe fn moveObjectsAtIndexes_toIndex(
            &mut self,
            indexes: &NSIndexSet,
            idx: NSUInteger,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSIndexSet"))]
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

        #[method(replaceObjectsInRange:withObjects:count:)]
        pub unsafe fn replaceObjectsInRange_withObjects_count(
            &mut self,
            range: NSRange,
            objects: *mut NonNull<ObjectType>,
            count: NSUInteger,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSIndexSet"))]
        #[method(replaceObjectsAtIndexes:withObjects:)]
        pub unsafe fn replaceObjectsAtIndexes_withObjects(
            &mut self,
            indexes: &NSIndexSet,
            objects: &NSArray<ObjectType>,
        );

        #[method(removeObjectsInRange:)]
        pub unsafe fn removeObjectsInRange(&mut self, range: NSRange);

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(removeObjectsAtIndexes:)]
        pub unsafe fn removeObjectsAtIndexes(&mut self, indexes: &NSIndexSet);

        #[method(removeAllObjects)]
        pub unsafe fn removeAllObjects(&mut self);

        #[method(removeObject:)]
        pub unsafe fn removeObject(&mut self, object: &ObjectType);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(removeObjectsInArray:)]
        pub unsafe fn removeObjectsInArray(&mut self, array: &NSArray<ObjectType>);

        #[cfg(feature = "Foundation_NSOrderedSet")]
        #[method(intersectOrderedSet:)]
        pub unsafe fn intersectOrderedSet(&mut self, other: &NSOrderedSet<ObjectType>);

        #[cfg(feature = "Foundation_NSOrderedSet")]
        #[method(minusOrderedSet:)]
        pub unsafe fn minusOrderedSet(&mut self, other: &NSOrderedSet<ObjectType>);

        #[cfg(feature = "Foundation_NSOrderedSet")]
        #[method(unionOrderedSet:)]
        pub unsafe fn unionOrderedSet(&mut self, other: &NSOrderedSet<ObjectType>);

        #[cfg(feature = "Foundation_NSSet")]
        #[method(intersectSet:)]
        pub unsafe fn intersectSet(&mut self, other: &NSSet<ObjectType>);

        #[cfg(feature = "Foundation_NSSet")]
        #[method(minusSet:)]
        pub unsafe fn minusSet(&mut self, other: &NSSet<ObjectType>);

        #[cfg(feature = "Foundation_NSSet")]
        #[method(unionSet:)]
        pub unsafe fn unionSet(&mut self, other: &NSSet<ObjectType>);

        #[method(sortUsingComparator:)]
        pub unsafe fn sortUsingComparator(&mut self, cmptr: NSComparator);

        #[method(sortWithOptions:usingComparator:)]
        pub unsafe fn sortWithOptions_usingComparator(
            &mut self,
            opts: NSSortOptions,
            cmptr: NSComparator,
        );

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
    #[cfg(feature = "Foundation_NSMutableOrderedSet")]
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[method_id(@__retain_semantics Other orderedSetWithCapacity:)]
        pub unsafe fn orderedSetWithCapacity(num_items: NSUInteger) -> Id<Self>;
    }
);

extern_methods!(
    /// NSMutableOrderedSetDiffing
    #[cfg(feature = "Foundation_NSMutableOrderedSet")]
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[cfg(feature = "Foundation_NSOrderedCollectionDifference")]
        #[method(applyDifference:)]
        pub unsafe fn applyDifference(
            &mut self,
            difference: &NSOrderedCollectionDifference<ObjectType>,
        );
    }
);
