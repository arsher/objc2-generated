//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewlayoutautomaticdimension?language=objc)
    #[cfg(feature = "objc2-core-foundation")]
    pub static UICollectionViewLayoutAutomaticDimension: CGFloat;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionelementkindsectionheader?language=objc)
    pub static UICollectionElementKindSectionHeader: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionelementkindsectionfooter?language=objc)
    pub static UICollectionElementKindSectionFooter: &'static NSString;
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewscrolldirection?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UICollectionViewScrollDirection(pub NSInteger);
impl UICollectionViewScrollDirection {
    #[doc(alias = "UICollectionViewScrollDirectionVertical")]
    pub const Vertical: Self = Self(0);
    #[doc(alias = "UICollectionViewScrollDirectionHorizontal")]
    pub const Horizontal: Self = Self(1);
}

unsafe impl Encode for UICollectionViewScrollDirection {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UICollectionViewScrollDirection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionelementcategory?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UICollectionElementCategory(pub NSUInteger);
impl UICollectionElementCategory {
    #[doc(alias = "UICollectionElementCategoryCell")]
    pub const Cell: Self = Self(0);
    #[doc(alias = "UICollectionElementCategorySupplementaryView")]
    pub const SupplementaryView: Self = Self(1);
    #[doc(alias = "UICollectionElementCategoryDecorationView")]
    pub const DecorationView: Self = Self(2);
}

unsafe impl Encode for UICollectionElementCategory {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UICollectionElementCategory {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewlayoutattributes?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UICollectionViewLayoutAttributes;
);

unsafe impl NSCopying for UICollectionViewLayoutAttributes {}

unsafe impl CopyingHelper for UICollectionViewLayoutAttributes {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UICollectionViewLayoutAttributes {}

#[cfg(feature = "UIDynamicBehavior")]
unsafe impl UIDynamicItem for UICollectionViewLayoutAttributes {}

extern_methods!(
    unsafe impl UICollectionViewLayoutAttributes {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(frame)]
        pub unsafe fn frame(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setFrame:)]
        pub unsafe fn setFrame(&self, frame: CGRect);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(center)]
        pub unsafe fn center(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setCenter:)]
        pub unsafe fn setCenter(&self, center: CGPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(size)]
        pub unsafe fn size(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: CGSize);

        #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-quartz-core"))]
        #[cfg(not(target_os = "watchos"))]
        #[method(transform3D)]
        pub unsafe fn transform3D(&self) -> CATransform3D;

        #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-quartz-core"))]
        #[cfg(not(target_os = "watchos"))]
        #[method(setTransform3D:)]
        pub unsafe fn setTransform3D(&self, transform3_d: CATransform3D);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(bounds)]
        pub unsafe fn bounds(&self) -> CGRect;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setBounds:)]
        pub unsafe fn setBounds(&self, bounds: CGRect);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(transform)]
        pub unsafe fn transform(&self) -> CGAffineTransform;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setTransform:)]
        pub unsafe fn setTransform(&self, transform: CGAffineTransform);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(alpha)]
        pub unsafe fn alpha(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setAlpha:)]
        pub unsafe fn setAlpha(&self, alpha: CGFloat);

        #[method(zIndex)]
        pub unsafe fn zIndex(&self) -> NSInteger;

        #[method(setZIndex:)]
        pub unsafe fn setZIndex(&self, z_index: NSInteger);

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);

        #[method_id(@__retain_semantics Other indexPath)]
        pub unsafe fn indexPath(&self) -> Retained<NSIndexPath>;

        #[method(setIndexPath:)]
        pub unsafe fn setIndexPath(&self, index_path: &NSIndexPath);

        #[method(representedElementCategory)]
        pub unsafe fn representedElementCategory(&self) -> UICollectionElementCategory;

        #[method_id(@__retain_semantics Other representedElementKind)]
        pub unsafe fn representedElementKind(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other layoutAttributesForCellWithIndexPath:)]
        pub unsafe fn layoutAttributesForCellWithIndexPath(
            index_path: &NSIndexPath,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other layoutAttributesForSupplementaryViewOfKind:withIndexPath:)]
        pub unsafe fn layoutAttributesForSupplementaryViewOfKind_withIndexPath(
            element_kind: &NSString,
            index_path: &NSIndexPath,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other layoutAttributesForDecorationViewOfKind:withIndexPath:)]
        pub unsafe fn layoutAttributesForDecorationViewOfKind_withIndexPath(
            decoration_view_kind: &NSString,
            index_path: &NSIndexPath,
            mtm: MainThreadMarker,
        ) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UICollectionViewLayoutAttributes {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewlayoutinvalidationcontext?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UICollectionViewLayoutInvalidationContext;
);

unsafe impl NSObjectProtocol for UICollectionViewLayoutInvalidationContext {}

extern_methods!(
    unsafe impl UICollectionViewLayoutInvalidationContext {
        #[method(invalidateEverything)]
        pub unsafe fn invalidateEverything(&self) -> bool;

        #[method(invalidateDataSourceCounts)]
        pub unsafe fn invalidateDataSourceCounts(&self) -> bool;

        #[method(invalidateItemsAtIndexPaths:)]
        pub unsafe fn invalidateItemsAtIndexPaths(&self, index_paths: &NSArray<NSIndexPath>);

        #[method(invalidateSupplementaryElementsOfKind:atIndexPaths:)]
        pub unsafe fn invalidateSupplementaryElementsOfKind_atIndexPaths(
            &self,
            element_kind: &NSString,
            index_paths: &NSArray<NSIndexPath>,
        );

        #[method(invalidateDecorationElementsOfKind:atIndexPaths:)]
        pub unsafe fn invalidateDecorationElementsOfKind_atIndexPaths(
            &self,
            element_kind: &NSString,
            index_paths: &NSArray<NSIndexPath>,
        );

        #[method_id(@__retain_semantics Other invalidatedItemIndexPaths)]
        pub unsafe fn invalidatedItemIndexPaths(&self) -> Option<Retained<NSArray<NSIndexPath>>>;

        #[method_id(@__retain_semantics Other invalidatedSupplementaryIndexPaths)]
        pub unsafe fn invalidatedSupplementaryIndexPaths(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, NSArray<NSIndexPath>>>>;

        #[method_id(@__retain_semantics Other invalidatedDecorationIndexPaths)]
        pub unsafe fn invalidatedDecorationIndexPaths(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, NSArray<NSIndexPath>>>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(contentOffsetAdjustment)]
        pub unsafe fn contentOffsetAdjustment(&self) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setContentOffsetAdjustment:)]
        pub unsafe fn setContentOffsetAdjustment(&self, content_offset_adjustment: CGPoint);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(contentSizeAdjustment)]
        pub unsafe fn contentSizeAdjustment(&self) -> CGSize;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setContentSizeAdjustment:)]
        pub unsafe fn setContentSizeAdjustment(&self, content_size_adjustment: CGSize);

        #[method_id(@__retain_semantics Other previousIndexPathsForInteractivelyMovingItems)]
        pub unsafe fn previousIndexPathsForInteractivelyMovingItems(
            &self,
        ) -> Option<Retained<NSArray<NSIndexPath>>>;

        #[method_id(@__retain_semantics Other targetIndexPathsForInteractivelyMovingItems)]
        pub unsafe fn targetIndexPathsForInteractivelyMovingItems(
            &self,
        ) -> Option<Retained<NSArray<NSIndexPath>>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(interactiveMovementTarget)]
        pub unsafe fn interactiveMovementTarget(&self) -> CGPoint;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UICollectionViewLayoutInvalidationContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uicollectionviewlayout?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UICollectionViewLayout;
);

unsafe impl NSCoding for UICollectionViewLayout {}

unsafe impl NSObjectProtocol for UICollectionViewLayout {}

extern_methods!(
    unsafe impl UICollectionViewLayout {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[cfg(all(
            feature = "UICollectionView",
            feature = "UIResponder",
            feature = "UIScrollView",
            feature = "UIView"
        ))]
        #[method_id(@__retain_semantics Other collectionView)]
        pub unsafe fn collectionView(&self) -> Option<Retained<UICollectionView>>;

        #[method(invalidateLayout)]
        pub unsafe fn invalidateLayout(&self);

        #[method(invalidateLayoutWithContext:)]
        pub unsafe fn invalidateLayoutWithContext(
            &self,
            context: &UICollectionViewLayoutInvalidationContext,
        );

        #[method(registerClass:forDecorationViewOfKind:)]
        pub unsafe fn registerClass_forDecorationViewOfKind(
            &self,
            view_class: Option<&AnyClass>,
            element_kind: &NSString,
        );

        #[cfg(feature = "UINib")]
        #[deprecated = "Loading Interface Builder products will not be supported in a future version of visionOS."]
        #[method(registerNib:forDecorationViewOfKind:)]
        pub unsafe fn registerNib_forDecorationViewOfKind(
            &self,
            nib: Option<&UINib>,
            element_kind: &NSString,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UICollectionViewLayout {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    /// UISubclassingHooks
    unsafe impl UICollectionViewLayout {
        #[method(layoutAttributesClass)]
        pub unsafe fn layoutAttributesClass(mtm: MainThreadMarker) -> &'static AnyClass;

        #[method(invalidationContextClass)]
        pub unsafe fn invalidationContextClass(mtm: MainThreadMarker) -> &'static AnyClass;

        #[method(prepareLayout)]
        pub unsafe fn prepareLayout(&self);

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other layoutAttributesForElementsInRect:)]
        pub unsafe fn layoutAttributesForElementsInRect(
            &self,
            rect: CGRect,
        ) -> Option<Retained<NSArray<UICollectionViewLayoutAttributes>>>;

        #[method_id(@__retain_semantics Other layoutAttributesForItemAtIndexPath:)]
        pub unsafe fn layoutAttributesForItemAtIndexPath(
            &self,
            index_path: &NSIndexPath,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;

        #[method_id(@__retain_semantics Other layoutAttributesForSupplementaryViewOfKind:atIndexPath:)]
        pub unsafe fn layoutAttributesForSupplementaryViewOfKind_atIndexPath(
            &self,
            element_kind: &NSString,
            index_path: &NSIndexPath,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;

        #[method_id(@__retain_semantics Other layoutAttributesForDecorationViewOfKind:atIndexPath:)]
        pub unsafe fn layoutAttributesForDecorationViewOfKind_atIndexPath(
            &self,
            element_kind: &NSString,
            index_path: &NSIndexPath,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(shouldInvalidateLayoutForBoundsChange:)]
        pub unsafe fn shouldInvalidateLayoutForBoundsChange(&self, new_bounds: CGRect) -> bool;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other invalidationContextForBoundsChange:)]
        pub unsafe fn invalidationContextForBoundsChange(
            &self,
            new_bounds: CGRect,
        ) -> Retained<UICollectionViewLayoutInvalidationContext>;

        #[method(shouldInvalidateLayoutForPreferredLayoutAttributes:withOriginalAttributes:)]
        pub unsafe fn shouldInvalidateLayoutForPreferredLayoutAttributes_withOriginalAttributes(
            &self,
            preferred_attributes: &UICollectionViewLayoutAttributes,
            original_attributes: &UICollectionViewLayoutAttributes,
        ) -> bool;

        #[method_id(@__retain_semantics Other invalidationContextForPreferredLayoutAttributes:withOriginalAttributes:)]
        pub unsafe fn invalidationContextForPreferredLayoutAttributes_withOriginalAttributes(
            &self,
            preferred_attributes: &UICollectionViewLayoutAttributes,
            original_attributes: &UICollectionViewLayoutAttributes,
        ) -> Retained<UICollectionViewLayoutInvalidationContext>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(targetContentOffsetForProposedContentOffset:withScrollingVelocity:)]
        pub unsafe fn targetContentOffsetForProposedContentOffset_withScrollingVelocity(
            &self,
            proposed_content_offset: CGPoint,
            velocity: CGPoint,
        ) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(targetContentOffsetForProposedContentOffset:)]
        pub unsafe fn targetContentOffsetForProposedContentOffset(
            &self,
            proposed_content_offset: CGPoint,
        ) -> CGPoint;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(collectionViewContentSize)]
        pub unsafe fn collectionViewContentSize(&self) -> CGSize;

        #[cfg(feature = "UIInterface")]
        #[method(developmentLayoutDirection)]
        pub unsafe fn developmentLayoutDirection(&self) -> UIUserInterfaceLayoutDirection;

        #[method(flipsHorizontallyInOppositeLayoutDirection)]
        pub unsafe fn flipsHorizontallyInOppositeLayoutDirection(&self) -> bool;
    }
);

extern_methods!(
    /// UIUpdateSupportHooks
    unsafe impl UICollectionViewLayout {
        #[cfg(feature = "UICollectionViewUpdateItem")]
        #[method(prepareForCollectionViewUpdates:)]
        pub unsafe fn prepareForCollectionViewUpdates(
            &self,
            update_items: &NSArray<UICollectionViewUpdateItem>,
        );

        #[method(finalizeCollectionViewUpdates)]
        pub unsafe fn finalizeCollectionViewUpdates(&self);

        #[cfg(feature = "objc2-core-foundation")]
        #[method(prepareForAnimatedBoundsChange:)]
        pub unsafe fn prepareForAnimatedBoundsChange(&self, old_bounds: CGRect);

        #[method(finalizeAnimatedBoundsChange)]
        pub unsafe fn finalizeAnimatedBoundsChange(&self);

        #[method(prepareForTransitionToLayout:)]
        pub unsafe fn prepareForTransitionToLayout(&self, new_layout: &UICollectionViewLayout);

        #[method(prepareForTransitionFromLayout:)]
        pub unsafe fn prepareForTransitionFromLayout(&self, old_layout: &UICollectionViewLayout);

        #[method(finalizeLayoutTransition)]
        pub unsafe fn finalizeLayoutTransition(&self);

        #[method_id(@__retain_semantics Other initialLayoutAttributesForAppearingItemAtIndexPath:)]
        pub unsafe fn initialLayoutAttributesForAppearingItemAtIndexPath(
            &self,
            item_index_path: &NSIndexPath,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;

        #[method_id(@__retain_semantics Other finalLayoutAttributesForDisappearingItemAtIndexPath:)]
        pub unsafe fn finalLayoutAttributesForDisappearingItemAtIndexPath(
            &self,
            item_index_path: &NSIndexPath,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;

        #[method_id(@__retain_semantics Other initialLayoutAttributesForAppearingSupplementaryElementOfKind:atIndexPath:)]
        pub unsafe fn initialLayoutAttributesForAppearingSupplementaryElementOfKind_atIndexPath(
            &self,
            element_kind: &NSString,
            element_index_path: &NSIndexPath,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;

        #[method_id(@__retain_semantics Other finalLayoutAttributesForDisappearingSupplementaryElementOfKind:atIndexPath:)]
        pub unsafe fn finalLayoutAttributesForDisappearingSupplementaryElementOfKind_atIndexPath(
            &self,
            element_kind: &NSString,
            element_index_path: &NSIndexPath,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;

        #[method_id(@__retain_semantics Other initialLayoutAttributesForAppearingDecorationElementOfKind:atIndexPath:)]
        pub unsafe fn initialLayoutAttributesForAppearingDecorationElementOfKind_atIndexPath(
            &self,
            element_kind: &NSString,
            decoration_index_path: &NSIndexPath,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;

        #[method_id(@__retain_semantics Other finalLayoutAttributesForDisappearingDecorationElementOfKind:atIndexPath:)]
        pub unsafe fn finalLayoutAttributesForDisappearingDecorationElementOfKind_atIndexPath(
            &self,
            element_kind: &NSString,
            decoration_index_path: &NSIndexPath,
        ) -> Option<Retained<UICollectionViewLayoutAttributes>>;

        #[method_id(@__retain_semantics Other indexPathsToDeleteForSupplementaryViewOfKind:)]
        pub unsafe fn indexPathsToDeleteForSupplementaryViewOfKind(
            &self,
            element_kind: &NSString,
        ) -> Retained<NSArray<NSIndexPath>>;

        #[method_id(@__retain_semantics Other indexPathsToDeleteForDecorationViewOfKind:)]
        pub unsafe fn indexPathsToDeleteForDecorationViewOfKind(
            &self,
            element_kind: &NSString,
        ) -> Retained<NSArray<NSIndexPath>>;

        #[method_id(@__retain_semantics Other indexPathsToInsertForSupplementaryViewOfKind:)]
        pub unsafe fn indexPathsToInsertForSupplementaryViewOfKind(
            &self,
            element_kind: &NSString,
        ) -> Retained<NSArray<NSIndexPath>>;

        #[method_id(@__retain_semantics Other indexPathsToInsertForDecorationViewOfKind:)]
        pub unsafe fn indexPathsToInsertForDecorationViewOfKind(
            &self,
            element_kind: &NSString,
        ) -> Retained<NSArray<NSIndexPath>>;
    }
);

extern_methods!(
    /// UIReorderingSupportHooks
    unsafe impl UICollectionViewLayout {
        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other targetIndexPathForInteractivelyMovingItem:withPosition:)]
        pub unsafe fn targetIndexPathForInteractivelyMovingItem_withPosition(
            &self,
            previous_index_path: &NSIndexPath,
            position: CGPoint,
        ) -> Retained<NSIndexPath>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other layoutAttributesForInteractivelyMovingItemAtIndexPath:withTargetPosition:)]
        pub unsafe fn layoutAttributesForInteractivelyMovingItemAtIndexPath_withTargetPosition(
            &self,
            index_path: &NSIndexPath,
            position: CGPoint,
        ) -> Retained<UICollectionViewLayoutAttributes>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Other invalidationContextForInteractivelyMovingItems:withTargetPosition:previousIndexPaths:previousPosition:)]
        pub unsafe fn invalidationContextForInteractivelyMovingItems_withTargetPosition_previousIndexPaths_previousPosition(
            &self,
            target_index_paths: &NSArray<NSIndexPath>,
            target_position: CGPoint,
            previous_index_paths: &NSArray<NSIndexPath>,
            previous_position: CGPoint,
        ) -> Retained<UICollectionViewLayoutInvalidationContext>;

        #[method_id(@__retain_semantics Other invalidationContextForEndingInteractiveMovementOfItemsToFinalIndexPaths:previousIndexPaths:movementCancelled:)]
        pub unsafe fn invalidationContextForEndingInteractiveMovementOfItemsToFinalIndexPaths_previousIndexPaths_movementCancelled(
            &self,
            index_paths: &NSArray<NSIndexPath>,
            previous_index_paths: &NSArray<NSIndexPath>,
            movement_cancelled: bool,
        ) -> Retained<UICollectionViewLayoutInvalidationContext>;
    }
);
