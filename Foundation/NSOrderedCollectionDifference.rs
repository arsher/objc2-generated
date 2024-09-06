//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSOrderedCollectionDifferenceCalculationOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSOrderedCollectionDifferenceCalculationOptions: NSUInteger {
        const NSOrderedCollectionDifferenceCalculationOmitInsertedObjects = 1<<0;
        const NSOrderedCollectionDifferenceCalculationOmitRemovedObjects = 1<<1;
        const NSOrderedCollectionDifferenceCalculationInferMoves = 1<<2;
    }
}

unsafe impl Encode for NSOrderedCollectionDifferenceCalculationOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSOrderedCollectionDifferenceCalculationOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSOrderedCollectionDifference<ObjectType: ?Sized = AnyObject> {
        __superclass: NSObject,
        _inner0: PhantomData<*mut ObjectType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<ObjectType: ?Sized + Message> ClassType for NSOrderedCollectionDifference<ObjectType> {
        type Super = NSObject;
        type Mutability = InteriorMutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }
    }
);

#[cfg(feature = "NSEnumerator")]
unsafe impl<ObjectType: ?Sized> NSFastEnumeration for NSOrderedCollectionDifference<ObjectType> {}

unsafe impl<ObjectType: ?Sized> NSObjectProtocol for NSOrderedCollectionDifference<ObjectType> {}

extern_methods!(
    unsafe impl<ObjectType: Message> NSOrderedCollectionDifference<ObjectType> {
        #[cfg(all(feature = "NSArray", feature = "NSOrderedCollectionChange"))]
        #[method_id(@__retain_semantics Init initWithChanges:)]
        pub unsafe fn initWithChanges(
            this: Allocated<Self>,
            changes: &NSArray<NSOrderedCollectionChange<ObjectType>>,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "NSArray",
            feature = "NSIndexSet",
            feature = "NSOrderedCollectionChange"
        ))]
        #[method_id(@__retain_semantics Init initWithInsertIndexes:insertedObjects:removeIndexes:removedObjects:additionalChanges:)]
        pub unsafe fn initWithInsertIndexes_insertedObjects_removeIndexes_removedObjects_additionalChanges(
            this: Allocated<Self>,
            inserts: &NSIndexSet,
            inserted_objects: Option<&NSArray<ObjectType>>,
            removes: &NSIndexSet,
            removed_objects: Option<&NSArray<ObjectType>>,
            changes: &NSArray<NSOrderedCollectionChange<ObjectType>>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSArray", feature = "NSIndexSet"))]
        #[method_id(@__retain_semantics Init initWithInsertIndexes:insertedObjects:removeIndexes:removedObjects:)]
        pub unsafe fn initWithInsertIndexes_insertedObjects_removeIndexes_removedObjects(
            this: Allocated<Self>,
            inserts: &NSIndexSet,
            inserted_objects: Option<&NSArray<ObjectType>>,
            removes: &NSIndexSet,
            removed_objects: Option<&NSArray<ObjectType>>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "NSArray", feature = "NSOrderedCollectionChange"))]
        #[method_id(@__retain_semantics Other insertions)]
        pub unsafe fn insertions(&self)
            -> Retained<NSArray<NSOrderedCollectionChange<ObjectType>>>;

        #[cfg(all(feature = "NSArray", feature = "NSOrderedCollectionChange"))]
        #[method_id(@__retain_semantics Other removals)]
        pub unsafe fn removals(&self) -> Retained<NSArray<NSOrderedCollectionChange<ObjectType>>>;

        #[method(hasChanges)]
        pub unsafe fn hasChanges(&self) -> bool;

        #[cfg(all(feature = "NSOrderedCollectionChange", feature = "block2"))]
        #[method_id(@__retain_semantics Other differenceByTransformingChangesWithBlock:)]
        pub unsafe fn differenceByTransformingChangesWithBlock(
            &self,
            block: &block2::Block<
                dyn Fn(
                        NonNull<NSOrderedCollectionChange<ObjectType>>,
                    ) -> NonNull<NSOrderedCollectionChange<AnyObject>>
                    + '_,
            >,
        ) -> Retained<NSOrderedCollectionDifference<AnyObject>>;

        #[method_id(@__retain_semantics Other inverseDifference)]
        pub unsafe fn inverseDifference(&self) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<ObjectType: Message> NSOrderedCollectionDifference<ObjectType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
