//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use objc2::__framework_prelude::*;

use crate::*;

extern_protocol!(
    /// The MTLParallelRenderCommandEncoder protocol is designed to allow a single render to texture operation to be efficiently (and safely) broken up across multiple threads.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder?language=objc)
    #[cfg(feature = "MTLCommandEncoder")]
    pub unsafe trait MTLParallelRenderCommandEncoder: MTLCommandEncoder {
        #[cfg(feature = "MTLRenderCommandEncoder")]
        /// Return a new autoreleased object that conforms to
        /// <MTLRenderCommandEncoder
        /// > that may be used to encode on a different thread.
        #[method_id(@__retain_semantics Other renderCommandEncoder)]
        fn renderCommandEncoder(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLRenderCommandEncoder>>>;

        #[cfg(feature = "MTLRenderPass")]
        /// If the the store action for a given color attachment was set to MTLStoreActionUnknown when the render command encoder was created,
        /// setColorStoreAction:atIndex: must be used to finalize the store action before endEncoding is called.
        ///
        /// Parameter `storeAction`: The desired store action for the given color attachment.  This may be set to any value other than MTLStoreActionUnknown.
        ///
        /// Parameter `colorAttachmentIndex`: The index of the color attachment
        #[method(setColorStoreAction:atIndex:)]
        unsafe fn setColorStoreAction_atIndex(
            &self,
            store_action: MTLStoreAction,
            color_attachment_index: NSUInteger,
        );

        #[cfg(feature = "MTLRenderPass")]
        /// If the the store action for the depth attachment was set to MTLStoreActionUnknown when the render command encoder was created,
        /// setDepthStoreAction: must be used to finalize the store action before endEncoding is called.
        #[method(setDepthStoreAction:)]
        unsafe fn setDepthStoreAction(&self, store_action: MTLStoreAction);

        #[cfg(feature = "MTLRenderPass")]
        /// If the the store action for the stencil attachment was set to MTLStoreActionUnknown when the render command encoder was created,
        /// setStencilStoreAction: must be used to finalize the store action before endEncoding is called.
        #[method(setStencilStoreAction:)]
        unsafe fn setStencilStoreAction(&self, store_action: MTLStoreAction);

        #[cfg(feature = "MTLRenderPass")]
        /// If the the store action for a given color attachment was set to MTLStoreActionUnknown when the render command encoder was created,
        /// setColorStoreActionOptions:atIndex: may be used to finalize the store action options before endEncoding is called.
        ///
        /// Parameter `storeActionOptions`: The desired store action options for the given color attachment.
        ///
        /// Parameter `colorAttachmentIndex`: The index of the color attachment
        #[method(setColorStoreActionOptions:atIndex:)]
        unsafe fn setColorStoreActionOptions_atIndex(
            &self,
            store_action_options: MTLStoreActionOptions,
            color_attachment_index: NSUInteger,
        );

        #[cfg(feature = "MTLRenderPass")]
        /// If the the store action for the depth attachment was set to MTLStoreActionUnknown when the render command encoder was created,
        /// setDepthStoreActionOptions: may be used to finalize the store action options before endEncoding is called.
        #[method(setDepthStoreActionOptions:)]
        unsafe fn setDepthStoreActionOptions(&self, store_action_options: MTLStoreActionOptions);

        #[cfg(feature = "MTLRenderPass")]
        /// If the the store action for the stencil attachment was set to MTLStoreActionUnknown when the render command encoder was created,
        /// setStencilStoreActionOptions: may be used to finalize the store action options before endEncoding is called.
        #[method(setStencilStoreActionOptions:)]
        unsafe fn setStencilStoreActionOptions(&self, store_action_options: MTLStoreActionOptions);
    }

    #[cfg(feature = "MTLCommandEncoder")]
    unsafe impl ProtocolType for dyn MTLParallelRenderCommandEncoder {}
);
