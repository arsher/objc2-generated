//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSUserDefaultsController")]
    pub struct NSUserDefaultsController;

    #[cfg(feature = "AppKit_NSUserDefaultsController")]
    unsafe impl ClassType for NSUserDefaultsController {
        #[inherits(NSObject)]
        type Super = NSController;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSUserDefaultsController")]
unsafe impl NSCoding for NSUserDefaultsController {}

#[cfg(feature = "AppKit_NSUserDefaultsController")]
unsafe impl NSEditor for NSUserDefaultsController {}

#[cfg(feature = "AppKit_NSUserDefaultsController")]
unsafe impl NSEditorRegistration for NSUserDefaultsController {}

#[cfg(feature = "AppKit_NSUserDefaultsController")]
unsafe impl NSObjectProtocol for NSUserDefaultsController {}

extern_methods!(
    #[cfg(feature = "AppKit_NSUserDefaultsController")]
    unsafe impl NSUserDefaultsController {
        #[method_id(@__retain_semantics Other sharedUserDefaultsController)]
        pub unsafe fn sharedUserDefaultsController() -> Id<NSUserDefaultsController>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSUserDefaults"
        ))]
        #[method_id(@__retain_semantics Init initWithDefaults:initialValues:)]
        pub unsafe fn initWithDefaults_initialValues(
            this: Option<Allocated<Self>>,
            defaults: Option<&NSUserDefaults>,
            initial_values: Option<&NSDictionary<NSString, Object>>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSUserDefaults")]
        #[method_id(@__retain_semantics Other defaults)]
        pub unsafe fn defaults(&self) -> Id<NSUserDefaults>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other initialValues)]
        pub unsafe fn initialValues(&self) -> Option<Id<NSDictionary<NSString, Object>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setInitialValues:)]
        pub unsafe fn setInitialValues(
            &self,
            initial_values: Option<&NSDictionary<NSString, Object>>,
        );

        #[method(appliesImmediately)]
        pub unsafe fn appliesImmediately(&self) -> bool;

        #[method(setAppliesImmediately:)]
        pub unsafe fn setAppliesImmediately(&self, applies_immediately: bool);

        #[method(hasUnappliedChanges)]
        pub unsafe fn hasUnappliedChanges(&self) -> bool;

        #[method_id(@__retain_semantics Other values)]
        pub unsafe fn values(&self) -> Id<Object>;

        #[method(revert:)]
        pub unsafe fn revert(&self, sender: Option<&Object>);

        #[method(save:)]
        pub unsafe fn save(&self, sender: Option<&Object>);

        #[method(revertToInitialValues:)]
        pub unsafe fn revertToInitialValues(&self, sender: Option<&Object>);
    }
);
