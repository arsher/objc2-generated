//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSColorListName = NSString;

pub type NSColorName = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSColorList")]
    pub struct NSColorList;

    #[cfg(feature = "AppKit_NSColorList")]
    unsafe impl ClassType for NSColorList {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSColorList")]
unsafe impl NSCoding for NSColorList {}

#[cfg(feature = "AppKit_NSColorList")]
unsafe impl NSObjectProtocol for NSColorList {}

#[cfg(feature = "AppKit_NSColorList")]
unsafe impl NSSecureCoding for NSColorList {}

extern_methods!(
    #[cfg(feature = "AppKit_NSColorList")]
    unsafe impl NSColorList {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other availableColorLists)]
        pub unsafe fn availableColorLists() -> Id<NSArray<NSColorList>, Shared>;

        #[method_id(@__retain_semantics Other colorListNamed:)]
        pub unsafe fn colorListNamed(name: &NSColorListName) -> Option<Id<NSColorList, Shared>>;

        #[method_id(@__retain_semantics Init initWithName:)]
        pub unsafe fn initWithName(
            this: Option<Allocated<Self>>,
            name: &NSColorListName,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithName:fromFile:)]
        pub unsafe fn initWithName_fromFile(
            this: Option<Allocated<Self>>,
            name: &NSColorListName,
            path: Option<&NSString>,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSColorListName, Shared>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setColor:forKey:)]
        pub unsafe fn setColor_forKey(&self, color: &NSColor, key: &NSColorName);

        #[cfg(feature = "AppKit_NSColor")]
        #[method(insertColor:key:atIndex:)]
        pub unsafe fn insertColor_key_atIndex(
            &self,
            color: &NSColor,
            key: &NSColorName,
            loc: NSUInteger,
        );

        #[method(removeColorWithKey:)]
        pub unsafe fn removeColorWithKey(&self, key: &NSColorName);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other colorWithKey:)]
        pub unsafe fn colorWithKey(&self, key: &NSColorName) -> Option<Id<NSColor, Shared>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other allKeys)]
        pub unsafe fn allKeys(&self) -> Id<NSArray<NSColorName>, Shared>;

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(writeToURL:error:_)]
        pub unsafe fn writeToURL_error(
            &self,
            url: Option<&NSURL>,
        ) -> Result<(), Id<NSError, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use -writeToURL:error: instead"]
        #[method(writeToFile:)]
        pub unsafe fn writeToFile(&self, path: Option<&NSString>) -> bool;

        #[method(removeFile)]
        pub unsafe fn removeFile(&self);
    }
);

extern_static!(NSColorListDidChangeNotification: &'static NSNotificationName);
