//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::ClassKit::*;
use crate::Foundation::*;

extern_protocol!(
    pub struct CLSContextProvider;

    unsafe impl ProtocolType for CLSContextProvider {
        #[cfg(all(feature = "ClassKit_CLSContext", feature = "Foundation_NSError"))]
        #[method(updateDescendantsOfContext:completion:)]
        pub unsafe fn updateDescendantsOfContext_completion(
            &self,
            context: &CLSContext,
            completion: &Block<(*mut NSError,), ()>,
        );
    }
);