//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSNibName = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSNib")]
    pub struct NSNib;

    #[cfg(feature = "AppKit_NSNib")]
    unsafe impl ClassType for NSNib {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSNib")]
unsafe impl NSCoding for NSNib {}

#[cfg(feature = "AppKit_NSNib")]
unsafe impl NSObjectProtocol for NSNib {}

extern_methods!(
    #[cfg(feature = "AppKit_NSNib")]
    unsafe impl NSNib {
        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Init initWithNibNamed:bundle:)]
        pub unsafe fn initWithNibNamed_bundle(
            this: Option<Allocated<Self>>,
            nib_name: &NSNibName,
            bundle: Option<&NSBundle>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(feature = "Foundation_NSBundle", feature = "Foundation_NSData"))]
        #[method_id(@__retain_semantics Init initWithNibData:bundle:)]
        pub unsafe fn initWithNibData_bundle(
            this: Option<Allocated<Self>>,
            nib_data: &NSData,
            bundle: Option<&NSBundle>,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSNib")]
    unsafe impl NSNib {
        #[cfg(feature = "Foundation_NSURL")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            nib_file_url: Option<&NSURL>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[deprecated]
        #[method(instantiateNibWithExternalNameTable:)]
        pub unsafe fn instantiateNibWithExternalNameTable(
            &self,
            external_name_table: Option<&NSDictionary>,
        ) -> bool;

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated]
        #[method(instantiateNibWithOwner:topLevelObjects:)]
        pub unsafe fn instantiateNibWithOwner_topLevelObjects(
            &self,
            owner: Option<&Object>,
            top_level_objects: *mut *mut NSArray,
        ) -> bool;
    }
);

extern_static!(NSNibOwner: &'static NSString);

extern_static!(NSNibTopLevelObjects: &'static NSString);
