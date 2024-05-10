//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-cloud-kit")]
use objc2_cloud_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIScene"))]
    pub struct UIWindowScene;

    #[cfg(all(feature = "UIResponder", feature = "UIScene"))]
    unsafe impl ClassType for UIWindowScene {
        #[inherits(UIResponder, NSObject)]
        type Super = UIScene;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(all(feature = "UIResponder", feature = "UIScene"))]
unsafe impl NSObjectProtocol for UIWindowScene {}

#[cfg(all(feature = "UIResponder", feature = "UIScene"))]
unsafe impl UIResponderStandardEditActions for UIWindowScene {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIScene"))]
    unsafe impl UIWindowScene {
        #[cfg(feature = "UIScreen")]
        #[method_id(@__retain_semantics Other screen)]
        pub unsafe fn screen(&self) -> Id<UIScreen>;

        #[cfg(feature = "UIOrientation")]
        #[method(interfaceOrientation)]
        pub unsafe fn interfaceOrientation(&self) -> UIInterfaceOrientation;

        #[cfg(feature = "UIView")]
        #[method_id(@__retain_semantics Other coordinateSpace)]
        pub unsafe fn coordinateSpace(&self) -> Id<ProtocolObject<dyn UICoordinateSpace>>;

        #[cfg(feature = "UITraitCollection")]
        #[method_id(@__retain_semantics Other traitCollection)]
        pub unsafe fn traitCollection(&self) -> Id<UITraitCollection>;

        #[cfg(all(feature = "UIWindowSceneGeometryPreferences", feature = "block2"))]
        #[method(requestGeometryUpdateWithPreferences:errorHandler:)]
        pub unsafe fn requestGeometryUpdateWithPreferences_errorHandler(
            &self,
            geometry_preferences: &UIWindowSceneGeometryPreferences,
            error_handler: Option<&block2::Block<dyn Fn(NonNull<NSError>)>>,
        );

        #[cfg(feature = "UIWindowSceneGeometry")]
        #[method_id(@__retain_semantics Other effectiveGeometry)]
        pub unsafe fn effectiveGeometry(&self) -> Id<UIWindowSceneGeometry>;

        #[method_id(@__retain_semantics Other sizeRestrictions)]
        pub unsafe fn sizeRestrictions(&self) -> Option<Id<UISceneSizeRestrictions>>;

        #[cfg(all(feature = "UIView", feature = "UIWindow"))]
        #[method_id(@__retain_semantics Other windows)]
        pub unsafe fn windows(&self) -> Id<NSArray<UIWindow>>;

        #[cfg(all(feature = "UIView", feature = "UIWindow"))]
        #[method_id(@__retain_semantics Other keyWindow)]
        pub unsafe fn keyWindow(&self) -> Option<Id<UIWindow>>;

        #[cfg(feature = "UIActivityItemsConfigurationReading")]
        #[method_id(@__retain_semantics Other activityItemsConfigurationSource)]
        pub unsafe fn activityItemsConfigurationSource(
            &self,
        ) -> Option<Id<ProtocolObject<dyn UIActivityItemsConfigurationProviding>>>;

        #[cfg(feature = "UIActivityItemsConfigurationReading")]
        #[method(setActivityItemsConfigurationSource:)]
        pub unsafe fn setActivityItemsConfigurationSource(
            &self,
            activity_items_configuration_source: Option<
                &ProtocolObject<dyn UIActivityItemsConfigurationProviding>,
            >,
        );

        #[cfg(feature = "UISceneWindowingBehaviors")]
        #[method_id(@__retain_semantics Other windowingBehaviors)]
        pub unsafe fn windowingBehaviors(&self) -> Option<Id<UISceneWindowingBehaviors>>;

        #[method(isFullScreen)]
        pub unsafe fn isFullScreen(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIScene`
    #[cfg(all(feature = "UIResponder", feature = "UIScene"))]
    unsafe impl UIWindowScene {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(feature = "UISceneOptions", feature = "UISceneSession"))]
        #[method_id(@__retain_semantics Init initWithSession:connectionOptions:)]
        pub unsafe fn initWithSession_connectionOptions(
            this: Allocated<Self>,
            session: &UISceneSession,
            connection_options: &UISceneConnectionOptions,
        ) -> Id<Self>;
    }
);

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIScene"))]
    unsafe impl UIWindowScene {
        #[cfg(feature = "UITraitCollection")]
        #[method_id(@__retain_semantics Other traitOverrides)]
        pub unsafe fn traitOverrides(&self) -> Id<ProtocolObject<dyn UITraitOverrides>>;
    }
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIScene",
    feature = "UITraitCollection"
))]
unsafe impl UITraitChangeObservable for UIWindowScene {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UIScene",
    feature = "UITraitCollection"
))]
unsafe impl UITraitEnvironment for UIWindowScene {}

extern_protocol!(
    #[cfg(feature = "UIScene")]
    pub unsafe trait UIWindowSceneDelegate: UISceneDelegate + IsMainThreadOnly {
        #[cfg(all(feature = "UIResponder", feature = "UIView", feature = "UIWindow"))]
        #[optional]
        #[method_id(@__retain_semantics Other window)]
        unsafe fn window(&self) -> Option<Id<UIWindow>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView", feature = "UIWindow"))]
        #[optional]
        #[method(setWindow:)]
        unsafe fn setWindow(&self, window: Option<&UIWindow>);

        #[cfg(all(
            feature = "UIOrientation",
            feature = "UIResponder",
            feature = "UITraitCollection",
            feature = "UIView"
        ))]
        #[optional]
        #[method(windowScene:didUpdateCoordinateSpace:interfaceOrientation:traitCollection:)]
        unsafe fn windowScene_didUpdateCoordinateSpace_interfaceOrientation_traitCollection(
            &self,
            window_scene: &UIWindowScene,
            previous_coordinate_space: &ProtocolObject<dyn UICoordinateSpace>,
            previous_interface_orientation: UIInterfaceOrientation,
            previous_trait_collection: &UITraitCollection,
        );

        #[cfg(all(
            feature = "UIApplicationShortcutItem",
            feature = "UIResponder",
            feature = "block2"
        ))]
        #[optional]
        #[method(windowScene:performActionForShortcutItem:completionHandler:)]
        unsafe fn windowScene_performActionForShortcutItem_completionHandler(
            &self,
            window_scene: &UIWindowScene,
            shortcut_item: &UIApplicationShortcutItem,
            completion_handler: &block2::Block<dyn Fn(Bool)>,
        );

        #[cfg(all(feature = "UIResponder", feature = "objc2-cloud-kit"))]
        #[optional]
        #[method(windowScene:userDidAcceptCloudKitShareWithMetadata:)]
        unsafe fn windowScene_userDidAcceptCloudKitShareWithMetadata(
            &self,
            window_scene: &UIWindowScene,
            cloud_kit_share_metadata: &CKShareMetadata,
        );
    }

    #[cfg(feature = "UIScene")]
    unsafe impl ProtocolType for dyn UIWindowSceneDelegate {}
);

extern "C" {
    #[cfg(feature = "UISceneDefinitions")]
    pub static UIWindowSceneSessionRoleApplication: &'static UISceneSessionRole;
}

extern "C" {
    #[cfg(feature = "UISceneDefinitions")]
    pub static UIWindowSceneSessionRoleExternalDisplayNonInteractive: &'static UISceneSessionRole;
}

extern "C" {
    #[cfg(feature = "UISceneDefinitions")]
    pub static UIWindowSceneSessionRoleExternalDisplay: &'static UISceneSessionRole;
}

extern "C" {
    #[cfg(feature = "UISceneDefinitions")]
    pub static UIWindowSceneSessionRoleVolumetricApplication: &'static UISceneSessionRole;
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIWindowSceneDismissalAnimation(pub NSInteger);
impl UIWindowSceneDismissalAnimation {
    #[doc(alias = "UIWindowSceneDismissalAnimationStandard")]
    pub const Standard: Self = Self(1);
    #[doc(alias = "UIWindowSceneDismissalAnimationCommit")]
    pub const Commit: Self = Self(2);
    #[doc(alias = "UIWindowSceneDismissalAnimationDecline")]
    pub const Decline: Self = Self(3);
}

unsafe impl Encode for UIWindowSceneDismissalAnimation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIWindowSceneDismissalAnimation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UISceneOptions")]
    pub struct UIWindowSceneDestructionRequestOptions;

    #[cfg(feature = "UISceneOptions")]
    unsafe impl ClassType for UIWindowSceneDestructionRequestOptions {
        #[inherits(NSObject)]
        type Super = UISceneDestructionRequestOptions;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "UISceneOptions")]
unsafe impl NSObjectProtocol for UIWindowSceneDestructionRequestOptions {}

extern_methods!(
    #[cfg(feature = "UISceneOptions")]
    unsafe impl UIWindowSceneDestructionRequestOptions {
        #[method(windowDismissalAnimation)]
        pub unsafe fn windowDismissalAnimation(&self) -> UIWindowSceneDismissalAnimation;

        #[method(setWindowDismissalAnimation:)]
        pub unsafe fn setWindowDismissalAnimation(
            &self,
            window_dismissal_animation: UIWindowSceneDismissalAnimation,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UISceneOptions")]
    unsafe impl UIWindowSceneDestructionRequestOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UISceneSizeRestrictions;

    unsafe impl ClassType for UISceneSizeRestrictions {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UISceneSizeRestrictions {}

extern_methods!(
    unsafe impl UISceneSizeRestrictions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;

        #[method(minimumSize)]
        pub unsafe fn minimumSize(&self) -> CGSize;

        #[method(setMinimumSize:)]
        pub unsafe fn setMinimumSize(&self, minimum_size: CGSize);

        #[method(maximumSize)]
        pub unsafe fn maximumSize(&self) -> CGSize;

        #[method(setMaximumSize:)]
        pub unsafe fn setMaximumSize(&self, maximum_size: CGSize);

        #[method(allowsFullScreen)]
        pub unsafe fn allowsFullScreen(&self) -> bool;

        #[method(setAllowsFullScreen:)]
        pub unsafe fn setAllowsFullScreen(&self, allows_full_screen: bool);
    }
);
