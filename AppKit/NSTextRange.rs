//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextlocation?language=objc)
    pub unsafe trait NSTextLocation: NSObjectProtocol {
        #[method(compare:)]
        unsafe fn compare(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> NSComparisonResult;
    }

    unsafe impl ProtocolType for dyn NSTextLocation {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextrange?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextRange;
);

unsafe impl NSObjectProtocol for NSTextRange {}

extern_methods!(
    unsafe impl NSTextRange {
        #[method_id(@__retain_semantics Init initWithLocation:endLocation:)]
        pub unsafe fn initWithLocation_endLocation(
            this: Allocated<Self>,
            location: &ProtocolObject<dyn NSTextLocation>,
            end_location: Option<&ProtocolObject<dyn NSTextLocation>>,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Init initWithLocation:)]
        pub unsafe fn initWithLocation(
            this: Allocated<Self>,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method(isEmpty)]
        pub unsafe fn isEmpty(&self) -> bool;

        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Retained<ProtocolObject<dyn NSTextLocation>>;

        #[method_id(@__retain_semantics Other endLocation)]
        pub unsafe fn endLocation(&self) -> Retained<ProtocolObject<dyn NSTextLocation>>;

        #[method(isEqualToTextRange:)]
        pub unsafe fn isEqualToTextRange(&self, text_range: &NSTextRange) -> bool;

        #[method(containsLocation:)]
        pub unsafe fn containsLocation(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> bool;

        #[method(containsRange:)]
        pub unsafe fn containsRange(&self, text_range: &NSTextRange) -> bool;

        #[method(intersectsWithTextRange:)]
        pub unsafe fn intersectsWithTextRange(&self, text_range: &NSTextRange) -> bool;

        #[method_id(@__retain_semantics Other textRangeByIntersectingWithTextRange:)]
        pub unsafe fn textRangeByIntersectingWithTextRange(
            &self,
            text_range: &NSTextRange,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other textRangeByFormingUnionWithTextRange:)]
        pub unsafe fn textRangeByFormingUnionWithTextRange(
            &self,
            text_range: &NSTextRange,
        ) -> Retained<Self>;
    }
);
