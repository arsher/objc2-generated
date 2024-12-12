//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsresponder?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSResponder;
);

unsafe impl NSCoding for NSResponder {}

unsafe impl NSObjectProtocol for NSResponder {}

extern_methods!(
    unsafe impl NSResponder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;

        #[method_id(@__retain_semantics Other nextResponder)]
        pub unsafe fn nextResponder(&self) -> Option<Retained<NSResponder>>;

        #[method(setNextResponder:)]
        pub unsafe fn setNextResponder(&self, next_responder: Option<&NSResponder>);

        #[method(tryToPerform:with:)]
        pub unsafe fn tryToPerform_with(&self, action: Sel, object: Option<&AnyObject>) -> bool;

        #[cfg(feature = "NSEvent")]
        #[method(performKeyEquivalent:)]
        pub unsafe fn performKeyEquivalent(&self, event: &NSEvent) -> bool;

        #[cfg(feature = "NSPasteboard")]
        #[method_id(@__retain_semantics Other validRequestorForSendType:returnType:)]
        pub unsafe fn validRequestorForSendType_returnType(
            &self,
            send_type: Option<&NSPasteboardType>,
            return_type: Option<&NSPasteboardType>,
        ) -> Option<Retained<AnyObject>>;

        #[cfg(feature = "NSEvent")]
        #[method(mouseDown:)]
        pub unsafe fn mouseDown(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(rightMouseDown:)]
        pub unsafe fn rightMouseDown(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(otherMouseDown:)]
        pub unsafe fn otherMouseDown(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(mouseUp:)]
        pub unsafe fn mouseUp(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(rightMouseUp:)]
        pub unsafe fn rightMouseUp(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(otherMouseUp:)]
        pub unsafe fn otherMouseUp(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(mouseMoved:)]
        pub unsafe fn mouseMoved(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(mouseDragged:)]
        pub unsafe fn mouseDragged(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(scrollWheel:)]
        pub unsafe fn scrollWheel(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(rightMouseDragged:)]
        pub unsafe fn rightMouseDragged(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(otherMouseDragged:)]
        pub unsafe fn otherMouseDragged(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(mouseEntered:)]
        pub unsafe fn mouseEntered(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(mouseExited:)]
        pub unsafe fn mouseExited(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(keyDown:)]
        pub unsafe fn keyDown(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(keyUp:)]
        pub unsafe fn keyUp(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(flagsChanged:)]
        pub unsafe fn flagsChanged(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(tabletPoint:)]
        pub unsafe fn tabletPoint(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(tabletProximity:)]
        pub unsafe fn tabletProximity(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(cursorUpdate:)]
        pub unsafe fn cursorUpdate(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(magnifyWithEvent:)]
        pub unsafe fn magnifyWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(rotateWithEvent:)]
        pub unsafe fn rotateWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(swipeWithEvent:)]
        pub unsafe fn swipeWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(beginGestureWithEvent:)]
        pub unsafe fn beginGestureWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(endGestureWithEvent:)]
        pub unsafe fn endGestureWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(smartMagnifyWithEvent:)]
        pub unsafe fn smartMagnifyWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(changeModeWithEvent:)]
        pub unsafe fn changeModeWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(touchesBeganWithEvent:)]
        pub unsafe fn touchesBeganWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(touchesMovedWithEvent:)]
        pub unsafe fn touchesMovedWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(touchesEndedWithEvent:)]
        pub unsafe fn touchesEndedWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(touchesCancelledWithEvent:)]
        pub unsafe fn touchesCancelledWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(quickLookWithEvent:)]
        pub unsafe fn quickLookWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(pressureChangeWithEvent:)]
        pub unsafe fn pressureChangeWithEvent(&self, event: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(contextMenuKeyDown:)]
        pub unsafe fn contextMenuKeyDown(&self, event: &NSEvent);

        #[method(noResponderFor:)]
        pub unsafe fn noResponderFor(&self, event_selector: Sel);

        #[method(acceptsFirstResponder)]
        pub unsafe fn acceptsFirstResponder(&self) -> bool;

        #[method(becomeFirstResponder)]
        pub unsafe fn becomeFirstResponder(&self) -> bool;

        #[method(resignFirstResponder)]
        pub unsafe fn resignFirstResponder(&self) -> bool;

        #[cfg(feature = "NSEvent")]
        #[method(interpretKeyEvents:)]
        pub unsafe fn interpretKeyEvents(&self, event_array: &NSArray<NSEvent>);

        #[method(flushBufferedKeyEvents)]
        pub unsafe fn flushBufferedKeyEvents(&self);

        #[cfg(feature = "NSMenu")]
        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Option<Retained<NSMenu>>;

        #[cfg(feature = "NSMenu")]
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);

        #[method(showContextHelp:)]
        pub unsafe fn showContextHelp(&self, sender: Option<&AnyObject>);

        #[cfg(feature = "NSEvent")]
        #[method(helpRequested:)]
        pub unsafe fn helpRequested(&self, event_ptr: &NSEvent);

        #[cfg(feature = "NSEvent")]
        #[method(shouldBeTreatedAsInkEvent:)]
        pub unsafe fn shouldBeTreatedAsInkEvent(&self, event: &NSEvent) -> bool;

        #[cfg(feature = "NSEvent")]
        #[method(wantsScrollEventsForSwipeTrackingOnAxis:)]
        pub unsafe fn wantsScrollEventsForSwipeTrackingOnAxis(
            &self,
            axis: NSEventGestureAxis,
        ) -> bool;

        #[cfg(feature = "NSEvent")]
        #[method(wantsForwardedScrollEventsForAxis:)]
        pub unsafe fn wantsForwardedScrollEventsForAxis(&self, axis: NSEventGestureAxis) -> bool;

        #[method_id(@__retain_semantics Other supplementalTargetForAction:sender:)]
        pub unsafe fn supplementalTargetForAction_sender(
            &self,
            action: Sel,
            sender: Option<&AnyObject>,
        ) -> Option<Retained<AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSResponder {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/appkit/nsstandardkeybindingresponding?language=objc)
    pub unsafe trait NSStandardKeyBindingResponding:
        NSObjectProtocol + MainThreadOnly
    {
        #[optional]
        #[method(insertText:)]
        unsafe fn insertText(&self, insert_string: &AnyObject);

        #[optional]
        #[method(doCommandBySelector:)]
        unsafe fn doCommandBySelector(&self, selector: Sel);

        #[optional]
        #[method(moveForward:)]
        unsafe fn moveForward(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveRight:)]
        unsafe fn moveRight(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveBackward:)]
        unsafe fn moveBackward(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveLeft:)]
        unsafe fn moveLeft(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveUp:)]
        unsafe fn moveUp(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveDown:)]
        unsafe fn moveDown(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveWordForward:)]
        unsafe fn moveWordForward(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveWordBackward:)]
        unsafe fn moveWordBackward(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveToBeginningOfLine:)]
        unsafe fn moveToBeginningOfLine(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveToEndOfLine:)]
        unsafe fn moveToEndOfLine(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveToBeginningOfParagraph:)]
        unsafe fn moveToBeginningOfParagraph(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveToEndOfParagraph:)]
        unsafe fn moveToEndOfParagraph(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveToEndOfDocument:)]
        unsafe fn moveToEndOfDocument(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveToBeginningOfDocument:)]
        unsafe fn moveToBeginningOfDocument(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(pageDown:)]
        unsafe fn pageDown(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(pageUp:)]
        unsafe fn pageUp(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(centerSelectionInVisibleArea:)]
        unsafe fn centerSelectionInVisibleArea(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveBackwardAndModifySelection:)]
        unsafe fn moveBackwardAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveForwardAndModifySelection:)]
        unsafe fn moveForwardAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveWordForwardAndModifySelection:)]
        unsafe fn moveWordForwardAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveWordBackwardAndModifySelection:)]
        unsafe fn moveWordBackwardAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveUpAndModifySelection:)]
        unsafe fn moveUpAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveDownAndModifySelection:)]
        unsafe fn moveDownAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveToBeginningOfLineAndModifySelection:)]
        unsafe fn moveToBeginningOfLineAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveToEndOfLineAndModifySelection:)]
        unsafe fn moveToEndOfLineAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveToBeginningOfParagraphAndModifySelection:)]
        unsafe fn moveToBeginningOfParagraphAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveToEndOfParagraphAndModifySelection:)]
        unsafe fn moveToEndOfParagraphAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveToEndOfDocumentAndModifySelection:)]
        unsafe fn moveToEndOfDocumentAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveToBeginningOfDocumentAndModifySelection:)]
        unsafe fn moveToBeginningOfDocumentAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(pageDownAndModifySelection:)]
        unsafe fn pageDownAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(pageUpAndModifySelection:)]
        unsafe fn pageUpAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveParagraphForwardAndModifySelection:)]
        unsafe fn moveParagraphForwardAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveParagraphBackwardAndModifySelection:)]
        unsafe fn moveParagraphBackwardAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveWordRight:)]
        unsafe fn moveWordRight(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveWordLeft:)]
        unsafe fn moveWordLeft(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveRightAndModifySelection:)]
        unsafe fn moveRightAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveLeftAndModifySelection:)]
        unsafe fn moveLeftAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveWordRightAndModifySelection:)]
        unsafe fn moveWordRightAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveWordLeftAndModifySelection:)]
        unsafe fn moveWordLeftAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveToLeftEndOfLine:)]
        unsafe fn moveToLeftEndOfLine(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveToRightEndOfLine:)]
        unsafe fn moveToRightEndOfLine(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveToLeftEndOfLineAndModifySelection:)]
        unsafe fn moveToLeftEndOfLineAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(moveToRightEndOfLineAndModifySelection:)]
        unsafe fn moveToRightEndOfLineAndModifySelection(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(scrollPageUp:)]
        unsafe fn scrollPageUp(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(scrollPageDown:)]
        unsafe fn scrollPageDown(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(scrollLineUp:)]
        unsafe fn scrollLineUp(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(scrollLineDown:)]
        unsafe fn scrollLineDown(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(scrollToBeginningOfDocument:)]
        unsafe fn scrollToBeginningOfDocument(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(scrollToEndOfDocument:)]
        unsafe fn scrollToEndOfDocument(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(transpose:)]
        unsafe fn transpose(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(transposeWords:)]
        unsafe fn transposeWords(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(selectAll:)]
        unsafe fn selectAll(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(selectParagraph:)]
        unsafe fn selectParagraph(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(selectLine:)]
        unsafe fn selectLine(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(selectWord:)]
        unsafe fn selectWord(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(indent:)]
        unsafe fn indent(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(insertTab:)]
        unsafe fn insertTab(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(insertBacktab:)]
        unsafe fn insertBacktab(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(insertNewline:)]
        unsafe fn insertNewline(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(insertParagraphSeparator:)]
        unsafe fn insertParagraphSeparator(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(insertNewlineIgnoringFieldEditor:)]
        unsafe fn insertNewlineIgnoringFieldEditor(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(insertTabIgnoringFieldEditor:)]
        unsafe fn insertTabIgnoringFieldEditor(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(insertLineBreak:)]
        unsafe fn insertLineBreak(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(insertContainerBreak:)]
        unsafe fn insertContainerBreak(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(insertSingleQuoteIgnoringSubstitution:)]
        unsafe fn insertSingleQuoteIgnoringSubstitution(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(insertDoubleQuoteIgnoringSubstitution:)]
        unsafe fn insertDoubleQuoteIgnoringSubstitution(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(changeCaseOfLetter:)]
        unsafe fn changeCaseOfLetter(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(uppercaseWord:)]
        unsafe fn uppercaseWord(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(lowercaseWord:)]
        unsafe fn lowercaseWord(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(capitalizeWord:)]
        unsafe fn capitalizeWord(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(deleteForward:)]
        unsafe fn deleteForward(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(deleteBackward:)]
        unsafe fn deleteBackward(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(deleteBackwardByDecomposingPreviousCharacter:)]
        unsafe fn deleteBackwardByDecomposingPreviousCharacter(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(deleteWordForward:)]
        unsafe fn deleteWordForward(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(deleteWordBackward:)]
        unsafe fn deleteWordBackward(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(deleteToBeginningOfLine:)]
        unsafe fn deleteToBeginningOfLine(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(deleteToEndOfLine:)]
        unsafe fn deleteToEndOfLine(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(deleteToBeginningOfParagraph:)]
        unsafe fn deleteToBeginningOfParagraph(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(deleteToEndOfParagraph:)]
        unsafe fn deleteToEndOfParagraph(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(yank:)]
        unsafe fn yank(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(complete:)]
        unsafe fn complete(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(setMark:)]
        unsafe fn setMark(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(deleteToMark:)]
        unsafe fn deleteToMark(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(selectToMark:)]
        unsafe fn selectToMark(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(swapWithMark:)]
        unsafe fn swapWithMark(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(cancelOperation:)]
        unsafe fn cancelOperation(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(makeBaseWritingDirectionNatural:)]
        unsafe fn makeBaseWritingDirectionNatural(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(makeBaseWritingDirectionLeftToRight:)]
        unsafe fn makeBaseWritingDirectionLeftToRight(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(makeBaseWritingDirectionRightToLeft:)]
        unsafe fn makeBaseWritingDirectionRightToLeft(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(makeTextWritingDirectionNatural:)]
        unsafe fn makeTextWritingDirectionNatural(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(makeTextWritingDirectionLeftToRight:)]
        unsafe fn makeTextWritingDirectionLeftToRight(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(makeTextWritingDirectionRightToLeft:)]
        unsafe fn makeTextWritingDirectionRightToLeft(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(quickLookPreviewItems:)]
        unsafe fn quickLookPreviewItems(&self, sender: Option<&AnyObject>);

        #[optional]
        #[method(showContextMenuForSelection:)]
        unsafe fn showContextMenuForSelection(&self, sender: Option<&AnyObject>);
    }

    unsafe impl ProtocolType for dyn NSStandardKeyBindingResponding {}
);

extern_methods!(
    /// NSStandardKeyBindingMethods
    unsafe impl NSResponder {}
);

unsafe impl NSStandardKeyBindingResponding for NSResponder {}

extern_methods!(
    /// NSUndoSupport
    unsafe impl NSResponder {
        #[method_id(@__retain_semantics Other undoManager)]
        pub unsafe fn undoManager(&self) -> Option<Retained<NSUndoManager>>;
    }
);

extern_methods!(
    /// NSControlEditingSupport
    unsafe impl NSResponder {
        #[cfg(feature = "NSEvent")]
        #[method(validateProposedFirstResponder:forEvent:)]
        pub unsafe fn validateProposedFirstResponder_forEvent(
            &self,
            responder: &NSResponder,
            event: Option<&NSEvent>,
        ) -> bool;
    }
);

extern_methods!(
    /// NSErrorPresentation
    unsafe impl NSResponder {
        #[cfg(feature = "NSWindow")]
        #[method(presentError:modalForWindow:delegate:didPresentSelector:contextInfo:)]
        pub unsafe fn presentError_modalForWindow_delegate_didPresentSelector_contextInfo(
            &self,
            error: &NSError,
            window: &NSWindow,
            delegate: Option<&AnyObject>,
            did_present_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[method(presentError:)]
        pub unsafe fn presentError(&self, error: &NSError) -> bool;

        #[method_id(@__retain_semantics Other willPresentError:)]
        pub unsafe fn willPresentError(&self, error: &NSError) -> Retained<NSError>;
    }
);

extern_methods!(
    /// NSTextFinderSupport
    unsafe impl NSResponder {
        #[method(performTextFinderAction:)]
        pub unsafe fn performTextFinderAction(&self, sender: Option<&AnyObject>);
    }
);

extern_methods!(
    /// NSWindowTabbing
    unsafe impl NSResponder {
        #[method(newWindowForTab:)]
        pub unsafe fn newWindowForTab(&self, sender: Option<&AnyObject>);
    }
);

extern_methods!(
    /// NSWritingToolsSupport
    unsafe impl NSResponder {
        #[method(showWritingTools:)]
        pub unsafe fn showWritingTools(&self, sender: Option<&AnyObject>);
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSResponder {
        #[deprecated = "This has always returned NO and had no effect on macOS"]
        #[method(performMnemonic:)]
        pub unsafe fn performMnemonic(&self, string: &NSString) -> bool;
    }
);
