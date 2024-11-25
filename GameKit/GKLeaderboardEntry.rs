//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/gamekit/gkleaderboardentry?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct GKLeaderboardEntry;
);

unsafe impl NSObjectProtocol for GKLeaderboardEntry {}

extern_methods!(
    unsafe impl GKLeaderboardEntry {
        #[cfg(all(feature = "GKBasePlayer", feature = "GKPlayer"))]
        #[method_id(@__retain_semantics Other player)]
        pub unsafe fn player(&self) -> Retained<GKPlayer>;

        #[method(rank)]
        pub unsafe fn rank(&self) -> NSInteger;

        #[method(score)]
        pub unsafe fn score(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other formattedScore)]
        pub unsafe fn formattedScore(&self) -> Retained<NSString>;

        #[method(context)]
        pub unsafe fn context(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Retained<NSDate>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl GKLeaderboardEntry {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
