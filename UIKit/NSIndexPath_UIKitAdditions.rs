//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_category!(
    /// Category "UIKitAdditions" on [`NSIndexPath`].
    #[doc(alias = "UIKitAdditions")]
    pub unsafe trait NSIndexPathUIKitAdditions {
        #[method_id(@__retain_semantics Other indexPathForRow:inSection:)]
        unsafe fn indexPathForRow_inSection(row: NSInteger, section: NSInteger) -> Retained<Self>;

        #[method_id(@__retain_semantics Other indexPathForItem:inSection:)]
        unsafe fn indexPathForItem_inSection(item: NSInteger, section: NSInteger)
            -> Retained<Self>;

        #[method(section)]
        unsafe fn section(&self) -> NSInteger;

        #[method(row)]
        unsafe fn row(&self) -> NSInteger;

        #[method(item)]
        unsafe fn item(&self) -> NSInteger;
    }

    unsafe impl NSIndexPathUIKitAdditions for NSIndexPath {}
);
