//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_protocol!(
    pub unsafe trait CLSContextProvider {
        #[cfg(all(feature = "CLSContext", feature = "CLSObject", feature = "block2"))]
        #[method(updateDescendantsOfContext:completion:)]
        unsafe fn updateDescendantsOfContext_completion(
            &self,
            context: &CLSContext,
            completion: &block2::Block<dyn Fn(*mut NSError)>,
        );
    }

    unsafe impl ProtocolType for dyn CLSContextProvider {}
);
