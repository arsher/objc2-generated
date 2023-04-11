//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::SoundAnalysis::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "SoundAnalysis_SNAudioStreamAnalyzer")]
    pub struct SNAudioStreamAnalyzer;

    #[cfg(feature = "SoundAnalysis_SNAudioStreamAnalyzer")]
    unsafe impl ClassType for SNAudioStreamAnalyzer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "SoundAnalysis_SNAudioStreamAnalyzer")]
unsafe impl NSObjectProtocol for SNAudioStreamAnalyzer {}

extern_methods!(
    #[cfg(feature = "SoundAnalysis_SNAudioStreamAnalyzer")]
    unsafe impl SNAudioStreamAnalyzer {
        #[cfg(feature = "AVFAudio_AVAudioFormat")]
        #[method_id(@__retain_semantics Init initWithFormat:)]
        pub unsafe fn initWithFormat(
            this: Option<Allocated<Self>>,
            format: &AVAudioFormat,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(addRequest:withObserver:error:_)]
        pub unsafe fn addRequest_withObserver_error(
            &self,
            request: &ProtocolObject<dyn SNRequest>,
            observer: &ProtocolObject<dyn SNResultsObserving>,
        ) -> Result<(), Id<NSError>>;

        #[method(removeRequest:)]
        pub unsafe fn removeRequest(&self, request: &ProtocolObject<dyn SNRequest>);

        #[method(removeAllRequests)]
        pub unsafe fn removeAllRequests(&self);

        #[cfg(feature = "AVFAudio_AVAudioBuffer")]
        #[method(analyzeAudioBuffer:atAudioFramePosition:)]
        pub unsafe fn analyzeAudioBuffer_atAudioFramePosition(
            &self,
            audio_buffer: &AVAudioBuffer,
            audio_frame_position: AVAudioFramePosition,
        );

        #[method(completeAnalysis)]
        pub unsafe fn completeAnalysis(&self);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "SoundAnalysis_SNAudioFileAnalyzer")]
    pub struct SNAudioFileAnalyzer;

    #[cfg(feature = "SoundAnalysis_SNAudioFileAnalyzer")]
    unsafe impl ClassType for SNAudioFileAnalyzer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "SoundAnalysis_SNAudioFileAnalyzer")]
unsafe impl NSObjectProtocol for SNAudioFileAnalyzer {}

extern_methods!(
    #[cfg(feature = "SoundAnalysis_SNAudioFileAnalyzer")]
    unsafe impl SNAudioFileAnalyzer {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithURL:error:_)]
        pub unsafe fn initWithURL_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(addRequest:withObserver:error:_)]
        pub unsafe fn addRequest_withObserver_error(
            &self,
            request: &ProtocolObject<dyn SNRequest>,
            observer: &ProtocolObject<dyn SNResultsObserving>,
        ) -> Result<(), Id<NSError>>;

        #[method(removeRequest:)]
        pub unsafe fn removeRequest(&self, request: &ProtocolObject<dyn SNRequest>);

        #[method(removeAllRequests)]
        pub unsafe fn removeAllRequests(&self);

        #[method(analyze)]
        pub unsafe fn analyze(&self);

        #[method(analyzeWithCompletionHandler:)]
        pub unsafe fn analyzeWithCompletionHandler(&self, completion_handler: &Block<(Bool,), ()>);

        #[method(cancelAnalysis)]
        pub unsafe fn cancelAnalysis(&self);
    }
);
