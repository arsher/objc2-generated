//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

typed_enum!(
    pub type SKAdNetworkCoarseConversionValue = NSString;
);

extern_static!(SKAdNetworkCoarseConversionValueHigh: &'static SKAdNetworkCoarseConversionValue);

extern_static!(SKAdNetworkCoarseConversionValueMedium: &'static SKAdNetworkCoarseConversionValue);

extern_static!(SKAdNetworkCoarseConversionValueLow: &'static SKAdNetworkCoarseConversionValue);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKAdNetwork")]
    pub struct SKAdNetwork;

    #[cfg(feature = "StoreKit_SKAdNetwork")]
    unsafe impl ClassType for SKAdNetwork {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "StoreKit_SKAdNetwork")]
unsafe impl NSObjectProtocol for SKAdNetwork {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKAdNetwork")]
    unsafe impl SKAdNetwork {
        #[cfg(all(feature = "Foundation_NSError", feature = "StoreKit_SKAdImpression"))]
        #[method(startImpression:completionHandler:)]
        pub unsafe fn startImpression_completionHandler(
            impression: &SKAdImpression,
            completion: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "StoreKit_SKAdImpression"))]
        #[method(endImpression:completionHandler:)]
        pub unsafe fn endImpression_completionHandler(
            impression: &SKAdImpression,
            completion: Option<&Block<(*mut NSError,), ()>>,
        );

        #[deprecated = "Use updatePostbackConversionValue:completionHandler: instead"]
        #[method(registerAppForAdNetworkAttribution)]
        pub unsafe fn registerAppForAdNetworkAttribution();

        #[deprecated = "Use updatePostbackConversionValue:completionHandler: instead"]
        #[method(updateConversionValue:)]
        pub unsafe fn updateConversionValue(conversion_value: NSInteger);

        #[cfg(feature = "Foundation_NSError")]
        #[method(updatePostbackConversionValue:completionHandler:)]
        pub unsafe fn updatePostbackConversionValue_completionHandler(
            conversion_value: NSInteger,
            completion: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(updatePostbackConversionValue:coarseValue:completionHandler:)]
        pub unsafe fn updatePostbackConversionValue_coarseValue_completionHandler(
            fine_value: NSInteger,
            coarse_value: &SKAdNetworkCoarseConversionValue,
            completion: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(updatePostbackConversionValue:coarseValue:lockWindow:completionHandler:)]
        pub unsafe fn updatePostbackConversionValue_coarseValue_lockWindow_completionHandler(
            fine_value: NSInteger,
            coarse_value: &SKAdNetworkCoarseConversionValue,
            lock_window: bool,
            completion: Option<&Block<(*mut NSError,), ()>>,
        );
    }
);

extern_static!(SKStoreProductParameterAdNetworkAttributionSignature: &'static NSString);

extern_static!(SKStoreProductParameterAdNetworkCampaignIdentifier: &'static NSString);

extern_static!(SKStoreProductParameterAdNetworkSourceIdentifier: &'static NSString);

extern_static!(SKStoreProductParameterAdNetworkIdentifier: &'static NSString);

extern_static!(SKStoreProductParameterAdNetworkNonce: &'static NSString);

extern_static!(SKStoreProductParameterAdNetworkTimestamp: &'static NSString);

extern_static!(SKStoreProductParameterAdNetworkSourceAppStoreIdentifier: &'static NSString);

extern_static!(SKStoreProductParameterAdNetworkVersion: &'static NSString);
