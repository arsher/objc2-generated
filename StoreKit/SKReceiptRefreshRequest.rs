//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/storekit/skreceiptrefreshrequest?language=objc)
    #[unsafe(super(SKRequest, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "SKRequest")]
    #[deprecated = "Use Transaction.all and AppTransaction.shared"]
    pub struct SKReceiptRefreshRequest;
);

#[cfg(feature = "SKRequest")]
unsafe impl NSObjectProtocol for SKReceiptRefreshRequest {}

extern_methods!(
    #[cfg(feature = "SKRequest")]
    unsafe impl SKReceiptRefreshRequest {
        #[deprecated = "Use Transaction.all and AppTransaction.shared"]
        #[method_id(@__retain_semantics Init initWithReceiptProperties:)]
        pub unsafe fn initWithReceiptProperties(
            this: Allocated<Self>,
            properties: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Retained<Self>;

        #[deprecated = "Use Transaction.all and AppTransaction.shared"]
        #[method_id(@__retain_semantics Other receiptProperties)]
        pub unsafe fn receiptProperties(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, AnyObject>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "SKRequest")]
    unsafe impl SKReceiptRefreshRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C-unwind" {
    pub fn SKTerminateForInvalidReceipt();
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/storekit/skreceiptpropertyisexpired?language=objc)
    pub static SKReceiptPropertyIsExpired: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/storekit/skreceiptpropertyisrevoked?language=objc)
    pub static SKReceiptPropertyIsRevoked: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/storekit/skreceiptpropertyisvolumepurchase?language=objc)
    pub static SKReceiptPropertyIsVolumePurchase: &'static NSString;
}
