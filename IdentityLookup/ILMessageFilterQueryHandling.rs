//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    /// Functionality related to MessageFilter extension query requests.
    ///
    /// Subclasses of ILMessageFilterExtension which support querying must conform to this protocol.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/identitylookup/ilmessagefilterqueryhandling?language=objc)
    pub unsafe trait ILMessageFilterQueryHandling: NSObjectProtocol {
        #[cfg(all(
            feature = "ILMessageFilterExtensionContext",
            feature = "ILMessageFilterQueryRequest",
            feature = "ILMessageFilterQueryResponse",
            feature = "block2"
        ))]
        /// Evaluate a query request and provide a response describing how the system should handle the message it represents.
        ///
        /// May include either (or both) of the following:
        ///
        /// - Using offline/stored information to form a response about the message described by the query request.
        ///
        /// - Deferring the query request to a network service associated with the app extension, allowing the network service to
        /// return data back to extension in order to form a response about the message. The `context` parameter provides an API which
        /// allows deferring a request to the network and receiving the response to that network request.
        ///
        /// Block specified in `completion` parameter must be invoked with a response describing how to handle the message, and may be
        /// invoked asynchronously.
        ///
        ///
        /// Parameter `queryRequest`: A query request to be handled which describes a received message.
        ///
        /// Parameter `context`: Extension context which offers API to defer request to network if necessary.
        ///
        /// Parameter `completion`: Completion block for returning a response.
        #[method(handleQueryRequest:context:completion:)]
        unsafe fn handleQueryRequest_context_completion(
            &self,
            query_request: &ILMessageFilterQueryRequest,
            context: &ILMessageFilterExtensionContext,
            completion: &block2::Block<dyn Fn(NonNull<ILMessageFilterQueryResponse>)>,
        );
    }

    unsafe impl ProtocolType for dyn ILMessageFilterQueryHandling {}
);
