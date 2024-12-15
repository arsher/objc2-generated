//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudioapplicationrecordpermission?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVAudioApplicationRecordPermission(pub NSInteger);
impl AVAudioApplicationRecordPermission {
    #[doc(alias = "AVAudioApplicationRecordPermissionUndetermined")]
    pub const Undetermined: Self = Self(0x756e6474);
    #[doc(alias = "AVAudioApplicationRecordPermissionDenied")]
    pub const Denied: Self = Self(0x64656e79);
    #[doc(alias = "AVAudioApplicationRecordPermissionGranted")]
    pub const Granted: Self = Self(0x67726e74);
}

unsafe impl Encode for AVAudioApplicationRecordPermission {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVAudioApplicationRecordPermission {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudioapplicationinputmutestatechangenotification?language=objc)
    pub static AVAudioApplicationInputMuteStateChangeNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudioapplicationmutestatekey?language=objc)
    pub static AVAudioApplicationMuteStateKey: &'static NSString;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudioapplication?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioApplication;
);

unsafe impl Send for AVAudioApplication {}

unsafe impl Sync for AVAudioApplication {}

unsafe impl NSObjectProtocol for AVAudioApplication {}

extern_methods!(
    unsafe impl AVAudioApplication {
        #[method_id(@__retain_semantics Other sharedInstance)]
        pub unsafe fn sharedInstance() -> Retained<AVAudioApplication>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method(setInputMuted:error:_)]
        pub unsafe fn setInputMuted_error(&self, muted: bool) -> Result<(), Retained<NSError>>;

        #[method(isInputMuted)]
        pub unsafe fn isInputMuted(&self) -> bool;

        #[cfg(feature = "block2")]
        #[method(setInputMuteStateChangeHandler:error:_)]
        pub unsafe fn setInputMuteStateChangeHandler_error(
            &self,
            input_mute_handler: Option<&block2::Block<dyn Fn(Bool) -> Bool>>,
        ) -> Result<(), Retained<NSError>>;

        #[method(recordPermission)]
        pub unsafe fn recordPermission(&self) -> AVAudioApplicationRecordPermission;

        #[cfg(feature = "block2")]
        #[method(requestRecordPermissionWithCompletionHandler:)]
        pub unsafe fn requestRecordPermissionWithCompletionHandler(
            response: &block2::Block<dyn Fn(Bool)>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioApplication {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);