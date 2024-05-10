//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait ILMessageFilterQueryHandling: NSObjectProtocol {
        #[cfg(all(
            feature = "ILMessageFilterExtensionContext",
            feature = "ILMessageFilterQueryRequest",
            feature = "ILMessageFilterQueryResponse",
            feature = "block2"
        ))]
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
