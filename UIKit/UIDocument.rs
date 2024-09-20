//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIDocumentChangeKind(pub NSInteger);
impl UIDocumentChangeKind {
    pub const UIDocumentChangeDone: Self = Self(0);
    pub const UIDocumentChangeUndone: Self = Self(1);
    pub const UIDocumentChangeRedone: Self = Self(2);
    pub const UIDocumentChangeCleared: Self = Self(3);
}

unsafe impl Encode for UIDocumentChangeKind {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIDocumentChangeKind {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIDocumentSaveOperation(pub NSInteger);
impl UIDocumentSaveOperation {
    pub const UIDocumentSaveForCreating: Self = Self(0);
    pub const UIDocumentSaveForOverwriting: Self = Self(1);
}

unsafe impl Encode for UIDocumentSaveOperation {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for UIDocumentSaveOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIDocumentState(pub NSUInteger);
bitflags::bitflags! {
    impl UIDocumentState: NSUInteger {
        #[doc(alias = "UIDocumentStateNormal")]
        const Normal = 0;
        #[doc(alias = "UIDocumentStateClosed")]
        const Closed = 1<<0;
        #[doc(alias = "UIDocumentStateInConflict")]
        const InConflict = 1<<1;
        #[doc(alias = "UIDocumentStateSavingError")]
        const SavingError = 1<<2;
        #[doc(alias = "UIDocumentStateEditingDisabled")]
        const EditingDisabled = 1<<3;
        #[doc(alias = "UIDocumentStateProgressAvailable")]
        const ProgressAvailable = 1<<4;
    }
}

unsafe impl Encode for UIDocumentState {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for UIDocumentState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

// NS_TYPED_EXTENSIBLE_ENUM
pub type UIDocumentCreationIntent = NSString;

extern "C" {
    pub static UIDocumentCreationIntentDefault: &'static UIDocumentCreationIntent;
}

extern "C" {
    pub static UIDocumentStateChangedNotification: &'static NSNotificationName;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct UIDocument;

    unsafe impl ClassType for UIDocument {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl NSFilePresenter for UIDocument {}

unsafe impl NSObjectProtocol for UIDocument {}

unsafe impl NSProgressReporting for UIDocument {}

extern_methods!(
    unsafe impl UIDocument {
        #[method_id(@__retain_semantics Init initWithFileURL:)]
        pub unsafe fn initWithFileURL(this: Allocated<Self>, url: &NSURL) -> Retained<Self>;

        #[method_id(@__retain_semantics Other fileURL)]
        pub unsafe fn fileURL(&self) -> Retained<NSURL>;

        #[method_id(@__retain_semantics Other localizedName)]
        pub unsafe fn localizedName(&self) -> Retained<NSString>;

        #[method_id(@__retain_semantics Other fileType)]
        pub unsafe fn fileType(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other fileModificationDate)]
        pub unsafe fn fileModificationDate(&self) -> Option<Retained<NSDate>>;

        #[method(setFileModificationDate:)]
        pub unsafe fn setFileModificationDate(&self, file_modification_date: Option<&NSDate>);

        #[method(documentState)]
        pub unsafe fn documentState(&self) -> UIDocumentState;

        #[method_id(@__retain_semantics Other progress)]
        pub unsafe fn progress(&self) -> Option<Retained<NSProgress>>;

        #[cfg(feature = "block2")]
        #[method(openWithCompletionHandler:)]
        pub unsafe fn openWithCompletionHandler(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn(Bool)>>,
        );

        #[cfg(feature = "block2")]
        #[method(closeWithCompletionHandler:)]
        pub unsafe fn closeWithCompletionHandler(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn(Bool)>>,
        );

        #[method(loadFromContents:ofType:error:_)]
        pub unsafe fn loadFromContents_ofType_error(
            &self,
            contents: &AnyObject,
            type_name: Option<&NSString>,
        ) -> Result<(), Retained<NSError>>;

        #[method_id(@__retain_semantics Other contentsForType:error:_)]
        pub unsafe fn contentsForType_error(
            &self,
            type_name: &NSString,
        ) -> Result<Retained<AnyObject>, Retained<NSError>>;

        #[method(disableEditing)]
        pub unsafe fn disableEditing(&self);

        #[method(enableEditing)]
        pub unsafe fn enableEditing(&self);

        #[method_id(@__retain_semantics Other undoManager)]
        pub unsafe fn undoManager(&self) -> Option<Retained<NSUndoManager>>;

        #[method(setUndoManager:)]
        pub unsafe fn setUndoManager(&self, undo_manager: Option<&NSUndoManager>);

        #[method(hasUnsavedChanges)]
        pub unsafe fn hasUnsavedChanges(&self) -> bool;

        #[method(updateChangeCount:)]
        pub unsafe fn updateChangeCount(&self, change: UIDocumentChangeKind);

        #[method_id(@__retain_semantics Other changeCountTokenForSaveOperation:)]
        pub unsafe fn changeCountTokenForSaveOperation(
            &self,
            save_operation: UIDocumentSaveOperation,
        ) -> Retained<AnyObject>;

        #[method(updateChangeCountWithToken:forSaveOperation:)]
        pub unsafe fn updateChangeCountWithToken_forSaveOperation(
            &self,
            change_count_token: &AnyObject,
            save_operation: UIDocumentSaveOperation,
        );

        #[cfg(feature = "block2")]
        #[method(saveToURL:forSaveOperation:completionHandler:)]
        pub unsafe fn saveToURL_forSaveOperation_completionHandler(
            &self,
            url: &NSURL,
            save_operation: UIDocumentSaveOperation,
            completion_handler: Option<&block2::Block<dyn Fn(Bool)>>,
        );

        #[cfg(feature = "block2")]
        #[method(autosaveWithCompletionHandler:)]
        pub unsafe fn autosaveWithCompletionHandler(
            &self,
            completion_handler: Option<&block2::Block<dyn Fn(Bool)>>,
        );

        #[method_id(@__retain_semantics Other savingFileType)]
        pub unsafe fn savingFileType(&self) -> Option<Retained<NSString>>;

        #[method_id(@__retain_semantics Other fileNameExtensionForType:saveOperation:)]
        pub unsafe fn fileNameExtensionForType_saveOperation(
            &self,
            type_name: Option<&NSString>,
            save_operation: UIDocumentSaveOperation,
        ) -> Retained<NSString>;

        #[method(writeContents:andAttributes:safelyToURL:forSaveOperation:error:_)]
        pub unsafe fn writeContents_andAttributes_safelyToURL_forSaveOperation_error(
            &self,
            contents: &AnyObject,
            additional_file_attributes: Option<&NSDictionary>,
            url: &NSURL,
            save_operation: UIDocumentSaveOperation,
        ) -> Result<(), Retained<NSError>>;

        #[method(writeContents:toURL:forSaveOperation:originalContentsURL:error:_)]
        pub unsafe fn writeContents_toURL_forSaveOperation_originalContentsURL_error(
            &self,
            contents: &AnyObject,
            url: &NSURL,
            save_operation: UIDocumentSaveOperation,
            original_contents_url: Option<&NSURL>,
        ) -> Result<(), Retained<NSError>>;

        #[method_id(@__retain_semantics Other fileAttributesToWriteToURL:forSaveOperation:error:_)]
        pub unsafe fn fileAttributesToWriteToURL_forSaveOperation_error(
            &self,
            url: &NSURL,
            save_operation: UIDocumentSaveOperation,
        ) -> Result<Retained<NSDictionary>, Retained<NSError>>;

        #[method(readFromURL:error:_)]
        pub unsafe fn readFromURL_error(&self, url: &NSURL) -> Result<(), Retained<NSError>>;

        #[cfg(feature = "block2")]
        #[method(performAsynchronousFileAccessUsingBlock:)]
        pub unsafe fn performAsynchronousFileAccessUsingBlock(
            &self,
            block: &block2::Block<dyn Fn()>,
        );

        #[method(handleError:userInteractionPermitted:)]
        pub unsafe fn handleError_userInteractionPermitted(
            &self,
            error: &NSError,
            user_interaction_permitted: bool,
        );

        #[method(finishedHandlingError:recovered:)]
        pub unsafe fn finishedHandlingError_recovered(&self, error: &NSError, recovered: bool);

        #[method(userInteractionNoLongerPermittedForError:)]
        pub unsafe fn userInteractionNoLongerPermittedForError(&self, error: &NSError);

        #[cfg(feature = "block2")]
        #[method(revertToContentsOfURL:completionHandler:)]
        pub unsafe fn revertToContentsOfURL_completionHandler(
            &self,
            url: &NSURL,
            completion_handler: Option<&block2::Block<dyn Fn(Bool)>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UIDocument {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_methods!(
    unsafe impl UIDocument {}
);

#[cfg(feature = "UINavigationItem")]
unsafe impl UINavigationItemRenameDelegate for UIDocument {}

extern "C" {
    pub static NSUserActivityDocumentURLKey: &'static NSString;
}

extern_methods!(
    /// ActivityContinuation
    unsafe impl UIDocument {
        #[method_id(@__retain_semantics Other userActivity)]
        pub unsafe fn userActivity(&self) -> Option<Retained<NSUserActivity>>;

        #[method(setUserActivity:)]
        pub unsafe fn setUserActivity(&self, user_activity: Option<&NSUserActivity>);

        #[method(updateUserActivityState:)]
        pub unsafe fn updateUserActivityState(&self, user_activity: &NSUserActivity);

        #[method(restoreUserActivityState:)]
        pub unsafe fn restoreUserActivityState(&self, user_activity: &NSUserActivity);
    }
);

#[cfg(feature = "UIUserActivity")]
unsafe impl UIUserActivityRestoring for UIDocument {}
