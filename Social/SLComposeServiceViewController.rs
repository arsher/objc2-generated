//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
use objc2_app_kit::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    pub struct SLComposeServiceViewController;

    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl ClassType for SLComposeServiceViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSCoding for SLComposeServiceViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSEditor for SLComposeServiceViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSObjectProtocol for SLComposeServiceViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSSeguePerforming for SLComposeServiceViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSTextDelegate for SLComposeServiceViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSTextViewDelegate for SLComposeServiceViewController {}

#[cfg(feature = "objc2-app-kit")]
#[cfg(target_os = "macos")]
unsafe impl NSUserInterfaceItemIdentification for SLComposeServiceViewController {}

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl SLComposeServiceViewController {
        #[method(presentationAnimationDidFinish)]
        pub unsafe fn presentationAnimationDidFinish(&self);

        #[method_id(@__retain_semantics Other textView)]
        pub unsafe fn textView(&self) -> Option<Retained<NSTextView>>;

        #[method_id(@__retain_semantics Other contentText)]
        pub unsafe fn contentText(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other placeholder)]
        pub unsafe fn placeholder(&self) -> Retained<NSString>;

        #[method(setPlaceholder:)]
        pub unsafe fn setPlaceholder(&self, placeholder: Option<&NSString>);

        #[method(didSelectPost)]
        pub unsafe fn didSelectPost(&self);

        #[method(didSelectCancel)]
        pub unsafe fn didSelectCancel(&self);

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(isContentValid)]
        pub unsafe fn isContentValid(&self) -> bool;

        #[method(validateContent)]
        pub unsafe fn validateContent(&self);

        #[method_id(@__retain_semantics Other charactersRemaining)]
        pub unsafe fn charactersRemaining(&self) -> Option<Retained<NSNumber>>;

        #[method(setCharactersRemaining:)]
        pub unsafe fn setCharactersRemaining(&self, characters_remaining: Option<&NSNumber>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl SLComposeServiceViewController {
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl SLComposeServiceViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-app-kit")]
    #[cfg(target_os = "macos")]
    unsafe impl SLComposeServiceViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
