//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsindexset?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSIndexSet;
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSIndexSet {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSIndexSet {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSIndexSet {
    type Result = Self;
}

#[cfg(feature = "NSObject")]
unsafe impl NSMutableCopying for NSIndexSet {}

#[cfg(feature = "NSObject")]
unsafe impl MutableCopyingHelper for NSIndexSet {
    type Result = NSMutableIndexSet;
}

unsafe impl NSObjectProtocol for NSIndexSet {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSIndexSet {}

extern_methods!(
    unsafe impl NSIndexSet {
        #[method_id(@__retain_semantics Other indexSet)]
        pub unsafe fn indexSet() -> Retained<Self>;

        #[method_id(@__retain_semantics Other indexSetWithIndex:)]
        pub unsafe fn indexSetWithIndex(value: NSUInteger) -> Retained<Self>;

        #[cfg(feature = "NSRange")]
        #[method_id(@__retain_semantics Other indexSetWithIndexesInRange:)]
        pub unsafe fn indexSetWithIndexesInRange(range: NSRange) -> Retained<Self>;

        #[cfg(feature = "NSRange")]
        #[method_id(@__retain_semantics Init initWithIndexesInRange:)]
        pub unsafe fn initWithIndexesInRange(
            this: Allocated<Self>,
            range: NSRange,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithIndexSet:)]
        pub unsafe fn initWithIndexSet(
            this: Allocated<Self>,
            index_set: &NSIndexSet,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithIndex:)]
        pub unsafe fn initWithIndex(this: Allocated<Self>, value: NSUInteger) -> Retained<Self>;

        #[method(isEqualToIndexSet:)]
        pub unsafe fn isEqualToIndexSet(&self, index_set: &NSIndexSet) -> bool;

        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[method(firstIndex)]
        pub unsafe fn firstIndex(&self) -> NSUInteger;

        #[method(lastIndex)]
        pub unsafe fn lastIndex(&self) -> NSUInteger;

        #[method(indexGreaterThanIndex:)]
        pub unsafe fn indexGreaterThanIndex(&self, value: NSUInteger) -> NSUInteger;

        #[method(indexLessThanIndex:)]
        pub unsafe fn indexLessThanIndex(&self, value: NSUInteger) -> NSUInteger;

        #[method(indexGreaterThanOrEqualToIndex:)]
        pub unsafe fn indexGreaterThanOrEqualToIndex(&self, value: NSUInteger) -> NSUInteger;

        #[method(indexLessThanOrEqualToIndex:)]
        pub unsafe fn indexLessThanOrEqualToIndex(&self, value: NSUInteger) -> NSUInteger;

        #[cfg(feature = "NSRange")]
        #[method(getIndexes:maxCount:inIndexRange:)]
        pub unsafe fn getIndexes_maxCount_inIndexRange(
            &self,
            index_buffer: NonNull<NSUInteger>,
            buffer_size: NSUInteger,
            range: NSRangePointer,
        ) -> NSUInteger;

        #[cfg(feature = "NSRange")]
        #[method(countOfIndexesInRange:)]
        pub unsafe fn countOfIndexesInRange(&self, range: NSRange) -> NSUInteger;

        #[method(containsIndex:)]
        pub unsafe fn containsIndex(&self, value: NSUInteger) -> bool;

        #[cfg(feature = "NSRange")]
        #[method(containsIndexesInRange:)]
        pub unsafe fn containsIndexesInRange(&self, range: NSRange) -> bool;

        #[method(containsIndexes:)]
        pub unsafe fn containsIndexes(&self, index_set: &NSIndexSet) -> bool;

        #[cfg(feature = "NSRange")]
        #[method(intersectsIndexesInRange:)]
        pub unsafe fn intersectsIndexesInRange(&self, range: NSRange) -> bool;

        #[cfg(feature = "block2")]
        #[method(enumerateIndexesUsingBlock:)]
        pub unsafe fn enumerateIndexesUsingBlock(
            &self,
            block: &block2::Block<dyn Fn(NSUInteger, NonNull<Bool>) + '_>,
        );

        #[cfg(all(feature = "NSObjCRuntime", feature = "block2"))]
        #[method(enumerateIndexesWithOptions:usingBlock:)]
        pub unsafe fn enumerateIndexesWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: &block2::Block<dyn Fn(NSUInteger, NonNull<Bool>) + '_>,
        );

        #[cfg(all(feature = "NSObjCRuntime", feature = "NSRange", feature = "block2"))]
        #[method(enumerateIndexesInRange:options:usingBlock:)]
        pub unsafe fn enumerateIndexesInRange_options_usingBlock(
            &self,
            range: NSRange,
            opts: NSEnumerationOptions,
            block: &block2::Block<dyn Fn(NSUInteger, NonNull<Bool>) + '_>,
        );

        #[cfg(feature = "block2")]
        #[method(indexPassingTest:)]
        pub unsafe fn indexPassingTest(
            &self,
            predicate: &block2::Block<dyn Fn(NSUInteger, NonNull<Bool>) -> Bool + '_>,
        ) -> NSUInteger;

        #[cfg(all(feature = "NSObjCRuntime", feature = "block2"))]
        #[method(indexWithOptions:passingTest:)]
        pub unsafe fn indexWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: &block2::Block<dyn Fn(NSUInteger, NonNull<Bool>) -> Bool + '_>,
        ) -> NSUInteger;

        #[cfg(all(feature = "NSObjCRuntime", feature = "NSRange", feature = "block2"))]
        #[method(indexInRange:options:passingTest:)]
        pub unsafe fn indexInRange_options_passingTest(
            &self,
            range: NSRange,
            opts: NSEnumerationOptions,
            predicate: &block2::Block<dyn Fn(NSUInteger, NonNull<Bool>) -> Bool + '_>,
        ) -> NSUInteger;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other indexesPassingTest:)]
        pub unsafe fn indexesPassingTest(
            &self,
            predicate: &block2::Block<dyn Fn(NSUInteger, NonNull<Bool>) -> Bool + '_>,
        ) -> Retained<NSIndexSet>;

        #[cfg(all(feature = "NSObjCRuntime", feature = "block2"))]
        #[method_id(@__retain_semantics Other indexesWithOptions:passingTest:)]
        pub unsafe fn indexesWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: &block2::Block<dyn Fn(NSUInteger, NonNull<Bool>) -> Bool + '_>,
        ) -> Retained<NSIndexSet>;

        #[cfg(all(feature = "NSObjCRuntime", feature = "NSRange", feature = "block2"))]
        #[method_id(@__retain_semantics Other indexesInRange:options:passingTest:)]
        pub unsafe fn indexesInRange_options_passingTest(
            &self,
            range: NSRange,
            opts: NSEnumerationOptions,
            predicate: &block2::Block<dyn Fn(NSUInteger, NonNull<Bool>) -> Bool + '_>,
        ) -> Retained<NSIndexSet>;

        #[cfg(all(feature = "NSRange", feature = "block2"))]
        #[method(enumerateRangesUsingBlock:)]
        pub unsafe fn enumerateRangesUsingBlock(
            &self,
            block: &block2::Block<dyn Fn(NSRange, NonNull<Bool>) + '_>,
        );

        #[cfg(all(feature = "NSObjCRuntime", feature = "NSRange", feature = "block2"))]
        #[method(enumerateRangesWithOptions:usingBlock:)]
        pub unsafe fn enumerateRangesWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: &block2::Block<dyn Fn(NSRange, NonNull<Bool>) + '_>,
        );

        #[cfg(all(feature = "NSObjCRuntime", feature = "NSRange", feature = "block2"))]
        #[method(enumerateRangesInRange:options:usingBlock:)]
        pub unsafe fn enumerateRangesInRange_options_usingBlock(
            &self,
            range: NSRange,
            opts: NSEnumerationOptions,
            block: &block2::Block<dyn Fn(NSRange, NonNull<Bool>) + '_>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSIndexSet {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsmutableindexset?language=objc)
    #[unsafe(super(NSIndexSet, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMutableIndexSet;
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSMutableIndexSet {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSMutableIndexSet {}

#[cfg(feature = "NSObject")]
unsafe impl CopyingHelper for NSMutableIndexSet {
    type Result = NSIndexSet;
}

#[cfg(feature = "NSObject")]
unsafe impl NSMutableCopying for NSMutableIndexSet {}

#[cfg(feature = "NSObject")]
unsafe impl MutableCopyingHelper for NSMutableIndexSet {
    type Result = Self;
}

unsafe impl NSObjectProtocol for NSMutableIndexSet {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSMutableIndexSet {}

extern_methods!(
    unsafe impl NSMutableIndexSet {
        #[method(addIndexes:)]
        pub unsafe fn addIndexes(&self, index_set: &NSIndexSet);

        #[method(removeIndexes:)]
        pub unsafe fn removeIndexes(&self, index_set: &NSIndexSet);

        #[method(removeAllIndexes)]
        pub unsafe fn removeAllIndexes(&self);

        #[method(addIndex:)]
        pub unsafe fn addIndex(&self, value: NSUInteger);

        #[method(removeIndex:)]
        pub unsafe fn removeIndex(&self, value: NSUInteger);

        #[cfg(feature = "NSRange")]
        #[method(addIndexesInRange:)]
        pub unsafe fn addIndexesInRange(&self, range: NSRange);

        #[cfg(feature = "NSRange")]
        #[method(removeIndexesInRange:)]
        pub unsafe fn removeIndexesInRange(&self, range: NSRange);

        #[method(shiftIndexesStartingAtIndex:by:)]
        pub unsafe fn shiftIndexesStartingAtIndex_by(&self, index: NSUInteger, delta: NSInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSIndexSet`
    unsafe impl NSMutableIndexSet {
        #[method_id(@__retain_semantics Other indexSet)]
        pub unsafe fn indexSet() -> Retained<Self>;

        #[method_id(@__retain_semantics Other indexSetWithIndex:)]
        pub unsafe fn indexSetWithIndex(value: NSUInteger) -> Retained<Self>;

        #[cfg(feature = "NSRange")]
        #[method_id(@__retain_semantics Other indexSetWithIndexesInRange:)]
        pub unsafe fn indexSetWithIndexesInRange(range: NSRange) -> Retained<Self>;

        #[cfg(feature = "NSRange")]
        #[method_id(@__retain_semantics Init initWithIndexesInRange:)]
        pub unsafe fn initWithIndexesInRange(
            this: Allocated<Self>,
            range: NSRange,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithIndexSet:)]
        pub unsafe fn initWithIndexSet(
            this: Allocated<Self>,
            index_set: &NSIndexSet,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithIndex:)]
        pub unsafe fn initWithIndex(this: Allocated<Self>, value: NSUInteger) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSMutableIndexSet {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
