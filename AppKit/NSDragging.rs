//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSDragOperation {
        NSDragOperationNone = 0,
        NSDragOperationCopy = 1,
        NSDragOperationLink = 2,
        NSDragOperationGeneric = 4,
        NSDragOperationPrivate = 8,
        NSDragOperationMove = 16,
        NSDragOperationDelete = 32,
        NSDragOperationEvery = NSUIntegerMax as _,
        NSDragOperationAll_Obsolete = 15,
        NSDragOperationAll = NSDragOperationAll_Obsolete,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSDraggingFormation {
        NSDraggingFormationDefault = 0,
        NSDraggingFormationNone = 1,
        NSDraggingFormationPile = 2,
        NSDraggingFormationList = 3,
        NSDraggingFormationStack = 4,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSDraggingContext {
        NSDraggingContextOutsideApplication = 0,
        NSDraggingContextWithinApplication = 1,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSDraggingItemEnumerationOptions {
        NSDraggingItemEnumerationConcurrent = NSEnumerationConcurrent,
        NSDraggingItemEnumerationClearNonenumeratedImages = 1 << 16,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSSpringLoadingHighlight {
        NSSpringLoadingHighlightNone = 0,
        NSSpringLoadingHighlightStandard = 1,
        NSSpringLoadingHighlightEmphasized = 2,
    }
);

extern_protocol!(
    pub struct NSDraggingInfo;

    unsafe impl ProtocolType for NSDraggingInfo {
        #[cfg(feature = "AppKit_NSWindow")]
        #[method_id(@__retain_semantics Other draggingDestinationWindow)]
        pub unsafe fn draggingDestinationWindow(&self) -> Option<Id<NSWindow, Shared>>;

        #[method(draggingSourceOperationMask)]
        pub unsafe fn draggingSourceOperationMask(&self) -> NSDragOperation;

        #[method(draggingLocation)]
        pub unsafe fn draggingLocation(&self) -> NSPoint;

        #[method(draggedImageLocation)]
        pub unsafe fn draggedImageLocation(&self) -> NSPoint;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other draggedImage)]
        pub unsafe fn draggedImage(&self) -> Option<Id<NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method_id(@__retain_semantics Other draggingPasteboard)]
        pub unsafe fn draggingPasteboard(&self) -> Id<NSPasteboard, Shared>;

        #[method_id(@__retain_semantics Other draggingSource)]
        pub unsafe fn draggingSource(&self) -> Option<Id<Object, Shared>>;

        #[method(draggingSequenceNumber)]
        pub unsafe fn draggingSequenceNumber(&self) -> NSInteger;

        #[method(slideDraggedImageTo:)]
        pub unsafe fn slideDraggedImageTo(&self, screenPoint: NSPoint);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other namesOfPromisedFilesDroppedAtDestination:)]
        pub unsafe fn namesOfPromisedFilesDroppedAtDestination(
            &self,
            dropDestination: &NSURL,
        ) -> Option<Id<NSArray<NSString>, Shared>>;

        #[method(draggingFormation)]
        pub unsafe fn draggingFormation(&self) -> NSDraggingFormation;

        #[method(setDraggingFormation:)]
        pub unsafe fn setDraggingFormation(&self, draggingFormation: NSDraggingFormation);

        #[method(animatesToDestination)]
        pub unsafe fn animatesToDestination(&self) -> bool;

        #[method(setAnimatesToDestination:)]
        pub unsafe fn setAnimatesToDestination(&self, animatesToDestination: bool);

        #[method(numberOfValidItemsForDrop)]
        pub unsafe fn numberOfValidItemsForDrop(&self) -> NSInteger;

        #[method(setNumberOfValidItemsForDrop:)]
        pub unsafe fn setNumberOfValidItemsForDrop(&self, numberOfValidItemsForDrop: NSInteger);

        #[cfg(all(
            feature = "AppKit_NSDraggingItem",
            feature = "AppKit_NSView",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary"
        ))]
        #[method(enumerateDraggingItemsWithOptions:forView:classes:searchOptions:usingBlock:)]
        pub unsafe fn enumerateDraggingItemsWithOptions_forView_classes_searchOptions_usingBlock(
            &self,
            enumOpts: NSDraggingItemEnumerationOptions,
            view: Option<&NSView>,
            classArray: &NSArray<TodoClass>,
            searchOptions: &NSDictionary<NSPasteboardReadingOptionKey, Object>,
            block: &Block<(NonNull<NSDraggingItem>, NSInteger, NonNull<Bool>), ()>,
        );

        #[method(springLoadingHighlight)]
        pub unsafe fn springLoadingHighlight(&self) -> NSSpringLoadingHighlight;

        #[method(resetSpringLoading)]
        pub unsafe fn resetSpringLoading(&self);
    }
);

extern_protocol!(
    pub struct NSDraggingDestination;

    unsafe impl ProtocolType for NSDraggingDestination {
        #[optional]
        #[method(draggingEntered:)]
        pub unsafe fn draggingEntered(&self, sender: &NSDraggingInfo) -> NSDragOperation;

        #[optional]
        #[method(draggingUpdated:)]
        pub unsafe fn draggingUpdated(&self, sender: &NSDraggingInfo) -> NSDragOperation;

        #[optional]
        #[method(draggingExited:)]
        pub unsafe fn draggingExited(&self, sender: Option<&NSDraggingInfo>);

        #[optional]
        #[method(prepareForDragOperation:)]
        pub unsafe fn prepareForDragOperation(&self, sender: &NSDraggingInfo) -> bool;

        #[optional]
        #[method(performDragOperation:)]
        pub unsafe fn performDragOperation(&self, sender: &NSDraggingInfo) -> bool;

        #[optional]
        #[method(concludeDragOperation:)]
        pub unsafe fn concludeDragOperation(&self, sender: Option<&NSDraggingInfo>);

        #[optional]
        #[method(draggingEnded:)]
        pub unsafe fn draggingEnded(&self, sender: &NSDraggingInfo);

        #[optional]
        #[method(wantsPeriodicDraggingUpdates)]
        pub unsafe fn wantsPeriodicDraggingUpdates(&self) -> bool;

        #[optional]
        #[method(updateDraggingItemsForDrag:)]
        pub unsafe fn updateDraggingItemsForDrag(&self, sender: Option<&NSDraggingInfo>);
    }
);

extern_protocol!(
    pub struct NSDraggingSource;

    unsafe impl ProtocolType for NSDraggingSource {
        #[cfg(feature = "AppKit_NSDraggingSession")]
        #[method(draggingSession:sourceOperationMaskForDraggingContext:)]
        pub unsafe fn draggingSession_sourceOperationMaskForDraggingContext(
            &self,
            session: &NSDraggingSession,
            context: NSDraggingContext,
        ) -> NSDragOperation;

        #[cfg(feature = "AppKit_NSDraggingSession")]
        #[optional]
        #[method(draggingSession:willBeginAtPoint:)]
        pub unsafe fn draggingSession_willBeginAtPoint(
            &self,
            session: &NSDraggingSession,
            screenPoint: NSPoint,
        );

        #[cfg(feature = "AppKit_NSDraggingSession")]
        #[optional]
        #[method(draggingSession:movedToPoint:)]
        pub unsafe fn draggingSession_movedToPoint(
            &self,
            session: &NSDraggingSession,
            screenPoint: NSPoint,
        );

        #[cfg(feature = "AppKit_NSDraggingSession")]
        #[optional]
        #[method(draggingSession:endedAtPoint:operation:)]
        pub unsafe fn draggingSession_endedAtPoint_operation(
            &self,
            session: &NSDraggingSession,
            screenPoint: NSPoint,
            operation: NSDragOperation,
        );

        #[cfg(feature = "AppKit_NSDraggingSession")]
        #[optional]
        #[method(ignoreModifierKeysForDraggingSession:)]
        pub unsafe fn ignoreModifierKeysForDraggingSession(
            &self,
            session: &NSDraggingSession,
        ) -> bool;
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSSpringLoadingOptions {
        NSSpringLoadingDisabled = 0,
        NSSpringLoadingEnabled = 1 << 0,
        NSSpringLoadingContinuousActivation = 1 << 1,
        NSSpringLoadingNoHover = 1 << 3,
    }
);

extern_protocol!(
    pub struct NSSpringLoadingDestination;

    unsafe impl ProtocolType for NSSpringLoadingDestination {
        #[method(springLoadingActivated:draggingInfo:)]
        pub unsafe fn springLoadingActivated_draggingInfo(
            &self,
            activated: bool,
            draggingInfo: &NSDraggingInfo,
        );

        #[method(springLoadingHighlightChanged:)]
        pub unsafe fn springLoadingHighlightChanged(&self, draggingInfo: &NSDraggingInfo);

        #[optional]
        #[method(springLoadingEntered:)]
        pub unsafe fn springLoadingEntered(
            &self,
            draggingInfo: &NSDraggingInfo,
        ) -> NSSpringLoadingOptions;

        #[optional]
        #[method(springLoadingUpdated:)]
        pub unsafe fn springLoadingUpdated(
            &self,
            draggingInfo: &NSDraggingInfo,
        ) -> NSSpringLoadingOptions;

        #[optional]
        #[method(springLoadingExited:)]
        pub unsafe fn springLoadingExited(&self, draggingInfo: &NSDraggingInfo);

        #[optional]
        #[method(draggingEnded:)]
        pub unsafe fn draggingEnded(&self, draggingInfo: &NSDraggingInfo);
    }
);
