//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct WKContentRuleListStore;

    unsafe impl ClassType for WKContentRuleListStore {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for WKContentRuleListStore {}

extern_methods!(
    unsafe impl WKContentRuleListStore {
        #[method_id(@__retain_semantics Other defaultStore)]
        pub unsafe fn defaultStore(mtm: MainThreadMarker) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other storeWithURL:)]
        pub unsafe fn storeWithURL(
            url: Option<&NSURL>,
            mtm: MainThreadMarker,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "WKContentRuleList", feature = "block2"))]
        #[method(compileContentRuleListForIdentifier:encodedContentRuleList:completionHandler:)]
        pub unsafe fn compileContentRuleListForIdentifier_encodedContentRuleList_completionHandler(
            &self,
            identifier: Option<&NSString>,
            encoded_content_rule_list: Option<&NSString>,
            completion_handler: Option<
                &block2::Block<dyn Fn(*mut WKContentRuleList, *mut NSError)>,
            >,
        );

        #[cfg(all(feature = "WKContentRuleList", feature = "block2"))]
        #[method(lookUpContentRuleListForIdentifier:completionHandler:)]
        pub unsafe fn lookUpContentRuleListForIdentifier_completionHandler(
            &self,
            identifier: Option<&NSString>,
            completion_handler: Option<
                &block2::Block<dyn Fn(*mut WKContentRuleList, *mut NSError)>,
            >,
        );

        #[cfg(feature = "block2")]
        #[method(removeContentRuleListForIdentifier:completionHandler:)]
        pub unsafe fn removeContentRuleListForIdentifier_completionHandler(
            &self,
            identifier: Option<&NSString>,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "block2")]
        #[method(getAvailableContentRuleListIdentifiers:)]
        pub unsafe fn getAvailableContentRuleListIdentifiers(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn(*mut NSArray<NSString>)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl WKContentRuleListStore {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
