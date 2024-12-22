//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// BCParameterName is used to define custom parameters when opening the transcript.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/businesschat/bcparametername?language=objc)
// NS_TYPED_EXTENSIBLE_ENUM
pub type BCParameterName = NSString;

extern "C" {
    /// Intent is used to help the support agent or business system identify the product, service, account, or other context when the customer tapped the button to send the message.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/businesschat/bcparameternameintent?language=objc)
    pub static BCParameterNameIntent: &'static BCParameterName;
}

extern "C" {
    /// Group is used to help the business or customer service platform route the message to the appropriate support agent group.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/businesschat/bcparameternamegroup?language=objc)
    pub static BCParameterNameGroup: &'static BCParameterName;
}

extern "C" {
    /// Body is used to help the customer by providing a pre-configured contextual message that the customer can tap to send to smooth the transition into Business Chat.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/businesschat/bcparameternamebody?language=objc)
    pub static BCParameterNameBody: &'static BCParameterName;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/businesschat/bcchataction?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated]
    pub struct BCChatAction;
);

unsafe impl NSObjectProtocol for BCChatAction {}

extern_methods!(
    unsafe impl BCChatAction {
        /// Open the chat transcript configured for a given business.
        ///
        ///
        /// Parameter `businessIdentifier`: The business identifier for the given business.
        ///
        /// Parameter `intentParameters`: Parameters to be sent with the initial message.
        #[deprecated]
        #[method(openTranscript:intentParameters:)]
        pub unsafe fn openTranscript_intentParameters(
            business_identifier: &NSString,
            intent_parameters: &NSDictionary<BCParameterName, NSString>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl BCChatAction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
