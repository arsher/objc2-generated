//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstreenode?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTreeNode;
);

unsafe impl NSObjectProtocol for NSTreeNode {}

extern_methods!(
    unsafe impl NSTreeNode {
        #[method_id(@__retain_semantics Other treeNodeWithRepresentedObject:)]
        pub unsafe fn treeNodeWithRepresentedObject(
            model_object: Option<&AnyObject>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithRepresentedObject:)]
        pub unsafe fn initWithRepresentedObject(
            this: Allocated<Self>,
            model_object: Option<&AnyObject>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Retained<AnyObject>>;

        #[method_id(@__retain_semantics Other indexPath)]
        pub unsafe fn indexPath(&self) -> Retained<NSIndexPath>;

        #[method(isLeaf)]
        pub unsafe fn isLeaf(&self) -> bool;

        #[method_id(@__retain_semantics Other childNodes)]
        pub unsafe fn childNodes(&self) -> Option<Retained<NSArray<NSTreeNode>>>;

        #[method_id(@__retain_semantics Other mutableChildNodes)]
        pub unsafe fn mutableChildNodes(&self) -> Retained<NSMutableArray<NSTreeNode>>;

        #[method_id(@__retain_semantics Other descendantNodeAtIndexPath:)]
        pub unsafe fn descendantNodeAtIndexPath(
            &self,
            index_path: &NSIndexPath,
        ) -> Option<Retained<NSTreeNode>>;

        #[method_id(@__retain_semantics Other parentNode)]
        pub unsafe fn parentNode(&self) -> Option<Retained<NSTreeNode>>;

        #[method(sortWithSortDescriptors:recursively:)]
        pub unsafe fn sortWithSortDescriptors_recursively(
            &self,
            sort_descriptors: &NSArray<NSSortDescriptor>,
            recursively: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTreeNode {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
