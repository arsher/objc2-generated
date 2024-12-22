//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
#[cfg(feature = "objc2-core-audio-types")]
use objc2_core_audio_types::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// An extended audio file object.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/extaudiofileref?language=objc)
pub type ExtAudioFileRef = *mut c_void;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/extaudiofilepackettableinfooverride?language=objc)
pub type ExtAudioFilePacketTableInfoOverride = i32;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofilepackettableinfooverride_usefilevalue?language=objc)
pub const kExtAudioFilePacketTableInfoOverride_UseFileValue: ExtAudioFilePacketTableInfoOverride =
    -1;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofilepackettableinfooverride_usefilevalueifvalid?language=objc)
pub const kExtAudioFilePacketTableInfoOverride_UseFileValueIfValid:
    ExtAudioFilePacketTableInfoOverride = -2;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/extaudiofilepropertyid?language=objc)
pub type ExtAudioFilePropertyID = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileproperty_filedataformat?language=objc)
pub const kExtAudioFileProperty_FileDataFormat: ExtAudioFilePropertyID = 0x66666d74;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileproperty_filechannellayout?language=objc)
pub const kExtAudioFileProperty_FileChannelLayout: ExtAudioFilePropertyID = 0x66636c6f;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileproperty_clientdataformat?language=objc)
pub const kExtAudioFileProperty_ClientDataFormat: ExtAudioFilePropertyID = 0x63666d74;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileproperty_clientchannellayout?language=objc)
pub const kExtAudioFileProperty_ClientChannelLayout: ExtAudioFilePropertyID = 0x63636c6f;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileproperty_codecmanufacturer?language=objc)
pub const kExtAudioFileProperty_CodecManufacturer: ExtAudioFilePropertyID = 0x636d616e;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileproperty_audioconverter?language=objc)
pub const kExtAudioFileProperty_AudioConverter: ExtAudioFilePropertyID = 0x61636e76;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileproperty_audiofile?language=objc)
pub const kExtAudioFileProperty_AudioFile: ExtAudioFilePropertyID = 0x6166696c;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileproperty_filemaxpacketsize?language=objc)
pub const kExtAudioFileProperty_FileMaxPacketSize: ExtAudioFilePropertyID = 0x666d7073;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileproperty_clientmaxpacketsize?language=objc)
pub const kExtAudioFileProperty_ClientMaxPacketSize: ExtAudioFilePropertyID = 0x636d7073;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileproperty_filelengthframes?language=objc)
pub const kExtAudioFileProperty_FileLengthFrames: ExtAudioFilePropertyID = 0x2366726d;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileproperty_converterconfig?language=objc)
pub const kExtAudioFileProperty_ConverterConfig: ExtAudioFilePropertyID = 0x61636366;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileproperty_iobuffersizebytes?language=objc)
pub const kExtAudioFileProperty_IOBufferSizeBytes: ExtAudioFilePropertyID = 0x696f6273;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileproperty_iobuffer?language=objc)
pub const kExtAudioFileProperty_IOBuffer: ExtAudioFilePropertyID = 0x696f6266;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileproperty_packettable?language=objc)
pub const kExtAudioFileProperty_PacketTable: ExtAudioFilePropertyID = 0x78707469;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileerror_invalidproperty?language=objc)
pub const kExtAudioFileError_InvalidProperty: OSStatus = -66561;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileerror_invalidpropertysize?language=objc)
pub const kExtAudioFileError_InvalidPropertySize: OSStatus = -66562;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileerror_nonpcmclientformat?language=objc)
pub const kExtAudioFileError_NonPCMClientFormat: OSStatus = -66563;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileerror_invalidchannelmap?language=objc)
pub const kExtAudioFileError_InvalidChannelMap: OSStatus = -66564;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileerror_invalidoperationorder?language=objc)
pub const kExtAudioFileError_InvalidOperationOrder: OSStatus = -66565;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileerror_invaliddataformat?language=objc)
pub const kExtAudioFileError_InvalidDataFormat: OSStatus = -66566;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileerror_maxpacketsizeunknown?language=objc)
pub const kExtAudioFileError_MaxPacketSizeUnknown: OSStatus = -66567;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileerror_invalidseek?language=objc)
pub const kExtAudioFileError_InvalidSeek: OSStatus = -66568;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileerror_asyncwritetoolarge?language=objc)
pub const kExtAudioFileError_AsyncWriteTooLarge: OSStatus = -66569;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kextaudiofileerror_asyncwritebufferoverflow?language=objc)
pub const kExtAudioFileError_AsyncWriteBufferOverflow: OSStatus = -66570;

extern "C-unwind" {
    /// Opens an audio file specified by a CFURLRef.
    ///
    /// Parameter `inURL`: The audio file to read.
    ///
    /// Parameter `outExtAudioFile`: On exit, a newly-allocated ExtAudioFileRef.
    ///
    /// Returns: An OSStatus error code.
    ///
    ///
    /// Allocates a new ExtAudioFileRef, for reading an existing audio file.
    #[cfg(feature = "objc2-core-foundation")]
    pub fn ExtAudioFileOpenURL(
        in_url: CFURLRef,
        out_ext_audio_file: NonNull<ExtAudioFileRef>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Wrap an AudioFileID in an ExtAudioFileRef.
    ///
    /// Parameter `inFileID`: The AudioFileID to wrap.
    ///
    /// Parameter `inForWriting`: True if the AudioFileID is a new file opened for writing.
    ///
    /// Parameter `outExtAudioFile`: On exit, a newly-allocated ExtAudioFileRef.
    ///
    /// Returns: An OSStatus error code.
    ///
    ///
    /// Allocates a new ExtAudioFileRef which wraps an existing AudioFileID. The
    /// client is responsible for keeping the AudioFileID open until the
    /// ExtAudioFileRef is disposed. Disposing the ExtAudioFileRef will not close
    /// the AudioFileID when this Wrap API call is used, so the client is also
    /// responsible for closing the AudioFileID when finished with it.
    #[cfg(all(feature = "AudioFile", feature = "AudioUnitProperties"))]
    pub fn ExtAudioFileWrapAudioFileID(
        in_file_id: AudioFileID,
        in_for_writing: Boolean,
        out_ext_audio_file: NonNull<ExtAudioFileRef>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Create a new audio file.
    ///
    /// Parameter `inURL`: The URL of the new audio file.
    ///
    /// Parameter `inFileType`: The type of file to create. This is a constant from AudioToolbox/AudioFile.h, e.g.
    /// kAudioFileAIFFType. Note that this is not an HFSTypeCode.
    ///
    /// Parameter `inStreamDesc`: The format of the audio data to be written to the file.
    ///
    /// Parameter `inChannelLayout`: The channel layout of the audio data. If non-null, this must be consistent
    /// with the number of channels specified by inStreamDesc.
    ///
    /// Parameter `inFlags`: The same flags as are used with AudioFileCreateWithURL
    /// Can use these to control whether an existing file is overwritten (or not).
    ///
    /// Parameter `outExtAudioFile`: On exit, a newly-allocated ExtAudioFileRef.
    ///
    /// Returns: An OSStatus error code.
    ///
    ///
    /// Creates a new audio file.
    ///
    /// If the file to be created is in an encoded format, it is permissible for the
    /// sample rate in inStreamDesc to be 0, since in all cases, the file's encoding
    /// AudioConverter may produce audio at a different sample rate than the source. The
    /// file will be created with the audio format actually produced by the encoder.
    #[cfg(all(
        feature = "AudioFile",
        feature = "objc2-core-audio-types",
        feature = "objc2-core-foundation"
    ))]
    pub fn ExtAudioFileCreateWithURL(
        in_url: CFURLRef,
        in_file_type: AudioFileTypeID,
        in_stream_desc: NonNull<AudioStreamBasicDescription>,
        in_channel_layout: *const AudioChannelLayout,
        in_flags: u32,
        out_ext_audio_file: NonNull<ExtAudioFileRef>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Close the file and dispose the object.
    ///
    /// Parameter `inExtAudioFile`: The extended audio file object.
    ///
    /// Returns: An OSStatus error code.
    ///
    ///
    /// Closes the file and deletes the object.
    pub fn ExtAudioFileDispose(in_ext_audio_file: ExtAudioFileRef) -> OSStatus;
}

extern "C-unwind" {
    /// Perform a synchronous sequential read.
    ///
    ///
    /// Parameter `inExtAudioFile`: The extended audio file object.
    ///
    /// Parameter `ioNumberFrames`: On entry, ioNumberFrames is the number of frames to be read from the file.
    /// On exit, it is the number of frames actually read. A number of factors may
    /// cause a fewer number of frames to be read, including the supplied buffers
    /// not being large enough, and internal optimizations. If 0 frames are
    /// returned, however, this indicates that end-of-file was reached.
    ///
    /// Parameter `ioData`: Buffer(s) into which the audio data is read.
    ///
    /// Returns: An OSStatus error code.
    ///
    ///
    /// If the file has a client data format, then the audio data from the file is
    /// translated from the file data format to the client format, via the
    /// ExtAudioFile's internal AudioConverter.
    ///
    /// (Note that the use of sequential reads/writes means that an ExtAudioFile must
    /// not be read on multiple threads; clients wishing to do this should use the
    /// lower-level AudioFile API set).
    #[cfg(feature = "objc2-core-audio-types")]
    pub fn ExtAudioFileRead(
        in_ext_audio_file: ExtAudioFileRef,
        io_number_frames: NonNull<u32>,
        io_data: NonNull<AudioBufferList>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Perform a synchronous sequential write.
    ///
    ///
    /// Parameter `inExtAudioFile`: The extended audio file object.
    ///
    /// Parameter `inNumberFrames`: The number of frames to write.
    ///
    /// Parameter `ioData`: The buffer(s) from which audio data is written to the file.
    ///
    /// Returns: An OSStatus error code.
    ///
    ///
    /// If the file has a client data format, then the audio data in ioData is
    /// translated from the client format to the file data format, via the
    /// ExtAudioFile's internal AudioConverter.
    #[cfg(feature = "objc2-core-audio-types")]
    pub fn ExtAudioFileWrite(
        in_ext_audio_file: ExtAudioFileRef,
        in_number_frames: u32,
        io_data: NonNull<AudioBufferList>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Perform an asynchronous sequential write.
    ///
    ///
    /// Parameter `inExtAudioFile`: The extended audio file object.
    ///
    /// Parameter `inNumberFrames`: The number of frames to write.
    ///
    /// Parameter `ioData`: The buffer(s) from which audio data is written to the file.
    ///
    /// Returns: An OSStatus error code.
    ///
    ///
    /// Writes the provided buffer list to an internal ring buffer and notifies an
    /// internal thread to perform the write at a later time. The first time this is
    /// called, allocations may be performed. You can call this with 0 frames and null
    /// buffer in a non-time-critical context to initialize the asynchronous mechanism.
    /// Once initialized, subsequent calls are very efficient and do not take locks;
    /// thus this may be used to write to a file from a realtime thread.
    ///
    /// The client must not mix synchronous and asynchronous writes to the same file.
    ///
    /// Pending writes are not guaranteed to be flushed to disk until
    /// ExtAudioFileDispose is called.
    ///
    /// N.B. Errors may occur after this call has returned. Such errors may be returned
    /// from subsequent calls to this function.
    #[cfg(feature = "objc2-core-audio-types")]
    pub fn ExtAudioFileWriteAsync(
        in_ext_audio_file: ExtAudioFileRef,
        in_number_frames: u32,
        io_data: *const AudioBufferList,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Seek to a specific frame position.
    ///
    ///
    /// Parameter `inExtAudioFile`: The extended audio file object.
    ///
    /// Parameter `inFrameOffset`: The desired seek position, in sample frames, relative to the beginning of
    /// the file. This is specified in the sample rate and frame count of the file's format
    /// (not the client format)
    ///
    /// Returns: An OSStatus error code.
    ///
    ///
    /// Sets the file's read position to the specified sample frame number. The next call
    /// to ExtAudioFileRead will return samples from precisely this location, even if it
    /// is located in the middle of a packet.
    ///
    /// This function's behavior with files open for writing is currently undefined.
    pub fn ExtAudioFileSeek(in_ext_audio_file: ExtAudioFileRef, in_frame_offset: i64) -> OSStatus;
}

extern "C-unwind" {
    /// Return the file's read/write position.
    ///
    ///
    /// Parameter `inExtAudioFile`: The extended audio file object.
    ///
    /// Parameter `outFrameOffset`: On exit, the file's current read/write position in sample frames. This is specified in the
    /// sample rate and frame count of the file's format (not the client format)
    ///
    /// Returns: An OSStatus error code.
    pub fn ExtAudioFileTell(
        in_ext_audio_file: ExtAudioFileRef,
        out_frame_offset: NonNull<i64>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Get information about a property
    ///
    ///
    /// Parameter `inExtAudioFile`: The extended audio file object.
    ///
    /// Parameter `inPropertyID`: The property being queried.
    ///
    /// Parameter `outSize`: If non-null, on exit, this is set to the size of the property's value.
    ///
    /// Parameter `outWritable`: If non-null, on exit, this indicates whether the property value is settable.
    ///
    /// Returns: An OSStatus error code.
    pub fn ExtAudioFileGetPropertyInfo(
        in_ext_audio_file: ExtAudioFileRef,
        in_property_id: ExtAudioFilePropertyID,
        out_size: *mut u32,
        out_writable: *mut Boolean,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Get a property value.
    ///
    ///
    /// Parameter `inExtAudioFile`: The extended audio file object.
    ///
    /// Parameter `inPropertyID`: The property being fetched.
    ///
    /// Parameter `ioPropertyDataSize`: On entry, the size (in bytes) of the memory pointed to by outPropertyData.
    /// On exit, the actual size of the property data returned.
    ///
    /// Parameter `outPropertyData`: The value of the property is copied to the memory this points to.
    ///
    /// Returns: An OSStatus error code.
    pub fn ExtAudioFileGetProperty(
        in_ext_audio_file: ExtAudioFileRef,
        in_property_id: ExtAudioFilePropertyID,
        io_property_data_size: NonNull<u32>,
        out_property_data: NonNull<c_void>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Set a property value.
    ///
    ///
    /// Parameter `inExtAudioFile`: The extended audio file object.
    ///
    /// Parameter `inPropertyID`: The property being set.
    ///
    /// Parameter `inPropertyDataSize`: The size of the property data, in bytes.
    ///
    /// Parameter `inPropertyData`: Points to the property's new value.
    ///
    /// Returns: An OSStatus error code.
    pub fn ExtAudioFileSetProperty(
        in_ext_audio_file: ExtAudioFileRef,
        in_property_id: ExtAudioFilePropertyID,
        in_property_data_size: u32,
        in_property_data: NonNull<c_void>,
    ) -> OSStatus;
}
