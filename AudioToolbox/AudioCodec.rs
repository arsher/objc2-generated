//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-audio-types")]
use objc2_core_audio_types::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocodec?language=objc)
#[cfg(feature = "AudioComponent")]
pub type AudioCodec = AudioComponentInstance;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocodecpropertyid?language=objc)
pub type AudioCodecPropertyID = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocodecmagiccookieinfo?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioCodecMagicCookieInfo {
    pub mMagicCookieSize: u32,
    pub mMagicCookie: *mut c_void,
}

unsafe impl Encode for AudioCodecMagicCookieInfo {
    const ENCODING: Encoding = Encoding::Struct(
        "AudioCodecMagicCookieInfo",
        &[<u32>::ENCODING, <*mut c_void>::ENCODING],
    );
}

unsafe impl RefEncode for AudioCodecMagicCookieInfo {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiodecodercomponenttype?language=objc)
pub const kAudioDecoderComponentType: u32 = 0x61646563;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudioencodercomponenttype?language=objc)
pub const kAudioEncoderComponentType: u32 = 0x61656e63;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiounitycodeccomponenttype?language=objc)
pub const kAudioUnityCodecComponentType: u32 = 0x61636463;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertysupportedinputformats?language=objc)
pub const kAudioCodecPropertySupportedInputFormats: AudioCodecPropertyID = 0x69666d23;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertysupportedoutputformats?language=objc)
pub const kAudioCodecPropertySupportedOutputFormats: AudioCodecPropertyID = 0x6f666d23;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyavailableinputsamplerates?language=objc)
pub const kAudioCodecPropertyAvailableInputSampleRates: AudioCodecPropertyID = 0x61697372;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyavailableoutputsamplerates?language=objc)
pub const kAudioCodecPropertyAvailableOutputSampleRates: AudioCodecPropertyID = 0x616f7372;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyavailablebitraterange?language=objc)
pub const kAudioCodecPropertyAvailableBitRateRange: AudioCodecPropertyID = 0x61627274;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyminimumnumberinputpackets?language=objc)
pub const kAudioCodecPropertyMinimumNumberInputPackets: AudioCodecPropertyID = 0x6d6e6970;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyminimumnumberoutputpackets?language=objc)
pub const kAudioCodecPropertyMinimumNumberOutputPackets: AudioCodecPropertyID = 0x6d6e6f70;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyavailablenumberchannels?language=objc)
pub const kAudioCodecPropertyAvailableNumberChannels: AudioCodecPropertyID = 0x636d6e63;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertydoessamplerateconversion?language=objc)
pub const kAudioCodecPropertyDoesSampleRateConversion: AudioCodecPropertyID = 0x6c6d7263;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyavailableinputchannellayouttags?language=objc)
pub const kAudioCodecPropertyAvailableInputChannelLayoutTags: AudioCodecPropertyID = 0x6169636c;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyavailableoutputchannellayouttags?language=objc)
pub const kAudioCodecPropertyAvailableOutputChannelLayoutTags: AudioCodecPropertyID = 0x616f636c;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyinputformatsforoutputformat?language=objc)
pub const kAudioCodecPropertyInputFormatsForOutputFormat: AudioCodecPropertyID = 0x6966346f;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyoutputformatsforinputformat?language=objc)
pub const kAudioCodecPropertyOutputFormatsForInputFormat: AudioCodecPropertyID = 0x6f663469;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyformatinfo?language=objc)
pub const kAudioCodecPropertyFormatInfo: AudioCodecPropertyID = 0x61636669;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyinputbuffersize?language=objc)
pub const kAudioCodecPropertyInputBufferSize: AudioCodecPropertyID = 0x74627566;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertypacketframesize?language=objc)
pub const kAudioCodecPropertyPacketFrameSize: AudioCodecPropertyID = 0x70616b66;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyhasvariablepacketbytesizes?language=objc)
pub const kAudioCodecPropertyHasVariablePacketByteSizes: AudioCodecPropertyID = 0x76706b3f;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyemploysdependentpackets?language=objc)
pub const kAudioCodecPropertyEmploysDependentPackets: AudioCodecPropertyID = 0x64706b3f;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertymaximumpacketbytesize?language=objc)
pub const kAudioCodecPropertyMaximumPacketByteSize: AudioCodecPropertyID = 0x70616b62;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertypacketsizelimitforvbr?language=objc)
pub const kAudioCodecPropertyPacketSizeLimitForVBR: AudioCodecPropertyID = 0x70616b6c;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertycurrentinputformat?language=objc)
pub const kAudioCodecPropertyCurrentInputFormat: AudioCodecPropertyID = 0x69666d74;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertycurrentoutputformat?language=objc)
pub const kAudioCodecPropertyCurrentOutputFormat: AudioCodecPropertyID = 0x6f666d74;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertymagiccookie?language=objc)
pub const kAudioCodecPropertyMagicCookie: AudioCodecPropertyID = 0x6b756b69;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyusedinputbuffersize?language=objc)
pub const kAudioCodecPropertyUsedInputBufferSize: AudioCodecPropertyID = 0x75627566;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyisinitialized?language=objc)
pub const kAudioCodecPropertyIsInitialized: AudioCodecPropertyID = 0x696e6974;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertycurrenttargetbitrate?language=objc)
pub const kAudioCodecPropertyCurrentTargetBitRate: AudioCodecPropertyID = 0x62726174;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertycurrentinputsamplerate?language=objc)
pub const kAudioCodecPropertyCurrentInputSampleRate: AudioCodecPropertyID = 0x63697372;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertycurrentoutputsamplerate?language=objc)
pub const kAudioCodecPropertyCurrentOutputSampleRate: AudioCodecPropertyID = 0x636f7372;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyqualitysetting?language=objc)
pub const kAudioCodecPropertyQualitySetting: AudioCodecPropertyID = 0x73726371;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyapplicablebitraterange?language=objc)
pub const kAudioCodecPropertyApplicableBitRateRange: AudioCodecPropertyID = 0x62727461;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyrecommendedbitraterange?language=objc)
pub const kAudioCodecPropertyRecommendedBitRateRange: AudioCodecPropertyID = 0x62727472;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyapplicableinputsamplerates?language=objc)
pub const kAudioCodecPropertyApplicableInputSampleRates: AudioCodecPropertyID = 0x69737261;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyapplicableoutputsamplerates?language=objc)
pub const kAudioCodecPropertyApplicableOutputSampleRates: AudioCodecPropertyID = 0x6f737261;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertypaddedzeros?language=objc)
pub const kAudioCodecPropertyPaddedZeros: AudioCodecPropertyID = 0x70616430;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyprimemethod?language=objc)
pub const kAudioCodecPropertyPrimeMethod: AudioCodecPropertyID = 0x70726d6d;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyprimeinfo?language=objc)
pub const kAudioCodecPropertyPrimeInfo: AudioCodecPropertyID = 0x7072696d;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertycurrentinputchannellayout?language=objc)
pub const kAudioCodecPropertyCurrentInputChannelLayout: AudioCodecPropertyID = 0x69636c20;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertycurrentoutputchannellayout?language=objc)
pub const kAudioCodecPropertyCurrentOutputChannelLayout: AudioCodecPropertyID = 0x6f636c20;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertysettings?language=objc)
pub const kAudioCodecPropertySettings: AudioCodecPropertyID = 0x61637320;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyformatlist?language=objc)
pub const kAudioCodecPropertyFormatList: AudioCodecPropertyID = 0x6163666c;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertybitratecontrolmode?language=objc)
pub const kAudioCodecPropertyBitRateControlMode: AudioCodecPropertyID = 0x61636266;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertysoundqualityforvbr?language=objc)
pub const kAudioCodecPropertySoundQualityForVBR: AudioCodecPropertyID = 0x76627271;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertybitrateforvbr?language=objc)
pub const kAudioCodecPropertyBitRateForVBR: AudioCodecPropertyID = 0x76627262;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertydelaymode?language=objc)
pub const kAudioCodecPropertyDelayMode: AudioCodecPropertyID = 0x646d6f64;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyadjustlocalquality?language=objc)
pub const kAudioCodecPropertyAdjustLocalQuality: AudioCodecPropertyID = 0x5e71616c;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertydynamicrangecontrolmode?language=objc)
pub const kAudioCodecPropertyDynamicRangeControlMode: AudioCodecPropertyID = 0x6d647263;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyadjustcompressionprofile?language=objc)
pub const kAudioCodecPropertyAdjustCompressionProfile: AudioCodecPropertyID = 0x5e70726f;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyprogramtargetlevelconstant?language=objc)
pub const kAudioCodecPropertyProgramTargetLevelConstant: AudioCodecPropertyID = 0x70746c63;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyadjusttargetlevelconstant?language=objc)
pub const kAudioCodecPropertyAdjustTargetLevelConstant: AudioCodecPropertyID = 0x5e746c63;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyprogramtargetlevel?language=objc)
pub const kAudioCodecPropertyProgramTargetLevel: AudioCodecPropertyID = 0x7070746c;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyadjusttargetlevel?language=objc)
pub const kAudioCodecPropertyAdjustTargetLevel: AudioCodecPropertyID = 0x5e70746c;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecquality_max?language=objc)
pub const kAudioCodecQuality_Max: u32 = 0x7F;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecquality_high?language=objc)
pub const kAudioCodecQuality_High: u32 = 0x60;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecquality_medium?language=objc)
pub const kAudioCodecQuality_Medium: u32 = 0x40;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecquality_low?language=objc)
pub const kAudioCodecQuality_Low: u32 = 0x20;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecquality_min?language=objc)
pub const kAudioCodecQuality_Min: u32 = 0;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecprimemethod_pre?language=objc)
pub const kAudioCodecPrimeMethod_Pre: u32 = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecprimemethod_normal?language=objc)
pub const kAudioCodecPrimeMethod_Normal: u32 = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecprimemethod_none?language=objc)
pub const kAudioCodecPrimeMethod_None: u32 = 2;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecbitratecontrolmode_constant?language=objc)
pub const kAudioCodecBitRateControlMode_Constant: u32 = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecbitratecontrolmode_longtermaverage?language=objc)
pub const kAudioCodecBitRateControlMode_LongTermAverage: u32 = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecbitratecontrolmode_variableconstrained?language=objc)
pub const kAudioCodecBitRateControlMode_VariableConstrained: u32 = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecbitratecontrolmode_variable?language=objc)
pub const kAudioCodecBitRateControlMode_Variable: u32 = 3;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecdelaymode_compatibility?language=objc)
pub const kAudioCodecDelayMode_Compatibility: u32 = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecdelaymode_minimum?language=objc)
pub const kAudioCodecDelayMode_Minimum: u32 = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecdelaymode_optimal?language=objc)
pub const kAudioCodecDelayMode_Optimal: u32 = 2;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kprogramtargetlevel_none?language=objc)
pub const kProgramTargetLevel_None: u32 = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kprogramtargetlevel_minus31db?language=objc)
pub const kProgramTargetLevel_Minus31dB: u32 = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kprogramtargetlevel_minus23db?language=objc)
pub const kProgramTargetLevel_Minus23dB: u32 = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kprogramtargetlevel_minus20db?language=objc)
pub const kProgramTargetLevel_Minus20dB: u32 = 3;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kdynamicrangecontrolmode_none?language=objc)
pub const kDynamicRangeControlMode_None: u32 = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kdynamicrangecontrolmode_light?language=objc)
pub const kDynamicRangeControlMode_Light: u32 = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kdynamicrangecontrolmode_heavy?language=objc)
pub const kDynamicRangeControlMode_Heavy: u32 = 2;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kdynamicrangecompressionprofile_none?language=objc)
pub const kDynamicRangeCompressionProfile_None: u32 = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kdynamicrangecompressionprofile_latenight?language=objc)
pub const kDynamicRangeCompressionProfile_LateNight: u32 = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kdynamicrangecompressionprofile_noisyenvironment?language=objc)
pub const kDynamicRangeCompressionProfile_NoisyEnvironment: u32 = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kdynamicrangecompressionprofile_limitedplaybackrange?language=objc)
pub const kDynamicRangeCompressionProfile_LimitedPlaybackRange: u32 = 3;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kdynamicrangecompressionprofile_generalcompression?language=objc)
pub const kDynamicRangeCompressionProfile_GeneralCompression: u32 = 6;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocodecprimeinfo?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioCodecPrimeInfo {
    pub leadingFrames: u32,
    pub trailingFrames: u32,
}

unsafe impl Encode for AudioCodecPrimeInfo {
    const ENCODING: Encoding =
        Encoding::Struct("AudioCodecPrimeInfo", &[<u32>::ENCODING, <u32>::ENCODING]);
}

unsafe impl RefEncode for AudioCodecPrimeInfo {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiosettingsflags?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AudioSettingsFlags(pub u32);
bitflags::bitflags! {
    impl AudioSettingsFlags: u32 {
        const kAudioSettingsFlags_ExpertParameter = 1<<0;
        const kAudioSettingsFlags_InvisibleParameter = 1<<1;
        const kAudioSettingsFlags_MetaParameter = 1<<2;
        const kAudioSettingsFlags_UserInterfaceParameter = 1<<3;
    }
}

unsafe impl Encode for AudioSettingsFlags {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for AudioSettingsFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecproduceoutputpacketfailure?language=objc)
pub const kAudioCodecProduceOutputPacketFailure: u32 = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecproduceoutputpacketsuccess?language=objc)
pub const kAudioCodecProduceOutputPacketSuccess: u32 = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecproduceoutputpacketsuccesshasmore?language=objc)
pub const kAudioCodecProduceOutputPacketSuccessHasMore: u32 = 3;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecproduceoutputpacketneedsmoreinputdata?language=objc)
pub const kAudioCodecProduceOutputPacketNeedsMoreInputData: u32 = 4;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecproduceoutputpacketateof?language=objc)
pub const kAudioCodecProduceOutputPacketAtEOF: u32 = 5;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecproduceoutputpacketsuccessconcealed?language=objc)
pub const kAudioCodecProduceOutputPacketSuccessConcealed: u32 = 6;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecgetpropertyinfoselect?language=objc)
pub const kAudioCodecGetPropertyInfoSelect: u32 = 0x0001;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecgetpropertyselect?language=objc)
pub const kAudioCodecGetPropertySelect: u32 = 0x0002;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecsetpropertyselect?language=objc)
pub const kAudioCodecSetPropertySelect: u32 = 0x0003;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecinitializeselect?language=objc)
pub const kAudioCodecInitializeSelect: u32 = 0x0004;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecuninitializeselect?language=objc)
pub const kAudioCodecUninitializeSelect: u32 = 0x0005;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecappendinputdataselect?language=objc)
pub const kAudioCodecAppendInputDataSelect: u32 = 0x0006;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecproduceoutputdataselect?language=objc)
pub const kAudioCodecProduceOutputDataSelect: u32 = 0x0007;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecresetselect?language=objc)
pub const kAudioCodecResetSelect: u32 = 0x0008;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecappendinputbufferlistselect?language=objc)
pub const kAudioCodecAppendInputBufferListSelect: u32 = 0x0009;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecproduceoutputbufferlistselect?language=objc)
pub const kAudioCodecProduceOutputBufferListSelect: u32 = 0x000A;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecnoerror?language=objc)
pub const kAudioCodecNoError: OSStatus = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecunspecifiederror?language=objc)
pub const kAudioCodecUnspecifiedError: OSStatus = 0x77686174;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecunknownpropertyerror?language=objc)
pub const kAudioCodecUnknownPropertyError: OSStatus = 0x77686f3f;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecbadpropertysizeerror?language=objc)
pub const kAudioCodecBadPropertySizeError: OSStatus = 0x2173697a;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecillegaloperationerror?language=objc)
pub const kAudioCodecIllegalOperationError: OSStatus = 0x6e6f7065;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecunsupportedformaterror?language=objc)
pub const kAudioCodecUnsupportedFormatError: OSStatus = 0x21646174;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecstateerror?language=objc)
pub const kAudioCodecStateError: OSStatus = 0x21737474;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecnotenoughbufferspaceerror?language=objc)
pub const kAudioCodecNotEnoughBufferSpaceError: OSStatus = 0x21627566;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecbaddataerror?language=objc)
pub const kAudioCodecBadDataError: OSStatus = 0x62616461;

extern "C-unwind" {
    #[cfg(feature = "AudioComponent")]
    pub fn AudioCodecGetPropertyInfo(
        in_codec: AudioCodec,
        in_property_id: AudioCodecPropertyID,
        out_size: *mut u32,
        out_writable: *mut Boolean,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "AudioComponent")]
    pub fn AudioCodecGetProperty(
        in_codec: AudioCodec,
        in_property_id: AudioCodecPropertyID,
        io_property_data_size: NonNull<u32>,
        out_property_data: NonNull<c_void>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "AudioComponent")]
    pub fn AudioCodecSetProperty(
        in_codec: AudioCodec,
        in_property_id: AudioCodecPropertyID,
        in_property_data_size: u32,
        in_property_data: NonNull<c_void>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(all(feature = "AudioComponent", feature = "objc2-core-audio-types"))]
    pub fn AudioCodecInitialize(
        in_codec: AudioCodec,
        in_input_format: *mut AudioStreamBasicDescription,
        in_output_format: *mut AudioStreamBasicDescription,
        in_magic_cookie: *mut c_void,
        in_magic_cookie_byte_size: u32,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "AudioComponent")]
    pub fn AudioCodecUninitialize(in_codec: AudioCodec) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(all(feature = "AudioComponent", feature = "objc2-core-audio-types"))]
    pub fn AudioCodecAppendInputData(
        in_codec: AudioCodec,
        in_input_data: NonNull<c_void>,
        io_input_data_byte_size: NonNull<u32>,
        io_number_packets: NonNull<u32>,
        in_packet_description: *mut AudioStreamPacketDescription,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(all(feature = "AudioComponent", feature = "objc2-core-audio-types"))]
    pub fn AudioCodecProduceOutputPackets(
        in_codec: AudioCodec,
        out_output_data: NonNull<c_void>,
        io_output_data_byte_size: NonNull<u32>,
        io_number_packets: NonNull<u32>,
        out_packet_description: *mut AudioStreamPacketDescription,
        out_status: NonNull<u32>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(all(feature = "AudioComponent", feature = "objc2-core-audio-types"))]
    pub fn AudioCodecAppendInputBufferList(
        in_codec: AudioCodec,
        in_buffer_list: NonNull<AudioBufferList>,
        io_number_packets: NonNull<u32>,
        in_packet_description: *mut AudioStreamPacketDescription,
        out_bytes_consumed: NonNull<u32>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(all(feature = "AudioComponent", feature = "objc2-core-audio-types"))]
    pub fn AudioCodecProduceOutputBufferList(
        in_codec: AudioCodec,
        io_buffer_list: NonNull<AudioBufferList>,
        io_number_packets: NonNull<u32>,
        out_packet_description: *mut AudioStreamPacketDescription,
        out_status: NonNull<u32>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[cfg(feature = "AudioComponent")]
    pub fn AudioCodecReset(in_codec: AudioCodec) -> OSStatus;
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocodecgetpropertyinfoproc?language=objc)
pub type AudioCodecGetPropertyInfoProc = Option<
    unsafe extern "C-unwind" fn(
        NonNull<c_void>,
        AudioCodecPropertyID,
        *mut u32,
        *mut Boolean,
    ) -> OSStatus,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocodecgetpropertyproc?language=objc)
pub type AudioCodecGetPropertyProc = Option<
    unsafe extern "C-unwind" fn(
        NonNull<c_void>,
        AudioCodecPropertyID,
        NonNull<u32>,
        NonNull<c_void>,
    ) -> OSStatus,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocodecsetpropertyproc?language=objc)
pub type AudioCodecSetPropertyProc = Option<
    unsafe extern "C-unwind" fn(
        NonNull<c_void>,
        AudioCodecPropertyID,
        u32,
        NonNull<c_void>,
    ) -> OSStatus,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocodecinitializeproc?language=objc)
#[cfg(feature = "objc2-core-audio-types")]
pub type AudioCodecInitializeProc = Option<
    unsafe extern "C-unwind" fn(
        NonNull<c_void>,
        *mut AudioStreamBasicDescription,
        *mut AudioStreamBasicDescription,
        *mut c_void,
        u32,
    ) -> OSStatus,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocodecuninitializeproc?language=objc)
pub type AudioCodecUninitializeProc =
    Option<unsafe extern "C-unwind" fn(NonNull<c_void>) -> OSStatus>;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocodecappendinputdataproc?language=objc)
#[cfg(feature = "objc2-core-audio-types")]
pub type AudioCodecAppendInputDataProc = Option<
    unsafe extern "C-unwind" fn(
        NonNull<c_void>,
        NonNull<c_void>,
        NonNull<u32>,
        NonNull<u32>,
        *mut AudioStreamPacketDescription,
    ) -> OSStatus,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocodecproduceoutputpacketsproc?language=objc)
#[cfg(feature = "objc2-core-audio-types")]
pub type AudioCodecProduceOutputPacketsProc = Option<
    unsafe extern "C-unwind" fn(
        NonNull<c_void>,
        NonNull<c_void>,
        NonNull<u32>,
        NonNull<u32>,
        *mut AudioStreamPacketDescription,
        NonNull<u32>,
    ) -> OSStatus,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocodecresetproc?language=objc)
pub type AudioCodecResetProc = Option<unsafe extern "C-unwind" fn(NonNull<c_void>) -> OSStatus>;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocodecappendinputbufferlistproc?language=objc)
#[cfg(feature = "objc2-core-audio-types")]
pub type AudioCodecAppendInputBufferListProc = Option<
    unsafe extern "C-unwind" fn(
        NonNull<c_void>,
        NonNull<AudioBufferList>,
        NonNull<u32>,
        *mut AudioStreamPacketDescription,
        NonNull<u32>,
    ) -> OSStatus,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiocodecproduceoutputbufferlistproc?language=objc)
#[cfg(feature = "objc2-core-audio-types")]
pub type AudioCodecProduceOutputBufferListProc = Option<
    unsafe extern "C-unwind" fn(
        NonNull<c_void>,
        NonNull<AudioBufferList>,
        NonNull<u32>,
        *mut AudioStreamPacketDescription,
        NonNull<u32>,
    ) -> OSStatus,
>;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyminimumdelaymode?language=objc)
pub const kAudioCodecPropertyMinimumDelayMode: AudioCodecPropertyID = 0x6d64656c;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertynamecfstring?language=objc)
pub const kAudioCodecPropertyNameCFString: AudioCodecPropertyID = 0x6c6e616d;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertymanufacturercfstring?language=objc)
pub const kAudioCodecPropertyManufacturerCFString: AudioCodecPropertyID = 0x6c6d616b;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyformatcfstring?language=objc)
pub const kAudioCodecPropertyFormatCFString: AudioCodecPropertyID = 0x6c666f72;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyrequirespacketdescription?language=objc)
pub const kAudioCodecPropertyRequiresPacketDescription: AudioCodecPropertyID = 0x70616b64;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyavailablebitrates?language=objc)
pub const kAudioCodecPropertyAvailableBitRates: AudioCodecPropertyID = 0x62727423;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecextendfrequencies?language=objc)
pub const kAudioCodecExtendFrequencies: AudioCodecPropertyID = 0x61636566;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecuserecommendedsamplerate?language=objc)
pub const kAudioCodecUseRecommendedSampleRate: AudioCodecPropertyID = 0x75727372;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecoutputprecedence?language=objc)
pub const kAudioCodecOutputPrecedence: AudioCodecPropertyID = 0x6f707072;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecbitrateformat?language=objc)
pub const kAudioCodecBitRateFormat: AudioCodecPropertyID = kAudioCodecPropertyBitRateControlMode;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecdoessamplerateconversion?language=objc)
pub const kAudioCodecDoesSampleRateConversion: AudioCodecPropertyID =
    kAudioCodecPropertyDoesSampleRateConversion;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecinputformatsforoutputformat?language=objc)
pub const kAudioCodecInputFormatsForOutputFormat: AudioCodecPropertyID =
    kAudioCodecPropertyInputFormatsForOutputFormat;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecoutputformatsforinputformat?language=objc)
pub const kAudioCodecOutputFormatsForInputFormat: AudioCodecPropertyID =
    kAudioCodecPropertyOutputFormatsForInputFormat;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyinputchannellayout?language=objc)
pub const kAudioCodecPropertyInputChannelLayout: AudioCodecPropertyID =
    kAudioCodecPropertyCurrentInputChannelLayout;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyoutputchannellayout?language=objc)
pub const kAudioCodecPropertyOutputChannelLayout: AudioCodecPropertyID =
    kAudioCodecPropertyCurrentOutputChannelLayout;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyavailableinputchannellayouts?language=objc)
pub const kAudioCodecPropertyAvailableInputChannelLayouts: AudioCodecPropertyID =
    kAudioCodecPropertyAvailableInputChannelLayoutTags;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyavailableoutputchannellayouts?language=objc)
pub const kAudioCodecPropertyAvailableOutputChannelLayouts: AudioCodecPropertyID =
    kAudioCodecPropertyAvailableOutputChannelLayoutTags;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecpropertyzeroframespadded?language=objc)
pub const kAudioCodecPropertyZeroFramesPadded: AudioCodecPropertyID =
    kAudioCodecPropertyPaddedZeros;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecbitrateformat_cbr?language=objc)
pub const kAudioCodecBitRateFormat_CBR: u32 = kAudioCodecBitRateControlMode_Constant;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecbitrateformat_abr?language=objc)
pub const kAudioCodecBitRateFormat_ABR: u32 = kAudioCodecBitRateControlMode_LongTermAverage;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecbitrateformat_vbr?language=objc)
pub const kAudioCodecBitRateFormat_VBR: u32 = kAudioCodecBitRateControlMode_VariableConstrained;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecoutputprecedencenone?language=objc)
pub const kAudioCodecOutputPrecedenceNone: u32 = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecoutputprecedencebitrate?language=objc)
pub const kAudioCodecOutputPrecedenceBitRate: u32 = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaudiocodecoutputprecedencesamplerate?language=objc)
pub const kAudioCodecOutputPrecedenceSampleRate: u32 = 2;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/magiccookieinfo?language=objc)
pub type MagicCookieInfo = AudioCodecMagicCookieInfo;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/khintbasic?language=objc)
pub const kHintBasic: u32 = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/khintadvanced?language=objc)
pub const kHintAdvanced: u32 = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/khinthidden?language=objc)
pub const kHintHidden: u32 = 2;
