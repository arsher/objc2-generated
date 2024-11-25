//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-cloud-kit")]
use objc2_cloud_kit::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-user-notifications")]
use objc2_user_notifications::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisceneconnectionoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UISceneConnectionOptions;
);

unsafe impl NSObjectProtocol for UISceneConnectionOptions {}

extern_methods!(
    unsafe impl UISceneConnectionOptions {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "UIOpenURLContext")]
        #[method_id(@__retain_semantics Other URLContexts)]
        pub unsafe fn URLContexts(&self) -> Retained<NSSet<UIOpenURLContext>>;

        #[method_id(@__retain_semantics Other sourceApplication)]
        pub unsafe fn sourceApplication(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other handoffUserActivityType)]
        pub unsafe fn handoffUserActivityType(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other userActivities)]
        pub unsafe fn userActivities(&self) -> Retained<NSSet<NSUserActivity>>;

        #[cfg(feature = "objc2-user-notifications")]
        #[method_id(@__retain_semantics Other notificationResponse)]
        pub unsafe fn notificationResponse(&self) -> Option<Retained<UNNotificationResponse>>;

        #[cfg(feature = "UIApplicationShortcutItem")]
        #[method_id(@__retain_semantics Other shortcutItem)]
        pub unsafe fn shortcutItem(&self) -> Option<Retained<UIApplicationShortcutItem>>;

        #[cfg(feature = "objc2-cloud-kit")]
        #[method_id(@__retain_semantics Other cloudKitShareMetadata)]
        pub unsafe fn cloudKitShareMetadata(&self) -> Option<Retained<CKShareMetadata>>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisceneopenurloptions?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UISceneOpenURLOptions;
);

unsafe impl NSObjectProtocol for UISceneOpenURLOptions {}

extern_methods!(
    unsafe impl UISceneOpenURLOptions {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Other sourceApplication)]
        pub unsafe fn sourceApplication(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other annotation)]
        pub unsafe fn annotation(&self) -> Option<Retained<AnyObject>>;

        #[method(openInPlace)]
        pub unsafe fn openInPlace(&self) -> bool;

        #[cfg(feature = "UIEventAttribution")]
        #[method_id(@__retain_semantics Other eventAttribution)]
        pub unsafe fn eventAttribution(&self) -> Option<Retained<UIEventAttribution>>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisceneopenexternalurloptions?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UISceneOpenExternalURLOptions;
);

unsafe impl NSObjectProtocol for UISceneOpenExternalURLOptions {}

extern_methods!(
    unsafe impl UISceneOpenExternalURLOptions {
        #[method(universalLinksOnly)]
        pub unsafe fn universalLinksOnly(&self) -> bool;

        #[method(setUniversalLinksOnly:)]
        pub unsafe fn setUniversalLinksOnly(&self, universal_links_only: bool);

        #[cfg(feature = "UIEventAttribution")]
        #[method_id(@__retain_semantics Other eventAttribution)]
        pub unsafe fn eventAttribution(&self) -> Option<Retained<UIEventAttribution>>;

        #[cfg(feature = "UIEventAttribution")]
        #[method(setEventAttribution:)]
        pub unsafe fn setEventAttribution(&self, event_attribution: Option<&UIEventAttribution>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UISceneOpenExternalURLOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

/// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiscenecollectionjoinbehavior?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UISceneCollectionJoinBehavior(pub NSInteger);
impl UISceneCollectionJoinBehavior {
    #[doc(alias = "UISceneCollectionJoinBehaviorAutomatic")]
    pub const Automatic: Self = Self(0);
    #[doc(alias = "UISceneCollectionJoinBehaviorPreferred")]
    pub const Preferred: Self = Self(1);
    #[doc(alias = "UISceneCollectionJoinBehaviorDisallowed")]
    pub const Disallowed: Self = Self(2);
    #[doc(alias = "UISceneCollectionJoinBehaviorPreferredWithoutActivating")]
    pub const PreferredWithoutActivating: Self = Self(3);
}

unsafe impl Encode for UISceneCollectionJoinBehavior {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UISceneCollectionJoinBehavior {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uisceneactivationrequestoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UISceneActivationRequestOptions;
);

unsafe impl NSObjectProtocol for UISceneActivationRequestOptions {}

extern_methods!(
    unsafe impl UISceneActivationRequestOptions {
        #[cfg(all(feature = "UIResponder", feature = "UIScene"))]
        #[method_id(@__retain_semantics Other requestingScene)]
        pub unsafe fn requestingScene(&self) -> Option<Retained<UIScene>>;

        #[cfg(all(feature = "UIResponder", feature = "UIScene"))]
        #[method(setRequestingScene:)]
        pub unsafe fn setRequestingScene(&self, requesting_scene: Option<&UIScene>);

        #[method(collectionJoinBehavior)]
        pub unsafe fn collectionJoinBehavior(&self) -> UISceneCollectionJoinBehavior;

        #[method(setCollectionJoinBehavior:)]
        pub unsafe fn setCollectionJoinBehavior(
            &self,
            collection_join_behavior: UISceneCollectionJoinBehavior,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UISceneActivationRequestOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uiscenedestructionrequestoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UISceneDestructionRequestOptions;
);

unsafe impl NSObjectProtocol for UISceneDestructionRequestOptions {}

extern_methods!(
    unsafe impl UISceneDestructionRequestOptions {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UISceneDestructionRequestOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
