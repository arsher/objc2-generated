//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/storekit/skpaymentqueue?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "No longer supported"]
    pub struct SKPaymentQueue;
);

unsafe impl Send for SKPaymentQueue {}

unsafe impl Sync for SKPaymentQueue {}

unsafe impl NSObjectProtocol for SKPaymentQueue {}

extern_methods!(
    unsafe impl SKPaymentQueue {
        #[deprecated = "No longer supported"]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn SKPaymentQueueDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[deprecated = "No longer supported"]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn SKPaymentQueueDelegate>>,
        );

        #[cfg(feature = "SKStorefront")]
        #[deprecated = "Use Storefront.current"]
        #[method_id(@__retain_semantics Other storefront)]
        pub unsafe fn storefront(&self) -> Option<Retained<SKStorefront>>;

        #[deprecated = "No longer supported"]
        #[method_id(@__retain_semantics Other defaultQueue)]
        pub unsafe fn defaultQueue() -> Retained<Self>;

        #[deprecated = "Use AppStore.canMakePayments"]
        #[method(canMakePayments)]
        pub unsafe fn canMakePayments() -> bool;

        #[cfg(feature = "SKPayment")]
        #[deprecated = "Use Product.purchase(confirmIn:options:)"]
        #[method(addPayment:)]
        pub unsafe fn addPayment(&self, payment: &SKPayment);

        #[deprecated = "Use AppStore.sync()"]
        #[method(restoreCompletedTransactions)]
        pub unsafe fn restoreCompletedTransactions(&self);

        #[deprecated = "Use AppStore.sync()"]
        #[method(restoreCompletedTransactionsWithApplicationUsername:)]
        pub unsafe fn restoreCompletedTransactionsWithApplicationUsername(
            &self,
            username: Option<&NSString>,
        );

        #[cfg(feature = "SKPaymentTransaction")]
        #[deprecated = "Use Transaction.finish()"]
        #[method(finishTransaction:)]
        pub unsafe fn finishTransaction(&self, transaction: &SKPaymentTransaction);

        #[cfg(feature = "SKDownload")]
        #[deprecated = "Hosted content is no longer supported"]
        #[method(startDownloads:)]
        pub unsafe fn startDownloads(&self, downloads: &NSArray<SKDownload>);

        #[cfg(feature = "SKDownload")]
        #[deprecated = "Hosted content is no longer supported"]
        #[method(pauseDownloads:)]
        pub unsafe fn pauseDownloads(&self, downloads: &NSArray<SKDownload>);

        #[cfg(feature = "SKDownload")]
        #[deprecated = "Hosted content is no longer supported"]
        #[method(resumeDownloads:)]
        pub unsafe fn resumeDownloads(&self, downloads: &NSArray<SKDownload>);

        #[cfg(feature = "SKDownload")]
        #[deprecated = "Hosted content is no longer supported"]
        #[method(cancelDownloads:)]
        pub unsafe fn cancelDownloads(&self, downloads: &NSArray<SKDownload>);

        #[deprecated = "Use Transaction.updates or PurchaseResult from Product.purchase(confirmIn:options:)"]
        #[method(addTransactionObserver:)]
        pub unsafe fn addTransactionObserver(
            &self,
            observer: &ProtocolObject<dyn SKPaymentTransactionObserver>,
        );

        #[deprecated = "No longer supported"]
        #[method(removeTransactionObserver:)]
        pub unsafe fn removeTransactionObserver(
            &self,
            observer: &ProtocolObject<dyn SKPaymentTransactionObserver>,
        );

        #[deprecated = "Use Transaction.updates or PurchaseResult from Product.purchase(confirmIn:options:)"]
        #[method_id(@__retain_semantics Other transactionObservers)]
        pub unsafe fn transactionObservers(
            &self,
        ) -> Retained<NSArray<ProtocolObject<dyn SKPaymentTransactionObserver>>>;

        #[cfg(feature = "SKPaymentTransaction")]
        #[deprecated = "Use Transaction.unfinished"]
        #[method_id(@__retain_semantics Other transactions)]
        pub unsafe fn transactions(&self) -> Retained<NSArray<SKPaymentTransaction>>;

        #[deprecated = "Use Message.messages and Message.display(in:)"]
        #[method(showPriceConsentIfNeeded)]
        pub unsafe fn showPriceConsentIfNeeded(&self);

        #[deprecated = "Use AppStore.presentOfferCodeRedeemSheet(in:)"]
        #[method(presentCodeRedemptionSheet)]
        pub unsafe fn presentCodeRedemptionSheet(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl SKPaymentQueue {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/storekit/skpaymentqueuedelegate?language=objc)
    #[deprecated = "No longer supported"]
    pub unsafe trait SKPaymentQueueDelegate: NSObjectProtocol {
        #[cfg(all(feature = "SKPaymentTransaction", feature = "SKStorefront"))]
        #[deprecated = "Pass Product.PurchaseOption.onStorefrontChange(shouldContinuePurchase:) to product.purchase(options:)"]
        #[optional]
        #[method(paymentQueue:shouldContinueTransaction:inStorefront:)]
        unsafe fn paymentQueue_shouldContinueTransaction_inStorefront(
            &self,
            payment_queue: &SKPaymentQueue,
            transaction: &SKPaymentTransaction,
            new_storefront: &SKStorefront,
        ) -> bool;

        #[deprecated = "Use Message.messages and Message.display(in:)"]
        #[optional]
        #[method(paymentQueueShouldShowPriceConsent:)]
        unsafe fn paymentQueueShouldShowPriceConsent(&self, payment_queue: &SKPaymentQueue)
            -> bool;
    }

    unsafe impl ProtocolType for dyn SKPaymentQueueDelegate {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/storekit/skpaymenttransactionobserver?language=objc)
    #[deprecated = "Use StoreKit 2 Transaction APIs"]
    pub unsafe trait SKPaymentTransactionObserver: NSObjectProtocol {
        #[cfg(feature = "SKPaymentTransaction")]
        #[deprecated = "Use StoreKit 2 Transaction APIs"]
        #[method(paymentQueue:updatedTransactions:)]
        unsafe fn paymentQueue_updatedTransactions(
            &self,
            queue: &SKPaymentQueue,
            transactions: &NSArray<SKPaymentTransaction>,
        );

        #[cfg(feature = "SKPaymentTransaction")]
        #[deprecated = "Use StoreKit 2 Transaction APIs"]
        #[optional]
        #[method(paymentQueue:removedTransactions:)]
        unsafe fn paymentQueue_removedTransactions(
            &self,
            queue: &SKPaymentQueue,
            transactions: &NSArray<SKPaymentTransaction>,
        );

        #[deprecated = "Use AppStore.sync()"]
        #[optional]
        #[method(paymentQueue:restoreCompletedTransactionsFailedWithError:)]
        unsafe fn paymentQueue_restoreCompletedTransactionsFailedWithError(
            &self,
            queue: &SKPaymentQueue,
            error: &NSError,
        );

        #[deprecated = "Use AppStore.sync()"]
        #[optional]
        #[method(paymentQueueRestoreCompletedTransactionsFinished:)]
        unsafe fn paymentQueueRestoreCompletedTransactionsFinished(&self, queue: &SKPaymentQueue);

        #[cfg(feature = "SKDownload")]
        #[deprecated = "Hosted content is no longer supported"]
        #[optional]
        #[method(paymentQueue:updatedDownloads:)]
        unsafe fn paymentQueue_updatedDownloads(
            &self,
            queue: &SKPaymentQueue,
            downloads: &NSArray<SKDownload>,
        );

        #[cfg(all(feature = "SKPayment", feature = "SKProduct"))]
        #[deprecated = "Use PurchaseIntent.intents"]
        #[optional]
        #[method(paymentQueue:shouldAddStorePayment:forProduct:)]
        unsafe fn paymentQueue_shouldAddStorePayment_forProduct(
            &self,
            queue: &SKPaymentQueue,
            payment: &SKPayment,
            product: &SKProduct,
        ) -> bool;

        #[deprecated = "Use Storefront.updates"]
        #[optional]
        #[method(paymentQueueDidChangeStorefront:)]
        unsafe fn paymentQueueDidChangeStorefront(&self, queue: &SKPaymentQueue);

        #[deprecated = "Use Transaction.updates"]
        #[optional]
        #[method(paymentQueue:didRevokeEntitlementsForProductIdentifiers:)]
        unsafe fn paymentQueue_didRevokeEntitlementsForProductIdentifiers(
            &self,
            queue: &SKPaymentQueue,
            product_identifiers: &NSArray<NSString>,
        );
    }

    unsafe impl ProtocolType for dyn SKPaymentTransactionObserver {}
);
