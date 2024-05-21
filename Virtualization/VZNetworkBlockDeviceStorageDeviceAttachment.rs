//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZStorageDeviceAttachment")]
    pub struct VZNetworkBlockDeviceStorageDeviceAttachment;

    #[cfg(feature = "VZStorageDeviceAttachment")]
    unsafe impl ClassType for VZNetworkBlockDeviceStorageDeviceAttachment {
        #[inherits(NSObject)]
        type Super = VZStorageDeviceAttachment;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "VZStorageDeviceAttachment")]
unsafe impl NSObjectProtocol for VZNetworkBlockDeviceStorageDeviceAttachment {}

extern_methods!(
    #[cfg(feature = "VZStorageDeviceAttachment")]
    unsafe impl VZNetworkBlockDeviceStorageDeviceAttachment {
        #[cfg(feature = "VZDiskSynchronizationMode")]
        #[method_id(@__retain_semantics Init initWithURL:timeout:forcedReadOnly:synchronizationMode:error:_)]
        pub unsafe fn initWithURL_timeout_forcedReadOnly_synchronizationMode_error(
            this: Allocated<Self>,
            url: &NSURL,
            timeout: NSTimeInterval,
            forced_read_only: bool,
            synchronization_mode: VZDiskSynchronizationMode,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Init initWithURL:error:_)]
        pub unsafe fn initWithURL_error(
            this: Allocated<Self>,
            url: &NSURL,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method(validateURL:error:_)]
        pub unsafe fn validateURL_error(url: &NSURL) -> Result<(), Retained<NSError>>;

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Retained<NSURL>;

        #[method(timeout)]
        pub unsafe fn timeout(&self) -> NSTimeInterval;

        #[method(isForcedReadOnly)]
        pub unsafe fn isForcedReadOnly(&self) -> bool;

        #[cfg(feature = "VZDiskSynchronizationMode")]
        #[method(synchronizationMode)]
        pub unsafe fn synchronizationMode(&self) -> VZDiskSynchronizationMode;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn VZNetworkBlockDeviceStorageDeviceAttachmentDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<
                &ProtocolObject<dyn VZNetworkBlockDeviceStorageDeviceAttachmentDelegate>,
            >,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `VZStorageDeviceAttachment`
    #[cfg(feature = "VZStorageDeviceAttachment")]
    unsafe impl VZNetworkBlockDeviceStorageDeviceAttachment {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_protocol!(
    pub unsafe trait VZNetworkBlockDeviceStorageDeviceAttachmentDelegate:
        NSObjectProtocol
    {
        #[cfg(feature = "VZStorageDeviceAttachment")]
        #[optional]
        #[method(attachmentWasConnected:)]
        unsafe fn attachmentWasConnected(
            &self,
            attachment: &VZNetworkBlockDeviceStorageDeviceAttachment,
        );

        #[cfg(feature = "VZStorageDeviceAttachment")]
        #[optional]
        #[method(attachment:didEncounterError:)]
        unsafe fn attachment_didEncounterError(
            &self,
            attachment: &VZNetworkBlockDeviceStorageDeviceAttachment,
            error: &NSError,
        );
    }

    unsafe impl ProtocolType for dyn VZNetworkBlockDeviceStorageDeviceAttachmentDelegate {}
);
