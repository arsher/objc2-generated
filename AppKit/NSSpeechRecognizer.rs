//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsspeechrecognizer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSpeechRecognizer;
);

unsafe impl NSObjectProtocol for NSSpeechRecognizer {}

extern_methods!(
    unsafe impl NSSpeechRecognizer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Option<Retained<Self>>;

        #[method(startListening)]
        pub unsafe fn startListening(&self);

        #[method(stopListening)]
        pub unsafe fn stopListening(&self);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
            mtm: MainThreadMarker,
        ) -> Option<Retained<ProtocolObject<dyn NSSpeechRecognizerDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSSpeechRecognizerDelegate>>,
        );

        #[method_id(@__retain_semantics Other commands)]
        pub unsafe fn commands(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`commands`][Self::commands].
        #[method(setCommands:)]
        pub unsafe fn setCommands(&self, commands: Option<&NSArray<NSString>>);

        #[method_id(@__retain_semantics Other displayedCommandsTitle)]
        pub unsafe fn displayedCommandsTitle(&self) -> Option<Retained<NSString>>;

        /// Setter for [`displayedCommandsTitle`][Self::displayedCommandsTitle].
        #[method(setDisplayedCommandsTitle:)]
        pub unsafe fn setDisplayedCommandsTitle(&self, displayed_commands_title: Option<&NSString>);

        #[method(listensInForegroundOnly)]
        pub unsafe fn listensInForegroundOnly(&self) -> bool;

        /// Setter for [`listensInForegroundOnly`][Self::listensInForegroundOnly].
        #[method(setListensInForegroundOnly:)]
        pub unsafe fn setListensInForegroundOnly(&self, listens_in_foreground_only: bool);

        #[method(blocksOtherRecognizers)]
        pub unsafe fn blocksOtherRecognizers(&self) -> bool;

        /// Setter for [`blocksOtherRecognizers`][Self::blocksOtherRecognizers].
        #[method(setBlocksOtherRecognizers:)]
        pub unsafe fn setBlocksOtherRecognizers(&self, blocks_other_recognizers: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSSpeechRecognizer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsspeechrecognizerdelegate?language=objc)
    pub unsafe trait NSSpeechRecognizerDelegate: NSObjectProtocol + MainThreadOnly {
        #[optional]
        #[method(speechRecognizer:didRecognizeCommand:)]
        unsafe fn speechRecognizer_didRecognizeCommand(
            &self,
            sender: &NSSpeechRecognizer,
            command: &NSString,
        );
    }

    unsafe impl ProtocolType for dyn NSSpeechRecognizerDelegate {}
);
