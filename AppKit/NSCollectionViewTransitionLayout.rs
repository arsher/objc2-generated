//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscollectionviewtransitionlayoutanimatedkey?language=objc)
pub type NSCollectionViewTransitionLayoutAnimatedKey = NSString;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nscollectionviewtransitionlayout?language=objc)
    #[unsafe(super(NSCollectionViewLayout, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSCollectionViewLayout")]
    pub struct NSCollectionViewTransitionLayout;
);

#[cfg(feature = "NSCollectionViewLayout")]
unsafe impl NSCoding for NSCollectionViewTransitionLayout {}

#[cfg(feature = "NSCollectionViewLayout")]
unsafe impl NSObjectProtocol for NSCollectionViewTransitionLayout {}

extern_methods!(
    #[cfg(feature = "NSCollectionViewLayout")]
    unsafe impl NSCollectionViewTransitionLayout {
        #[cfg(feature = "objc2-core-foundation")]
        #[method(transitionProgress)]
        pub unsafe fn transitionProgress(&self) -> CGFloat;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(setTransitionProgress:)]
        pub unsafe fn setTransitionProgress(&self, transition_progress: CGFloat);

        #[method_id(@__retain_semantics Other currentLayout)]
        pub unsafe fn currentLayout(&self) -> Retained<NSCollectionViewLayout>;

        #[method_id(@__retain_semantics Other nextLayout)]
        pub unsafe fn nextLayout(&self) -> Retained<NSCollectionViewLayout>;

        #[method_id(@__retain_semantics Init initWithCurrentLayout:nextLayout:)]
        pub unsafe fn initWithCurrentLayout_nextLayout(
            this: Allocated<Self>,
            current_layout: &NSCollectionViewLayout,
            new_layout: &NSCollectionViewLayout,
        ) -> Retained<Self>;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(updateValue:forAnimatedKey:)]
        pub unsafe fn updateValue_forAnimatedKey(
            &self,
            value: CGFloat,
            key: &NSCollectionViewTransitionLayoutAnimatedKey,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(valueForAnimatedKey:)]
        pub unsafe fn valueForAnimatedKey(
            &self,
            key: &NSCollectionViewTransitionLayoutAnimatedKey,
        ) -> CGFloat;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSCollectionViewLayout")]
    unsafe impl NSCollectionViewTransitionLayout {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
