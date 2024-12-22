//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextinputsourceidentifier?language=objc)
pub type NSTextInputSourceIdentifier = NSString;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextinputcontext?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextInputContext;
);

unsafe impl NSObjectProtocol for NSTextInputContext {}

extern_methods!(
    unsafe impl NSTextInputContext {
        #[method_id(@__retain_semantics Other currentInputContext)]
        pub unsafe fn currentInputContext(
            mtm: MainThreadMarker,
        ) -> Option<Retained<NSTextInputContext>>;

        #[cfg(feature = "NSTextInputClient")]
        #[method_id(@__retain_semantics Init initWithClient:)]
        pub unsafe fn initWithClient(
            this: Allocated<Self>,
            client: &ProtocolObject<dyn NSTextInputClient>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "NSTextInputClient")]
        /// ** Properties ****
        #[method_id(@__retain_semantics Other client)]
        pub unsafe fn client(&self) -> Retained<ProtocolObject<dyn NSTextInputClient>>;

        #[method(acceptsGlyphInfo)]
        pub unsafe fn acceptsGlyphInfo(&self) -> bool;

        /// Setter for [`acceptsGlyphInfo`][Self::acceptsGlyphInfo].
        #[method(setAcceptsGlyphInfo:)]
        pub unsafe fn setAcceptsGlyphInfo(&self, accepts_glyph_info: bool);

        #[method_id(@__retain_semantics Other allowedInputSourceLocales)]
        pub unsafe fn allowedInputSourceLocales(&self) -> Option<Retained<NSArray<NSString>>>;

        /// Setter for [`allowedInputSourceLocales`][Self::allowedInputSourceLocales].
        #[method(setAllowedInputSourceLocales:)]
        pub unsafe fn setAllowedInputSourceLocales(
            &self,
            allowed_input_source_locales: Option<&NSArray<NSString>>,
        );

        /// ** Activation ****
        #[method(activate)]
        pub unsafe fn activate(&self);

        #[method(deactivate)]
        pub unsafe fn deactivate(&self);

        #[cfg(feature = "NSEvent")]
        /// ** Input source interface ***
        #[method(handleEvent:)]
        pub unsafe fn handleEvent(&self, event: &NSEvent) -> bool;

        #[method(discardMarkedText)]
        pub fn discardMarkedText(&self);

        #[method(invalidateCharacterCoordinates)]
        pub fn invalidateCharacterCoordinates(&self);

        #[method(textInputClientWillStartScrollingOrZooming)]
        pub unsafe fn textInputClientWillStartScrollingOrZooming(&self);

        #[method(textInputClientDidEndScrollingOrZooming)]
        pub unsafe fn textInputClientDidEndScrollingOrZooming(&self);

        /// ** Text Input sources handling ***
        #[method_id(@__retain_semantics Other keyboardInputSources)]
        pub unsafe fn keyboardInputSources(
            &self,
        ) -> Option<Retained<NSArray<NSTextInputSourceIdentifier>>>;

        #[method_id(@__retain_semantics Other selectedKeyboardInputSource)]
        pub fn selectedKeyboardInputSource(&self) -> Option<Retained<NSTextInputSourceIdentifier>>;

        /// Setter for [`selectedKeyboardInputSource`][Self::selectedKeyboardInputSource].
        #[method(setSelectedKeyboardInputSource:)]
        pub unsafe fn setSelectedKeyboardInputSource(
            &self,
            selected_keyboard_input_source: Option<&NSTextInputSourceIdentifier>,
        );

        /// ** Text Input source attributes ***
        #[method_id(@__retain_semantics Other localizedNameForInputSource:)]
        pub unsafe fn localizedNameForInputSource(
            input_source_identifier: &NSTextInputSourceIdentifier,
            mtm: MainThreadMarker,
        ) -> Option<Retained<NSString>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextInputContext {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern "C" {
    /// ** Notifications ***
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextinputcontextkeyboardselectiondidchangenotification?language=objc)
    pub static NSTextInputContextKeyboardSelectionDidChangeNotification:
        &'static NSNotificationName;
}
