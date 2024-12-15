//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-audio-toolbox")]
#[cfg(not(target_os = "watchos"))]
use objc2_audio_toolbox::*;
use objc2_foundation::*;

use crate::*;

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiounittypeoutput?language=objc)
    pub static AVAudioUnitTypeOutput: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiounittypemusicdevice?language=objc)
    pub static AVAudioUnitTypeMusicDevice: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiounittypemusiceffect?language=objc)
    pub static AVAudioUnitTypeMusicEffect: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiounittypeformatconverter?language=objc)
    pub static AVAudioUnitTypeFormatConverter: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiounittypeeffect?language=objc)
    pub static AVAudioUnitTypeEffect: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiounittypemixer?language=objc)
    pub static AVAudioUnitTypeMixer: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiounittypepanner?language=objc)
    pub static AVAudioUnitTypePanner: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiounittypegenerator?language=objc)
    pub static AVAudioUnitTypeGenerator: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiounittypeofflineeffect?language=objc)
    pub static AVAudioUnitTypeOfflineEffect: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiounittypemidiprocessor?language=objc)
    pub static AVAudioUnitTypeMIDIProcessor: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiounitmanufacturernameapple?language=objc)
    pub static AVAudioUnitManufacturerNameApple: &'static NSString;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiounitcomponent?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioUnitComponent;
);

unsafe impl NSObjectProtocol for AVAudioUnitComponent {}

extern_methods!(
    unsafe impl AVAudioUnitComponent {
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other typeName)]
        pub unsafe fn typeName(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other localizedTypeName)]
        pub unsafe fn localizedTypeName(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other manufacturerName)]
        pub unsafe fn manufacturerName(&self) -> Retained<NSString>;

        #[method(version)]
        pub unsafe fn version(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other versionString)]
        pub unsafe fn versionString(&self) -> Retained<NSString>;

        #[deprecated]
        #[method_id(@__retain_semantics Other componentURL)]
        pub unsafe fn componentURL(&self) -> Option<Retained<NSURL>>;

        #[method_id(@__retain_semantics Other availableArchitectures)]
        pub unsafe fn availableArchitectures(&self) -> Retained<NSArray<NSNumber>>;

        #[method(isSandboxSafe)]
        pub unsafe fn isSandboxSafe(&self) -> bool;

        #[method(hasMIDIInput)]
        pub unsafe fn hasMIDIInput(&self) -> bool;

        #[method(hasMIDIOutput)]
        pub unsafe fn hasMIDIOutput(&self) -> bool;

        #[cfg(feature = "objc2-audio-toolbox")]
        #[cfg(not(target_os = "watchos"))]
        #[method(audioComponent)]
        pub unsafe fn audioComponent(&self) -> AudioComponent;

        #[method_id(@__retain_semantics Other userTagNames)]
        pub unsafe fn userTagNames(&self) -> Retained<NSArray<NSString>>;

        #[method(setUserTagNames:)]
        pub unsafe fn setUserTagNames(&self, user_tag_names: &NSArray<NSString>);

        #[method_id(@__retain_semantics Other allTagNames)]
        pub unsafe fn allTagNames(&self) -> Retained<NSArray<NSString>>;

        #[cfg(feature = "objc2-audio-toolbox")]
        #[cfg(not(target_os = "watchos"))]
        #[method(audioComponentDescription)]
        pub unsafe fn audioComponentDescription(&self) -> AudioComponentDescription;

        #[method_id(@__retain_semantics Other iconURL)]
        pub unsafe fn iconURL(&self) -> Option<Retained<NSURL>>;

        #[method(passesAUVal)]
        pub unsafe fn passesAUVal(&self) -> bool;

        #[method(hasCustomView)]
        pub unsafe fn hasCustomView(&self) -> bool;

        #[method_id(@__retain_semantics Other configurationDictionary)]
        pub unsafe fn configurationDictionary(&self)
            -> Retained<NSDictionary<NSString, AnyObject>>;

        #[method(supportsNumberInputChannels:outputChannels:)]
        pub unsafe fn supportsNumberInputChannels_outputChannels(
            &self,
            num_input_channels: NSInteger,
            num_output_channels: NSInteger,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioUnitComponent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiounitcomponenttagsdidchangenotification?language=objc)
    pub static AVAudioUnitComponentTagsDidChangeNotification: &'static NSString;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiounitcomponentmanager?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVAudioUnitComponentManager;
);

unsafe impl NSObjectProtocol for AVAudioUnitComponentManager {}

extern_methods!(
    unsafe impl AVAudioUnitComponentManager {
        #[method_id(@__retain_semantics Other tagNames)]
        pub unsafe fn tagNames(&self) -> Retained<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other standardLocalizedTagNames)]
        pub unsafe fn standardLocalizedTagNames(&self) -> Retained<NSArray<NSString>>;

        #[method_id(@__retain_semantics Other sharedAudioUnitComponentManager)]
        pub unsafe fn sharedAudioUnitComponentManager() -> Retained<Self>;

        #[method_id(@__retain_semantics Other componentsMatchingPredicate:)]
        pub unsafe fn componentsMatchingPredicate(
            &self,
            predicate: &NSPredicate,
        ) -> Retained<NSArray<AVAudioUnitComponent>>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other componentsPassingTest:)]
        pub unsafe fn componentsPassingTest(
            &self,
            test_handler: &block2::Block<
                dyn Fn(NonNull<AVAudioUnitComponent>, NonNull<Bool>) -> Bool,
            >,
        ) -> Retained<NSArray<AVAudioUnitComponent>>;

        #[cfg(feature = "objc2-audio-toolbox")]
        #[cfg(not(target_os = "watchos"))]
        #[method_id(@__retain_semantics Other componentsMatchingDescription:)]
        pub unsafe fn componentsMatchingDescription(
            &self,
            desc: AudioComponentDescription,
        ) -> Retained<NSArray<AVAudioUnitComponent>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl AVAudioUnitComponentManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/avfaudio/avaudiounitcomponentmanagerregistrationschangednotification?language=objc)
    pub static AVAudioUnitComponentManagerRegistrationsChangedNotification:
        &'static NSNotificationName;
}
