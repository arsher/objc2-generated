//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitabbarsystemitem?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UITabBarSystemItem(pub NSInteger);
impl UITabBarSystemItem {
    #[doc(alias = "UITabBarSystemItemMore")]
    pub const More: Self = Self(0);
    #[doc(alias = "UITabBarSystemItemFavorites")]
    pub const Favorites: Self = Self(1);
    #[doc(alias = "UITabBarSystemItemFeatured")]
    pub const Featured: Self = Self(2);
    #[doc(alias = "UITabBarSystemItemTopRated")]
    pub const TopRated: Self = Self(3);
    #[doc(alias = "UITabBarSystemItemRecents")]
    pub const Recents: Self = Self(4);
    #[doc(alias = "UITabBarSystemItemContacts")]
    pub const Contacts: Self = Self(5);
    #[doc(alias = "UITabBarSystemItemHistory")]
    pub const History: Self = Self(6);
    #[doc(alias = "UITabBarSystemItemBookmarks")]
    pub const Bookmarks: Self = Self(7);
    #[doc(alias = "UITabBarSystemItemSearch")]
    pub const Search: Self = Self(8);
    #[doc(alias = "UITabBarSystemItemDownloads")]
    pub const Downloads: Self = Self(9);
    #[doc(alias = "UITabBarSystemItemMostRecent")]
    pub const MostRecent: Self = Self(10);
    #[doc(alias = "UITabBarSystemItemMostViewed")]
    pub const MostViewed: Self = Self(11);
}

unsafe impl Encode for UITabBarSystemItem {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UITabBarSystemItem {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uitabbaritem?language=objc)
    #[unsafe(super(UIBarItem, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UIBarItem")]
    pub struct UITabBarItem;
);

#[cfg(feature = "UIBarItem")]
unsafe impl NSCoding for UITabBarItem {}

#[cfg(feature = "UIBarItem")]
unsafe impl NSObjectProtocol for UITabBarItem {}

#[cfg(all(feature = "UIAppearance", feature = "UIBarItem"))]
unsafe impl UIAppearance for UITabBarItem {}

extern_methods!(
    #[cfg(feature = "UIBarItem")]
    unsafe impl UITabBarItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Init initWithTitle:image:tag:)]
        pub unsafe fn initWithTitle_image_tag(
            this: Allocated<Self>,
            title: Option<&NSString>,
            image: Option<&UIImage>,
            tag: NSInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Init initWithTitle:image:selectedImage:)]
        pub unsafe fn initWithTitle_image_selectedImage(
            this: Allocated<Self>,
            title: Option<&NSString>,
            image: Option<&UIImage>,
            selected_image: Option<&UIImage>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithTabBarSystemItem:tag:)]
        pub unsafe fn initWithTabBarSystemItem_tag(
            this: Allocated<Self>,
            system_item: UITabBarSystemItem,
            tag: NSInteger,
        ) -> Retained<Self>;

        #[cfg(feature = "UIImage")]
        #[method_id(@__retain_semantics Other selectedImage)]
        pub unsafe fn selectedImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[method(setSelectedImage:)]
        pub unsafe fn setSelectedImage(&self, selected_image: Option<&UIImage>);

        #[method_id(@__retain_semantics Other badgeValue)]
        pub unsafe fn badgeValue(&self) -> Option<Retained<NSString>>;

        #[method(setBadgeValue:)]
        pub unsafe fn setBadgeValue(&self, badge_value: Option<&NSString>);

        #[cfg(feature = "UIImage")]
        #[deprecated = "Use initWithTitle:image:selectedImage: or the image and selectedImage properties along with UIImageRenderingModeAlwaysOriginal"]
        #[method(setFinishedSelectedImage:withFinishedUnselectedImage:)]
        pub unsafe fn setFinishedSelectedImage_withFinishedUnselectedImage(
            &self,
            selected_image: Option<&UIImage>,
            unselected_image: Option<&UIImage>,
        );

        #[cfg(feature = "UIImage")]
        #[deprecated]
        #[method_id(@__retain_semantics Other finishedSelectedImage)]
        pub unsafe fn finishedSelectedImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(feature = "UIImage")]
        #[deprecated]
        #[method_id(@__retain_semantics Other finishedUnselectedImage)]
        pub unsafe fn finishedUnselectedImage(&self) -> Option<Retained<UIImage>>;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(titlePositionAdjustment)]
        pub unsafe fn titlePositionAdjustment(&self) -> UIOffset;

        #[cfg(all(feature = "UIGeometry", feature = "objc2-core-foundation"))]
        #[method(setTitlePositionAdjustment:)]
        pub unsafe fn setTitlePositionAdjustment(&self, title_position_adjustment: UIOffset);

        #[cfg(feature = "UIColor")]
        #[method_id(@__retain_semantics Other badgeColor)]
        pub unsafe fn badgeColor(&self) -> Option<Retained<UIColor>>;

        #[cfg(feature = "UIColor")]
        #[method(setBadgeColor:)]
        pub unsafe fn setBadgeColor(&self, badge_color: Option<&UIColor>);

        #[cfg(feature = "UIControl")]
        #[method(setBadgeTextAttributes:forState:)]
        pub unsafe fn setBadgeTextAttributes_forState(
            &self,
            text_attributes: Option<&NSDictionary<NSAttributedStringKey, AnyObject>>,
            state: UIControlState,
        );

        #[cfg(feature = "UIControl")]
        #[method_id(@__retain_semantics Other badgeTextAttributesForState:)]
        pub unsafe fn badgeTextAttributesForState(
            &self,
            state: UIControlState,
        ) -> Option<Retained<NSDictionary<NSAttributedStringKey, AnyObject>>>;

        #[cfg(all(feature = "UIBarAppearance", feature = "UITabBarAppearance"))]
        #[method_id(@__retain_semantics Other standardAppearance)]
        pub unsafe fn standardAppearance(&self) -> Option<Retained<UITabBarAppearance>>;

        #[cfg(all(feature = "UIBarAppearance", feature = "UITabBarAppearance"))]
        #[method(setStandardAppearance:)]
        pub unsafe fn setStandardAppearance(
            &self,
            standard_appearance: Option<&UITabBarAppearance>,
        );

        #[cfg(all(feature = "UIBarAppearance", feature = "UITabBarAppearance"))]
        #[method_id(@__retain_semantics Other scrollEdgeAppearance)]
        pub unsafe fn scrollEdgeAppearance(&self) -> Option<Retained<UITabBarAppearance>>;

        #[cfg(all(feature = "UIBarAppearance", feature = "UITabBarAppearance"))]
        #[method(setScrollEdgeAppearance:)]
        pub unsafe fn setScrollEdgeAppearance(
            &self,
            scroll_edge_appearance: Option<&UITabBarAppearance>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UIBarItem")]
    unsafe impl UITabBarItem {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// SpringLoading
    #[cfg(feature = "UIBarItem")]
    unsafe impl UITabBarItem {}
);

#[cfg(all(feature = "UIBarItem", feature = "UISpringLoadedInteractionSupporting"))]
unsafe impl UISpringLoadedInteractionSupporting for UITabBarItem {}
