//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odquerydelegate?language=objc)
    pub unsafe trait ODQueryDelegate: NSObjectProtocol {
        #[method(query:foundResults:error:)]
        unsafe fn query_foundResults_error(
            &self,
            in_query: Option<&ODQuery>,
            in_results: Option<&NSArray>,
            in_error: Option<&NSError>,
        );
    }

    unsafe impl ProtocolType for dyn ODQueryDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/opendirectory/odquery?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ODQuery;
);

unsafe impl NSCopying for ODQuery {}

unsafe impl CopyingHelper for ODQuery {
    type Result = Self;
}

unsafe impl NSObjectProtocol for ODQuery {}

extern_methods!(
    unsafe impl ODQuery {
        #[cfg(all(feature = "CFOpenDirectoryConstants", feature = "ODNode"))]
        #[method_id(@__retain_semantics Other queryWithNode:forRecordTypes:attribute:matchType:queryValues:returnAttributes:maximumResults:error:)]
        pub unsafe fn queryWithNode_forRecordTypes_attribute_matchType_queryValues_returnAttributes_maximumResults_error(
            in_node: Option<&ODNode>,
            in_record_type_or_list: Option<&AnyObject>,
            in_attribute: Option<&ODAttributeType>,
            in_match_type: ODMatchType,
            in_query_value_or_list: Option<&AnyObject>,
            in_return_attribute_or_list: Option<&AnyObject>,
            in_maximum_results: NSInteger,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<ODQuery>>;

        #[cfg(all(feature = "CFOpenDirectoryConstants", feature = "ODNode"))]
        #[method_id(@__retain_semantics Init initWithNode:forRecordTypes:attribute:matchType:queryValues:returnAttributes:maximumResults:error:)]
        pub unsafe fn initWithNode_forRecordTypes_attribute_matchType_queryValues_returnAttributes_maximumResults_error(
            this: Allocated<Self>,
            in_node: Option<&ODNode>,
            in_record_type_or_list: Option<&AnyObject>,
            in_attribute: Option<&ODAttributeType>,
            in_match_type: ODMatchType,
            in_query_value_or_list: Option<&AnyObject>,
            in_return_attribute_or_list: Option<&AnyObject>,
            in_maximum_results: NSInteger,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other resultsAllowingPartial:error:)]
        pub unsafe fn resultsAllowingPartial_error(
            &self,
            in_allow_partial_results: bool,
            out_error: Option<&mut Option<Retained<NSError>>>,
        ) -> Option<Retained<NSArray>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Retained<ProtocolObject<dyn ODQueryDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn ODQueryDelegate>>);

        #[method(scheduleInRunLoop:forMode:)]
        pub unsafe fn scheduleInRunLoop_forMode(
            &self,
            in_run_loop: Option<&NSRunLoop>,
            in_mode: Option<&NSString>,
        );

        #[method(removeFromRunLoop:forMode:)]
        pub unsafe fn removeFromRunLoop_forMode(
            &self,
            in_run_loop: Option<&NSRunLoop>,
            in_mode: Option<&NSString>,
        );

        #[method(synchronize)]
        pub unsafe fn synchronize(&self);

        #[method_id(@__retain_semantics Other operationQueue)]
        pub unsafe fn operationQueue(&self) -> Option<Retained<NSOperationQueue>>;

        #[method(setOperationQueue:)]
        pub unsafe fn setOperationQueue(&self, operation_queue: Option<&NSOperationQueue>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ODQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
