//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextstorageeditactions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextStorageEditActions(pub NSUInteger);
bitflags::bitflags! {
    impl NSTextStorageEditActions: NSUInteger {
        const NSTextStorageEditedAttributes = 1<<0;
        const NSTextStorageEditedCharacters = 1<<1;
    }
}

unsafe impl Encode for NSTextStorageEditActions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSTextStorageEditActions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextstorage?language=objc)
    #[unsafe(super(NSMutableAttributedString, NSAttributedString, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextStorage;
);

unsafe impl NSCoding for NSTextStorage {}

unsafe impl NSObjectProtocol for NSTextStorage {}

unsafe impl NSSecureCoding for NSTextStorage {}

extern_methods!(
    unsafe impl NSTextStorage {
        #[cfg(feature = "NSLayoutManager")]
        /// ************************** Layout manager ***************************
        #[method_id(@__retain_semantics Other layoutManagers)]
        pub unsafe fn layoutManagers(&self) -> Retained<NSArray<NSLayoutManager>>;

        #[cfg(feature = "NSLayoutManager")]
        #[method(addLayoutManager:)]
        pub unsafe fn addLayoutManager(&self, a_layout_manager: &NSLayoutManager);

        #[cfg(feature = "NSLayoutManager")]
        #[method(removeLayoutManager:)]
        pub unsafe fn removeLayoutManager(&self, a_layout_manager: &NSLayoutManager);

        /// ************************** Pending edit info ***************************
        #[method(editedMask)]
        pub unsafe fn editedMask(&self) -> NSTextStorageEditActions;

        #[method(editedRange)]
        pub unsafe fn editedRange(&self) -> NSRange;

        #[method(changeInLength)]
        pub unsafe fn changeInLength(&self) -> NSInteger;

        /// ************************** Delegate ***************************
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSTextStorageDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`delegate`][Self::delegate].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSTextStorageDelegate>>,
        );

        /// ************************** Edit management ***************************
        #[method(edited:range:changeInLength:)]
        pub unsafe fn edited_range_changeInLength(
            &self,
            edited_mask: NSTextStorageEditActions,
            edited_range: NSRange,
            delta: NSInteger,
        );

        #[method(processEditing)]
        pub unsafe fn processEditing(&self);

        /// ************************** Attribute fixing ***************************
        #[method(fixesAttributesLazily)]
        pub unsafe fn fixesAttributesLazily(&self) -> bool;

        #[method(invalidateAttributesInRange:)]
        pub unsafe fn invalidateAttributesInRange(&self, range: NSRange);

        #[method(ensureAttributesAreFixedInRange:)]
        pub unsafe fn ensureAttributesAreFixedInRange(&self, range: NSRange);

        /// ************************** NSTextStorageObserving ***************************
        #[method_id(@__retain_semantics Other textStorageObserver)]
        pub unsafe fn textStorageObserver(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn NSTextStorageObserving>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        /// Setter for [`textStorageObserver`][Self::textStorageObserver].
        #[method(setTextStorageObserver:)]
        pub unsafe fn setTextStorageObserver(
            &self,
            text_storage_observer: Option<&ProtocolObject<dyn NSTextStorageObserving>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextStorage {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_protocol!(
    /// **  NSTextStorage delegate methods ***
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextstoragedelegate?language=objc)
    pub unsafe trait NSTextStorageDelegate: NSObjectProtocol {
        #[optional]
        #[method(textStorage:willProcessEditing:range:changeInLength:)]
        unsafe fn textStorage_willProcessEditing_range_changeInLength(
            &self,
            text_storage: &NSTextStorage,
            edited_mask: NSTextStorageEditActions,
            edited_range: NSRange,
            delta: NSInteger,
        );

        #[optional]
        #[method(textStorage:didProcessEditing:range:changeInLength:)]
        unsafe fn textStorage_didProcessEditing_range_changeInLength(
            &self,
            text_storage: &NSTextStorage,
            edited_mask: NSTextStorageEditActions,
            edited_range: NSRange,
            delta: NSInteger,
        );
    }

    unsafe impl ProtocolType for dyn NSTextStorageDelegate {}
);

extern "C" {
    /// ** Notifications ***
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextstoragewillprocesseditingnotification?language=objc)
    pub static NSTextStorageWillProcessEditingNotification: &'static NSNotificationName;
}

extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextstoragedidprocesseditingnotification?language=objc)
    pub static NSTextStorageDidProcessEditingNotification: &'static NSNotificationName;
}

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextstorageobserving?language=objc)
    pub unsafe trait NSTextStorageObserving: NSObjectProtocol {
        #[method_id(@__retain_semantics Other textStorage)]
        unsafe fn textStorage(&self) -> Option<Retained<NSTextStorage>>;

        /// Setter for [`textStorage`][Self::textStorage].
        #[method(setTextStorage:)]
        unsafe fn setTextStorage(&self, text_storage: Option<&NSTextStorage>);

        #[method(processEditingForTextStorage:edited:range:changeInLength:invalidatedRange:)]
        unsafe fn processEditingForTextStorage_edited_range_changeInLength_invalidatedRange(
            &self,
            text_storage: &NSTextStorage,
            edit_mask: NSTextStorageEditActions,
            new_char_range: NSRange,
            delta: NSInteger,
            invalidated_char_range: NSRange,
        );

        #[cfg(feature = "block2")]
        #[method(performEditingTransactionForTextStorage:usingBlock:)]
        unsafe fn performEditingTransactionForTextStorage_usingBlock(
            &self,
            text_storage: &NSTextStorage,
            transaction: &block2::Block<dyn Fn() + '_>,
        );
    }

    unsafe impl ProtocolType for dyn NSTextStorageObserving {}
);

/// ** Deprecations ***
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/appkit/nstextstorageeditedoptions?language=objc)
pub type NSTextStorageEditedOptions = NSUInteger;
