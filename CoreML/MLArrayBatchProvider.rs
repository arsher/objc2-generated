//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MLArrayBatchProvider;

    unsafe impl ClassType for MLArrayBatchProvider {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MLBatchProvider")]
unsafe impl MLBatchProvider for MLArrayBatchProvider {}

unsafe impl NSObjectProtocol for MLArrayBatchProvider {}

extern_methods!(
    unsafe impl MLArrayBatchProvider {
        #[cfg(feature = "MLFeatureProvider")]
        #[method_id(@__retain_semantics Other array)]
        pub unsafe fn array(&self) -> Retained<NSArray<ProtocolObject<dyn MLFeatureProvider>>>;

        #[cfg(feature = "MLFeatureProvider")]
        #[method_id(@__retain_semantics Init initWithFeatureProviderArray:)]
        pub unsafe fn initWithFeatureProviderArray(
            this: Allocated<Self>,
            array: &NSArray<ProtocolObject<dyn MLFeatureProvider>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithDictionary:error:_)]
        pub unsafe fn initWithDictionary_error(
            this: Allocated<Self>,
            dictionary: &NSDictionary<NSString, NSArray>,
        ) -> Result<Retained<Self>, Retained<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MLArrayBatchProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
