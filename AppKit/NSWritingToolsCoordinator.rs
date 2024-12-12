//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
#[cfg(feature = "objc2-core-graphics")]
#[cfg(target_vendor = "apple")]
use objc2_core_graphics::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nswritingtoolscoordinatortextupdatereason?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSWritingToolsCoordinatorTextUpdateReason(pub NSInteger);
impl NSWritingToolsCoordinatorTextUpdateReason {
    #[doc(alias = "NSWritingToolsCoordinatorTextUpdateReasonTyping")]
    pub const Typing: Self = Self(0);
    #[doc(alias = "NSWritingToolsCoordinatorTextUpdateReasonUndoRedo")]
    pub const UndoRedo: Self = Self(1);
}

unsafe impl Encode for NSWritingToolsCoordinatorTextUpdateReason {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSWritingToolsCoordinatorTextUpdateReason {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nswritingtoolscoordinatorstate?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSWritingToolsCoordinatorState(pub NSInteger);
impl NSWritingToolsCoordinatorState {
    #[doc(alias = "NSWritingToolsCoordinatorStateInactive")]
    pub const Inactive: Self = Self(0);
    #[doc(alias = "NSWritingToolsCoordinatorStateNoninteractive")]
    pub const Noninteractive: Self = Self(1);
    #[doc(alias = "NSWritingToolsCoordinatorStateInteractiveResting")]
    pub const InteractiveResting: Self = Self(2);
    #[doc(alias = "NSWritingToolsCoordinatorStateInteractiveStreaming")]
    pub const InteractiveStreaming: Self = Self(3);
}

unsafe impl Encode for NSWritingToolsCoordinatorState {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSWritingToolsCoordinatorState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nswritingtoolscoordinatortextreplacementreason?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSWritingToolsCoordinatorTextReplacementReason(pub NSInteger);
impl NSWritingToolsCoordinatorTextReplacementReason {
    #[doc(alias = "NSWritingToolsCoordinatorTextReplacementReasonInteractive")]
    pub const Interactive: Self = Self(0);
    #[doc(alias = "NSWritingToolsCoordinatorTextReplacementReasonNoninteractive")]
    pub const Noninteractive: Self = Self(1);
}

unsafe impl Encode for NSWritingToolsCoordinatorTextReplacementReason {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSWritingToolsCoordinatorTextReplacementReason {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nswritingtoolscoordinatorcontextscope?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSWritingToolsCoordinatorContextScope(pub NSInteger);
impl NSWritingToolsCoordinatorContextScope {
    #[doc(alias = "NSWritingToolsCoordinatorContextScopeUserSelection")]
    pub const UserSelection: Self = Self(0);
    #[doc(alias = "NSWritingToolsCoordinatorContextScopeFullDocument")]
    pub const FullDocument: Self = Self(1);
    #[doc(alias = "NSWritingToolsCoordinatorContextScopeVisibleArea")]
    pub const VisibleArea: Self = Self(2);
}

unsafe impl Encode for NSWritingToolsCoordinatorContextScope {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSWritingToolsCoordinatorContextScope {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nswritingtoolscoordinatortextanimation?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSWritingToolsCoordinatorTextAnimation(pub NSInteger);
impl NSWritingToolsCoordinatorTextAnimation {
    #[doc(alias = "NSWritingToolsCoordinatorTextAnimationAnticipate")]
    pub const Anticipate: Self = Self(0);
    #[doc(alias = "NSWritingToolsCoordinatorTextAnimationRemove")]
    pub const Remove: Self = Self(1);
    #[doc(alias = "NSWritingToolsCoordinatorTextAnimationInsert")]
    pub const Insert: Self = Self(2);
    #[doc(alias = "NSWritingToolsCoordinatorTextAnimationAnticipateInactive")]
    pub const AnticipateInactive: Self = Self(8);
    #[doc(alias = "NSWritingToolsCoordinatorTextAnimationTranslate")]
    pub const Translate: Self = Self(9);
}

unsafe impl Encode for NSWritingToolsCoordinatorTextAnimation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for NSWritingToolsCoordinatorTextAnimation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nswritingtoolscoordinator?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSWritingToolsCoordinator;
);

unsafe impl NSObjectProtocol for NSWritingToolsCoordinator {}

extern_methods!(
    unsafe impl NSWritingToolsCoordinator {
        #[method(isWritingToolsAvailable)]
        pub unsafe fn isWritingToolsAvailable(mtm: MainThreadMarker) -> bool;

        #[method_id(@__retain_semantics Init initWithDelegate:)]
        pub unsafe fn initWithDelegate(
            this: Allocated<Self>,
            delegate: Option<&ProtocolObject<dyn NSWritingToolsCoordinatorDelegate>>,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSWritingToolsCoordinatorDelegate>>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Retained<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other effectContainerView)]
        pub unsafe fn effectContainerView(&self) -> Option<Retained<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setEffectContainerView:)]
        pub unsafe fn setEffectContainerView(&self, effect_container_view: Option<&NSView>);

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        #[method_id(@__retain_semantics Other decorationContainerView)]
        pub unsafe fn decorationContainerView(&self) -> Option<Retained<NSView>>;

        #[cfg(all(feature = "NSResponder", feature = "NSView"))]
        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDecorationContainerView:)]
        pub unsafe fn setDecorationContainerView(&self, decoration_container_view: Option<&NSView>);

        #[method(state)]
        pub unsafe fn state(&self) -> NSWritingToolsCoordinatorState;

        #[method(stopWritingTools)]
        pub unsafe fn stopWritingTools(&self);

        #[cfg(feature = "NSTextCheckingClient")]
        #[method(preferredBehavior)]
        pub unsafe fn preferredBehavior(&self) -> NSWritingToolsBehavior;

        #[cfg(feature = "NSTextCheckingClient")]
        #[method(setPreferredBehavior:)]
        pub unsafe fn setPreferredBehavior(&self, preferred_behavior: NSWritingToolsBehavior);

        #[cfg(feature = "NSTextCheckingClient")]
        #[method(behavior)]
        pub unsafe fn behavior(&self) -> NSWritingToolsBehavior;

        #[cfg(feature = "NSTextCheckingClient")]
        #[method(preferredResultOptions)]
        pub unsafe fn preferredResultOptions(&self) -> NSWritingToolsResultOptions;

        #[cfg(feature = "NSTextCheckingClient")]
        #[method(setPreferredResultOptions:)]
        pub unsafe fn setPreferredResultOptions(
            &self,
            preferred_result_options: NSWritingToolsResultOptions,
        );

        #[cfg(feature = "NSTextCheckingClient")]
        #[method(resultOptions)]
        pub unsafe fn resultOptions(&self) -> NSWritingToolsResultOptions;

        #[method(updateRange:withText:reason:forContextWithIdentifier:)]
        pub unsafe fn updateRange_withText_reason_forContextWithIdentifier(
            &self,
            range: NSRange,
            replacement_text: &NSAttributedString,
            reason: NSWritingToolsCoordinatorTextUpdateReason,
            context_id: &NSUUID,
        );

        #[method(updateForReflowedTextInContextWithIdentifier:)]
        pub unsafe fn updateForReflowedTextInContextWithIdentifier(&self, context_id: &NSUUID);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSWritingToolsCoordinator {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nswritingtoolscoordinatordelegate?language=objc)
    pub unsafe trait NSWritingToolsCoordinatorDelegate: NSObjectProtocol {
        #[cfg(all(feature = "NSWritingToolsCoordinatorContext", feature = "block2"))]
        #[method(writingToolsCoordinator:requestsContextsForScope:completion:)]
        unsafe fn writingToolsCoordinator_requestsContextsForScope_completion(
            &self,
            writing_tools_coordinator: &NSWritingToolsCoordinator,
            scope: NSWritingToolsCoordinatorContextScope,
            completion: &block2::Block<dyn Fn(NonNull<NSArray<NSWritingToolsCoordinatorContext>>)>,
        );

        #[cfg(all(
            feature = "NSWritingToolsCoordinatorAnimationParameters",
            feature = "NSWritingToolsCoordinatorContext",
            feature = "block2"
        ))]
        #[method(writingToolsCoordinator:replaceRange:inContext:proposedText:reason:animationParameters:completion:)]
        unsafe fn writingToolsCoordinator_replaceRange_inContext_proposedText_reason_animationParameters_completion(
            &self,
            writing_tools_coordinator: &NSWritingToolsCoordinator,
            range: NSRange,
            context: &NSWritingToolsCoordinatorContext,
            replacement_text: &NSAttributedString,
            reason: NSWritingToolsCoordinatorTextReplacementReason,
            animation_parameters: Option<&NSWritingToolsCoordinatorAnimationParameters>,
            completion: &block2::Block<dyn Fn(*mut NSAttributedString)>,
        );

        #[cfg(all(feature = "NSWritingToolsCoordinatorContext", feature = "block2"))]
        #[method(writingToolsCoordinator:selectRanges:inContext:completion:)]
        unsafe fn writingToolsCoordinator_selectRanges_inContext_completion(
            &self,
            writing_tools_coordinator: &NSWritingToolsCoordinator,
            ranges: &NSArray<NSValue>,
            context: &NSWritingToolsCoordinatorContext,
            completion: &block2::Block<dyn Fn()>,
        );

        #[cfg(all(feature = "block2", feature = "objc2-core-foundation"))]
        #[method(writingToolsCoordinator:requestsRangeInContextWithIdentifierForPoint:completion:)]
        unsafe fn writingToolsCoordinator_requestsRangeInContextWithIdentifierForPoint_completion(
            &self,
            writing_tools_coordinator: &NSWritingToolsCoordinator,
            point: NSPoint,
            completion: &block2::Block<dyn Fn(NSRange, NonNull<NSUUID>)>,
        );

        #[cfg(all(
            feature = "NSBezierPath",
            feature = "NSWritingToolsCoordinatorContext",
            feature = "block2"
        ))]
        #[method(writingToolsCoordinator:requestsBoundingBezierPathsForRange:inContext:completion:)]
        unsafe fn writingToolsCoordinator_requestsBoundingBezierPathsForRange_inContext_completion(
            &self,
            writing_tools_coordinator: &NSWritingToolsCoordinator,
            range: NSRange,
            context: &NSWritingToolsCoordinatorContext,
            completion: &block2::Block<dyn Fn(NonNull<NSArray<NSBezierPath>>)>,
        );

        #[cfg(all(
            feature = "NSBezierPath",
            feature = "NSWritingToolsCoordinatorContext",
            feature = "block2"
        ))]
        #[method(writingToolsCoordinator:requestsUnderlinePathsForRange:inContext:completion:)]
        unsafe fn writingToolsCoordinator_requestsUnderlinePathsForRange_inContext_completion(
            &self,
            writing_tools_coordinator: &NSWritingToolsCoordinator,
            range: NSRange,
            context: &NSWritingToolsCoordinatorContext,
            completion: &block2::Block<dyn Fn(NonNull<NSArray<NSBezierPath>>)>,
        );

        #[cfg(all(feature = "NSWritingToolsCoordinatorContext", feature = "block2"))]
        #[method(writingToolsCoordinator:prepareForTextAnimation:forRange:inContext:completion:)]
        unsafe fn writingToolsCoordinator_prepareForTextAnimation_forRange_inContext_completion(
            &self,
            writing_tools_coordinator: &NSWritingToolsCoordinator,
            text_animation: NSWritingToolsCoordinatorTextAnimation,
            range: NSRange,
            context: &NSWritingToolsCoordinatorContext,
            completion: &block2::Block<dyn Fn()>,
        );

        #[cfg(all(feature = "NSWritingToolsCoordinatorContext", feature = "block2"))]
        #[method(writingToolsCoordinator:requestsPreviewForTextAnimation:ofRange:inContext:completion:)]
        unsafe fn writingToolsCoordinator_requestsPreviewForTextAnimation_ofRange_inContext_completion(
            &self,
            writing_tools_coordinator: &NSWritingToolsCoordinator,
            text_animation: NSWritingToolsCoordinatorTextAnimation,
            range: NSRange,
            context: &NSWritingToolsCoordinatorContext,
            completion: &block2::Block<dyn Fn(*mut NSArray<NSTextPreview>)>,
        );

        #[cfg(all(
            feature = "NSWritingToolsCoordinatorContext",
            feature = "block2",
            feature = "objc2-core-foundation"
        ))]
        #[method(writingToolsCoordinator:requestsPreviewForRect:inContext:completion:)]
        unsafe fn writingToolsCoordinator_requestsPreviewForRect_inContext_completion(
            &self,
            writing_tools_coordinator: &NSWritingToolsCoordinator,
            rect: NSRect,
            context: &NSWritingToolsCoordinatorContext,
            completion: &block2::Block<dyn Fn(*mut NSTextPreview)>,
        );

        #[cfg(all(feature = "NSWritingToolsCoordinatorContext", feature = "block2"))]
        #[method(writingToolsCoordinator:finishTextAnimation:forRange:inContext:completion:)]
        unsafe fn writingToolsCoordinator_finishTextAnimation_forRange_inContext_completion(
            &self,
            writing_tools_coordinator: &NSWritingToolsCoordinator,
            text_animation: NSWritingToolsCoordinatorTextAnimation,
            range: NSRange,
            context: &NSWritingToolsCoordinatorContext,
            completion: &block2::Block<dyn Fn()>,
        );

        #[cfg(all(feature = "NSWritingToolsCoordinatorContext", feature = "block2"))]
        #[optional]
        #[method(writingToolsCoordinator:requestsSingleContainerSubrangesOfRange:inContext:completion:)]
        unsafe fn writingToolsCoordinator_requestsSingleContainerSubrangesOfRange_inContext_completion(
            &self,
            writing_tools_coordinator: &NSWritingToolsCoordinator,
            range: NSRange,
            context: &NSWritingToolsCoordinatorContext,
            completion: &block2::Block<dyn Fn(NonNull<NSArray<NSValue>>)>,
        );

        #[cfg(all(
            feature = "NSResponder",
            feature = "NSView",
            feature = "NSWritingToolsCoordinatorContext",
            feature = "block2"
        ))]
        #[optional]
        #[method(writingToolsCoordinator:requestsDecorationContainerViewForRange:inContext:completion:)]
        unsafe fn writingToolsCoordinator_requestsDecorationContainerViewForRange_inContext_completion(
            &self,
            writing_tools_coordinator: &NSWritingToolsCoordinator,
            range: NSRange,
            context: &NSWritingToolsCoordinatorContext,
            completion: &block2::Block<dyn Fn(NonNull<NSView>)>,
        );

        #[cfg(feature = "block2")]
        #[optional]
        #[method(writingToolsCoordinator:willChangeToState:completion:)]
        unsafe fn writingToolsCoordinator_willChangeToState_completion(
            &self,
            writing_tools_coordinator: &NSWritingToolsCoordinator,
            new_state: NSWritingToolsCoordinatorState,
            completion: &block2::Block<dyn Fn()>,
        );
    }

    unsafe impl ProtocolType for dyn NSWritingToolsCoordinatorDelegate {}
);

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextpreview?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextPreview;
);

unsafe impl NSObjectProtocol for NSTextPreview {}

extern_methods!(
    unsafe impl NSTextPreview {
        #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-core-graphics"))]
        #[cfg(target_vendor = "apple")]
        #[method_id(@__retain_semantics Init initWithSnapshotImage:presentationFrame:candidateRects:)]
        pub unsafe fn initWithSnapshotImage_presentationFrame_candidateRects(
            this: Allocated<Self>,
            snapshot_image: CGImageRef,
            presentation_frame: NSRect,
            candidate_rects: &NSArray<NSValue>,
        ) -> Retained<Self>;

        #[cfg(all(feature = "objc2-core-foundation", feature = "objc2-core-graphics"))]
        #[cfg(target_vendor = "apple")]
        #[method_id(@__retain_semantics Init initWithSnapshotImage:presentationFrame:)]
        pub unsafe fn initWithSnapshotImage_presentationFrame(
            this: Allocated<Self>,
            snapshot_image: CGImageRef,
            presentation_frame: NSRect,
        ) -> Retained<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[cfg(feature = "objc2-core-graphics")]
        #[cfg(target_vendor = "apple")]
        #[method(previewImage)]
        pub unsafe fn previewImage(&self) -> CGImageRef;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(presentationFrame)]
        pub unsafe fn presentationFrame(&self) -> NSRect;

        #[method_id(@__retain_semantics Other candidateRects)]
        pub unsafe fn candidateRects(&self) -> Retained<NSArray<NSValue>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextPreview {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
