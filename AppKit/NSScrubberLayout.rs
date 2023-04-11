//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSScrubberLayoutAttributes")]
    pub struct NSScrubberLayoutAttributes;

    #[cfg(feature = "AppKit_NSScrubberLayoutAttributes")]
    unsafe impl ClassType for NSScrubberLayoutAttributes {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSScrubberLayoutAttributes")]
unsafe impl NSCopying for NSScrubberLayoutAttributes {}

#[cfg(feature = "AppKit_NSScrubberLayoutAttributes")]
unsafe impl NSObjectProtocol for NSScrubberLayoutAttributes {}

extern_methods!(
    #[cfg(feature = "AppKit_NSScrubberLayoutAttributes")]
    unsafe impl NSScrubberLayoutAttributes {
        #[method(itemIndex)]
        pub unsafe fn itemIndex(&self) -> NSInteger;

        #[method(setItemIndex:)]
        pub unsafe fn setItemIndex(&self, item_index: NSInteger);

        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;

        #[method(setFrame:)]
        pub unsafe fn setFrame(&self, frame: NSRect);

        #[method(alpha)]
        pub unsafe fn alpha(&self) -> CGFloat;

        #[method(setAlpha:)]
        pub unsafe fn setAlpha(&self, alpha: CGFloat);

        #[method_id(@__retain_semantics Other layoutAttributesForItemAtIndex:)]
        pub unsafe fn layoutAttributesForItemAtIndex(index: NSInteger) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSScrubberLayout")]
    pub struct NSScrubberLayout;

    #[cfg(feature = "AppKit_NSScrubberLayout")]
    unsafe impl ClassType for NSScrubberLayout {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSScrubberLayout")]
unsafe impl NSCoding for NSScrubberLayout {}

#[cfg(feature = "AppKit_NSScrubberLayout")]
unsafe impl NSObjectProtocol for NSScrubberLayout {}

extern_methods!(
    #[cfg(feature = "AppKit_NSScrubberLayout")]
    unsafe impl NSScrubberLayout {
        #[method(layoutAttributesClass)]
        pub unsafe fn layoutAttributesClass() -> &'static Class;

        #[cfg(feature = "AppKit_NSScrubber")]
        #[method_id(@__retain_semantics Other scrubber)]
        pub unsafe fn scrubber(&self) -> Option<Id<NSScrubber>>;

        #[method(visibleRect)]
        pub unsafe fn visibleRect(&self) -> NSRect;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

        #[method(invalidateLayout)]
        pub unsafe fn invalidateLayout(&self);

        #[method(prepareLayout)]
        pub unsafe fn prepareLayout(&self);

        #[method(scrubberContentSize)]
        pub unsafe fn scrubberContentSize(&self) -> NSSize;

        #[cfg(feature = "AppKit_NSScrubberLayoutAttributes")]
        #[method_id(@__retain_semantics Other layoutAttributesForItemAtIndex:)]
        pub unsafe fn layoutAttributesForItemAtIndex(
            &self,
            index: NSInteger,
        ) -> Option<Id<NSScrubberLayoutAttributes>>;

        #[cfg(all(
            feature = "AppKit_NSScrubberLayoutAttributes",
            feature = "Foundation_NSSet"
        ))]
        #[method_id(@__retain_semantics Other layoutAttributesForItemsInRect:)]
        pub unsafe fn layoutAttributesForItemsInRect(
            &self,
            rect: NSRect,
        ) -> Id<NSSet<NSScrubberLayoutAttributes>>;

        #[method(shouldInvalidateLayoutForSelectionChange)]
        pub unsafe fn shouldInvalidateLayoutForSelectionChange(&self) -> bool;

        #[method(shouldInvalidateLayoutForHighlightChange)]
        pub unsafe fn shouldInvalidateLayoutForHighlightChange(&self) -> bool;

        #[method(shouldInvalidateLayoutForChangeFromVisibleRect:toVisibleRect:)]
        pub unsafe fn shouldInvalidateLayoutForChangeFromVisibleRect_toVisibleRect(
            &self,
            from_visible_rect: NSRect,
            to_visible_rect: NSRect,
        ) -> bool;

        #[method(automaticallyMirrorsInRightToLeftLayout)]
        pub unsafe fn automaticallyMirrorsInRightToLeftLayout(&self) -> bool;
    }
);

extern_protocol!(
    pub unsafe trait NSScrubberFlowLayoutDelegate: NSScrubberDelegate {
        #[cfg(all(feature = "AppKit_NSScrubber", feature = "AppKit_NSScrubberFlowLayout"))]
        #[optional]
        #[method(scrubber:layout:sizeForItemAtIndex:)]
        unsafe fn scrubber_layout_sizeForItemAtIndex(
            &self,
            scrubber: &NSScrubber,
            layout: &NSScrubberFlowLayout,
            item_index: NSInteger,
        ) -> NSSize;
    }

    unsafe impl ProtocolType for dyn NSScrubberFlowLayoutDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSScrubberFlowLayout")]
    pub struct NSScrubberFlowLayout;

    #[cfg(feature = "AppKit_NSScrubberFlowLayout")]
    unsafe impl ClassType for NSScrubberFlowLayout {
        #[inherits(NSObject)]
        type Super = NSScrubberLayout;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSScrubberFlowLayout")]
unsafe impl NSCoding for NSScrubberFlowLayout {}

#[cfg(feature = "AppKit_NSScrubberFlowLayout")]
unsafe impl NSObjectProtocol for NSScrubberFlowLayout {}

extern_methods!(
    #[cfg(feature = "AppKit_NSScrubberFlowLayout")]
    unsafe impl NSScrubberFlowLayout {
        #[method(itemSpacing)]
        pub unsafe fn itemSpacing(&self) -> CGFloat;

        #[method(setItemSpacing:)]
        pub unsafe fn setItemSpacing(&self, item_spacing: CGFloat);

        #[method(itemSize)]
        pub unsafe fn itemSize(&self) -> NSSize;

        #[method(setItemSize:)]
        pub unsafe fn setItemSize(&self, item_size: NSSize);

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(invalidateLayoutForItemsAtIndexes:)]
        pub unsafe fn invalidateLayoutForItemsAtIndexes(&self, invalid_item_indexes: &NSIndexSet);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSScrubberProportionalLayout")]
    pub struct NSScrubberProportionalLayout;

    #[cfg(feature = "AppKit_NSScrubberProportionalLayout")]
    unsafe impl ClassType for NSScrubberProportionalLayout {
        #[inherits(NSObject)]
        type Super = NSScrubberLayout;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSScrubberProportionalLayout")]
unsafe impl NSCoding for NSScrubberProportionalLayout {}

#[cfg(feature = "AppKit_NSScrubberProportionalLayout")]
unsafe impl NSObjectProtocol for NSScrubberProportionalLayout {}

extern_methods!(
    #[cfg(feature = "AppKit_NSScrubberProportionalLayout")]
    unsafe impl NSScrubberProportionalLayout {
        #[method(numberOfVisibleItems)]
        pub unsafe fn numberOfVisibleItems(&self) -> NSInteger;

        #[method(setNumberOfVisibleItems:)]
        pub unsafe fn setNumberOfVisibleItems(&self, number_of_visible_items: NSInteger);

        #[method_id(@__retain_semantics Init initWithNumberOfVisibleItems:)]
        pub unsafe fn initWithNumberOfVisibleItems(
            this: Option<Allocated<Self>>,
            number_of_visible_items: NSInteger,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;
    }
);
