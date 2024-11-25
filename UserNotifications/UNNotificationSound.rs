//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/unnotificationsoundname?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type UNNotificationSoundName = NSString;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/usernotifications/unnotificationsound?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UNNotificationSound;
);

unsafe impl NSCoding for UNNotificationSound {}

unsafe impl NSCopying for UNNotificationSound {}

unsafe impl CopyingHelper for UNNotificationSound {
    type Result = Self;
}

unsafe impl NSObjectProtocol for UNNotificationSound {}

unsafe impl NSSecureCoding for UNNotificationSound {}

extern_methods!(
    unsafe impl UNNotificationSound {
        #[method_id(@__retain_semantics Other defaultSound)]
        pub unsafe fn defaultSound() -> Retained<UNNotificationSound>;

        #[method_id(@__retain_semantics Other defaultRingtoneSound)]
        pub unsafe fn defaultRingtoneSound() -> Retained<UNNotificationSound>;

        #[method_id(@__retain_semantics Other defaultCriticalSound)]
        pub unsafe fn defaultCriticalSound() -> Retained<UNNotificationSound>;

        #[method_id(@__retain_semantics Other defaultCriticalSoundWithAudioVolume:)]
        pub unsafe fn defaultCriticalSoundWithAudioVolume(volume: c_float) -> Retained<Self>;

        #[method_id(@__retain_semantics Other soundNamed:)]
        pub unsafe fn soundNamed(name: &UNNotificationSoundName) -> Retained<Self>;

        #[method_id(@__retain_semantics Other ringtoneSoundNamed:)]
        pub unsafe fn ringtoneSoundNamed(name: &UNNotificationSoundName) -> Retained<Self>;

        #[method_id(@__retain_semantics Other criticalSoundNamed:)]
        pub unsafe fn criticalSoundNamed(name: &UNNotificationSoundName) -> Retained<Self>;

        #[method_id(@__retain_semantics Other criticalSoundNamed:withAudioVolume:)]
        pub unsafe fn criticalSoundNamed_withAudioVolume(
            name: &UNNotificationSoundName,
            volume: c_float,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UNNotificationSound {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
