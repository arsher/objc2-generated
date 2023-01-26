//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Automator::*;
use crate::Foundation::*;
use crate::OSAKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Automator_AMWorkspace")]
    pub struct AMWorkspace;

    #[cfg(feature = "Automator_AMWorkspace")]
    unsafe impl ClassType for AMWorkspace {
        type Super = NSObject;
    }
);

#[cfg(feature = "Automator_AMWorkspace")]
unsafe impl NSObjectProtocol for AMWorkspace {}

extern_methods!(
    #[cfg(feature = "Automator_AMWorkspace")]
    unsafe impl AMWorkspace {
        #[method_id(@__retain_semantics Other sharedWorkspace)]
        pub unsafe fn sharedWorkspace() -> Option<Id<AMWorkspace, Shared>>;
    }
);
