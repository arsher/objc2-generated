//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKAdImpression")]
    pub struct SKAdImpression;

    #[cfg(feature = "StoreKit_SKAdImpression")]
    unsafe impl ClassType for SKAdImpression {
        type Super = NSObject;
    }
);

#[cfg(feature = "StoreKit_SKAdImpression")]
unsafe impl NSObjectProtocol for SKAdImpression {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKAdImpression")]
    unsafe impl SKAdImpression {
        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other sourceAppStoreItemIdentifier)]
        pub unsafe fn sourceAppStoreItemIdentifier(&self) -> Id<NSNumber, Shared>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method(setSourceAppStoreItemIdentifier:)]
        pub unsafe fn setSourceAppStoreItemIdentifier(
            &self,
            source_app_store_item_identifier: &NSNumber,
        );

        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other advertisedAppStoreItemIdentifier)]
        pub unsafe fn advertisedAppStoreItemIdentifier(&self) -> Id<NSNumber, Shared>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method(setAdvertisedAppStoreItemIdentifier:)]
        pub unsafe fn setAdvertisedAppStoreItemIdentifier(
            &self,
            advertised_app_store_item_identifier: &NSNumber,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other adNetworkIdentifier)]
        pub unsafe fn adNetworkIdentifier(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAdNetworkIdentifier:)]
        pub unsafe fn setAdNetworkIdentifier(&self, ad_network_identifier: &NSString);

        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other adCampaignIdentifier)]
        pub unsafe fn adCampaignIdentifier(&self) -> Id<NSNumber, Shared>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method(setAdCampaignIdentifier:)]
        pub unsafe fn setAdCampaignIdentifier(&self, ad_campaign_identifier: &NSNumber);

        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other sourceIdentifier)]
        pub unsafe fn sourceIdentifier(&self) -> Id<NSNumber, Shared>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method(setSourceIdentifier:)]
        pub unsafe fn setSourceIdentifier(&self, source_identifier: &NSNumber);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other adImpressionIdentifier)]
        pub unsafe fn adImpressionIdentifier(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAdImpressionIdentifier:)]
        pub unsafe fn setAdImpressionIdentifier(&self, ad_impression_identifier: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other adType)]
        pub unsafe fn adType(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAdType:)]
        pub unsafe fn setAdType(&self, ad_type: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other adDescription)]
        pub unsafe fn adDescription(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAdDescription:)]
        pub unsafe fn setAdDescription(&self, ad_description: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other adPurchaserName)]
        pub unsafe fn adPurchaserName(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAdPurchaserName:)]
        pub unsafe fn setAdPurchaserName(&self, ad_purchaser_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other timestamp)]
        pub unsafe fn timestamp(&self) -> Id<NSNumber, Shared>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method(setTimestamp:)]
        pub unsafe fn setTimestamp(&self, timestamp: &NSNumber);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other signature)]
        pub unsafe fn signature(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSignature:)]
        pub unsafe fn setSignature(&self, signature: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other version)]
        pub unsafe fn version(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setVersion:)]
        pub unsafe fn setVersion(&self, version: &NSString);
    }
);