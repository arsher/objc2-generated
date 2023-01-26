//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSScriptSuiteRegistry")]
    pub struct NSScriptSuiteRegistry;

    #[cfg(feature = "Foundation_NSScriptSuiteRegistry")]
    unsafe impl ClassType for NSScriptSuiteRegistry {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSScriptSuiteRegistry")]
unsafe impl NSObjectProtocol for NSScriptSuiteRegistry {}

extern_methods!(
    #[cfg(feature = "Foundation_NSScriptSuiteRegistry")]
    unsafe impl NSScriptSuiteRegistry {
        #[method_id(@__retain_semantics Other sharedScriptSuiteRegistry)]
        pub unsafe fn sharedScriptSuiteRegistry() -> Id<NSScriptSuiteRegistry, Shared>;

        #[method(setSharedScriptSuiteRegistry:)]
        pub unsafe fn setSharedScriptSuiteRegistry(registry: &NSScriptSuiteRegistry);

        #[cfg(feature = "Foundation_NSBundle")]
        #[method(loadSuitesFromBundle:)]
        pub unsafe fn loadSuitesFromBundle(&self, bundle: &NSBundle);

        #[cfg(all(feature = "Foundation_NSBundle", feature = "Foundation_NSDictionary"))]
        #[method(loadSuiteWithDictionary:fromBundle:)]
        pub unsafe fn loadSuiteWithDictionary_fromBundle(
            &self,
            suite_declaration: &NSDictionary,
            bundle: &NSBundle,
        );

        #[cfg(feature = "Foundation_NSScriptClassDescription")]
        #[method(registerClassDescription:)]
        pub unsafe fn registerClassDescription(&self, class_description: &NSScriptClassDescription);

        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method(registerCommandDescription:)]
        pub unsafe fn registerCommandDescription(
            &self,
            command_description: &NSScriptCommandDescription,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other suiteNames)]
        pub unsafe fn suiteNames(&self) -> Id<NSArray<NSString>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(appleEventCodeForSuite:)]
        pub unsafe fn appleEventCodeForSuite(&self, suite_name: &NSString) -> FourCharCode;

        #[cfg(all(feature = "Foundation_NSBundle", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other bundleForSuite:)]
        pub unsafe fn bundleForSuite(&self, suite_name: &NSString) -> Option<Id<NSBundle, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSScriptClassDescription",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other classDescriptionsInSuite:)]
        pub unsafe fn classDescriptionsInSuite(
            &self,
            suite_name: &NSString,
        ) -> Option<Id<NSDictionary<NSString, NSScriptClassDescription>, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSScriptCommandDescription",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other commandDescriptionsInSuite:)]
        pub unsafe fn commandDescriptionsInSuite(
            &self,
            suite_name: &NSString,
        ) -> Option<Id<NSDictionary<NSString, NSScriptCommandDescription>, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other suiteForAppleEventCode:)]
        pub unsafe fn suiteForAppleEventCode(
            &self,
            apple_event_code: FourCharCode,
        ) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSScriptClassDescription")]
        #[method_id(@__retain_semantics Other classDescriptionWithAppleEventCode:)]
        pub unsafe fn classDescriptionWithAppleEventCode(
            &self,
            apple_event_code: FourCharCode,
        ) -> Option<Id<NSScriptClassDescription, Shared>>;

        #[cfg(feature = "Foundation_NSScriptCommandDescription")]
        #[method_id(@__retain_semantics Other commandDescriptionWithAppleEventClass:andAppleEventCode:)]
        pub unsafe fn commandDescriptionWithAppleEventClass_andAppleEventCode(
            &self,
            apple_event_class_code: FourCharCode,
            apple_event_id_code: FourCharCode,
        ) -> Option<Id<NSScriptCommandDescription, Shared>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other aeteResource:)]
        pub unsafe fn aeteResource(&self, language_name: &NSString) -> Option<Id<NSData, Shared>>;
    }
);
