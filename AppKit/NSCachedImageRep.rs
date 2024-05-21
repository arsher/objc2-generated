//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "NSImageRep")]
    #[deprecated]
    pub struct NSCachedImageRep;

    #[cfg(feature = "NSImageRep")]
    unsafe impl ClassType for NSCachedImageRep {
        #[inherits(NSObject)]
        type Super = NSImageRep;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "NSImageRep")]
unsafe impl NSCoding for NSCachedImageRep {}

#[cfg(feature = "NSImageRep")]
unsafe impl NSCopying for NSCachedImageRep {}

#[cfg(feature = "NSImageRep")]
unsafe impl NSObjectProtocol for NSCachedImageRep {}

extern_methods!(
    #[cfg(feature = "NSImageRep")]
    unsafe impl NSCachedImageRep {
        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithWindow:rect:)]
        pub unsafe fn initWithWindow_rect(
            this: Allocated<Self>,
            win: Option<&NSWindow>,
            rect: NSRect,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "NSGraphics")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithSize:depth:separate:alpha:)]
        pub unsafe fn initWithSize_depth_separate_alpha(
            this: Allocated<Self>,
            size: NSSize,
            depth: NSWindowDepth,
            flag: bool,
            alpha: bool,
        ) -> Option<Retained<Self>>;

        #[cfg(all(feature = "NSResponder", feature = "NSWindow"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other window)]
        pub unsafe fn window(&self, mtm: MainThreadMarker) -> Option<Retained<NSWindow>>;

        #[deprecated]
        #[method(rect)]
        pub unsafe fn rect(&self) -> NSRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSImageRep`
    #[cfg(feature = "NSImageRep")]
    unsafe impl NSCachedImageRep {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "NSImageRep")]
    unsafe impl NSCachedImageRep {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
