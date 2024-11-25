//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/mediaplayer/mpplayablecontentmanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use CarPlay framework"]
    pub struct MPPlayableContentManager;
);

unsafe impl NSObjectProtocol for MPPlayableContentManager {}

extern_methods!(
    unsafe impl MPPlayableContentManager {
        #[cfg(feature = "MPPlayableContentDataSource")]
        #[deprecated = "Use CarPlay framework"]
        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MPPlayableContentDataSource>>>;

        #[cfg(feature = "MPPlayableContentDataSource")]
        /// This is a [weak property][objc2::topics::weak_property].
        #[deprecated = "Use CarPlay framework"]
        #[method(setDataSource:)]
        pub unsafe fn setDataSource(
            &self,
            data_source: Option<&ProtocolObject<dyn MPPlayableContentDataSource>>,
        );

        #[cfg(feature = "MPPlayableContentDelegate")]
        #[deprecated = "Use CarPlay framework"]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MPPlayableContentDelegate>>>;

        #[cfg(feature = "MPPlayableContentDelegate")]
        /// This is a [weak property][objc2::topics::weak_property].
        #[deprecated = "Use CarPlay framework"]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn MPPlayableContentDelegate>>,
        );

        #[cfg(feature = "MPPlayableContentManagerContext")]
        #[deprecated = "Use CarPlay framework"]
        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Retained<MPPlayableContentManagerContext>;

        #[deprecated = "Use CarPlay framework"]
        #[method_id(@__retain_semantics Other nowPlayingIdentifiers)]
        pub unsafe fn nowPlayingIdentifiers(&self) -> Retained<NSArray<NSString>>;

        #[deprecated = "Use CarPlay framework"]
        #[method(setNowPlayingIdentifiers:)]
        pub unsafe fn setNowPlayingIdentifiers(&self, now_playing_identifiers: &NSArray<NSString>);

        #[deprecated = "Use CarPlay framework"]
        #[method_id(@__retain_semantics Other sharedContentManager)]
        pub unsafe fn sharedContentManager() -> Retained<Self>;

        #[deprecated = "Use CarPlay framework"]
        #[method(reloadData)]
        pub unsafe fn reloadData(&self);

        #[deprecated = "Use CarPlay framework"]
        #[method(beginUpdates)]
        pub unsafe fn beginUpdates(&self);

        #[deprecated = "Use CarPlay framework"]
        #[method(endUpdates)]
        pub unsafe fn endUpdates(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MPPlayableContentManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
