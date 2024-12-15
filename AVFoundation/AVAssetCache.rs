//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avassetcache?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAssetCache;
);

unsafe impl Send for AVAssetCache {}

unsafe impl Sync for AVAssetCache {}

unsafe impl NSObjectProtocol for AVAssetCache {}

extern_methods!(
    unsafe impl AVAssetCache {
        #[method(isPlayableOffline)]
        pub unsafe fn isPlayableOffline(&self) -> bool;

        #[cfg(feature = "AVMediaSelectionGroup")]
        #[method_id(@__retain_semantics Other mediaSelectionOptionsInMediaSelectionGroup:)]
        pub unsafe fn mediaSelectionOptionsInMediaSelectionGroup(
            &self,
            media_selection_group: &AVMediaSelectionGroup,
        ) -> Retained<NSArray<AVMediaSelectionOption>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
