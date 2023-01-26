//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSStoryboardName = NSString;

pub type NSStoryboardSceneIdentifier = NSString;

pub type NSStoryboardControllerCreator = *mut Block<(NonNull<NSCoder>,), *mut Object>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSStoryboard")]
    pub struct NSStoryboard;

    #[cfg(feature = "AppKit_NSStoryboard")]
    unsafe impl ClassType for NSStoryboard {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSStoryboard")]
unsafe impl NSObjectProtocol for NSStoryboard {}

extern_methods!(
    #[cfg(feature = "AppKit_NSStoryboard")]
    unsafe impl NSStoryboard {
        #[method_id(@__retain_semantics Other mainStoryboard)]
        pub unsafe fn mainStoryboard() -> Option<Id<NSStoryboard, Shared>>;

        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Other storyboardWithName:bundle:)]
        pub unsafe fn storyboardWithName_bundle(
            name: &NSStoryboardName,
            storyboard_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other instantiateInitialController)]
        pub unsafe fn instantiateInitialController(&self) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other instantiateInitialControllerWithCreator:)]
        pub unsafe fn instantiateInitialControllerWithCreator(
            &self,
            block: NSStoryboardControllerCreator,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other instantiateControllerWithIdentifier:)]
        pub unsafe fn instantiateControllerWithIdentifier(
            &self,
            identifier: &NSStoryboardSceneIdentifier,
        ) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other instantiateControllerWithIdentifier:creator:)]
        pub unsafe fn instantiateControllerWithIdentifier_creator(
            &self,
            identifier: &NSStoryboardSceneIdentifier,
            block: NSStoryboardControllerCreator,
        ) -> Id<Object, Shared>;
    }
);
