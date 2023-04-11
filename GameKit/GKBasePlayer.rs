//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKBasePlayer")]
    pub struct GKBasePlayer;

    #[cfg(feature = "GameKit_GKBasePlayer")]
    unsafe impl ClassType for GKBasePlayer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameKit_GKBasePlayer")]
unsafe impl NSObjectProtocol for GKBasePlayer {}

extern_methods!(
    #[cfg(feature = "GameKit_GKBasePlayer")]
    unsafe impl GKBasePlayer {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "use the teamPlayerID property to identify a player"]
        #[method_id(@__retain_semantics Other playerID)]
        pub unsafe fn playerID(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Option<Id<NSString>>;
    }
);
