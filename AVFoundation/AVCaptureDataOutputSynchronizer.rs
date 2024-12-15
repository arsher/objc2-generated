//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-media")]
use objc2_core_media::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturedataoutputsynchronizer?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureDataOutputSynchronizer;
);

unsafe impl NSObjectProtocol for AVCaptureDataOutputSynchronizer {}

extern_methods!(
    unsafe impl AVCaptureDataOutputSynchronizer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "AVCaptureOutputBase")]
        #[method_id(@__retain_semantics Init initWithDataOutputs:)]
        pub unsafe fn initWithDataOutputs(
            this: Allocated<Self>,
            data_outputs: &NSArray<AVCaptureOutput>,
        ) -> Retained<Self>;

        #[cfg(feature = "AVCaptureOutputBase")]
        #[method_id(@__retain_semantics Other dataOutputs)]
        pub unsafe fn dataOutputs(&self) -> Retained<NSArray<AVCaptureOutput>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn AVCaptureDataOutputSynchronizerDelegate>>>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturedataoutputsynchronizerdelegate?language=objc)
    pub unsafe trait AVCaptureDataOutputSynchronizerDelegate: NSObjectProtocol {
        #[method(dataOutputSynchronizer:didOutputSynchronizedDataCollection:)]
        unsafe fn dataOutputSynchronizer_didOutputSynchronizedDataCollection(
            &self,
            synchronizer: &AVCaptureDataOutputSynchronizer,
            synchronized_data_collection: &AVCaptureSynchronizedDataCollection,
        );
    }

    unsafe impl ProtocolType for dyn AVCaptureDataOutputSynchronizerDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturesynchronizeddatacollection?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureSynchronizedDataCollection;
);

unsafe impl NSFastEnumeration for AVCaptureSynchronizedDataCollection {}

unsafe impl NSObjectProtocol for AVCaptureSynchronizedDataCollection {}

extern_methods!(
    unsafe impl AVCaptureSynchronizedDataCollection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "AVCaptureOutputBase")]
        #[method_id(@__retain_semantics Other synchronizedDataForCaptureOutput:)]
        pub unsafe fn synchronizedDataForCaptureOutput(
            &self,
            capture_output: &AVCaptureOutput,
        ) -> Option<Retained<AVCaptureSynchronizedData>>;

        #[cfg(feature = "AVCaptureOutputBase")]
        #[method_id(@__retain_semantics Other objectForKeyedSubscript:)]
        pub unsafe fn objectForKeyedSubscript(
            &self,
            key: &AVCaptureOutput,
        ) -> Option<Retained<AVCaptureSynchronizedData>>;

        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturesynchronizeddata?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureSynchronizedData;
);

unsafe impl NSObjectProtocol for AVCaptureSynchronizedData {}

extern_methods!(
    unsafe impl AVCaptureSynchronizedData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[cfg(feature = "objc2-core-media")]
        #[method(timestamp)]
        pub unsafe fn timestamp(&self) -> CMTime;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturesynchronizedsamplebufferdata?language=objc)
    #[unsafe(super(AVCaptureSynchronizedData, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureSynchronizedSampleBufferData;
);

unsafe impl NSObjectProtocol for AVCaptureSynchronizedSampleBufferData {}

extern_methods!(
    unsafe impl AVCaptureSynchronizedSampleBufferData {
        #[cfg(feature = "objc2-core-media")]
        #[method(sampleBuffer)]
        pub unsafe fn sampleBuffer(&self) -> CMSampleBufferRef;

        #[method(sampleBufferWasDropped)]
        pub unsafe fn sampleBufferWasDropped(&self) -> bool;

        #[cfg(feature = "AVCaptureOutputBase")]
        #[method(droppedReason)]
        pub unsafe fn droppedReason(&self) -> AVCaptureOutputDataDroppedReason;
    }
);

extern_methods!(
    /// Methods declared on superclass `AVCaptureSynchronizedData`
    unsafe impl AVCaptureSynchronizedSampleBufferData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturesynchronizedmetadataobjectdata?language=objc)
    #[unsafe(super(AVCaptureSynchronizedData, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureSynchronizedMetadataObjectData;
);

unsafe impl NSObjectProtocol for AVCaptureSynchronizedMetadataObjectData {}

extern_methods!(
    unsafe impl AVCaptureSynchronizedMetadataObjectData {
        #[cfg(feature = "AVMetadataObject")]
        #[method_id(@__retain_semantics Other metadataObjects)]
        pub unsafe fn metadataObjects(&self) -> Retained<NSArray<AVMetadataObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `AVCaptureSynchronizedData`
    unsafe impl AVCaptureSynchronizedMetadataObjectData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avcapturesynchronizeddepthdata?language=objc)
    #[unsafe(super(AVCaptureSynchronizedData, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureSynchronizedDepthData;
);

unsafe impl NSObjectProtocol for AVCaptureSynchronizedDepthData {}

extern_methods!(
    unsafe impl AVCaptureSynchronizedDepthData {
        #[cfg(feature = "AVDepthData")]
        #[method_id(@__retain_semantics Other depthData)]
        pub unsafe fn depthData(&self) -> Retained<AVDepthData>;

        #[method(depthDataWasDropped)]
        pub unsafe fn depthDataWasDropped(&self) -> bool;

        #[cfg(feature = "AVCaptureOutputBase")]
        #[method(droppedReason)]
        pub unsafe fn droppedReason(&self) -> AVCaptureOutputDataDroppedReason;
    }
);

extern_methods!(
    /// Methods declared on superclass `AVCaptureSynchronizedData`
    unsafe impl AVCaptureSynchronizedDepthData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
