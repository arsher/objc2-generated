//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEHotspotHelperCommandType(pub NSInteger);
impl NEHotspotHelperCommandType {
    pub const kNEHotspotHelperCommandTypeNone: Self = Self(0);
    pub const kNEHotspotHelperCommandTypeFilterScanList: Self = Self(1);
    pub const kNEHotspotHelperCommandTypeEvaluate: Self = Self(2);
    pub const kNEHotspotHelperCommandTypeAuthenticate: Self = Self(3);
    pub const kNEHotspotHelperCommandTypePresentUI: Self = Self(4);
    pub const kNEHotspotHelperCommandTypeMaintain: Self = Self(5);
    pub const kNEHotspotHelperCommandTypeLogoff: Self = Self(6);
}

unsafe impl Encode for NEHotspotHelperCommandType {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEHotspotHelperCommandType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEHotspotHelperResult(pub NSInteger);
impl NEHotspotHelperResult {
    pub const kNEHotspotHelperResultSuccess: Self = Self(0);
    pub const kNEHotspotHelperResultFailure: Self = Self(1);
    pub const kNEHotspotHelperResultUIRequired: Self = Self(2);
    pub const kNEHotspotHelperResultCommandNotRecognized: Self = Self(3);
    pub const kNEHotspotHelperResultAuthenticationRequired: Self = Self(4);
    pub const kNEHotspotHelperResultUnsupportedNetwork: Self = Self(5);
    pub const kNEHotspotHelperResultTemporaryFailure: Self = Self(6);
}

unsafe impl Encode for NEHotspotHelperResult {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEHotspotHelperResult {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NEHotspotHelperConfidence(pub NSInteger);
impl NEHotspotHelperConfidence {
    pub const kNEHotspotHelperConfidenceNone: Self = Self(0);
    pub const kNEHotspotHelperConfidenceLow: Self = Self(1);
    pub const kNEHotspotHelperConfidenceHigh: Self = Self(2);
}

unsafe impl Encode for NEHotspotHelperConfidence {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NEHotspotHelperConfidence {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_methods!(
    /// HotspotHelper
    #[cfg(feature = "NEHotspotNetwork")]
    unsafe impl NEHotspotNetwork {
        #[method(signalStrength)]
        pub unsafe fn signalStrength(&self) -> c_double;

        #[method(isSecure)]
        pub unsafe fn isSecure(&self) -> bool;

        #[method(didAutoJoin)]
        pub unsafe fn didAutoJoin(&self) -> bool;

        #[method(didJustJoin)]
        pub unsafe fn didJustJoin(&self) -> bool;

        #[method(isChosenHelper)]
        pub unsafe fn isChosenHelper(&self) -> bool;

        #[method(setConfidence:)]
        pub unsafe fn setConfidence(&self, confidence: NEHotspotHelperConfidence);

        #[method(setPassword:)]
        pub unsafe fn setPassword(&self, password: &NSString);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEHotspotHelperCommand;

    unsafe impl ClassType for NEHotspotHelperCommand {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NEHotspotHelperCommand {}

extern_methods!(
    unsafe impl NEHotspotHelperCommand {
        #[method(commandType)]
        pub unsafe fn commandType(&self) -> NEHotspotHelperCommandType;

        #[cfg(feature = "NEHotspotNetwork")]
        #[method_id(@__retain_semantics Other network)]
        pub unsafe fn network(&self) -> Option<Retained<NEHotspotNetwork>>;

        #[cfg(feature = "NEHotspotNetwork")]
        #[method_id(@__retain_semantics Other networkList)]
        pub unsafe fn networkList(&self) -> Option<Retained<NSArray<NEHotspotNetwork>>>;

        #[method_id(@__retain_semantics Other createResponse:)]
        pub unsafe fn createResponse(
            &self,
            result: NEHotspotHelperResult,
        ) -> Retained<NEHotspotHelperResponse>;

        #[cfg(all(feature = "NWEndpoint", feature = "NWTCPConnection"))]
        #[method_id(@__retain_semantics Other createTCPConnection:)]
        pub unsafe fn createTCPConnection(
            &self,
            endpoint: &NWEndpoint,
        ) -> Retained<NWTCPConnection>;

        #[cfg(all(feature = "NWEndpoint", feature = "NWUDPSession"))]
        #[method_id(@__retain_semantics Other createUDPSession:)]
        pub unsafe fn createUDPSession(&self, endpoint: &NWEndpoint) -> Retained<NWUDPSession>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEHotspotHelperCommand {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEHotspotHelperResponse;

    unsafe impl ClassType for NEHotspotHelperResponse {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NEHotspotHelperResponse {}

extern_methods!(
    unsafe impl NEHotspotHelperResponse {
        #[cfg(feature = "NEHotspotNetwork")]
        #[method(setNetwork:)]
        pub unsafe fn setNetwork(&self, network: &NEHotspotNetwork);

        #[cfg(feature = "NEHotspotNetwork")]
        #[method(setNetworkList:)]
        pub unsafe fn setNetworkList(&self, network_list: &NSArray<NEHotspotNetwork>);

        #[method(deliver)]
        pub unsafe fn deliver(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEHotspotHelperResponse {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

#[cfg(feature = "block2")]
pub type NEHotspotHelperHandler = *mut block2::Block<dyn Fn(NonNull<NEHotspotHelperCommand>)>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NEHotspotHelper;

    unsafe impl ClassType for NEHotspotHelper {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NEHotspotHelper {}

extern_methods!(
    unsafe impl NEHotspotHelper {
        #[cfg(feature = "NEHotspotNetwork")]
        #[method(logoff:)]
        pub unsafe fn logoff(network: &NEHotspotNetwork) -> bool;

        #[method_id(@__retain_semantics Other supportedNetworkInterfaces)]
        pub unsafe fn supportedNetworkInterfaces() -> Option<Retained<NSArray>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NEHotspotHelper {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_category!(
    /// Category "NEHotspotHelper" on [`NSMutableURLRequest`].
    #[doc(alias = "NEHotspotHelper")]
    pub unsafe trait NSMutableURLRequestNEHotspotHelper {
        #[method(bindToHotspotHelperCommand:)]
        unsafe fn bindToHotspotHelperCommand(&self, command: &NEHotspotHelperCommand);
    }

    unsafe impl NSMutableURLRequestNEHotspotHelper for NSMutableURLRequest {}
);
