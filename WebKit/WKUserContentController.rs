//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKUserContentController;

    unsafe impl ClassType for WKUserContentController {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSCoding for WKUserContentController {}

unsafe impl NSObjectProtocol for WKUserContentController {}

unsafe impl NSSecureCoding for WKUserContentController {}

extern_methods!(
    unsafe impl WKUserContentController {
        #[cfg(feature = "WKUserScript")]
        #[method_id(@__retain_semantics Other userScripts)]
        pub unsafe fn userScripts(&self) -> Retained<NSArray<WKUserScript>>;

        #[cfg(feature = "WKUserScript")]
        #[method(addUserScript:)]
        pub unsafe fn addUserScript(&self, user_script: &WKUserScript);

        #[method(removeAllUserScripts)]
        pub unsafe fn removeAllUserScripts(&self);

        #[cfg(all(feature = "WKContentWorld", feature = "WKScriptMessageHandler"))]
        #[method(addScriptMessageHandler:contentWorld:name:)]
        pub unsafe fn addScriptMessageHandler_contentWorld_name(
            &self,
            script_message_handler: &ProtocolObject<dyn WKScriptMessageHandler>,
            world: &WKContentWorld,
            name: &NSString,
        );

        #[cfg(all(
            feature = "WKContentWorld",
            feature = "WKScriptMessageHandlerWithReply"
        ))]
        #[method(addScriptMessageHandlerWithReply:contentWorld:name:)]
        pub unsafe fn addScriptMessageHandlerWithReply_contentWorld_name(
            &self,
            script_message_handler_with_reply: &ProtocolObject<dyn WKScriptMessageHandlerWithReply>,
            content_world: &WKContentWorld,
            name: &NSString,
        );

        #[cfg(feature = "WKScriptMessageHandler")]
        #[method(addScriptMessageHandler:name:)]
        pub unsafe fn addScriptMessageHandler_name(
            &self,
            script_message_handler: &ProtocolObject<dyn WKScriptMessageHandler>,
            name: &NSString,
        );

        #[cfg(feature = "WKContentWorld")]
        #[method(removeScriptMessageHandlerForName:contentWorld:)]
        pub unsafe fn removeScriptMessageHandlerForName_contentWorld(
            &self,
            name: &NSString,
            content_world: &WKContentWorld,
        );

        #[method(removeScriptMessageHandlerForName:)]
        pub unsafe fn removeScriptMessageHandlerForName(&self, name: &NSString);

        #[cfg(feature = "WKContentWorld")]
        #[method(removeAllScriptMessageHandlersFromContentWorld:)]
        pub unsafe fn removeAllScriptMessageHandlersFromContentWorld(
            &self,
            content_world: &WKContentWorld,
        );

        #[method(removeAllScriptMessageHandlers)]
        pub unsafe fn removeAllScriptMessageHandlers(&self);

        #[cfg(feature = "WKContentRuleList")]
        #[method(addContentRuleList:)]
        pub unsafe fn addContentRuleList(&self, content_rule_list: &WKContentRuleList);

        #[cfg(feature = "WKContentRuleList")]
        #[method(removeContentRuleList:)]
        pub unsafe fn removeContentRuleList(&self, content_rule_list: &WKContentRuleList);

        #[method(removeAllContentRuleLists)]
        pub unsafe fn removeAllContentRuleLists(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKUserContentController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
