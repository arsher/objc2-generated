//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSpeechRecognizer")]
    pub struct NSSpeechRecognizer;

    #[cfg(feature = "AppKit_NSSpeechRecognizer")]
    unsafe impl ClassType for NSSpeechRecognizer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSSpeechRecognizer")]
unsafe impl NSObjectProtocol for NSSpeechRecognizer {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSpeechRecognizer")]
    unsafe impl NSSpeechRecognizer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Option<Id<Self>>;

        #[method(startListening)]
        pub unsafe fn startListening(&self);

        #[method(stopListening)]
        pub unsafe fn stopListening(&self);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self)
            -> Option<Id<ProtocolObject<dyn NSSpeechRecognizerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSSpeechRecognizerDelegate>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other commands)]
        pub unsafe fn commands(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setCommands:)]
        pub unsafe fn setCommands(&self, commands: Option<&NSArray<NSString>>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other displayedCommandsTitle)]
        pub unsafe fn displayedCommandsTitle(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDisplayedCommandsTitle:)]
        pub unsafe fn setDisplayedCommandsTitle(&self, displayed_commands_title: Option<&NSString>);

        #[method(listensInForegroundOnly)]
        pub unsafe fn listensInForegroundOnly(&self) -> bool;

        #[method(setListensInForegroundOnly:)]
        pub unsafe fn setListensInForegroundOnly(&self, listens_in_foreground_only: bool);

        #[method(blocksOtherRecognizers)]
        pub unsafe fn blocksOtherRecognizers(&self) -> bool;

        #[method(setBlocksOtherRecognizers:)]
        pub unsafe fn setBlocksOtherRecognizers(&self, blocks_other_recognizers: bool);
    }
);

extern_protocol!(
    pub unsafe trait NSSpeechRecognizerDelegate: NSObjectProtocol {
        #[cfg(all(feature = "AppKit_NSSpeechRecognizer", feature = "Foundation_NSString"))]
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
