//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_enum!(
    #[underlying(c_uint)]
    #[deprecated]
    pub enum __anonymous__ {
        #[deprecated]
        DOM_DOM_DELTA_PIXEL = 0x00,
        #[deprecated]
        DOM_DOM_DELTA_LINE = 0x01,
        #[deprecated]
        DOM_DOM_DELTA_PAGE = 0x02,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMWheelEvent")]
    #[deprecated]
    pub struct DOMWheelEvent;

    #[cfg(feature = "WebKit_DOMWheelEvent")]
    unsafe impl ClassType for DOMWheelEvent {
        #[inherits(DOMUIEvent, DOMEvent, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMMouseEvent;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMWheelEvent")]
unsafe impl NSCopying for DOMWheelEvent {}

#[cfg(feature = "WebKit_DOMWheelEvent")]
unsafe impl NSObjectProtocol for DOMWheelEvent {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMWheelEvent")]
    unsafe impl DOMWheelEvent {
        #[method(wheelDeltaX)]
        pub unsafe fn wheelDeltaX(&self) -> c_int;

        #[method(wheelDeltaY)]
        pub unsafe fn wheelDeltaY(&self) -> c_int;

        #[method(wheelDelta)]
        pub unsafe fn wheelDelta(&self) -> c_int;

        #[method(isHorizontal)]
        pub unsafe fn isHorizontal(&self) -> bool;

        #[cfg(feature = "WebKit_DOMAbstractView")]
        #[method(initWheelEvent:wheelDeltaY:view:screenX:screenY:clientX:clientY:ctrlKey:altKey:shiftKey:metaKey:)]
        pub unsafe fn initWheelEvent_wheelDeltaY_view_screenX_screenY_clientX_clientY_ctrlKey_altKey_shiftKey_metaKey(
            &self,
            wheel_delta_x: c_int,
            wheel_delta_y: c_int,
            view: Option<&DOMAbstractView>,
            screen_x: c_int,
            screen_y: c_int,
            client_x: c_int,
            client_y: c_int,
            ctrl_key: bool,
            alt_key: bool,
            shift_key: bool,
            meta_key: bool,
        );
    }
);
