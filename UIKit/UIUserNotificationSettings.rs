//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_OPTIONS
#[deprecated = "Use UserNotifications Framework's UNAuthorizationOptions"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIUserNotificationType(pub NSUInteger);
bitflags::bitflags! {
    impl UIUserNotificationType: NSUInteger {
#[deprecated = "Use UserNotifications Framework's UNAuthorizationOptions"]
        #[doc(alias = "UIUserNotificationTypeNone")]
        const None = 0;
#[deprecated = "Use UserNotifications Framework's UNAuthorizationOptions"]
        #[doc(alias = "UIUserNotificationTypeBadge")]
        const Badge = 1<<0;
#[deprecated = "Use UserNotifications Framework's UNAuthorizationOptions"]
        #[doc(alias = "UIUserNotificationTypeSound")]
        const Sound = 1<<1;
#[deprecated = "Use UserNotifications Framework's UNAuthorizationOptions"]
        #[doc(alias = "UIUserNotificationTypeAlert")]
        const Alert = 1<<2;
    }
}

unsafe impl Encode for UIUserNotificationType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIUserNotificationType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[deprecated = "Use UserNotifications Framework's UNNotificationAction or UNTextInputNotificationAction"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIUserNotificationActionBehavior(pub NSUInteger);
impl UIUserNotificationActionBehavior {
    #[deprecated = "Use UserNotifications Framework's UNNotificationAction or UNTextInputNotificationAction"]
    #[doc(alias = "UIUserNotificationActionBehaviorDefault")]
    pub const Default: Self = Self(0);
    #[deprecated = "Use UserNotifications Framework's UNNotificationAction or UNTextInputNotificationAction"]
    #[doc(alias = "UIUserNotificationActionBehaviorTextInput")]
    pub const TextInput: Self = Self(1);
}

unsafe impl Encode for UIUserNotificationActionBehavior {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIUserNotificationActionBehavior {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[deprecated = "Use UserNotifications Framework's UNNotificationActionOptions"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIUserNotificationActivationMode(pub NSUInteger);
impl UIUserNotificationActivationMode {
    #[deprecated = "Use UserNotifications Framework's UNNotificationActionOptions"]
    #[doc(alias = "UIUserNotificationActivationModeForeground")]
    pub const Foreground: Self = Self(0);
    #[deprecated = "Use UserNotifications Framework's UNNotificationActionOptions"]
    #[doc(alias = "UIUserNotificationActivationModeBackground")]
    pub const Background: Self = Self(1);
}

unsafe impl Encode for UIUserNotificationActivationMode {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIUserNotificationActivationMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[deprecated = "Use UserNotifications Framework's -[UNNotificationCategory actions] or -[UNNotificationCategory minimalActions]"]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIUserNotificationActionContext(pub NSUInteger);
impl UIUserNotificationActionContext {
    #[deprecated = "Use UserNotifications Framework's -[UNNotificationCategory actions] or -[UNNotificationCategory minimalActions]"]
    #[doc(alias = "UIUserNotificationActionContextDefault")]
    pub const Default: Self = Self(0);
    #[deprecated = "Use UserNotifications Framework's -[UNNotificationCategory actions] or -[UNNotificationCategory minimalActions]"]
    #[doc(alias = "UIUserNotificationActionContextMinimal")]
    pub const Minimal: Self = Self(1);
}

unsafe impl Encode for UIUserNotificationActionContext {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIUserNotificationActionContext {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C" {
    pub static UIUserNotificationTextInputActionButtonTitleKey: &'static NSString;
}

extern "C" {
    pub static UIUserNotificationActionResponseTypedTextKey: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use UserNotifications Framework's UNNotificationSettings"]
    pub struct UIUserNotificationSettings;

    unsafe impl ClassType for UIUserNotificationSettings {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UIUserNotificationSettings {}

extern_methods!(
    unsafe impl UIUserNotificationSettings {
        #[deprecated = "Use UserNotifications Framework's UNNotificationSettings"]
        #[method_id(@__retain_semantics Other settingsForTypes:categories:)]
        pub unsafe fn settingsForTypes_categories(
            types: UIUserNotificationType,
            categories: Option<&NSSet<UIUserNotificationCategory>>,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[deprecated = "Use UserNotifications Framework's UNNotificationSettings"]
        #[method(types)]
        pub unsafe fn types(&self) -> UIUserNotificationType;

        #[deprecated = "Use UserNotifications Framework's UNNotificationSettings"]
        #[method_id(@__retain_semantics Other categories)]
        pub unsafe fn categories(&self) -> Option<Id<NSSet<UIUserNotificationCategory>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIUserNotificationSettings {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use UserNotifications Framework's UNNotificationCategory"]
    pub struct UIUserNotificationCategory;

    unsafe impl ClassType for UIUserNotificationCategory {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSCoding for UIUserNotificationCategory {}

unsafe impl NSCopying for UIUserNotificationCategory {}

unsafe impl NSMutableCopying for UIUserNotificationCategory {}

unsafe impl NSObjectProtocol for UIUserNotificationCategory {}

unsafe impl NSSecureCoding for UIUserNotificationCategory {}

extern_methods!(
    unsafe impl UIUserNotificationCategory {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other actionsForContext:)]
        pub unsafe fn actionsForContext(
            &self,
            context: UIUserNotificationActionContext,
        ) -> Option<Id<NSArray<UIUserNotificationAction>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIUserNotificationCategory {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use UserNotifications Framework's UNNotificationCategory"]
    pub struct UIMutableUserNotificationCategory;

    unsafe impl ClassType for UIMutableUserNotificationCategory {
        #[inherits(NSObject)]
        type Super = UIUserNotificationCategory;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSCoding for UIMutableUserNotificationCategory {}

unsafe impl NSCopying for UIMutableUserNotificationCategory {}

unsafe impl NSMutableCopying for UIMutableUserNotificationCategory {}

unsafe impl NSObjectProtocol for UIMutableUserNotificationCategory {}

unsafe impl NSSecureCoding for UIMutableUserNotificationCategory {}

extern_methods!(
    unsafe impl UIMutableUserNotificationCategory {
        #[deprecated = "Use UserNotifications Framework's UNNotificationCategory"]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[deprecated = "Use UserNotifications Framework's UNNotificationCategory"]
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);

        #[deprecated = "Use UserNotifications Framework's UNNotificationCategory"]
        #[method(setActions:forContext:)]
        pub unsafe fn setActions_forContext(
            &self,
            actions: Option<&NSArray<UIUserNotificationAction>>,
            context: UIUserNotificationActionContext,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `UIUserNotificationCategory`
    unsafe impl UIMutableUserNotificationCategory {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIMutableUserNotificationCategory {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use UserNotifications Framework's UNNotificationAction"]
    pub struct UIUserNotificationAction;

    unsafe impl ClassType for UIUserNotificationAction {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSCoding for UIUserNotificationAction {}

unsafe impl NSCopying for UIUserNotificationAction {}

unsafe impl NSMutableCopying for UIUserNotificationAction {}

unsafe impl NSObjectProtocol for UIUserNotificationAction {}

unsafe impl NSSecureCoding for UIUserNotificationAction {}

extern_methods!(
    unsafe impl UIUserNotificationAction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[method(behavior)]
        pub unsafe fn behavior(&self) -> UIUserNotificationActionBehavior;

        #[method_id(@__retain_semantics Other parameters)]
        pub unsafe fn parameters(&self) -> Id<NSDictionary>;

        #[method(activationMode)]
        pub unsafe fn activationMode(&self) -> UIUserNotificationActivationMode;

        #[method(isAuthenticationRequired)]
        pub unsafe fn isAuthenticationRequired(&self) -> bool;

        #[method(isDestructive)]
        pub unsafe fn isDestructive(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIUserNotificationAction {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "Use UserNotifications Framework's UNNotificationAction"]
    pub struct UIMutableUserNotificationAction;

    unsafe impl ClassType for UIMutableUserNotificationAction {
        #[inherits(NSObject)]
        type Super = UIUserNotificationAction;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSCoding for UIMutableUserNotificationAction {}

unsafe impl NSCopying for UIMutableUserNotificationAction {}

unsafe impl NSMutableCopying for UIMutableUserNotificationAction {}

unsafe impl NSObjectProtocol for UIMutableUserNotificationAction {}

unsafe impl NSSecureCoding for UIMutableUserNotificationAction {}

extern_methods!(
    unsafe impl UIMutableUserNotificationAction {
        #[deprecated = "Use UserNotifications Framework's UNNotificationAction"]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[deprecated = "Use UserNotifications Framework's UNNotificationAction"]
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);

        #[deprecated = "Use UserNotifications Framework's UNNotificationAction"]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[deprecated = "Use UserNotifications Framework's UNNotificationAction"]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[method(behavior)]
        pub unsafe fn behavior(&self) -> UIUserNotificationActionBehavior;

        #[method(setBehavior:)]
        pub unsafe fn setBehavior(&self, behavior: UIUserNotificationActionBehavior);

        #[method_id(@__retain_semantics Other parameters)]
        pub unsafe fn parameters(&self) -> Id<NSDictionary>;

        #[method(setParameters:)]
        pub unsafe fn setParameters(&self, parameters: &NSDictionary);

        #[deprecated = "Use UserNotifications Framework's UNNotificationAction"]
        #[method(activationMode)]
        pub unsafe fn activationMode(&self) -> UIUserNotificationActivationMode;

        #[deprecated = "Use UserNotifications Framework's UNNotificationAction"]
        #[method(setActivationMode:)]
        pub unsafe fn setActivationMode(&self, activation_mode: UIUserNotificationActivationMode);

        #[deprecated = "Use UserNotifications Framework's UNNotificationAction"]
        #[method(isAuthenticationRequired)]
        pub unsafe fn isAuthenticationRequired(&self) -> bool;

        #[deprecated = "Use UserNotifications Framework's UNNotificationAction"]
        #[method(setAuthenticationRequired:)]
        pub unsafe fn setAuthenticationRequired(&self, authentication_required: bool);

        #[deprecated = "Use UserNotifications Framework's UNNotificationAction"]
        #[method(isDestructive)]
        pub unsafe fn isDestructive(&self) -> bool;

        #[deprecated = "Use UserNotifications Framework's UNNotificationAction"]
        #[method(setDestructive:)]
        pub unsafe fn setDestructive(&self, destructive: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `UIUserNotificationAction`
    unsafe impl UIMutableUserNotificationAction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIMutableUserNotificationAction {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
