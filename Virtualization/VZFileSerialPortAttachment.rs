//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZSerialPortAttachment")]
    pub struct VZFileSerialPortAttachment;

    #[cfg(feature = "VZSerialPortAttachment")]
    unsafe impl ClassType for VZFileSerialPortAttachment {
        #[inherits(NSObject)]
        type Super = VZSerialPortAttachment;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "VZSerialPortAttachment")]
unsafe impl NSObjectProtocol for VZFileSerialPortAttachment {}

extern_methods!(
    #[cfg(feature = "VZSerialPortAttachment")]
    unsafe impl VZFileSerialPortAttachment {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithURL:append:error:_)]
        pub unsafe fn initWithURL_append_error(
            this: Allocated<Self>,
            url: &NSURL,
            should_append: bool,
        ) -> Result<Retained<Self>, Retained<NSError>>;

        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Retained<NSURL>;

        #[method(append)]
        pub unsafe fn append(&self) -> bool;
    }
);
