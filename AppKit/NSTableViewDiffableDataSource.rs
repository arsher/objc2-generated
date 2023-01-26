//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSTableViewDiffableDataSourceCellProvider = *mut Block<
    (
        NonNull<NSTableView>,
        NonNull<NSTableColumn>,
        NSInteger,
        NonNull<Object>,
    ),
    NonNull<NSView>,
>;

pub type NSTableViewDiffableDataSourceRowProvider =
    *mut Block<(NonNull<NSTableView>, NSInteger, NonNull<Object>), NonNull<NSTableRowView>>;

pub type NSTableViewDiffableDataSourceSectionHeaderViewProvider =
    *mut Block<(NonNull<NSTableView>, NSInteger, NonNull<Object>), NonNull<NSView>>;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTableViewDiffableDataSource")]
    pub struct NSTableViewDiffableDataSource<
        SectionIdentifierType: Message = Object,
        ItemIdentifierType: Message = Object,
        SectionIdentifierTypeOwnership: Ownership = Shared,
        ItemIdentifierTypeOwnership: Ownership = Shared,
    > {
        _inner0: PhantomData<*mut (SectionIdentifierType, SectionIdentifierTypeOwnership)>,
        _inner1: PhantomData<*mut (ItemIdentifierType, ItemIdentifierTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "AppKit_NSTableViewDiffableDataSource")]
    unsafe impl<
            SectionIdentifierType: Message,
            ItemIdentifierType: Message,
            SectionIdentifierTypeOwnership: Ownership,
            ItemIdentifierTypeOwnership: Ownership,
        > ClassType
        for NSTableViewDiffableDataSource<
            SectionIdentifierType,
            ItemIdentifierType,
            SectionIdentifierTypeOwnership,
            ItemIdentifierTypeOwnership,
        >
    {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSTableViewDiffableDataSource")]
unsafe impl<
        SectionIdentifierType: Message,
        ItemIdentifierType: Message,
        SectionIdentifierTypeOwnership: Ownership,
        ItemIdentifierTypeOwnership: Ownership,
    > NSObjectProtocol
    for NSTableViewDiffableDataSource<
        SectionIdentifierType,
        ItemIdentifierType,
        SectionIdentifierTypeOwnership,
        ItemIdentifierTypeOwnership,
    >
{
}

#[cfg(feature = "AppKit_NSTableViewDiffableDataSource")]
unsafe impl<
        SectionIdentifierType: Message,
        ItemIdentifierType: Message,
        SectionIdentifierTypeOwnership: Ownership,
        ItemIdentifierTypeOwnership: Ownership,
    > NSTableViewDataSource
    for NSTableViewDiffableDataSource<
        SectionIdentifierType,
        ItemIdentifierType,
        SectionIdentifierTypeOwnership,
        ItemIdentifierTypeOwnership,
    >
{
}

extern_methods!(
    #[cfg(feature = "AppKit_NSTableViewDiffableDataSource")]
    unsafe impl<
            SectionIdentifierType: Message,
            ItemIdentifierType: Message,
            SectionIdentifierTypeOwnership: Ownership,
            ItemIdentifierTypeOwnership: Ownership,
        >
        NSTableViewDiffableDataSource<
            SectionIdentifierType,
            ItemIdentifierType,
            SectionIdentifierTypeOwnership,
            ItemIdentifierTypeOwnership,
        >
    {
        #[cfg(feature = "AppKit_NSTableView")]
        #[method_id(@__retain_semantics Init initWithTableView:cellProvider:)]
        pub unsafe fn initWithTableView_cellProvider(
            this: Option<Allocated<Self>>,
            table_view: &NSTableView,
            cell_provider: NSTableViewDiffableDataSourceCellProvider,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSDiffableDataSourceSnapshot")]
        #[method_id(@__retain_semantics Other snapshot)]
        pub unsafe fn snapshot(
            &self,
        ) -> Id<NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>, Shared>;

        #[cfg(feature = "AppKit_NSDiffableDataSourceSnapshot")]
        #[method(applySnapshot:animatingDifferences:)]
        pub unsafe fn applySnapshot_animatingDifferences(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            animating_differences: bool,
        );

        #[cfg(feature = "AppKit_NSDiffableDataSourceSnapshot")]
        #[method(applySnapshot:animatingDifferences:completion:)]
        pub unsafe fn applySnapshot_animatingDifferences_completion(
            &self,
            snapshot: &NSDiffableDataSourceSnapshot<SectionIdentifierType, ItemIdentifierType>,
            animating_differences: bool,
            completion: Option<&Block<(), ()>>,
        );

        #[method_id(@__retain_semantics Other itemIdentifierForRow:)]
        pub unsafe fn itemIdentifierForRow(
            &self,
            row: NSInteger,
        ) -> Option<Id<ItemIdentifierType, ItemIdentifierTypeOwnership>>;

        #[method(rowForItemIdentifier:)]
        pub unsafe fn rowForItemIdentifier(&self, identifier: &ItemIdentifierType) -> NSInteger;

        #[method_id(@__retain_semantics Other sectionIdentifierForRow:)]
        pub unsafe fn sectionIdentifierForRow(
            &self,
            row: NSInteger,
        ) -> Option<Id<SectionIdentifierType, SectionIdentifierTypeOwnership>>;

        #[method(rowForSectionIdentifier:)]
        pub unsafe fn rowForSectionIdentifier(
            &self,
            identifier: &SectionIdentifierType,
        ) -> NSInteger;

        #[method(rowViewProvider)]
        pub unsafe fn rowViewProvider(&self) -> NSTableViewDiffableDataSourceRowProvider;

        #[method(setRowViewProvider:)]
        pub unsafe fn setRowViewProvider(
            &self,
            row_view_provider: NSTableViewDiffableDataSourceRowProvider,
        );

        #[method(sectionHeaderViewProvider)]
        pub unsafe fn sectionHeaderViewProvider(
            &self,
        ) -> NSTableViewDiffableDataSourceSectionHeaderViewProvider;

        #[method(setSectionHeaderViewProvider:)]
        pub unsafe fn setSectionHeaderViewProvider(
            &self,
            section_header_view_provider: NSTableViewDiffableDataSourceSectionHeaderViewProvider,
        );

        #[method(defaultRowAnimation)]
        pub unsafe fn defaultRowAnimation(&self) -> NSTableViewAnimationOptions;

        #[method(setDefaultRowAnimation:)]
        pub unsafe fn setDefaultRowAnimation(
            &self,
            default_row_animation: NSTableViewAnimationOptions,
        );
    }
);
