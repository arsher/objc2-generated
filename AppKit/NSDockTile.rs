//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_static!(NSAppKitVersionNumberWithDockTilePlugInSupport: NSAppKitVersion = 1001.0);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSDockTile")]
    pub struct NSDockTile;

    #[cfg(feature = "AppKit_NSDockTile")]
    unsafe impl ClassType for NSDockTile {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSDockTile")]
unsafe impl NSObjectProtocol for NSDockTile {}

extern_methods!(
    #[cfg(feature = "AppKit_NSDockTile")]
    unsafe impl NSDockTile {
        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other contentView)]
        pub unsafe fn contentView(&self) -> Option<Id<NSView, Shared>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setContentView:)]
        pub unsafe fn setContentView(&self, content_view: Option<&NSView>);

        #[method(display)]
        pub unsafe fn display(&self);

        #[method(showsApplicationBadge)]
        pub unsafe fn showsApplicationBadge(&self) -> bool;

        #[method(setShowsApplicationBadge:)]
        pub unsafe fn setShowsApplicationBadge(&self, shows_application_badge: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other badgeLabel)]
        pub unsafe fn badgeLabel(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setBadgeLabel:)]
        pub unsafe fn setBadgeLabel(&self, badge_label: Option<&NSString>);

        #[method_id(@__retain_semantics Other owner)]
        pub unsafe fn owner(&self) -> Option<Id<Object, Shared>>;
    }
);

extern_protocol!(
    pub unsafe trait NSDockTilePlugIn: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSDockTile")]
        #[method(setDockTile:)]
        unsafe fn setDockTile(&self, dock_tile: Option<&NSDockTile>);

        #[cfg(feature = "AppKit_NSMenu")]
        #[optional]
        #[method_id(@__retain_semantics Other dockMenu)]
        unsafe fn dockMenu(&self) -> Option<Id<NSMenu, Shared>>;
    }

    unsafe impl ProtocolType for dyn NSDockTilePlugIn {}
);
