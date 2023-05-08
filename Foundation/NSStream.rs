//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type NSStreamPropertyKey = NSString;
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSStreamStatus {
        NSStreamStatusNotOpen = 0,
        NSStreamStatusOpening = 1,
        NSStreamStatusOpen = 2,
        NSStreamStatusReading = 3,
        NSStreamStatusWriting = 4,
        NSStreamStatusAtEnd = 5,
        NSStreamStatusClosed = 6,
        NSStreamStatusError = 7,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSStreamEvent {
        NSStreamEventNone = 0,
        NSStreamEventOpenCompleted = 1 << 0,
        NSStreamEventHasBytesAvailable = 1 << 1,
        NSStreamEventHasSpaceAvailable = 1 << 2,
        NSStreamEventErrorOccurred = 1 << 3,
        NSStreamEventEndEncountered = 1 << 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSStream")]
    pub struct NSStream;

    #[cfg(feature = "Foundation_NSStream")]
    unsafe impl ClassType for NSStream {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSStream")]
unsafe impl NSObjectProtocol for NSStream {}

extern_methods!(
    #[cfg(feature = "Foundation_NSStream")]
    unsafe impl NSStream {
        #[method(open)]
        pub unsafe fn open(&self);

        #[method(close)]
        pub unsafe fn close(&self);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSStreamDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSStreamDelegate>>);

        #[method_id(@__retain_semantics Other propertyForKey:)]
        pub unsafe fn propertyForKey(&self, key: &NSStreamPropertyKey) -> Option<Id<Object>>;

        #[method(setProperty:forKey:)]
        pub unsafe fn setProperty_forKey(
            &self,
            property: Option<&Object>,
            key: &NSStreamPropertyKey,
        ) -> bool;

        #[cfg(feature = "Foundation_NSRunLoop")]
        #[method(scheduleInRunLoop:forMode:)]
        pub unsafe fn scheduleInRunLoop_forMode(
            &self,
            a_run_loop: &NSRunLoop,
            mode: &NSRunLoopMode,
        );

        #[cfg(feature = "Foundation_NSRunLoop")]
        #[method(removeFromRunLoop:forMode:)]
        pub unsafe fn removeFromRunLoop_forMode(
            &self,
            a_run_loop: &NSRunLoop,
            mode: &NSRunLoopMode,
        );

        #[method(streamStatus)]
        pub unsafe fn streamStatus(&self) -> NSStreamStatus;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other streamError)]
        pub unsafe fn streamError(&self) -> Option<Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSStream")]
    unsafe impl NSStream {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSInputStream")]
    pub struct NSInputStream;

    #[cfg(feature = "Foundation_NSInputStream")]
    unsafe impl ClassType for NSInputStream {
        #[inherits(NSObject)]
        type Super = NSStream;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSInputStream")]
unsafe impl NSObjectProtocol for NSInputStream {}

extern_methods!(
    #[cfg(feature = "Foundation_NSInputStream")]
    unsafe impl NSInputStream {
        #[method(read:maxLength:)]
        pub unsafe fn read_maxLength(&self, buffer: NonNull<u8>, len: NSUInteger) -> NSInteger;

        #[method(getBuffer:length:)]
        pub unsafe fn getBuffer_length(
            &self,
            buffer: NonNull<*mut u8>,
            len: NonNull<NSUInteger>,
        ) -> bool;

        #[method(hasBytesAvailable)]
        pub unsafe fn hasBytesAvailable(&self) -> bool;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(this: Option<Allocated<Self>>, data: &NSData) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithURL:)]
        pub unsafe fn initWithURL(this: Option<Allocated<Self>>, url: &NSURL) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSInputStream")]
    unsafe impl NSInputStream {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSOutputStream")]
    pub struct NSOutputStream;

    #[cfg(feature = "Foundation_NSOutputStream")]
    unsafe impl ClassType for NSOutputStream {
        #[inherits(NSObject)]
        type Super = NSStream;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSOutputStream")]
unsafe impl NSObjectProtocol for NSOutputStream {}

extern_methods!(
    #[cfg(feature = "Foundation_NSOutputStream")]
    unsafe impl NSOutputStream {
        #[method(write:maxLength:)]
        pub unsafe fn write_maxLength(&self, buffer: NonNull<u8>, len: NSUInteger) -> NSInteger;

        #[method(hasSpaceAvailable)]
        pub unsafe fn hasSpaceAvailable(&self) -> bool;

        #[method_id(@__retain_semantics Init initToMemory)]
        pub unsafe fn initToMemory(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initToBuffer:capacity:)]
        pub unsafe fn initToBuffer_capacity(
            this: Option<Allocated<Self>>,
            buffer: NonNull<u8>,
            capacity: NSUInteger,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithURL:append:)]
        pub unsafe fn initWithURL_append(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            should_append: bool,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSOutputStream")]
    unsafe impl NSOutputStream {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSSocketStreamCreationExtensions
    #[cfg(feature = "Foundation_NSStream")]
    unsafe impl NSStream {
        #[cfg(all(
            feature = "Foundation_NSInputStream",
            feature = "Foundation_NSOutputStream",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use nw_connection_t in Network framework instead"]
        #[method(getStreamsToHostWithName:port:inputStream:outputStream:)]
        pub unsafe fn getStreamsToHostWithName_port_inputStream_outputStream(
            hostname: &NSString,
            port: NSInteger,
            input_stream: Option<&mut Option<Id<NSInputStream>>>,
            output_stream: Option<&mut Option<Id<NSOutputStream>>>,
        );

        #[cfg(all(
            feature = "Foundation_NSHost",
            feature = "Foundation_NSInputStream",
            feature = "Foundation_NSOutputStream"
        ))]
        #[deprecated = "Use nw_connection_t in Network framework instead"]
        #[method(getStreamsToHost:port:inputStream:outputStream:)]
        pub unsafe fn getStreamsToHost_port_inputStream_outputStream(
            host: &NSHost,
            port: NSInteger,
            input_stream: Option<&mut Option<Id<NSInputStream>>>,
            output_stream: Option<&mut Option<Id<NSOutputStream>>>,
        );
    }
);

extern_methods!(
    /// NSStreamBoundPairCreationExtensions
    #[cfg(feature = "Foundation_NSStream")]
    unsafe impl NSStream {
        #[cfg(all(
            feature = "Foundation_NSInputStream",
            feature = "Foundation_NSOutputStream"
        ))]
        #[method(getBoundStreamsWithBufferSize:inputStream:outputStream:)]
        pub unsafe fn getBoundStreamsWithBufferSize_inputStream_outputStream(
            buffer_size: NSUInteger,
            input_stream: Option<&mut Option<Id<NSInputStream>>>,
            output_stream: Option<&mut Option<Id<NSOutputStream>>>,
        );
    }
);

extern_methods!(
    /// NSInputStreamExtensions
    #[cfg(feature = "Foundation_NSInputStream")]
    unsafe impl NSInputStream {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithFileAtPath:)]
        pub unsafe fn initWithFileAtPath(
            this: Option<Allocated<Self>>,
            path: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other inputStreamWithData:)]
        pub unsafe fn inputStreamWithData(data: &NSData) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other inputStreamWithFileAtPath:)]
        pub unsafe fn inputStreamWithFileAtPath(path: &NSString) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other inputStreamWithURL:)]
        pub unsafe fn inputStreamWithURL(url: &NSURL) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// NSOutputStreamExtensions
    #[cfg(feature = "Foundation_NSOutputStream")]
    unsafe impl NSOutputStream {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initToFileAtPath:append:)]
        pub unsafe fn initToFileAtPath_append(
            this: Option<Allocated<Self>>,
            path: &NSString,
            should_append: bool,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other outputStreamToMemory)]
        pub unsafe fn outputStreamToMemory() -> Id<Self>;

        #[method_id(@__retain_semantics Other outputStreamToBuffer:capacity:)]
        pub unsafe fn outputStreamToBuffer_capacity(
            buffer: NonNull<u8>,
            capacity: NSUInteger,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other outputStreamToFileAtPath:append:)]
        pub unsafe fn outputStreamToFileAtPath_append(
            path: &NSString,
            should_append: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other outputStreamWithURL:append:)]
        pub unsafe fn outputStreamWithURL_append(
            url: &NSURL,
            should_append: bool,
        ) -> Option<Id<Self>>;
    }
);

extern_protocol!(
    pub unsafe trait NSStreamDelegate: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSStream")]
        #[optional]
        #[method(stream:handleEvent:)]
        unsafe fn stream_handleEvent(&self, a_stream: &NSStream, event_code: NSStreamEvent);
    }

    unsafe impl ProtocolType for dyn NSStreamDelegate {}
);

extern_static!(NSStreamSocketSecurityLevelKey: &'static NSStreamPropertyKey);

typed_enum!(
    pub type NSStreamSocketSecurityLevel = NSString;
);

extern_static!(NSStreamSocketSecurityLevelNone: &'static NSStreamSocketSecurityLevel);

extern_static!(NSStreamSocketSecurityLevelSSLv2: &'static NSStreamSocketSecurityLevel);

extern_static!(NSStreamSocketSecurityLevelSSLv3: &'static NSStreamSocketSecurityLevel);

extern_static!(NSStreamSocketSecurityLevelTLSv1: &'static NSStreamSocketSecurityLevel);

extern_static!(NSStreamSocketSecurityLevelNegotiatedSSL: &'static NSStreamSocketSecurityLevel);

extern_static!(NSStreamSOCKSProxyConfigurationKey: &'static NSStreamPropertyKey);

typed_enum!(
    pub type NSStreamSOCKSProxyConfiguration = NSString;
);

extern_static!(NSStreamSOCKSProxyHostKey: &'static NSStreamSOCKSProxyConfiguration);

extern_static!(NSStreamSOCKSProxyPortKey: &'static NSStreamSOCKSProxyConfiguration);

extern_static!(NSStreamSOCKSProxyVersionKey: &'static NSStreamSOCKSProxyConfiguration);

extern_static!(NSStreamSOCKSProxyUserKey: &'static NSStreamSOCKSProxyConfiguration);

extern_static!(NSStreamSOCKSProxyPasswordKey: &'static NSStreamSOCKSProxyConfiguration);

typed_enum!(
    pub type NSStreamSOCKSProxyVersion = NSString;
);

extern_static!(NSStreamSOCKSProxyVersion4: &'static NSStreamSOCKSProxyVersion);

extern_static!(NSStreamSOCKSProxyVersion5: &'static NSStreamSOCKSProxyVersion);

extern_static!(NSStreamDataWrittenToMemoryStreamKey: &'static NSStreamPropertyKey);

extern_static!(NSStreamFileCurrentOffsetKey: &'static NSStreamPropertyKey);

extern_static!(NSStreamSocketSSLErrorDomain: &'static NSErrorDomain);

extern_static!(NSStreamSOCKSErrorDomain: &'static NSErrorDomain);

extern_static!(NSStreamNetworkServiceType: &'static NSStreamPropertyKey);

typed_enum!(
    pub type NSStreamNetworkServiceTypeValue = NSString;
);

extern_static!(NSStreamNetworkServiceTypeVoIP: &'static NSStreamNetworkServiceTypeValue);

extern_static!(NSStreamNetworkServiceTypeVideo: &'static NSStreamNetworkServiceTypeValue);

extern_static!(NSStreamNetworkServiceTypeBackground: &'static NSStreamNetworkServiceTypeValue);

extern_static!(NSStreamNetworkServiceTypeVoice: &'static NSStreamNetworkServiceTypeValue);

extern_static!(NSStreamNetworkServiceTypeCallSignaling: &'static NSStreamNetworkServiceTypeValue);
