//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;

use crate::*;

/// An object conforming to UISpringLoadedInteractionEffect uses UISpringLoadedInteractionEffectState
/// to style the interaction view for the current springloading state.
///
/// - UISpringLoadedInteractionEffectStateInactive: the view is not engaged in springloading and should be displayed with its default style.
/// - UISpringLoadedInteractionEffectStatePossible: the view may springload and should provide a visual cue to the user. The default effect highlights the view.
/// - UISpringLoadedInteractionEffectStateActivating: the view is about to springload. The default effect will briefly flash while in this state.
/// - UISpringLoadedInteractionEffectStateActivated: the view springloaded and the activation handler is called. The default effect will restore the view to its original appearance.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/uikit/uispringloadedinteractioneffectstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UISpringLoadedInteractionEffectState(pub NSInteger);
impl UISpringLoadedInteractionEffectState {
    #[doc(alias = "UISpringLoadedInteractionEffectStateInactive")]
    pub const Inactive: Self = Self(0);
    #[doc(alias = "UISpringLoadedInteractionEffectStatePossible")]
    pub const Possible: Self = Self(1);
    #[doc(alias = "UISpringLoadedInteractionEffectStateActivating")]
    pub const Activating: Self = Self(2);
    #[doc(alias = "UISpringLoadedInteractionEffectStateActivated")]
    pub const Activated: Self = Self(3);
}

unsafe impl Encode for UISpringLoadedInteractionEffectState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UISpringLoadedInteractionEffectState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uispringloadedinteraction?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UISpringLoadedInteraction;
);

unsafe impl NSObjectProtocol for UISpringLoadedInteraction {}

#[cfg(feature = "UIInteraction")]
unsafe impl UIInteraction for UISpringLoadedInteraction {}

extern_methods!(
    unsafe impl UISpringLoadedInteraction {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// The designated `UISpringLoadedInteraction` initializer.
        ///
        ///
        /// Parameter `interactionBehavior`: The interaction behavior object controlling the springloaded interaction activation. If nil, the default behavior will be used.
        ///
        /// Parameter `interactionEffect`: The interaction effect object styling the interaction's view. If nil, the default effect will be used.
        ///
        /// Parameter `handler`: The handler to be performed when springloading is activated.
        ///
        /// Returns: An initialized springloaded interaction object or `nil` if the springloaded interaction could not be initialized.
        #[method_id(@__retain_semantics Init initWithInteractionBehavior:interactionEffect:activationHandler:)]
        pub unsafe fn initWithInteractionBehavior_interactionEffect_activationHandler(
            this: Allocated<Self>,
            interaction_behavior: Option<&ProtocolObject<dyn UISpringLoadedInteractionBehavior>>,
            interaction_effect: Option<&ProtocolObject<dyn UISpringLoadedInteractionEffect>>,
            handler: &block2::Block<
                dyn Fn(
                    NonNull<UISpringLoadedInteraction>,
                    NonNull<ProtocolObject<dyn UISpringLoadedInteractionContext>>,
                ),
            >,
        ) -> Retained<Self>;

        #[cfg(feature = "block2")]
        /// A springloaded interaction with the default interaction behavior and effect.
        ///
        ///
        /// Parameter `handler`: The handler to be performed when springloading is activated.
        ///
        /// Returns: An initialized springloaded interaction object or `nil` if the springloaded interaction could not be initialized.
        #[method_id(@__retain_semantics Init initWithActivationHandler:)]
        pub unsafe fn initWithActivationHandler(
            this: Allocated<Self>,
            handler: &block2::Block<
                dyn Fn(
                    NonNull<UISpringLoadedInteraction>,
                    NonNull<ProtocolObject<dyn UISpringLoadedInteractionContext>>,
                ),
            >,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other interactionBehavior)]
        pub unsafe fn interactionBehavior(
            &self,
        ) -> Retained<ProtocolObject<dyn UISpringLoadedInteractionBehavior>>;

        #[method_id(@__retain_semantics Other interactionEffect)]
        pub unsafe fn interactionEffect(
            &self,
        ) -> Retained<ProtocolObject<dyn UISpringLoadedInteractionEffect>>;
    }
);

extern_protocol!(
    /// The interaction behavior of a `UISpringLoadedInteraction` object must adopt the `UISpringLoadedInteractionBehavior` protocol.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/uikit/uispringloadedinteractionbehavior?language=objc)
    pub unsafe trait UISpringLoadedInteractionBehavior:
        NSObjectProtocol + MainThreadOnly
    {
        /// Returns whether springloading should begin or continue for a given interaction.
        ///
        ///
        /// Parameter `interaction`: The springloaded interaction object requesting this information
        ///
        /// Parameter `context`: An object that provides information about the current drag.
        ///
        /// Returns: true if the interaction should begin or continue springloading.
        #[method(shouldAllowInteraction:withContext:)]
        unsafe fn shouldAllowInteraction_withContext(
            &self,
            interaction: &UISpringLoadedInteraction,
            context: &ProtocolObject<dyn UISpringLoadedInteractionContext>,
        ) -> bool;

        /// Informs the behavior that springloading for a given interaction was cancelled or activated.
        ///
        ///
        /// Parameter `interaction`: The springloaded interaction object providing this information.
        #[optional]
        #[method(interactionDidFinish:)]
        unsafe fn interactionDidFinish(&self, interaction: &UISpringLoadedInteraction);
    }

    unsafe impl ProtocolType for dyn UISpringLoadedInteractionBehavior {}
);

extern_protocol!(
    /// The interaction effect of a `UISpringLoadedInteraction` object must adopt the `UISpringLoadedInteractionEffect` protocol.
    /// It is responsible for styling the interaction view according to the current springloading state.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/uikit/uispringloadedinteractioneffect?language=objc)
    pub unsafe trait UISpringLoadedInteractionEffect:
        NSObjectProtocol + MainThreadOnly
    {
        /// Informs the effect that the springloading state changed.
        ///
        ///
        /// Parameter `interaction`: The springloaded interaction providing this information.
        ///
        /// Parameter `context`: An object that provides information about the current springloading state.
        #[method(interaction:didChangeWithContext:)]
        unsafe fn interaction_didChangeWithContext(
            &self,
            interaction: &UISpringLoadedInteraction,
            context: &ProtocolObject<dyn UISpringLoadedInteractionContext>,
        );
    }

    unsafe impl ProtocolType for dyn UISpringLoadedInteractionEffect {}
);

extern_protocol!(
    /// UISpringLoadedContext supplies information about the springloading state and current drag.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/uikit/uispringloadedinteractioncontext?language=objc)
    pub unsafe trait UISpringLoadedInteractionContext:
        NSObjectProtocol + MainThreadOnly
    {
        /// The state that describes the current springloading style.
        #[method(state)]
        unsafe fn state(&self) -> UISpringLoadedInteractionEffectState;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        /// The view to which the interaction effect is applied. Defaults to the interaction's view.
        #[method_id(@__retain_semantics Other targetView)]
        unsafe fn targetView(&self) -> Option<Retained<UIView>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        /// Setter for [`targetView`][Self::targetView].
        #[method(setTargetView:)]
        unsafe fn setTargetView(&self, target_view: Option<&UIView>);

        /// The `targetItem` allows to distinguish a region of the view on which the interaction is installed.
        /// It is convenient to set this property to a model object associated to `targetView`.
        #[method_id(@__retain_semantics Other targetItem)]
        unsafe fn targetItem(&self) -> Option<Retained<AnyObject>>;

        /// Setter for [`targetItem`][Self::targetItem].
        #[method(setTargetItem:)]
        unsafe fn setTargetItem(&self, target_item: Option<&AnyObject>);

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        /// Returns the point computed as the location of the current drag in a given view.
        ///
        ///
        /// Parameter `view`: A view on which the drag is taking place. Specify nil to indicate the window.
        ///
        /// Returns: A point in the local coordinate system of `view`.
        #[method(locationInView:)]
        unsafe fn locationInView(&self, view: Option<&UIView>) -> CGPoint;
    }

    unsafe impl ProtocolType for dyn UISpringLoadedInteractionContext {}
);
