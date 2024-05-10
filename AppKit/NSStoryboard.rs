//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

pub type NSStoryboardName = NSString;

pub type NSStoryboardSceneIdentifier = NSString;

#[cfg(feature = "block2")]
pub type NSStoryboardControllerCreator =
    *mut block2::Block<dyn Fn(NonNull<NSCoder>) -> *mut AnyObject>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSStoryboard;

    unsafe impl ClassType for NSStoryboard {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSStoryboard {}

extern_methods!(
    unsafe impl NSStoryboard {
        #[method_id(@__retain_semantics Other mainStoryboard)]
        pub unsafe fn mainStoryboard() -> Option<Id<NSStoryboard>>;

        #[method_id(@__retain_semantics Other storyboardWithName:bundle:)]
        pub unsafe fn storyboardWithName_bundle(
            name: &NSStoryboardName,
            storyboard_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other instantiateInitialController)]
        pub unsafe fn instantiateInitialController(&self) -> Option<Id<AnyObject>>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other instantiateInitialControllerWithCreator:)]
        pub unsafe fn instantiateInitialControllerWithCreator(
            &self,
            block: NSStoryboardControllerCreator,
        ) -> Option<Id<AnyObject>>;

        #[method_id(@__retain_semantics Other instantiateControllerWithIdentifier:)]
        pub unsafe fn instantiateControllerWithIdentifier(
            &self,
            identifier: &NSStoryboardSceneIdentifier,
        ) -> Id<AnyObject>;

        #[cfg(feature = "block2")]
        #[method_id(@__retain_semantics Other instantiateControllerWithIdentifier:creator:)]
        pub unsafe fn instantiateControllerWithIdentifier_creator(
            &self,
            identifier: &NSStoryboardSceneIdentifier,
            block: NSStoryboardControllerCreator,
        ) -> Id<AnyObject>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSStoryboard {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
