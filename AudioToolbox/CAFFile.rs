//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_filetype?language=objc)
pub const kCAF_FileType: u32 = 0x63616666;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_fileversion_initial?language=objc)
pub const kCAF_FileVersion_Initial: u32 = 1;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_streamdescriptionchunkid?language=objc)
pub const kCAF_StreamDescriptionChunkID: u32 = 0x64657363;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_audiodatachunkid?language=objc)
pub const kCAF_AudioDataChunkID: u32 = 0x64617461;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_channellayoutchunkid?language=objc)
pub const kCAF_ChannelLayoutChunkID: u32 = 0x6368616e;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_fillerchunkid?language=objc)
pub const kCAF_FillerChunkID: u32 = 0x66726565;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_markerchunkid?language=objc)
pub const kCAF_MarkerChunkID: u32 = 0x6d61726b;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_regionchunkid?language=objc)
pub const kCAF_RegionChunkID: u32 = 0x7265676e;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_instrumentchunkid?language=objc)
pub const kCAF_InstrumentChunkID: u32 = 0x696e7374;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_magiccookieid?language=objc)
pub const kCAF_MagicCookieID: u32 = 0x6b756b69;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_infostringschunkid?language=objc)
pub const kCAF_InfoStringsChunkID: u32 = 0x696e666f;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_editcommentschunkid?language=objc)
pub const kCAF_EditCommentsChunkID: u32 = 0x65646374;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_packettablechunkid?language=objc)
pub const kCAF_PacketTableChunkID: u32 = 0x70616b74;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_stringschunkid?language=objc)
pub const kCAF_StringsChunkID: u32 = 0x73747267;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_uuidchunkid?language=objc)
pub const kCAF_UUIDChunkID: u32 = 0x75756964;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_peakchunkid?language=objc)
pub const kCAF_PeakChunkID: u32 = 0x7065616b;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_overviewchunkid?language=objc)
pub const kCAF_OverviewChunkID: u32 = 0x6f767677;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_midichunkid?language=objc)
pub const kCAF_MIDIChunkID: u32 = 0x6d696469;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_umidchunkid?language=objc)
pub const kCAF_UMIDChunkID: u32 = 0x756d6964;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_formatlistid?language=objc)
pub const kCAF_FormatListID: u32 = 0x6c647363;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_ixmlchunkid?language=objc)
pub const kCAF_iXMLChunkID: u32 = 0x69584d4c;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/caffileheader?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFFileHeader {
    pub mFileType: u32,
    pub mFileVersion: u16,
    pub mFileFlags: u16,
}

unsafe impl Encode for CAFFileHeader {
    const ENCODING: Encoding = Encoding::Struct(
        "CAFFileHeader",
        &[<u32>::ENCODING, <u16>::ENCODING, <u16>::ENCODING],
    );
}

unsafe impl RefEncode for CAFFileHeader {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cafchunkheader?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFChunkHeader {
    pub mChunkType: u32,
    pub mChunkSize: i64,
}

unsafe impl Encode for CAFChunkHeader {
    const ENCODING: Encoding =
        Encoding::Struct("CAFChunkHeader", &[<u32>::ENCODING, <i64>::ENCODING]);
}

unsafe impl RefEncode for CAFChunkHeader {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/caf_uuid_chunkheader?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAF_UUID_ChunkHeader {
    pub mHeader: CAFChunkHeader,
    pub mUUID: [u8; 16],
}

unsafe impl Encode for CAF_UUID_ChunkHeader {
    const ENCODING: Encoding = Encoding::Struct(
        "CAF_UUID_ChunkHeader",
        &[<CAFChunkHeader>::ENCODING, <[u8; 16]>::ENCODING],
    );
}

unsafe impl RefEncode for CAF_UUID_ChunkHeader {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cafformatflags?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CAFFormatFlags(pub u32);
bitflags::bitflags! {
    impl CAFFormatFlags: u32 {
        const kCAFLinearPCMFormatFlagIsFloat = 1<<0;
        const kCAFLinearPCMFormatFlagIsLittleEndian = 1<<1;
    }
}

unsafe impl Encode for CAFFormatFlags {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for CAFFormatFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cafaudiodescription?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFAudioDescription {
    pub mSampleRate: f64,
    pub mFormatID: u32,
    pub mFormatFlags: CAFFormatFlags,
    pub mBytesPerPacket: u32,
    pub mFramesPerPacket: u32,
    pub mChannelsPerFrame: u32,
    pub mBitsPerChannel: u32,
}

unsafe impl Encode for CAFAudioDescription {
    const ENCODING: Encoding = Encoding::Struct(
        "CAFAudioDescription",
        &[
            <f64>::ENCODING,
            <u32>::ENCODING,
            <CAFFormatFlags>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for CAFAudioDescription {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cafaudioformatlistitem?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFAudioFormatListItem {
    pub mFormat: CAFAudioDescription,
    pub mChannelLayoutTag: u32,
}

unsafe impl Encode for CAFAudioFormatListItem {
    const ENCODING: Encoding = Encoding::Struct(
        "CAFAudioFormatListItem",
        &[<CAFAudioDescription>::ENCODING, <u32>::ENCODING],
    );
}

unsafe impl RefEncode for CAFAudioFormatListItem {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cafpackettableheader?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFPacketTableHeader {
    pub mNumberPackets: i64,
    pub mNumberValidFrames: i64,
    pub mPrimingFrames: i32,
    pub mRemainderFrames: i32,
    pub mPacketDescriptions: [u8; 1],
}

unsafe impl Encode for CAFPacketTableHeader {
    const ENCODING: Encoding = Encoding::Struct(
        "CAFPacketTableHeader",
        &[
            <i64>::ENCODING,
            <i64>::ENCODING,
            <i32>::ENCODING,
            <i32>::ENCODING,
            <[u8; 1]>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for CAFPacketTableHeader {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cafdatachunk?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFDataChunk {
    pub mEditCount: u32,
    pub mData: [u8; 1],
}

unsafe impl Encode for CAFDataChunk {
    const ENCODING: Encoding =
        Encoding::Struct("CAFDataChunk", &[<u32>::ENCODING, <[u8; 1]>::ENCODING]);
}

unsafe impl RefEncode for CAFDataChunk {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_generic?language=objc)
pub const kCAFMarkerType_Generic: u32 = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_programstart?language=objc)
pub const kCAFMarkerType_ProgramStart: u32 = 0x70626567;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_programend?language=objc)
pub const kCAFMarkerType_ProgramEnd: u32 = 0x70656e64;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_trackstart?language=objc)
pub const kCAFMarkerType_TrackStart: u32 = 0x74626567;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_trackend?language=objc)
pub const kCAFMarkerType_TrackEnd: u32 = 0x74656e64;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_index?language=objc)
pub const kCAFMarkerType_Index: u32 = 0x696e6478;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_regionstart?language=objc)
pub const kCAFMarkerType_RegionStart: u32 = 0x72626567;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_regionend?language=objc)
pub const kCAFMarkerType_RegionEnd: u32 = 0x72656e64;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_regionsyncpoint?language=objc)
pub const kCAFMarkerType_RegionSyncPoint: u32 = 0x72737963;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_selectionstart?language=objc)
pub const kCAFMarkerType_SelectionStart: u32 = 0x73626567;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_selectionend?language=objc)
pub const kCAFMarkerType_SelectionEnd: u32 = 0x73656e64;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_editsourcebegin?language=objc)
pub const kCAFMarkerType_EditSourceBegin: u32 = 0x63626567;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_editsourceend?language=objc)
pub const kCAFMarkerType_EditSourceEnd: u32 = 0x63656e64;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_editdestinationbegin?language=objc)
pub const kCAFMarkerType_EditDestinationBegin: u32 = 0x64626567;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_editdestinationend?language=objc)
pub const kCAFMarkerType_EditDestinationEnd: u32 = 0x64656e64;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_sustainloopstart?language=objc)
pub const kCAFMarkerType_SustainLoopStart: u32 = 0x736c6267;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_sustainloopend?language=objc)
pub const kCAFMarkerType_SustainLoopEnd: u32 = 0x736c656e;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_releaseloopstart?language=objc)
pub const kCAFMarkerType_ReleaseLoopStart: u32 = 0x726c6267;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_releaseloopend?language=objc)
pub const kCAFMarkerType_ReleaseLoopEnd: u32 = 0x726c656e;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_savedplayposition?language=objc)
pub const kCAFMarkerType_SavedPlayPosition: u32 = 0x73706c79;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_tempo?language=objc)
pub const kCAFMarkerType_Tempo: u32 = 0x746d706f;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_timesignature?language=objc)
pub const kCAFMarkerType_TimeSignature: u32 = 0x74736967;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcafmarkertype_keysignature?language=objc)
pub const kCAFMarkerType_KeySignature: u32 = 0x6b736967;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_smpte_timetypenone?language=objc)
pub const kCAF_SMPTE_TimeTypeNone: u32 = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_smpte_timetype24?language=objc)
pub const kCAF_SMPTE_TimeType24: u32 = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_smpte_timetype25?language=objc)
pub const kCAF_SMPTE_TimeType25: u32 = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_smpte_timetype30drop?language=objc)
pub const kCAF_SMPTE_TimeType30Drop: u32 = 3;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_smpte_timetype30?language=objc)
pub const kCAF_SMPTE_TimeType30: u32 = 4;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_smpte_timetype2997?language=objc)
pub const kCAF_SMPTE_TimeType2997: u32 = 5;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_smpte_timetype2997drop?language=objc)
pub const kCAF_SMPTE_TimeType2997Drop: u32 = 6;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_smpte_timetype60?language=objc)
pub const kCAF_SMPTE_TimeType60: u32 = 7;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_smpte_timetype5994?language=objc)
pub const kCAF_SMPTE_TimeType5994: u32 = 8;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_smpte_timetype60drop?language=objc)
pub const kCAF_SMPTE_TimeType60Drop: u32 = 9;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_smpte_timetype5994drop?language=objc)
pub const kCAF_SMPTE_TimeType5994Drop: u32 = 10;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_smpte_timetype50?language=objc)
pub const kCAF_SMPTE_TimeType50: u32 = 11;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kcaf_smpte_timetype2398?language=objc)
pub const kCAF_SMPTE_TimeType2398: u32 = 12;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/caf_smpte_time?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAF_SMPTE_Time {
    pub mHours: i8,
    pub mMinutes: i8,
    pub mSeconds: i8,
    pub mFrames: i8,
    pub mSubFrameSampleOffset: u32,
}

unsafe impl Encode for CAF_SMPTE_Time {
    const ENCODING: Encoding = Encoding::Struct(
        "CAF_SMPTE_Time",
        &[
            <i8>::ENCODING,
            <i8>::ENCODING,
            <i8>::ENCODING,
            <i8>::ENCODING,
            <u32>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for CAF_SMPTE_Time {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cafmarker?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFMarker {
    pub mType: u32,
    pub mFramePosition: f64,
    pub mMarkerID: u32,
    pub mSMPTETime: CAF_SMPTE_Time,
    pub mChannel: u32,
}

unsafe impl Encode for CAFMarker {
    const ENCODING: Encoding = Encoding::Struct(
        "CAFMarker",
        &[
            <u32>::ENCODING,
            <f64>::ENCODING,
            <u32>::ENCODING,
            <CAF_SMPTE_Time>::ENCODING,
            <u32>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for CAFMarker {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cafmarkerchunk?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFMarkerChunk {
    pub mSMPTE_TimeType: u32,
    pub mNumberMarkers: u32,
    pub mMarkers: [CAFMarker; 1],
}

unsafe impl Encode for CAFMarkerChunk {
    const ENCODING: Encoding = Encoding::Struct(
        "CAFMarkerChunk",
        &[<u32>::ENCODING, <u32>::ENCODING, <[CAFMarker; 1]>::ENCODING],
    );
}

unsafe impl RefEncode for CAFMarkerChunk {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cafregionflags?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CAFRegionFlags(pub u32);
bitflags::bitflags! {
    impl CAFRegionFlags: u32 {
        const kCAFRegionFlag_LoopEnable = 1;
        const kCAFRegionFlag_PlayForward = 2;
        const kCAFRegionFlag_PlayBackward = 4;
    }
}

unsafe impl Encode for CAFRegionFlags {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for CAFRegionFlags {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cafregion?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFRegion {
    pub mRegionID: u32,
    pub mFlags: CAFRegionFlags,
    pub mNumberMarkers: u32,
    pub mMarkers: [CAFMarker; 1],
}

unsafe impl Encode for CAFRegion {
    const ENCODING: Encoding = Encoding::Struct(
        "CAFRegion",
        &[
            <u32>::ENCODING,
            <CAFRegionFlags>::ENCODING,
            <u32>::ENCODING,
            <[CAFMarker; 1]>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for CAFRegion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cafregionchunk?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFRegionChunk {
    pub mSMPTE_TimeType: u32,
    pub mNumberRegions: u32,
    pub mRegions: [CAFRegion; 1],
}

unsafe impl Encode for CAFRegionChunk {
    const ENCODING: Encoding = Encoding::Struct(
        "CAFRegionChunk",
        &[<u32>::ENCODING, <u32>::ENCODING, <[CAFRegion; 1]>::ENCODING],
    );
}

unsafe impl RefEncode for CAFRegionChunk {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cafinstrumentchunk?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFInstrumentChunk {
    pub mBaseNote: f32,
    pub mMIDILowNote: u8,
    pub mMIDIHighNote: u8,
    pub mMIDILowVelocity: u8,
    pub mMIDIHighVelocity: u8,
    pub mdBGain: f32,
    pub mStartRegionID: u32,
    pub mSustainRegionID: u32,
    pub mReleaseRegionID: u32,
    pub mInstrumentID: u32,
}

unsafe impl Encode for CAFInstrumentChunk {
    const ENCODING: Encoding = Encoding::Struct(
        "CAFInstrumentChunk",
        &[
            <f32>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <u8>::ENCODING,
            <f32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for CAFInstrumentChunk {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cafstringid?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFStringID {
    pub mStringID: u32,
    pub mStringStartByteOffset: i64,
}

unsafe impl Encode for CAFStringID {
    const ENCODING: Encoding = Encoding::Struct("CAFStringID", &[<u32>::ENCODING, <i64>::ENCODING]);
}

unsafe impl RefEncode for CAFStringID {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cafstrings?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFStrings {
    pub mNumEntries: u32,
    pub mStringsIDs: [CAFStringID; 1],
}

unsafe impl Encode for CAFStrings {
    const ENCODING: Encoding = Encoding::Struct(
        "CAFStrings",
        &[<u32>::ENCODING, <[CAFStringID; 1]>::ENCODING],
    );
}

unsafe impl RefEncode for CAFStrings {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cafinfostrings?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFInfoStrings {
    pub mNumEntries: u32,
}

unsafe impl Encode for CAFInfoStrings {
    const ENCODING: Encoding = Encoding::Struct("CAFInfoStrings", &[<u32>::ENCODING]);
}

unsafe impl RefEncode for CAFInfoStrings {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cafpositionpeak?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFPositionPeak {
    pub mValue: f32,
    pub mFrameNumber: u64,
}

unsafe impl Encode for CAFPositionPeak {
    const ENCODING: Encoding =
        Encoding::Struct("CAFPositionPeak", &[<f32>::ENCODING, <u64>::ENCODING]);
}

unsafe impl RefEncode for CAFPositionPeak {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cafpeakchunk?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFPeakChunk {
    pub mEditCount: u32,
    pub mPeaks: [CAFPositionPeak; 1],
}

unsafe impl Encode for CAFPeakChunk {
    const ENCODING: Encoding = Encoding::Struct(
        "CAFPeakChunk",
        &[<u32>::ENCODING, <[CAFPositionPeak; 1]>::ENCODING],
    );
}

unsafe impl RefEncode for CAFPeakChunk {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cafoverviewsample?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFOverviewSample {
    pub mMinValue: i16,
    pub mMaxValue: i16,
}

unsafe impl Encode for CAFOverviewSample {
    const ENCODING: Encoding =
        Encoding::Struct("CAFOverviewSample", &[<i16>::ENCODING, <i16>::ENCODING]);
}

unsafe impl RefEncode for CAFOverviewSample {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cafoverviewchunk?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFOverviewChunk {
    pub mEditCount: u32,
    pub mNumFramesPerOVWSample: u32,
    pub mData: [CAFOverviewSample; 1],
}

unsafe impl Encode for CAFOverviewChunk {
    const ENCODING: Encoding = Encoding::Struct(
        "CAFOverviewChunk",
        &[
            <u32>::ENCODING,
            <u32>::ENCODING,
            <[CAFOverviewSample; 1]>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for CAFOverviewChunk {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/cafumidchunk?language=objc)
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CAFUMIDChunk {
    pub mBytes: [u8; 64],
}

unsafe impl Encode for CAFUMIDChunk {
    const ENCODING: Encoding = Encoding::Struct("CAFUMIDChunk", &[<[u8; 64]>::ENCODING]);
}

unsafe impl RefEncode for CAFUMIDChunk {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
