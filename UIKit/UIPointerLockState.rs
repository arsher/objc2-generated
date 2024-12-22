//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipointerlockstate?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIPointerLockState;
);

unsafe impl NSObjectProtocol for UIPointerLockState {}

extern_methods!(
    unsafe impl UIPointerLockState {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        /// The status of the pointer lock for this scene as determined by the system.
        /// The preferred pointer lock value specified by a view controller is only a request, which may or may not be honored.
        /// This property is key-value observable and UIPointerLockState.didChangeNotification is posted when it changes.
        #[method(isLocked)]
        pub unsafe fn isLocked(&self) -> bool;
    }
);

extern_methods!(
    /// PointerLockState
    #[cfg(all(feature = "UIResponder", feature = "UIScene"))]
    unsafe impl UIScene {
        #[method_id(@__retain_semantics Other pointerLockState)]
        pub unsafe fn pointerLockState(&self) -> Option<Retained<UIPointerLockState>>;
    }
);

extern "C" {
    /// A notification that is posted when UIPointerLockState.locked changes values for a scene.
    /// It contains the related UIScene in the user info dictionary of the notification.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/uikit/uipointerlockstatedidchangenotification?language=objc)
    pub static UIPointerLockStateDidChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipointerlockstatesceneuserinfokey?language=objc)
    pub static UIPointerLockStateSceneUserInfoKey: &'static NSString;
}
