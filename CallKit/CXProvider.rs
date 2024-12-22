//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-avf-audio")]
use objc2_avf_audio::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/callkit/cxcallendedreason?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CXCallEndedReason(pub NSInteger);
impl CXCallEndedReason {
    #[doc(alias = "CXCallEndedReasonFailed")]
    pub const Failed: Self = Self(1);
    #[doc(alias = "CXCallEndedReasonRemoteEnded")]
    pub const RemoteEnded: Self = Self(2);
    #[doc(alias = "CXCallEndedReasonUnanswered")]
    pub const Unanswered: Self = Self(3);
    #[doc(alias = "CXCallEndedReasonAnsweredElsewhere")]
    pub const AnsweredElsewhere: Self = Self(4);
    #[doc(alias = "CXCallEndedReasonDeclinedElsewhere")]
    pub const DeclinedElsewhere: Self = Self(5);
}

unsafe impl Encode for CXCallEndedReason {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for CXCallEndedReason {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/callkit/cxproviderdelegate?language=objc)
    pub unsafe trait CXProviderDelegate: NSObjectProtocol {
        /// Called when the provider has been reset. Delegates must respond to this callback by cleaning up all internal call state (disconnecting communication channels, releasing network resources, etc.). This callback can be treated as a request to end all calls without the need to respond to any actions
        #[method(providerDidReset:)]
        unsafe fn providerDidReset(&self, provider: &CXProvider);

        /// Called when the provider has been fully created and is ready to send actions and receive updates
        #[optional]
        #[method(providerDidBegin:)]
        unsafe fn providerDidBegin(&self, provider: &CXProvider);

        #[cfg(feature = "CXTransaction")]
        /// Called whenever a new transaction should be executed. Return whether or not the transaction was handled:
        ///
        /// - NO: the transaction was not handled indicating that the perform*CallAction methods should be called sequentially for each action in the transaction
        /// - YES: the transaction was handled and the perform*CallAction methods should not be called sequentially
        ///
        /// If the method is not implemented, NO is assumed.
        #[optional]
        #[method(provider:executeTransaction:)]
        unsafe fn provider_executeTransaction(
            &self,
            provider: &CXProvider,
            transaction: &CXTransaction,
        ) -> bool;

        #[cfg(all(
            feature = "CXAction",
            feature = "CXCallAction",
            feature = "CXStartCallAction"
        ))]
        #[optional]
        #[method(provider:performStartCallAction:)]
        unsafe fn provider_performStartCallAction(
            &self,
            provider: &CXProvider,
            action: &CXStartCallAction,
        );

        #[cfg(all(
            feature = "CXAction",
            feature = "CXAnswerCallAction",
            feature = "CXCallAction"
        ))]
        #[optional]
        #[method(provider:performAnswerCallAction:)]
        unsafe fn provider_performAnswerCallAction(
            &self,
            provider: &CXProvider,
            action: &CXAnswerCallAction,
        );

        #[cfg(all(
            feature = "CXAction",
            feature = "CXCallAction",
            feature = "CXEndCallAction"
        ))]
        #[optional]
        #[method(provider:performEndCallAction:)]
        unsafe fn provider_performEndCallAction(
            &self,
            provider: &CXProvider,
            action: &CXEndCallAction,
        );

        #[cfg(all(
            feature = "CXAction",
            feature = "CXCallAction",
            feature = "CXSetHeldCallAction"
        ))]
        #[optional]
        #[method(provider:performSetHeldCallAction:)]
        unsafe fn provider_performSetHeldCallAction(
            &self,
            provider: &CXProvider,
            action: &CXSetHeldCallAction,
        );

        #[cfg(all(
            feature = "CXAction",
            feature = "CXCallAction",
            feature = "CXSetMutedCallAction"
        ))]
        #[optional]
        #[method(provider:performSetMutedCallAction:)]
        unsafe fn provider_performSetMutedCallAction(
            &self,
            provider: &CXProvider,
            action: &CXSetMutedCallAction,
        );

        #[cfg(all(
            feature = "CXAction",
            feature = "CXCallAction",
            feature = "CXSetGroupCallAction"
        ))]
        #[optional]
        #[method(provider:performSetGroupCallAction:)]
        unsafe fn provider_performSetGroupCallAction(
            &self,
            provider: &CXProvider,
            action: &CXSetGroupCallAction,
        );

        #[cfg(all(
            feature = "CXAction",
            feature = "CXCallAction",
            feature = "CXPlayDTMFCallAction"
        ))]
        #[optional]
        #[method(provider:performPlayDTMFCallAction:)]
        unsafe fn provider_performPlayDTMFCallAction(
            &self,
            provider: &CXProvider,
            action: &CXPlayDTMFCallAction,
        );

        #[cfg(feature = "CXAction")]
        /// Called when an action was not performed in time and has been inherently failed. Depending on the action, this timeout may also force the call to end. An action that has already timed out should not be fulfilled or failed by the provider delegate
        #[optional]
        #[method(provider:timedOutPerformingAction:)]
        unsafe fn provider_timedOutPerformingAction(
            &self,
            provider: &CXProvider,
            action: &CXAction,
        );

        #[cfg(feature = "objc2-avf-audio")]
        /// Called when the provider's audio session activation state changes.
        #[optional]
        #[method(provider:didActivateAudioSession:)]
        unsafe fn provider_didActivateAudioSession(
            &self,
            provider: &CXProvider,
            audio_session: &AVAudioSession,
        );

        #[cfg(feature = "objc2-avf-audio")]
        #[optional]
        #[method(provider:didDeactivateAudioSession:)]
        unsafe fn provider_didDeactivateAudioSession(
            &self,
            provider: &CXProvider,
            audio_session: &AVAudioSession,
        );
    }

    unsafe impl ProtocolType for dyn CXProviderDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/callkit/cxprovider?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct CXProvider;
);

unsafe impl NSObjectProtocol for CXProvider {}

extern_methods!(
    unsafe impl CXProvider {
        #[cfg(feature = "CXProviderConfiguration")]
        /// Initialize a new provider instance with the supplied configuration
        #[method_id(@__retain_semantics Init initWithConfiguration:)]
        pub unsafe fn initWithConfiguration(
            this: Allocated<Self>,
            configuration: &CXProviderConfiguration,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(all(feature = "CXCallUpdate", feature = "block2"))]
        /// Report a new incoming call to the system.
        ///
        /// If completion is invoked with a non-nil `error`, the incoming call has been disallowed by the system and will not be displayed, so the provider should not proceed with the call.
        ///
        /// Completion block will be called on delegate queue, if specified, otherwise on a private serial queue.
        #[method(reportNewIncomingCallWithUUID:update:completion:)]
        pub unsafe fn reportNewIncomingCallWithUUID_update_completion(
            &self,
            uuid: &NSUUID,
            update: &CXCallUpdate,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );

        #[cfg(feature = "CXCallUpdate")]
        /// Report an update to call information.
        #[method(reportCallWithUUID:updated:)]
        pub unsafe fn reportCallWithUUID_updated(&self, uuid: &NSUUID, update: &CXCallUpdate);

        /// Report that a call ended. A nil value for `dateEnded` results in the ended date being set to now.
        #[method(reportCallWithUUID:endedAtDate:reason:)]
        pub unsafe fn reportCallWithUUID_endedAtDate_reason(
            &self,
            uuid: &NSUUID,
            date_ended: Option<&NSDate>,
            ended_reason: CXCallEndedReason,
        );

        /// Report that an outgoing call started connecting. A nil value for `dateStartedConnecting` results in the started connecting date being set to now.
        #[method(reportOutgoingCallWithUUID:startedConnectingAtDate:)]
        pub unsafe fn reportOutgoingCallWithUUID_startedConnectingAtDate(
            &self,
            uuid: &NSUUID,
            date_started_connecting: Option<&NSDate>,
        );

        /// Report that an outgoing call connected. A nil value for `dateConnected` results in the connected date being set to now.
        #[method(reportOutgoingCallWithUUID:connectedAtDate:)]
        pub unsafe fn reportOutgoingCallWithUUID_connectedAtDate(
            &self,
            uuid: &NSUUID,
            date_connected: Option<&NSDate>,
        );

        #[cfg(feature = "block2")]
        /// From within a Notification Service Extension, request the containing application be launched to handle an incoming VoIP call. The application's PKPushRegistryDelegate must handle the push upon launch.
        #[method(reportNewIncomingVoIPPushPayload:completion:)]
        pub unsafe fn reportNewIncomingVoIPPushPayload_completion(
            dictionary_payload: &NSDictionary,
            completion: Option<&block2::Block<dyn Fn(*mut NSError)>>,
        );

        #[cfg(feature = "CXProviderConfiguration")]
        /// The receiver's current configuration.
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Retained<CXProviderConfiguration>;

        #[cfg(feature = "CXProviderConfiguration")]
        /// Setter for [`configuration`][Self::configuration].
        #[method(setConfiguration:)]
        pub unsafe fn setConfiguration(&self, configuration: &CXProviderConfiguration);

        /// Invalidate the receiver. All existing calls will be marked as ended in failure. The provider must be invalidated before it is deallocated.
        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[cfg(feature = "CXTransaction")]
        /// List of all transactions that are incomplete.
        #[method_id(@__retain_semantics Other pendingTransactions)]
        pub unsafe fn pendingTransactions(&self) -> Retained<NSArray<CXTransaction>>;

        #[cfg(all(feature = "CXAction", feature = "CXCallAction"))]
        /// Returns subset of call actions contained in any transaction in -pendingTransactions of the specified class and with the specified call UUID.
        #[method_id(@__retain_semantics Other pendingCallActionsOfClass:withCallUUID:)]
        pub unsafe fn pendingCallActionsOfClass_withCallUUID(
            &self,
            call_action_class: &AnyClass,
            call_uuid: &NSUUID,
        ) -> Retained<NSArray<CXCallAction>>;
    }
);
