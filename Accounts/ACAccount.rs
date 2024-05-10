//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use appropriate non-Apple SDK corresponding to the type of account you want to reference instead"]
    pub struct ACAccount;

    unsafe impl ClassType for ACAccount {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for ACAccount {}

extern_methods!(
    unsafe impl ACAccount {
        #[cfg(feature = "ACAccountType")]
        #[deprecated = "Use appropriate non-Apple SDK corresponding to the type of account you want to reference instead"]
        #[method_id(@__retain_semantics Init initWithAccountType:)]
        pub unsafe fn initWithAccountType(
            this: Allocated<Self>,
            r#type: Option<&ACAccountType>,
        ) -> Option<Id<Self>>;

        #[deprecated = "Use appropriate non-Apple SDK corresponding to the type of account you want to reference instead"]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "ACAccountType")]
        #[deprecated = "Use appropriate non-Apple SDK corresponding to the type of account you want to reference instead"]
        #[method_id(@__retain_semantics Other accountType)]
        pub unsafe fn accountType(&self) -> Option<Id<ACAccountType>>;

        #[cfg(feature = "ACAccountType")]
        #[deprecated = "Use appropriate non-Apple SDK corresponding to the type of account you want to reference instead"]
        #[method(setAccountType:)]
        pub unsafe fn setAccountType(&self, account_type: Option<&ACAccountType>);

        #[deprecated = "Use appropriate non-Apple SDK corresponding to the type of account you want to reference instead"]
        #[method_id(@__retain_semantics Other accountDescription)]
        pub unsafe fn accountDescription(&self) -> Id<NSString>;

        #[deprecated = "Use appropriate non-Apple SDK corresponding to the type of account you want to reference instead"]
        #[method(setAccountDescription:)]
        pub unsafe fn setAccountDescription(&self, account_description: Option<&NSString>);

        #[deprecated = "Use appropriate non-Apple SDK corresponding to the type of account you want to reference instead"]
        #[method_id(@__retain_semantics Other username)]
        pub unsafe fn username(&self) -> Id<NSString>;

        #[deprecated = "Use appropriate non-Apple SDK corresponding to the type of account you want to reference instead"]
        #[method(setUsername:)]
        pub unsafe fn setUsername(&self, username: Option<&NSString>);

        #[method_id(@__retain_semantics Other userFullName)]
        pub unsafe fn userFullName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "ACAccountCredential")]
        #[deprecated = "Use appropriate non-Apple SDK corresponding to the type of account you want to reference instead"]
        #[method_id(@__retain_semantics Other credential)]
        pub unsafe fn credential(&self) -> Option<Id<ACAccountCredential>>;

        #[cfg(feature = "ACAccountCredential")]
        #[deprecated = "Use appropriate non-Apple SDK corresponding to the type of account you want to reference instead"]
        #[method(setCredential:)]
        pub unsafe fn setCredential(&self, credential: Option<&ACAccountCredential>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl ACAccount {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
