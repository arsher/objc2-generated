//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_protocol!(
    #[deprecated = "You should instead implement the GKChallengeListener protocol and register a listener with GKLocalPlayer."]
    pub unsafe trait GKChallengeEventHandlerDelegate: NSObjectProtocol {
        #[cfg(feature = "GameKit_GKChallenge")]
        #[optional]
        #[method(localPlayerDidSelectChallenge:)]
        unsafe fn localPlayerDidSelectChallenge(&self, challenge: Option<&GKChallenge>);

        #[cfg(feature = "GameKit_GKChallenge")]
        #[optional]
        #[method(shouldShowBannerForLocallyReceivedChallenge:)]
        unsafe fn shouldShowBannerForLocallyReceivedChallenge(
            &self,
            challenge: Option<&GKChallenge>,
        ) -> bool;

        #[cfg(feature = "GameKit_GKChallenge")]
        #[optional]
        #[method(localPlayerDidReceiveChallenge:)]
        unsafe fn localPlayerDidReceiveChallenge(&self, challenge: Option<&GKChallenge>);

        #[cfg(feature = "GameKit_GKChallenge")]
        #[optional]
        #[method(shouldShowBannerForLocallyCompletedChallenge:)]
        unsafe fn shouldShowBannerForLocallyCompletedChallenge(
            &self,
            challenge: Option<&GKChallenge>,
        ) -> bool;

        #[cfg(feature = "GameKit_GKChallenge")]
        #[optional]
        #[method(localPlayerDidCompleteChallenge:)]
        unsafe fn localPlayerDidCompleteChallenge(&self, challenge: Option<&GKChallenge>);

        #[cfg(feature = "GameKit_GKChallenge")]
        #[optional]
        #[method(shouldShowBannerForRemotelyCompletedChallenge:)]
        unsafe fn shouldShowBannerForRemotelyCompletedChallenge(
            &self,
            challenge: Option<&GKChallenge>,
        ) -> bool;

        #[cfg(feature = "GameKit_GKChallenge")]
        #[optional]
        #[method(remotePlayerDidCompleteChallenge:)]
        unsafe fn remotePlayerDidCompleteChallenge(&self, challenge: Option<&GKChallenge>);
    }

    unsafe impl ProtocolType for dyn GKChallengeEventHandlerDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKChallengeEventHandler")]
    #[deprecated = "You should instead implement the GKChallengeListener protocol and register a listener with GKLocalPlayer."]
    pub struct GKChallengeEventHandler;

    #[cfg(feature = "GameKit_GKChallengeEventHandler")]
    unsafe impl ClassType for GKChallengeEventHandler {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameKit_GKChallengeEventHandler")]
unsafe impl NSObjectProtocol for GKChallengeEventHandler {}

extern_methods!(
    #[cfg(feature = "GameKit_GKChallengeEventHandler")]
    unsafe impl GKChallengeEventHandler {
        #[deprecated]
        #[method_id(@__retain_semantics Other challengeEventHandler)]
        pub unsafe fn challengeEventHandler() -> Option<Id<GKChallengeEventHandler>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn GKChallengeEventHandlerDelegate>>>;

        #[deprecated]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn GKChallengeEventHandlerDelegate>>,
        );
    }
);
