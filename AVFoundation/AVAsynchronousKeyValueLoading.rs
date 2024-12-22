//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avkeyvaluestatus?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AVKeyValueStatus(pub NSInteger);
impl AVKeyValueStatus {
    #[doc(alias = "AVKeyValueStatusUnknown")]
    pub const Unknown: Self = Self(0);
    #[doc(alias = "AVKeyValueStatusLoading")]
    pub const Loading: Self = Self(1);
    #[doc(alias = "AVKeyValueStatusLoaded")]
    pub const Loaded: Self = Self(2);
    #[doc(alias = "AVKeyValueStatusFailed")]
    pub const Failed: Self = Self(3);
    #[doc(alias = "AVKeyValueStatusCancelled")]
    pub const Cancelled: Self = Self(4);
}

unsafe impl Encode for AVKeyValueStatus {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for AVKeyValueStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_protocol!(
    /// The AVAsynchronousKeyValueLoading protocol defines methods that let clients use an AVAsset or AVAssetTrack object without blocking a thread. Using methods in the protocol, one can find out the current status of a key (for example, whether the corresponding value has been loaded); and ask the object to load values asynchronously, informing the client when the operation has completed.
    ///
    ///
    /// Because of the nature of timed audiovisual media, successful initialization of an asset does not necessarily mean that all its data is immediately available. Instead, an asset will wait to load data until an operation is performed on it (for example, directly invoking any relevant AVAsset methods, playback via an AVPlayerItem object, export using AVAssetExportSession, reading using an instance of AVAssetReader, and so on). This means that although you can request the value of any key at any time, and its value will be returned synchronously, the calling thread may be blocked until the request can be satisfied. To avoid blocking, you can:
    ///
    /// 1. First, determine whether the value for a given key is available using statusOfValueForKey:error:.
    /// 2. If a value has not been loaded yet, you can ask for to load one or more values and be notified when they become available using loadValuesAsynchronouslyForKeys:completionHandler:.
    ///
    /// Even for use cases that may typically support ready access to some keys (such as for assets initialized with URLs for files in the local filesystem), slow I/O may require AVAsset to block before returning their values. Although blocking may be acceptable for macOS API clients in cases where assets are being prepared on background threads or in operation queues, in all cases in which blocking should be avoided you should use loadValuesAsynchronouslyForKeys:completionHandler:. For clients of platforms other than macOS, blocking to obtain the value of a key synchronously is never recommended under any circumstances.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/avfoundation/avasynchronouskeyvalueloading?language=objc)
    pub unsafe trait AVAsynchronousKeyValueLoading {
        #[cfg(feature = "block2")]
        /// Directs the target to load the values of any of the specified keys that are not already loaded.
        ///
        /// Parameter `keys`: An instance of NSArray, containing NSStrings for the specified keys.
        ///
        /// Parameter `handler`: The block to be invoked when loading succeeds, fails, or is cancelled.
        #[method(loadValuesAsynchronouslyForKeys:completionHandler:)]
        unsafe fn loadValuesAsynchronouslyForKeys_completionHandler(
            &self,
            keys: &NSArray<NSString>,
            handler: Option<&block2::Block<dyn Fn()>>,
        );
    }

    unsafe impl ProtocolType for dyn AVAsynchronousKeyValueLoading {}
);
