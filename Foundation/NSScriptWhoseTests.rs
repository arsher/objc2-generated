//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTestComparisonOperation {
        NSEqualToComparison = 0,
        NSLessThanOrEqualToComparison = 1,
        NSLessThanComparison = 2,
        NSGreaterThanOrEqualToComparison = 3,
        NSGreaterThanComparison = 4,
        NSBeginsWithComparison = 5,
        NSEndsWithComparison = 6,
        NSContainsComparison = 7,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSScriptWhoseTest")]
    pub struct NSScriptWhoseTest;

    #[cfg(feature = "Foundation_NSScriptWhoseTest")]
    unsafe impl ClassType for NSScriptWhoseTest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSScriptWhoseTest")]
unsafe impl NSCoding for NSScriptWhoseTest {}

#[cfg(feature = "Foundation_NSScriptWhoseTest")]
unsafe impl NSObjectProtocol for NSScriptWhoseTest {}

extern_methods!(
    #[cfg(feature = "Foundation_NSScriptWhoseTest")]
    unsafe impl NSScriptWhoseTest {
        #[method(isTrue)]
        pub unsafe fn isTrue(&self) -> bool;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            in_coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSLogicalTest")]
    pub struct NSLogicalTest;

    #[cfg(feature = "Foundation_NSLogicalTest")]
    unsafe impl ClassType for NSLogicalTest {
        #[inherits(NSObject)]
        type Super = NSScriptWhoseTest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSLogicalTest")]
unsafe impl NSCoding for NSLogicalTest {}

#[cfg(feature = "Foundation_NSLogicalTest")]
unsafe impl NSObjectProtocol for NSLogicalTest {}

extern_methods!(
    #[cfg(feature = "Foundation_NSLogicalTest")]
    unsafe impl NSLogicalTest {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSSpecifierTest"))]
        #[method_id(@__retain_semantics Init initAndTestWithTests:)]
        pub unsafe fn initAndTestWithTests(
            this: Option<Allocated<Self>>,
            sub_tests: &NSArray<NSSpecifierTest>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSSpecifierTest"))]
        #[method_id(@__retain_semantics Init initOrTestWithTests:)]
        pub unsafe fn initOrTestWithTests(
            this: Option<Allocated<Self>>,
            sub_tests: &NSArray<NSSpecifierTest>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initNotTestWithTest:)]
        pub unsafe fn initNotTestWithTest(
            this: Option<Allocated<Self>>,
            sub_test: &NSScriptWhoseTest,
        ) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSSpecifierTest")]
    pub struct NSSpecifierTest;

    #[cfg(feature = "Foundation_NSSpecifierTest")]
    unsafe impl ClassType for NSSpecifierTest {
        #[inherits(NSObject)]
        type Super = NSScriptWhoseTest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSSpecifierTest")]
unsafe impl NSCoding for NSSpecifierTest {}

#[cfg(feature = "Foundation_NSSpecifierTest")]
unsafe impl NSObjectProtocol for NSSpecifierTest {}

extern_methods!(
    #[cfg(feature = "Foundation_NSSpecifierTest")]
    unsafe impl NSSpecifierTest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            in_coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSScriptObjectSpecifier")]
        #[method_id(@__retain_semantics Init initWithObjectSpecifier:comparisonOperator:testObject:)]
        pub unsafe fn initWithObjectSpecifier_comparisonOperator_testObject(
            this: Option<Allocated<Self>>,
            obj1: Option<&NSScriptObjectSpecifier>,
            comp_op: NSTestComparisonOperation,
            obj2: Option<&Object>,
        ) -> Id<Self>;
    }
);
