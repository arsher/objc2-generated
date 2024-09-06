//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDiffableDataSourceSnapshot<
        SectionIdentifierType: ?Sized = AnyObject,
        ItemIdentifierType: ?Sized = AnyObject,
    > {
        __superclass: NSObject,
        _inner0: PhantomData<*mut SectionIdentifierType>,
        _inner1: PhantomData<*mut ItemIdentifierType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<SectionIdentifierType: ?Sized + Message, ItemIdentifierType: ?Sized + Message>
        ClassType for NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>
    {
        type Super = NSObject;
        type Mutability = InteriorMutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }
    }
);

unsafe impl<SectionIdentifierType: ?Sized + IsIdCloneable, ItemIdentifierType: ?Sized + IsIdCloneable>
    NSCopying for NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>
{
}

unsafe impl<SectionIdentifierType: ?Sized, ItemIdentifierType: ?Sized> NSObjectProtocol
    for NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>
{
}

extern_methods!(
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>
    {
        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[method(numberOfSections)]
        pub unsafe fn numberOfSections(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other sectionIdentifiers)]
        pub unsafe fn sectionIdentifiers(&self) -> Retained<NSArray<SectionIdentifierType>>;

        #[method_id(@__retain_semantics Other itemIdentifiers)]
        pub unsafe fn itemIdentifiers(&self) -> Retained<NSArray<ItemIdentifierType>>;

        #[method_id(@__retain_semantics Other reloadedSectionIdentifiers)]
        pub unsafe fn reloadedSectionIdentifiers(&self)
            -> Retained<NSArray<SectionIdentifierType>>;

        #[method_id(@__retain_semantics Other reloadedItemIdentifiers)]
        pub unsafe fn reloadedItemIdentifiers(&self) -> Retained<NSArray<ItemIdentifierType>>;

        #[method_id(@__retain_semantics Other reconfiguredItemIdentifiers)]
        pub unsafe fn reconfiguredItemIdentifiers(&self) -> Retained<NSArray<ItemIdentifierType>>;

        #[method(numberOfItemsInSection:)]
        pub unsafe fn numberOfItemsInSection(
            &self,
            section_identifier: &SectionIdentifierType,
        ) -> NSInteger;

        #[method_id(@__retain_semantics Other itemIdentifiersInSectionWithIdentifier:)]
        pub unsafe fn itemIdentifiersInSectionWithIdentifier(
            &self,
            section_identifier: &SectionIdentifierType,
        ) -> Retained<NSArray<ItemIdentifierType>>;

        #[method_id(@__retain_semantics Other sectionIdentifierForSectionContainingItemIdentifier:)]
        pub unsafe fn sectionIdentifierForSectionContainingItemIdentifier(
            &self,
            item_identifier: &ItemIdentifierType,
        ) -> Option<Retained<SectionIdentifierType>>;

        #[method(indexOfItemIdentifier:)]
        pub unsafe fn indexOfItemIdentifier(
            &self,
            item_identifier: &ItemIdentifierType,
        ) -> NSInteger;

        #[method(indexOfSectionIdentifier:)]
        pub unsafe fn indexOfSectionIdentifier(
            &self,
            section_identifier: &SectionIdentifierType,
        ) -> NSInteger;

        #[method(appendItemsWithIdentifiers:)]
        pub unsafe fn appendItemsWithIdentifiers(&self, identifiers: &NSArray<ItemIdentifierType>);

        #[method(appendItemsWithIdentifiers:intoSectionWithIdentifier:)]
        pub unsafe fn appendItemsWithIdentifiers_intoSectionWithIdentifier(
            &self,
            identifiers: &NSArray<ItemIdentifierType>,
            section_identifier: &SectionIdentifierType,
        );

        #[method(insertItemsWithIdentifiers:beforeItemWithIdentifier:)]
        pub unsafe fn insertItemsWithIdentifiers_beforeItemWithIdentifier(
            &self,
            identifiers: &NSArray<ItemIdentifierType>,
            item_identifier: &ItemIdentifierType,
        );

        #[method(insertItemsWithIdentifiers:afterItemWithIdentifier:)]
        pub unsafe fn insertItemsWithIdentifiers_afterItemWithIdentifier(
            &self,
            identifiers: &NSArray<ItemIdentifierType>,
            item_identifier: &ItemIdentifierType,
        );

        #[method(deleteItemsWithIdentifiers:)]
        pub unsafe fn deleteItemsWithIdentifiers(&self, identifiers: &NSArray<ItemIdentifierType>);

        #[method(deleteAllItems)]
        pub unsafe fn deleteAllItems(&self);

        #[method(moveItemWithIdentifier:beforeItemWithIdentifier:)]
        pub unsafe fn moveItemWithIdentifier_beforeItemWithIdentifier(
            &self,
            from_identifier: &ItemIdentifierType,
            to_identifier: &ItemIdentifierType,
        );

        #[method(moveItemWithIdentifier:afterItemWithIdentifier:)]
        pub unsafe fn moveItemWithIdentifier_afterItemWithIdentifier(
            &self,
            from_identifier: &ItemIdentifierType,
            to_identifier: &ItemIdentifierType,
        );

        #[method(reloadItemsWithIdentifiers:)]
        pub unsafe fn reloadItemsWithIdentifiers(&self, identifiers: &NSArray<ItemIdentifierType>);

        #[method(reconfigureItemsWithIdentifiers:)]
        pub unsafe fn reconfigureItemsWithIdentifiers(
            &self,
            identifiers: &NSArray<ItemIdentifierType>,
        );

        #[method(appendSectionsWithIdentifiers:)]
        pub unsafe fn appendSectionsWithIdentifiers(
            &self,
            section_identifiers: &NSArray<SectionIdentifierType>,
        );

        #[method(insertSectionsWithIdentifiers:beforeSectionWithIdentifier:)]
        pub unsafe fn insertSectionsWithIdentifiers_beforeSectionWithIdentifier(
            &self,
            section_identifiers: &NSArray<SectionIdentifierType>,
            to_section_identifier: &SectionIdentifierType,
        );

        #[method(insertSectionsWithIdentifiers:afterSectionWithIdentifier:)]
        pub unsafe fn insertSectionsWithIdentifiers_afterSectionWithIdentifier(
            &self,
            section_identifiers: &NSArray<SectionIdentifierType>,
            to_section_identifier: &SectionIdentifierType,
        );

        #[method(deleteSectionsWithIdentifiers:)]
        pub unsafe fn deleteSectionsWithIdentifiers(
            &self,
            section_identifiers: &NSArray<SectionIdentifierType>,
        );

        #[method(moveSectionWithIdentifier:beforeSectionWithIdentifier:)]
        pub unsafe fn moveSectionWithIdentifier_beforeSectionWithIdentifier(
            &self,
            from_section_identifier: &SectionIdentifierType,
            to_section_identifier: &SectionIdentifierType,
        );

        #[method(moveSectionWithIdentifier:afterSectionWithIdentifier:)]
        pub unsafe fn moveSectionWithIdentifier_afterSectionWithIdentifier(
            &self,
            from_section_identifier: &SectionIdentifierType,
            to_section_identifier: &SectionIdentifierType,
        );

        #[method(reloadSectionsWithIdentifiers:)]
        pub unsafe fn reloadSectionsWithIdentifiers(
            &self,
            section_identifiers: &NSArray<SectionIdentifierType>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>
    {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

#[cfg(all(
    feature = "UICollectionView",
    feature = "UICollectionViewCell",
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UIView",
    feature = "block2"
))]
pub type UICollectionViewDiffableDataSourceCellProvider = *mut block2::Block<
    dyn Fn(
        NonNull<UICollectionView>,
        NonNull<NSIndexPath>,
        NonNull<AnyObject>,
    ) -> *mut UICollectionViewCell,
>;

#[cfg(all(
    feature = "UICollectionView",
    feature = "UICollectionViewCell",
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UIView",
    feature = "block2"
))]
pub type UICollectionViewDiffableDataSourceSupplementaryViewProvider = *mut block2::Block<
    dyn Fn(
        NonNull<UICollectionView>,
        NonNull<NSString>,
        NonNull<NSIndexPath>,
    ) -> *mut UICollectionReusableView,
>;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDiffableDataSourceSectionTransaction<
        SectionIdentifierType: ?Sized = AnyObject,
        ItemIdentifierType: ?Sized = AnyObject,
    > {
        __superclass: NSObject,
        _inner0: PhantomData<*mut SectionIdentifierType>,
        _inner1: PhantomData<*mut ItemIdentifierType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<SectionIdentifierType: ?Sized + Message, ItemIdentifierType: ?Sized + Message>
        ClassType
        for NSDiffableDataSourceSectionTransaction<SectionIdentifierType, ItemIdentifierType>
    {
        type Super = NSObject;
        type Mutability = InteriorMutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }
    }
);

unsafe impl<SectionIdentifierType: ?Sized, ItemIdentifierType: ?Sized> NSObjectProtocol
    for NSDiffableDataSourceSectionTransaction<SectionIdentifierType, ItemIdentifierType>
{
}

extern_methods!(
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        NSDiffableDataSourceSectionTransaction<SectionIdentifierType, ItemIdentifierType>
    {
        #[method_id(@__retain_semantics Other sectionIdentifier)]
        pub unsafe fn sectionIdentifier(&self) -> Retained<SectionIdentifierType>;

        #[cfg(feature = "NSDiffableDataSourceSectionSnapshot")]
        #[method_id(@__retain_semantics Other initialSnapshot)]
        pub unsafe fn initialSnapshot(
            &self,
        ) -> Retained<NSDiffableDataSourceSectionSnapshot<ItemIdentifierType>>;

        #[cfg(feature = "NSDiffableDataSourceSectionSnapshot")]
        #[method_id(@__retain_semantics Other finalSnapshot)]
        pub unsafe fn finalSnapshot(
            &self,
        ) -> Retained<NSDiffableDataSourceSectionSnapshot<ItemIdentifierType>>;

        #[method_id(@__retain_semantics Other difference)]
        pub unsafe fn difference(
            &self,
        ) -> Retained<NSOrderedCollectionDifference<ItemIdentifierType>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        NSDiffableDataSourceSectionTransaction<SectionIdentifierType, ItemIdentifierType>
    {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSDiffableDataSourceTransaction<
        SectionIdentifierType: ?Sized = AnyObject,
        ItemIdentifierType: ?Sized = AnyObject,
    > {
        __superclass: NSObject,
        _inner0: PhantomData<*mut SectionIdentifierType>,
        _inner1: PhantomData<*mut ItemIdentifierType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<SectionIdentifierType: ?Sized + Message, ItemIdentifierType: ?Sized + Message>
        ClassType for NSDiffableDataSourceTransaction<SectionIdentifierType, ItemIdentifierType>
    {
        type Super = NSObject;
        type Mutability = InteriorMutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }
    }
);

unsafe impl<SectionIdentifierType: ?Sized, ItemIdentifierType: ?Sized> NSObjectProtocol
    for NSDiffableDataSourceTransaction<SectionIdentifierType, ItemIdentifierType>
{
}

extern_methods!(
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        NSDiffableDataSourceTransaction<SectionIdentifierType, ItemIdentifierType>
    {
        #[method_id(@__retain_semantics Other initialSnapshot)]
        pub unsafe fn initialSnapshot(
            &self,
        ) -> Retained<NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>>;

        #[method_id(@__retain_semantics Other finalSnapshot)]
        pub unsafe fn finalSnapshot(
            &self,
        ) -> Retained<NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>>;

        #[method_id(@__retain_semantics Other difference)]
        pub unsafe fn difference(
            &self,
        ) -> Retained<NSOrderedCollectionDifference<ItemIdentifierType>>;

        #[method_id(@__retain_semantics Other sectionTransactions)]
        pub unsafe fn sectionTransactions(
            &self,
        ) -> Retained<
            NSArray<
                NSDiffableDataSourceSectionTransaction<SectionIdentifierType, ItemIdentifierType>,
            >,
        >;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        NSDiffableDataSourceTransaction<SectionIdentifierType, ItemIdentifierType>
    {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UICollectionViewDiffableDataSourceReorderingHandlers<
        SectionType: ?Sized = AnyObject,
        ItemType: ?Sized = AnyObject,
    > {
        __superclass: NSObject,
        _inner0: PhantomData<*mut SectionType>,
        _inner1: PhantomData<*mut ItemType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<SectionType: ?Sized + Message, ItemType: ?Sized + Message> ClassType
        for UICollectionViewDiffableDataSourceReorderingHandlers<SectionType, ItemType>
    {
        type Super = NSObject;
        type Mutability = MainThreadOnly;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }
    }
);

unsafe impl<SectionType: ?Sized + IsIdCloneable, ItemType: ?Sized + IsIdCloneable> NSCopying
    for UICollectionViewDiffableDataSourceReorderingHandlers<SectionType, ItemType>
{
}

unsafe impl<SectionType: ?Sized, ItemType: ?Sized> NSObjectProtocol
    for UICollectionViewDiffableDataSourceReorderingHandlers<SectionType, ItemType>
{
}

extern_methods!(
    unsafe impl<SectionType: Message, ItemType: Message>
        UICollectionViewDiffableDataSourceReorderingHandlers<SectionType, ItemType>
    {
        #[cfg(feature = "block2")]
        #[method(canReorderItemHandler)]
        pub unsafe fn canReorderItemHandler(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<ItemType>) -> Bool>;

        #[cfg(feature = "block2")]
        #[method(setCanReorderItemHandler:)]
        pub unsafe fn setCanReorderItemHandler(
            &self,
            can_reorder_item_handler: Option<&block2::Block<dyn Fn(NonNull<ItemType>) -> Bool>>,
        );

        #[cfg(feature = "block2")]
        #[method(willReorderHandler)]
        pub unsafe fn willReorderHandler(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(NonNull<NSDiffableDataSourceTransaction<SectionType, ItemType>>),
        >;

        #[cfg(feature = "block2")]
        #[method(setWillReorderHandler:)]
        pub unsafe fn setWillReorderHandler(
            &self,
            will_reorder_handler: Option<
                &block2::Block<
                    dyn Fn(NonNull<NSDiffableDataSourceTransaction<SectionType, ItemType>>),
                >,
            >,
        );

        #[cfg(feature = "block2")]
        #[method(didReorderHandler)]
        pub unsafe fn didReorderHandler(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(NonNull<NSDiffableDataSourceTransaction<SectionType, ItemType>>),
        >;

        #[cfg(feature = "block2")]
        #[method(setDidReorderHandler:)]
        pub unsafe fn setDidReorderHandler(
            &self,
            did_reorder_handler: Option<
                &block2::Block<
                    dyn Fn(NonNull<NSDiffableDataSourceTransaction<SectionType, ItemType>>),
                >,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<SectionType: Message, ItemType: Message>
        UICollectionViewDiffableDataSourceReorderingHandlers<SectionType, ItemType>
    {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UICollectionViewDiffableDataSourceSectionSnapshotHandlers<
        ItemType: ?Sized = AnyObject,
    > {
        __superclass: NSObject,
        _inner0: PhantomData<*mut ItemType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<ItemType: ?Sized + Message> ClassType
        for UICollectionViewDiffableDataSourceSectionSnapshotHandlers<ItemType>
    {
        type Super = NSObject;
        type Mutability = MainThreadOnly;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }
    }
);

unsafe impl<ItemType: ?Sized + IsIdCloneable> NSCopying
    for UICollectionViewDiffableDataSourceSectionSnapshotHandlers<ItemType>
{
}

unsafe impl<ItemType: ?Sized> NSObjectProtocol
    for UICollectionViewDiffableDataSourceSectionSnapshotHandlers<ItemType>
{
}

extern_methods!(
    unsafe impl<ItemType: Message> UICollectionViewDiffableDataSourceSectionSnapshotHandlers<ItemType> {
        #[cfg(feature = "block2")]
        #[method(shouldExpandItemHandler)]
        pub unsafe fn shouldExpandItemHandler(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<ItemType>) -> Bool>;

        #[cfg(feature = "block2")]
        #[method(setShouldExpandItemHandler:)]
        pub unsafe fn setShouldExpandItemHandler(
            &self,
            should_expand_item_handler: Option<&block2::Block<dyn Fn(NonNull<ItemType>) -> Bool>>,
        );

        #[cfg(feature = "block2")]
        #[method(willExpandItemHandler)]
        pub unsafe fn willExpandItemHandler(&self)
            -> *mut block2::Block<dyn Fn(NonNull<ItemType>)>;

        #[cfg(feature = "block2")]
        #[method(setWillExpandItemHandler:)]
        pub unsafe fn setWillExpandItemHandler(
            &self,
            will_expand_item_handler: Option<&block2::Block<dyn Fn(NonNull<ItemType>)>>,
        );

        #[cfg(feature = "block2")]
        #[method(shouldCollapseItemHandler)]
        pub unsafe fn shouldCollapseItemHandler(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<ItemType>) -> Bool>;

        #[cfg(feature = "block2")]
        #[method(setShouldCollapseItemHandler:)]
        pub unsafe fn setShouldCollapseItemHandler(
            &self,
            should_collapse_item_handler: Option<&block2::Block<dyn Fn(NonNull<ItemType>) -> Bool>>,
        );

        #[cfg(feature = "block2")]
        #[method(willCollapseItemHandler)]
        pub unsafe fn willCollapseItemHandler(
            &self,
        ) -> *mut block2::Block<dyn Fn(NonNull<ItemType>)>;

        #[cfg(feature = "block2")]
        #[method(setWillCollapseItemHandler:)]
        pub unsafe fn setWillCollapseItemHandler(
            &self,
            will_collapse_item_handler: Option<&block2::Block<dyn Fn(NonNull<ItemType>)>>,
        );

        #[cfg(all(feature = "NSDiffableDataSourceSectionSnapshot", feature = "block2"))]
        #[method(snapshotForExpandingParentItemHandler)]
        pub unsafe fn snapshotForExpandingParentItemHandler(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(
                NonNull<ItemType>,
                NonNull<NSDiffableDataSourceSectionSnapshot<ItemType>>,
            ) -> NonNull<NSDiffableDataSourceSectionSnapshot<ItemType>>,
        >;

        #[cfg(all(feature = "NSDiffableDataSourceSectionSnapshot", feature = "block2"))]
        #[method(setSnapshotForExpandingParentItemHandler:)]
        pub unsafe fn setSnapshotForExpandingParentItemHandler(
            &self,
            snapshot_for_expanding_parent_item_handler: Option<
                &block2::Block<
                    dyn Fn(
                        NonNull<ItemType>,
                        NonNull<NSDiffableDataSourceSectionSnapshot<ItemType>>,
                    )
                        -> NonNull<NSDiffableDataSourceSectionSnapshot<ItemType>>,
                >,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl<ItemType: Message> UICollectionViewDiffableDataSourceSectionSnapshotHandlers<ItemType> {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UICollectionViewDiffableDataSource<
        SectionIdentifierType: ?Sized = AnyObject,
        ItemIdentifierType: ?Sized = AnyObject,
    > {
        __superclass: NSObject,
        _inner0: PhantomData<*mut SectionIdentifierType>,
        _inner1: PhantomData<*mut ItemIdentifierType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<SectionIdentifierType: ?Sized + Message, ItemIdentifierType: ?Sized + Message>
        ClassType
        for UICollectionViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
    {
        type Super = NSObject;
        type Mutability = MainThreadOnly;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }
    }
);

unsafe impl<SectionIdentifierType: ?Sized, ItemIdentifierType: ?Sized> NSObjectProtocol
    for UICollectionViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
{
}

#[cfg(feature = "UICollectionView")]
unsafe impl<SectionIdentifierType: ?Sized + Message, ItemIdentifierType: ?Sized + Message>
    UICollectionViewDataSource
    for UICollectionViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
{
}

extern_methods!(
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        UICollectionViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
    {
        #[cfg(all(
            feature = "UICollectionView",
            feature = "UICollectionViewCell",
            feature = "UIResponder",
            feature = "UIScrollView",
            feature = "UIView",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithCollectionView:cellProvider:)]
        pub unsafe fn initWithCollectionView_cellProvider(
            this: Allocated<Self>,
            collection_view: &UICollectionView,
            cell_provider: UICollectionViewDiffableDataSourceCellProvider,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[cfg(all(
            feature = "UICollectionView",
            feature = "UICollectionViewCell",
            feature = "UIResponder",
            feature = "UIScrollView",
            feature = "UIView",
            feature = "block2"
        ))]
        #[method(supplementaryViewProvider)]
        pub unsafe fn supplementaryViewProvider(
            &self,
        ) -> UICollectionViewDiffableDataSourceSupplementaryViewProvider;

        #[cfg(all(
            feature = "UICollectionView",
            feature = "UICollectionViewCell",
            feature = "UIResponder",
            feature = "UIScrollView",
            feature = "UIView",
            feature = "block2"
        ))]
        #[method(setSupplementaryViewProvider:)]
        pub unsafe fn setSupplementaryViewProvider(
            &self,
            supplementary_view_provider: UICollectionViewDiffableDataSourceSupplementaryViewProvider,
        );

        #[method_id(@__retain_semantics Other snapshot)]
        pub unsafe fn snapshot(
            &self,
        ) -> Retained<NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>>;

        #[method(applySnapshot:animatingDifferences:)]
        pub unsafe fn applySnapshot_animatingDifferences(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            animating_differences: bool,
        );

        #[cfg(feature = "block2")]
        #[method(applySnapshot:animatingDifferences:completion:)]
        pub unsafe fn applySnapshot_animatingDifferences_completion(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            animating_differences: bool,
            completion: Option<&block2::Block<dyn Fn()>>,
        );

        #[method(applySnapshotUsingReloadData:)]
        pub unsafe fn applySnapshotUsingReloadData(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
        );

        #[cfg(feature = "block2")]
        #[method(applySnapshotUsingReloadData:completion:)]
        pub unsafe fn applySnapshotUsingReloadData_completion(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            completion: Option<&block2::Block<dyn Fn()>>,
        );

        #[method_id(@__retain_semantics Other sectionIdentifierForIndex:)]
        pub unsafe fn sectionIdentifierForIndex(
            &self,
            index: NSInteger,
        ) -> Option<Retained<SectionIdentifierType>>;

        #[method(indexForSectionIdentifier:)]
        pub unsafe fn indexForSectionIdentifier(
            &self,
            identifier: &SectionIdentifierType,
        ) -> NSInteger;

        #[method_id(@__retain_semantics Other itemIdentifierForIndexPath:)]
        pub unsafe fn itemIdentifierForIndexPath(
            &self,
            index_path: &NSIndexPath,
        ) -> Option<Retained<ItemIdentifierType>>;

        #[method_id(@__retain_semantics Other indexPathForItemIdentifier:)]
        pub unsafe fn indexPathForItemIdentifier(
            &self,
            identifier: &ItemIdentifierType,
        ) -> Option<Retained<NSIndexPath>>;

        #[method_id(@__retain_semantics Other reorderingHandlers)]
        pub unsafe fn reorderingHandlers(
            &self,
        ) -> Retained<
            UICollectionViewDiffableDataSourceReorderingHandlers<
                SectionIdentifierType,
                ItemIdentifierType,
            >,
        >;

        #[method(setReorderingHandlers:)]
        pub unsafe fn setReorderingHandlers(
            &self,
            reordering_handlers: &UICollectionViewDiffableDataSourceReorderingHandlers<
                SectionIdentifierType,
                ItemIdentifierType,
            >,
        );

        #[cfg(feature = "NSDiffableDataSourceSectionSnapshot")]
        #[method(applySnapshot:toSection:animatingDifferences:)]
        pub unsafe fn applySnapshot_toSection_animatingDifferences(
            &self,
            snapshot: &NSDiffableDataSourceSectionSnapshot<ItemIdentifierType>,
            section_identifier: &SectionIdentifierType,
            animating_differences: bool,
        );

        #[cfg(all(feature = "NSDiffableDataSourceSectionSnapshot", feature = "block2"))]
        #[method(applySnapshot:toSection:animatingDifferences:completion:)]
        pub unsafe fn applySnapshot_toSection_animatingDifferences_completion(
            &self,
            snapshot: &NSDiffableDataSourceSectionSnapshot<ItemIdentifierType>,
            section_identifier: &SectionIdentifierType,
            animating_differences: bool,
            completion: Option<&block2::Block<dyn Fn()>>,
        );

        #[cfg(feature = "NSDiffableDataSourceSectionSnapshot")]
        #[method_id(@__retain_semantics Other snapshotForSection:)]
        pub unsafe fn snapshotForSection(
            &self,
            section: &SectionIdentifierType,
        ) -> Retained<NSDiffableDataSourceSectionSnapshot<ItemIdentifierType>>;

        #[method_id(@__retain_semantics Other sectionSnapshotHandlers)]
        pub unsafe fn sectionSnapshotHandlers(
            &self,
        ) -> Retained<UICollectionViewDiffableDataSourceSectionSnapshotHandlers<ItemIdentifierType>>;

        #[method(setSectionSnapshotHandlers:)]
        pub unsafe fn setSectionSnapshotHandlers(
            &self,
            section_snapshot_handlers: &UICollectionViewDiffableDataSourceSectionSnapshotHandlers<
                ItemIdentifierType,
            >,
        );
    }
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIScrollView",
    feature = "UITableView",
    feature = "UITableViewCell",
    feature = "UIView",
    feature = "block2"
))]
pub type UITableViewDiffableDataSourceCellProvider = *mut block2::Block<
    dyn Fn(NonNull<UITableView>, NonNull<NSIndexPath>, NonNull<AnyObject>) -> *mut UITableViewCell,
>;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UITableViewDiffableDataSource<
        SectionIdentifierType: ?Sized = AnyObject,
        ItemIdentifierType: ?Sized = AnyObject,
    > {
        __superclass: NSObject,
        _inner0: PhantomData<*mut SectionIdentifierType>,
        _inner1: PhantomData<*mut ItemIdentifierType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<SectionIdentifierType: ?Sized + Message, ItemIdentifierType: ?Sized + Message>
        ClassType for UITableViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
    {
        type Super = NSObject;
        type Mutability = MainThreadOnly;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }
    }
);

unsafe impl<SectionIdentifierType: ?Sized, ItemIdentifierType: ?Sized> NSObjectProtocol
    for UITableViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
{
}

#[cfg(feature = "UITableView")]
unsafe impl<SectionIdentifierType: ?Sized + Message, ItemIdentifierType: ?Sized + Message>
    UITableViewDataSource
    for UITableViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
{
}

extern_methods!(
    unsafe impl<SectionIdentifierType: Message, ItemIdentifierType: Message>
        UITableViewDiffableDataSource<SectionIdentifierType, ItemIdentifierType>
    {
        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScrollView",
            feature = "UITableView",
            feature = "UITableViewCell",
            feature = "UIView",
            feature = "block2"
        ))]
        #[method_id(@__retain_semantics Init initWithTableView:cellProvider:)]
        pub unsafe fn initWithTableView_cellProvider(
            this: Allocated<Self>,
            table_view: &UITableView,
            cell_provider: UITableViewDiffableDataSourceCellProvider,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other snapshot)]
        pub unsafe fn snapshot(
            &self,
        ) -> Retained<NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>>;

        #[method(applySnapshot:animatingDifferences:)]
        pub unsafe fn applySnapshot_animatingDifferences(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            animating_differences: bool,
        );

        #[cfg(feature = "block2")]
        #[method(applySnapshot:animatingDifferences:completion:)]
        pub unsafe fn applySnapshot_animatingDifferences_completion(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            animating_differences: bool,
            completion: Option<&block2::Block<dyn Fn()>>,
        );

        #[method(applySnapshotUsingReloadData:)]
        pub unsafe fn applySnapshotUsingReloadData(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
        );

        #[cfg(feature = "block2")]
        #[method(applySnapshotUsingReloadData:completion:)]
        pub unsafe fn applySnapshotUsingReloadData_completion(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            completion: Option<&block2::Block<dyn Fn()>>,
        );

        #[method_id(@__retain_semantics Other sectionIdentifierForIndex:)]
        pub unsafe fn sectionIdentifierForIndex(
            &self,
            index: NSInteger,
        ) -> Option<Retained<SectionIdentifierType>>;

        #[method(indexForSectionIdentifier:)]
        pub unsafe fn indexForSectionIdentifier(
            &self,
            identifier: &SectionIdentifierType,
        ) -> NSInteger;

        #[method_id(@__retain_semantics Other itemIdentifierForIndexPath:)]
        pub unsafe fn itemIdentifierForIndexPath(
            &self,
            index_path: &NSIndexPath,
        ) -> Option<Retained<ItemIdentifierType>>;

        #[method_id(@__retain_semantics Other indexPathForItemIdentifier:)]
        pub unsafe fn indexPathForItemIdentifier(
            &self,
            identifier: &ItemIdentifierType,
        ) -> Option<Retained<NSIndexPath>>;

        #[cfg(feature = "UITableView")]
        #[method(defaultRowAnimation)]
        pub unsafe fn defaultRowAnimation(&self) -> UITableViewRowAnimation;

        #[cfg(feature = "UITableView")]
        #[method(setDefaultRowAnimation:)]
        pub unsafe fn setDefaultRowAnimation(&self, default_row_animation: UITableViewRowAnimation);
    }
);
