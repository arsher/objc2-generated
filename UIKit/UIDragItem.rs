//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidragitem?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIDragItem;
);

unsafe impl NSObjectProtocol for UIDragItem {}

extern_methods!(
    unsafe impl UIDragItem {
        #[method_id(@__retain_semantics Init initWithItemProvider:)]
        pub unsafe fn initWithItemProvider(
            this: Allocated<Self>,
            item_provider: &NSItemProvider,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Other itemProvider)]
        pub unsafe fn itemProvider(&self) -> Retained<NSItemProvider>;

        #[method_id(@__retain_semantics Other localObject)]
        pub unsafe fn localObject(&self) -> Option<Retained<AnyObject>>;

        #[method(setLocalObject:)]
        pub unsafe fn setLocalObject(&self, local_object: Option<&AnyObject>);

        #[cfg(all(feature = "UIDragPreview", feature = "block2"))]
        #[method(previewProvider)]
        pub unsafe fn previewProvider(&self) -> *mut block2::Block<dyn Fn() -> *mut UIDragPreview>;

        #[cfg(all(feature = "UIDragPreview", feature = "block2"))]
        #[method(setPreviewProvider:)]
        pub unsafe fn setPreviewProvider(
            &self,
            preview_provider: Option<&block2::Block<dyn Fn() -> *mut UIDragPreview>>,
        );

        #[method(setNeedsDropPreviewUpdate)]
        pub unsafe fn setNeedsDropPreviewUpdate(&self);
    }
);
