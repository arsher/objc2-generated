//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiaccessibilitylocationdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIAccessibilityLocationDescriptor;
);

unsafe impl NSObjectProtocol for UIAccessibilityLocationDescriptor {}

extern_methods!(
    unsafe impl UIAccessibilityLocationDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Init initWithName:view:)]
        pub unsafe fn initWithName_view(
            this: Allocated<Self>,
            name: &NSString,
            view: &UIView,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[method_id(@__retain_semantics Init initWithName:point:inView:)]
        pub unsafe fn initWithName_point_inView(
            this: Allocated<Self>,
            name: &NSString,
            point: CGPoint,
            view: &UIView,
        ) -> Retained<Self>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[method_id(@__retain_semantics Init initWithAttributedName:point:inView:)]
        pub unsafe fn initWithAttributedName_point_inView(
            this: Allocated<Self>,
            attributed_name: &NSAttributedString,
            point: CGPoint,
            view: &UIView,
        ) -> Retained<Self>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Retained<UIView>>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(point)]
        pub unsafe fn point(&self) -> CGPoint;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other attributedName)]
        pub unsafe fn attributedName(&self) -> Retained<NSAttributedString>;
    }
);
