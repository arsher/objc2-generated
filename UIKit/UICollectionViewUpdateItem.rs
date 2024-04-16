//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UICollectionUpdateAction(pub NSInteger);
impl UICollectionUpdateAction {
    #[doc(alias = "UICollectionUpdateActionInsert")]
    pub const Insert: Self = Self(0);
    #[doc(alias = "UICollectionUpdateActionDelete")]
    pub const Delete: Self = Self(1);
    #[doc(alias = "UICollectionUpdateActionReload")]
    pub const Reload: Self = Self(2);
    #[doc(alias = "UICollectionUpdateActionMove")]
    pub const Move: Self = Self(3);
    #[doc(alias = "UICollectionUpdateActionNone")]
    pub const None: Self = Self(4);
}

unsafe impl Encode for UICollectionUpdateAction {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UICollectionUpdateAction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UICollectionViewUpdateItem;

    unsafe impl ClassType for UICollectionViewUpdateItem {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UICollectionViewUpdateItem {}

extern_methods!(
    unsafe impl UICollectionViewUpdateItem {
        #[method_id(@__retain_semantics Other indexPathBeforeUpdate)]
        pub unsafe fn indexPathBeforeUpdate(&self) -> Option<Id<NSIndexPath>>;

        #[method_id(@__retain_semantics Other indexPathAfterUpdate)]
        pub unsafe fn indexPathAfterUpdate(&self) -> Option<Id<NSIndexPath>>;

        #[method(updateAction)]
        pub unsafe fn updateAction(&self) -> UICollectionUpdateAction;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UICollectionViewUpdateItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
