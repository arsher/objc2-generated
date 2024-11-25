//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uidocumentproperties?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIDocumentProperties;
);

unsafe impl NSObjectProtocol for UIDocumentProperties {}

extern_methods!(
    unsafe impl UIDocumentProperties {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithURL:)]
        pub unsafe fn initWithURL(this: Allocated<Self>, url: &NSURL) -> Retained<Self>;

        #[cfg(all(feature = "UIDragItem", feature = "UIDragSession", feature = "block2"))]
        #[method(dragItemsProvider)]
        pub unsafe fn dragItemsProvider(
            &self,
        ) -> *mut block2::Block<
            dyn Fn(NonNull<ProtocolObject<dyn UIDragSession>>) -> NonNull<NSArray<UIDragItem>>,
        >;

        #[cfg(all(feature = "UIDragItem", feature = "UIDragSession", feature = "block2"))]
        #[method(setDragItemsProvider:)]
        pub unsafe fn setDragItemsProvider(
            &self,
            drag_items_provider: Option<
                &block2::Block<
                    dyn Fn(
                        NonNull<ProtocolObject<dyn UIDragSession>>,
                    ) -> NonNull<NSArray<UIDragItem>>,
                >,
            >,
        );

        #[cfg(all(
            feature = "UIActivityViewController",
            feature = "UIResponder",
            feature = "UIViewController",
            feature = "block2"
        ))]
        #[method(activityViewControllerProvider)]
        pub unsafe fn activityViewControllerProvider(
            &self,
        ) -> *mut block2::Block<dyn Fn() -> NonNull<UIActivityViewController>>;

        #[cfg(all(
            feature = "UIActivityViewController",
            feature = "UIResponder",
            feature = "UIViewController",
            feature = "block2"
        ))]
        #[method(setActivityViewControllerProvider:)]
        pub unsafe fn setActivityViewControllerProvider(
            &self,
            activity_view_controller_provider: Option<
                &block2::Block<dyn Fn() -> NonNull<UIActivityViewController>>,
            >,
        );

        #[method(wantsIconRepresentation)]
        pub unsafe fn wantsIconRepresentation(&self) -> bool;

        #[method(setWantsIconRepresentation:)]
        pub unsafe fn setWantsIconRepresentation(&self, wants_icon_representation: bool);
    }
);
