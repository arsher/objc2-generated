//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionlayoutlistappearance?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UICollectionLayoutListAppearance(pub NSInteger);
impl UICollectionLayoutListAppearance {
    #[doc(alias = "UICollectionLayoutListAppearancePlain")]
    pub const Plain: Self = Self(0);
    #[doc(alias = "UICollectionLayoutListAppearanceGrouped")]
    pub const Grouped: Self = Self(1);
    #[doc(alias = "UICollectionLayoutListAppearanceInsetGrouped")]
    pub const InsetGrouped: Self = Self(2);
    #[doc(alias = "UICollectionLayoutListAppearanceSidebar")]
    pub const Sidebar: Self = Self(3);
    #[doc(alias = "UICollectionLayoutListAppearanceSidebarPlain")]
    pub const SidebarPlain: Self = Self(4);
}

unsafe impl Encode for UICollectionLayoutListAppearance {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UICollectionLayoutListAppearance {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionlayoutlistheadermode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UICollectionLayoutListHeaderMode(pub NSInteger);
impl UICollectionLayoutListHeaderMode {
    /// No headers are shown
    #[doc(alias = "UICollectionLayoutListHeaderModeNone")]
    pub const None: Self = Self(0);
    /// Uses supplementary views of kind UICollectionElementKindSectionHeader to show headers
    #[doc(alias = "UICollectionLayoutListHeaderModeSupplementary")]
    pub const Supplementary: Self = Self(1);
    /// Styles the first item in a section as a header. This is especially useful when using hierarchical data sources to be able to expand and collapse the header.
    #[doc(alias = "UICollectionLayoutListHeaderModeFirstItemInSection")]
    pub const FirstItemInSection: Self = Self(2);
}

unsafe impl Encode for UICollectionLayoutListHeaderMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UICollectionLayoutListHeaderMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionlayoutlistfootermode?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UICollectionLayoutListFooterMode(pub NSInteger);
impl UICollectionLayoutListFooterMode {
    /// No footers are shown
    #[doc(alias = "UICollectionLayoutListFooterModeNone")]
    pub const None: Self = Self(0);
    /// Uses supplementary views of kind UICollectionElementKindSectionFooter to show footers
    #[doc(alias = "UICollectionLayoutListFooterModeSupplementary")]
    pub const Supplementary: Self = Self(1);
}

unsafe impl Encode for UICollectionLayoutListFooterMode {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UICollectionLayoutListFooterMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionlayoutlistswipeactionsconfigurationprovider?language=objc)
#[cfg(all(feature = "UISwipeActionsConfiguration", feature = "block2"))]
pub type UICollectionLayoutListSwipeActionsConfigurationProvider =
    *mut block2::Block<dyn Fn(NonNull<NSIndexPath>) -> *mut UISwipeActionsConfiguration>;

/// A block that is executed by list sections to provide granular control over separator appearance.
///
///
/// Parameter `itemIndexPath`: The index path of the item for which separators are being configured.
///
/// Parameter `sectionSeparatorConfiguration`: The list section's separator configuration for this cell. This configuration contains
/// the values for separator visibility and insets according to the current state of the item.
///
///
/// Returns: The configuration to use for separators at
/// `itemIndexPath.`
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionlayoutlistitemseparatorhandler?language=objc)
#[cfg(all(feature = "UIListSeparatorConfiguration", feature = "block2"))]
pub type UICollectionLayoutListItemSeparatorHandler = *mut block2::Block<
    dyn Fn(
        NonNull<NSIndexPath>,
        NonNull<UIListSeparatorConfiguration>,
    ) -> NonNull<UIListSeparatorConfiguration>,
>;

/// A setting for which items in the layout should tightly hug their content
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionlayoutlistcontenthuggingelements?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UICollectionLayoutListContentHuggingElements(pub NSUInteger);
bitflags::bitflags! {
    impl UICollectionLayoutListContentHuggingElements: NSUInteger {
        #[doc(alias = "UICollectionLayoutListContentHuggingElementsNone")]
        const None = 0;
        #[doc(alias = "UICollectionLayoutListContentHuggingElementsSupplementaryHeader")]
        const SupplementaryHeader = 1<<0;
    }
}

unsafe impl Encode for UICollectionLayoutListContentHuggingElements {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UICollectionLayoutListContentHuggingElements {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// A list configuration can be used to layout a section inside a UICollectionViewCompositionalLayout as a list.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionlayoutlistconfiguration?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UICollectionLayoutListConfiguration;
);

unsafe impl NSCopying for UICollectionLayoutListConfiguration {}

unsafe impl CopyingHelper for UICollectionLayoutListConfiguration {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UICollectionLayoutListConfiguration {}

extern_methods!(
    unsafe impl UICollectionLayoutListConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithAppearance:)]
        pub unsafe fn initWithAppearance(
            this: Allocated<Self>,
            appearance: UICollectionLayoutListAppearance,
        ) -> Retained<Self>;

        /// The overall appearance of the section.
        #[method(appearance)]
        pub unsafe fn appearance(&self) -> UICollectionLayoutListAppearance;

        /// Whether this section shows separators or not. For additional control, see separatorConfiguration.
        /// Note that when this property is NO, the separatorConfiguration is ineffective.
        #[method(showsSeparators)]
        pub unsafe fn showsSeparators(&self) -> bool;

        /// Setter for [`showsSeparators`][Self::showsSeparators].
        #[method(setShowsSeparators:)]
        pub unsafe fn setShowsSeparators(&self, shows_separators: bool);

        #[cfg(feature = "UIListSeparatorConfiguration")]
        /// The preferred configuration for separators. Used as a baseline for a section in a list using this
        /// `UICollectionLayoutListConfiguration`
        #[method_id(@__retain_semantics Other separatorConfiguration)]
        pub unsafe fn separatorConfiguration(&self) -> Retained<UIListSeparatorConfiguration>;

        #[cfg(feature = "UIListSeparatorConfiguration")]
        /// Setter for [`separatorConfiguration`][Self::separatorConfiguration].
        #[method(setSeparatorConfiguration:)]
        pub unsafe fn setSeparatorConfiguration(
            &self,
            separator_configuration: &UIListSeparatorConfiguration,
        );

        #[cfg(all(feature = "UIListSeparatorConfiguration", feature = "block2"))]
        /// This handler is executed when the list section is configuring separator appearance for an item. The index path for the item being configured and
        /// a resolved separator configuration are passed in to this block. The configuration returned from this block will be treated as the final
        /// separator configuration for this item.
        #[method(itemSeparatorHandler)]
        pub unsafe fn itemSeparatorHandler(&self) -> UICollectionLayoutListItemSeparatorHandler;

        #[cfg(all(feature = "UIListSeparatorConfiguration", feature = "block2"))]
        /// Setter for [`itemSeparatorHandler`][Self::itemSeparatorHandler].
        #[method(setItemSeparatorHandler:)]
        pub unsafe fn setItemSeparatorHandler(
            &self,
            item_separator_handler: UICollectionLayoutListItemSeparatorHandler,
        );

        #[cfg(feature = "UIColor")]
        /// The background color of the section.
        /// Defaults to nil, indicating the system background color for the specified appearance is used.
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        /// Setter for [`backgroundColor`][Self::backgroundColor].
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&UIColor>);

        #[cfg(all(feature = "UISwipeActionsConfiguration", feature = "block2"))]
        /// Called when list is about to show leading swipe actions for a particular index path.
        /// Return either a UISwipeActionsConfiguration object or nil if this index path does not show swipe actions.
        #[method(leadingSwipeActionsConfigurationProvider)]
        pub unsafe fn leadingSwipeActionsConfigurationProvider(
            &self,
        ) -> UICollectionLayoutListSwipeActionsConfigurationProvider;

        #[cfg(all(feature = "UISwipeActionsConfiguration", feature = "block2"))]
        /// Setter for [`leadingSwipeActionsConfigurationProvider`][Self::leadingSwipeActionsConfigurationProvider].
        #[method(setLeadingSwipeActionsConfigurationProvider:)]
        pub unsafe fn setLeadingSwipeActionsConfigurationProvider(
            &self,
            leading_swipe_actions_configuration_provider: UICollectionLayoutListSwipeActionsConfigurationProvider,
        );

        #[cfg(all(feature = "UISwipeActionsConfiguration", feature = "block2"))]
        /// Called when list is about to show trailing swipe actions for a particular index path.
        /// Return either a UISwipeActionsConfiguration object or nil if this index path does not show swipe actions.
        #[method(trailingSwipeActionsConfigurationProvider)]
        pub unsafe fn trailingSwipeActionsConfigurationProvider(
            &self,
        ) -> UICollectionLayoutListSwipeActionsConfigurationProvider;

        #[cfg(all(feature = "UISwipeActionsConfiguration", feature = "block2"))]
        /// Setter for [`trailingSwipeActionsConfigurationProvider`][Self::trailingSwipeActionsConfigurationProvider].
        #[method(setTrailingSwipeActionsConfigurationProvider:)]
        pub unsafe fn setTrailingSwipeActionsConfigurationProvider(
            &self,
            trailing_swipe_actions_configuration_provider: UICollectionLayoutListSwipeActionsConfigurationProvider,
        );

        /// Defines whether the section has a header. Defaults to UICollectionLayoutListHeaderModeNone.
        #[method(headerMode)]
        pub unsafe fn headerMode(&self) -> UICollectionLayoutListHeaderMode;

        /// Setter for [`headerMode`][Self::headerMode].
        #[method(setHeaderMode:)]
        pub unsafe fn setHeaderMode(&self, header_mode: UICollectionLayoutListHeaderMode);

        /// Defines whether the section has a footer. Defaults to UICollectionLayoutListFooterModeNone.
        #[method(footerMode)]
        pub unsafe fn footerMode(&self) -> UICollectionLayoutListFooterMode;

        /// Setter for [`footerMode`][Self::footerMode].
        #[method(setFooterMode:)]
        pub unsafe fn setFooterMode(&self, footer_mode: UICollectionLayoutListFooterMode);

        #[cfg(feature = "objc2-core-foundation")]
        /// Padding above each section header. The default value is `UICollectionViewLayoutAutomaticDimension`
        #[method(headerTopPadding)]
        pub unsafe fn headerTopPadding(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        /// Setter for [`headerTopPadding`][Self::headerTopPadding].
        #[method(setHeaderTopPadding:)]
        pub unsafe fn setHeaderTopPadding(&self, header_top_padding: CGFloat);

        /// Determines the type of items that will tightly hug their content.
        ///
        /// The default value for this property is `UICollectionLayoutListContentHuggingElementsSupplementaryHeader` on visionOS for plain style table views and an empty set on all other platforms.
        /// When the value of this property is `UICollectionLayoutListContentHuggingElementsSupplementaryHeader`, the header view will not stretch the width of the collection view if its content's intrinsic content size is less than the collection view's width.
        #[method(contentHuggingElements)]
        pub unsafe fn contentHuggingElements(&self)
            -> UICollectionLayoutListContentHuggingElements;

        /// Setter for [`contentHuggingElements`][Self::contentHuggingElements].
        #[method(setContentHuggingElements:)]
        pub unsafe fn setContentHuggingElements(
            &self,
            content_hugging_elements: UICollectionLayoutListContentHuggingElements,
        );
    }
);

extern_methods!(
    /// UICollectionLayoutListSection
    #[cfg(feature = "UICollectionViewCompositionalLayout")]
    unsafe impl NSCollectionLayoutSection {
        /// Creates a list section using the specified configuration. You should pass the layoutEnvironment from inside the UICollectionViewCompositionalLayoutSectionProvider.
        #[method_id(@__retain_semantics Other sectionWithListConfiguration:layoutEnvironment:)]
        pub unsafe fn sectionWithListConfiguration_layoutEnvironment(
            configuration: &UICollectionLayoutListConfiguration,
            layout_environment: &ProtocolObject<dyn NSCollectionLayoutEnvironment>,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// UICollectionLayoutListSection
    #[cfg(all(
        feature = "UICollectionViewCompositionalLayout",
        feature = "UICollectionViewLayout"
    ))]
    unsafe impl UICollectionViewCompositionalLayout {
        /// Creates a compositional layout containing only list sections of the specified configuration.
        #[method_id(@__retain_semantics Other layoutWithListConfiguration:)]
        pub unsafe fn layoutWithListConfiguration(
            configuration: &UICollectionLayoutListConfiguration,
        ) -> Retained<Self>;
    }
);
