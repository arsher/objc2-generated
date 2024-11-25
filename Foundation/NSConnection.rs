//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsconnection?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use NSXPCConnection instead"]
    pub struct NSConnection;
);

unsafe impl NSObjectProtocol for NSConnection {}

extern_methods!(
    unsafe impl NSConnection {
        #[cfg(all(feature = "NSDictionary", feature = "NSString", feature = "NSValue"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other statistics)]
        pub unsafe fn statistics(&self) -> Retained<NSDictionary<NSString, NSNumber>>;

        #[cfg(feature = "NSArray")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other allConnections)]
        pub unsafe fn allConnections() -> Retained<NSArray<NSConnection>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other defaultConnection)]
        pub unsafe fn defaultConnection() -> Retained<NSConnection>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other connectionWithRegisteredName:host:)]
        pub unsafe fn connectionWithRegisteredName_host(
            name: &NSString,
            host_name: Option<&NSString>,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "NSPortNameServer", feature = "NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other connectionWithRegisteredName:host:usingNameServer:)]
        pub unsafe fn connectionWithRegisteredName_host_usingNameServer(
            name: &NSString,
            host_name: Option<&NSString>,
            server: &NSPortNameServer,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "NSDistantObject", feature = "NSProxy", feature = "NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other rootProxyForConnectionWithRegisteredName:host:)]
        pub unsafe fn rootProxyForConnectionWithRegisteredName_host(
            name: &NSString,
            host_name: Option<&NSString>,
        ) -> Option<Retained<NSDistantObject>>;

        #[cfg(all(
            feature = "NSDistantObject",
            feature = "NSPortNameServer",
            feature = "NSProxy",
            feature = "NSString"
        ))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other rootProxyForConnectionWithRegisteredName:host:usingNameServer:)]
        pub unsafe fn rootProxyForConnectionWithRegisteredName_host_usingNameServer(
            name: &NSString,
            host_name: Option<&NSString>,
            server: &NSPortNameServer,
        ) -> Option<Retained<NSDistantObject>>;

        #[cfg(all(feature = "NSPortNameServer", feature = "NSString"))]
        #[method_id(@__retain_semantics Other serviceConnectionWithName:rootObject:usingNameServer:)]
        pub unsafe fn serviceConnectionWithName_rootObject_usingNameServer(
            name: &NSString,
            root: &AnyObject,
            server: &NSPortNameServer,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other serviceConnectionWithName:rootObject:)]
        pub unsafe fn serviceConnectionWithName_rootObject(
            name: &NSString,
            root: &AnyObject,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSDate")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(requestTimeout)]
        pub unsafe fn requestTimeout(&self) -> NSTimeInterval;

        #[cfg(feature = "NSDate")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(setRequestTimeout:)]
        pub unsafe fn setRequestTimeout(&self, request_timeout: NSTimeInterval);

        #[cfg(feature = "NSDate")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(replyTimeout)]
        pub unsafe fn replyTimeout(&self) -> NSTimeInterval;

        #[cfg(feature = "NSDate")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(setReplyTimeout:)]
        pub unsafe fn setReplyTimeout(&self, reply_timeout: NSTimeInterval);

        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other rootObject)]
        pub unsafe fn rootObject(&self) -> Option<Retained<AnyObject>>;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(setRootObject:)]
        pub unsafe fn setRootObject(&self, root_object: Option<&AnyObject>);

        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self)
            -> Option<Retained<ProtocolObject<dyn NSConnectionDelegate>>>;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSConnectionDelegate>>,
        );

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(independentConversationQueueing)]
        pub unsafe fn independentConversationQueueing(&self) -> bool;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(setIndependentConversationQueueing:)]
        pub unsafe fn setIndependentConversationQueueing(
            &self,
            independent_conversation_queueing: bool,
        );

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(isValid)]
        pub unsafe fn isValid(&self) -> bool;

        #[cfg(all(feature = "NSDistantObject", feature = "NSProxy"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other rootProxy)]
        pub unsafe fn rootProxy(&self) -> Retained<NSDistantObject>;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(addRequestMode:)]
        pub unsafe fn addRequestMode(&self, rmode: &NSString);

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(removeRequestMode:)]
        pub unsafe fn removeRequestMode(&self, rmode: &NSString);

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other requestModes)]
        pub unsafe fn requestModes(&self) -> Retained<NSArray<NSString>>;

        #[cfg(feature = "NSString")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(registerName:)]
        pub unsafe fn registerName(&self, name: Option<&NSString>) -> bool;

        #[cfg(all(feature = "NSPortNameServer", feature = "NSString"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(registerName:withNameServer:)]
        pub unsafe fn registerName_withNameServer(
            &self,
            name: Option<&NSString>,
            server: &NSPortNameServer,
        ) -> bool;

        #[cfg(feature = "NSPort")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other connectionWithReceivePort:sendPort:)]
        pub unsafe fn connectionWithReceivePort_sendPort(
            receive_port: Option<&NSPort>,
            send_port: Option<&NSPort>,
        ) -> Option<Retained<Self>>;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other currentConversation)]
        pub unsafe fn currentConversation() -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSPort")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Init initWithReceivePort:sendPort:)]
        pub unsafe fn initWithReceivePort_sendPort(
            this: Allocated<Self>,
            receive_port: Option<&NSPort>,
            send_port: Option<&NSPort>,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSPort")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other sendPort)]
        pub unsafe fn sendPort(&self) -> Retained<NSPort>;

        #[cfg(feature = "NSPort")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other receivePort)]
        pub unsafe fn receivePort(&self) -> Retained<NSPort>;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(enableMultipleThreads)]
        pub unsafe fn enableMultipleThreads(&self);

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(multipleThreadsEnabled)]
        pub unsafe fn multipleThreadsEnabled(&self) -> bool;

        #[cfg(feature = "NSRunLoop")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(addRunLoop:)]
        pub unsafe fn addRunLoop(&self, runloop: &NSRunLoop);

        #[cfg(feature = "NSRunLoop")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(removeRunLoop:)]
        pub unsafe fn removeRunLoop(&self, runloop: &NSRunLoop);

        #[deprecated = "Use NSXPCConnection instead"]
        #[method(runInNewThread)]
        pub unsafe fn runInNewThread(&self);

        #[cfg(feature = "NSArray")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other remoteObjects)]
        pub unsafe fn remoteObjects(&self) -> Retained<NSArray>;

        #[cfg(feature = "NSArray")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other localObjects)]
        pub unsafe fn localObjects(&self) -> Retained<NSArray>;

        #[cfg(feature = "NSArray")]
        #[method(dispatchWithComponents:)]
        pub unsafe fn dispatchWithComponents(&self, components: &NSArray);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSConnection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsconnectionreplymode?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSConnectionReplyMode: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsconnectiondiddienotification?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSConnectionDidDieNotification: &'static NSString;
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsconnectiondelegate?language=objc)
    #[deprecated = "Use NSXPCConnection instead"]
    pub unsafe trait NSConnectionDelegate: NSObjectProtocol {
        #[deprecated = "Use NSXPCConnection instead"]
        #[optional]
        #[method(makeNewConnection:sender:)]
        unsafe fn makeNewConnection_sender(
            &self,
            conn: &NSConnection,
            ancestor: &NSConnection,
        ) -> bool;

        #[deprecated = "Use NSXPCConnection instead"]
        #[optional]
        #[method(connection:shouldMakeNewConnection:)]
        unsafe fn connection_shouldMakeNewConnection(
            &self,
            ancestor: &NSConnection,
            conn: &NSConnection,
        ) -> bool;

        #[cfg(all(feature = "NSArray", feature = "NSData"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[optional]
        #[method_id(@__retain_semantics Other authenticationDataForComponents:)]
        unsafe fn authenticationDataForComponents(&self, components: &NSArray) -> Retained<NSData>;

        #[cfg(all(feature = "NSArray", feature = "NSData"))]
        #[deprecated = "Use NSXPCConnection instead"]
        #[optional]
        #[method(authenticateComponents:withData:)]
        unsafe fn authenticateComponents_withData(
            &self,
            components: &NSArray,
            signature: &NSData,
        ) -> bool;

        #[deprecated = "Use NSXPCConnection instead"]
        #[optional]
        #[method_id(@__retain_semantics Other createConversationForConnection:)]
        unsafe fn createConversationForConnection(
            &self,
            conn: &NSConnection,
        ) -> Retained<AnyObject>;

        #[deprecated = "Use NSXPCConnection instead"]
        #[optional]
        #[method(connection:handleRequest:)]
        unsafe fn connection_handleRequest(
            &self,
            connection: &NSConnection,
            doreq: &NSDistantObjectRequest,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn NSConnectionDelegate {}
);

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsfailedauthenticationexception?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSFailedAuthenticationException: &'static NSString;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsconnectiondidinitializenotification?language=objc)
    #[cfg(feature = "NSString")]
    pub static NSConnectionDidInitializeNotification: &'static NSString;
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/foundation/nsdistantobjectrequest?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use NSXPCConnection instead"]
    pub struct NSDistantObjectRequest;
);

unsafe impl NSObjectProtocol for NSDistantObjectRequest {}

extern_methods!(
    unsafe impl NSDistantObjectRequest {
        #[cfg(feature = "NSInvocation")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other invocation)]
        pub unsafe fn invocation(&self) -> Retained<NSInvocation>;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other connection)]
        pub unsafe fn connection(&self) -> Retained<NSConnection>;

        #[deprecated = "Use NSXPCConnection instead"]
        #[method_id(@__retain_semantics Other conversation)]
        pub unsafe fn conversation(&self) -> Retained<AnyObject>;

        #[cfg(feature = "NSException")]
        #[deprecated = "Use NSXPCConnection instead"]
        #[method(replyWithException:)]
        pub unsafe fn replyWithException(&self, exception: Option<&NSException>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSDistantObjectRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
