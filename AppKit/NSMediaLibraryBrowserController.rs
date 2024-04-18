//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSMediaLibrary(pub NSUInteger);
bitflags::bitflags! {
    impl NSMediaLibrary: NSUInteger {
        #[doc(alias = "NSMediaLibraryAudio")]
        const Audio = 1<<0;
        #[doc(alias = "NSMediaLibraryImage")]
        const Image = 1<<1;
        #[doc(alias = "NSMediaLibraryMovie")]
        const Movie = 1<<2;
    }
}

unsafe impl Encode for NSMediaLibrary {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSMediaLibrary {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSMediaLibraryBrowserController;

    unsafe impl ClassType for NSMediaLibraryBrowserController {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for NSMediaLibraryBrowserController {}

extern_methods!(
    unsafe impl NSMediaLibraryBrowserController {
        #[method_id(@__retain_semantics Other sharedMediaLibraryBrowserController)]
        pub unsafe fn sharedMediaLibraryBrowserController() -> Id<NSMediaLibraryBrowserController>;

        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;

        #[method(setVisible:)]
        pub unsafe fn setVisible(&self, visible: bool);

        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;

        #[method(setFrame:)]
        pub unsafe fn setFrame(&self, frame: NSRect);

        #[method(mediaLibraries)]
        pub unsafe fn mediaLibraries(&self) -> NSMediaLibrary;

        #[method(setMediaLibraries:)]
        pub unsafe fn setMediaLibraries(&self, media_libraries: NSMediaLibrary);

        #[method(togglePanel:)]
        pub unsafe fn togglePanel(&self, sender: Option<&AnyObject>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSMediaLibraryBrowserController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
