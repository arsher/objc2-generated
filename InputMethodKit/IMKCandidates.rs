//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
use objc2_app_kit::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/inputmethodkit/kimksinglecolumnscrollingcandidatepanel?language=objc)
pub const kIMKSingleColumnScrollingCandidatePanel: c_uint = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/inputmethodkit/kimkscrollinggridcandidatepanel?language=objc)
pub const kIMKScrollingGridCandidatePanel: c_uint = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/inputmethodkit/kimksinglerowsteppingcandidatepanel?language=objc)
pub const kIMKSingleRowSteppingCandidatePanel: c_uint = 3;

/// [Apple's documentation](https://developer.apple.com/documentation/inputmethodkit/imkcandidatepaneltype?language=objc)
pub type IMKCandidatePanelType = NSUInteger;

/// [Apple's documentation](https://developer.apple.com/documentation/inputmethodkit/kimkmain?language=objc)
pub const kIMKMain: c_uint = 0;
/// [Apple's documentation](https://developer.apple.com/documentation/inputmethodkit/kimkannotation?language=objc)
pub const kIMKAnnotation: c_uint = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/inputmethodkit/kimksublist?language=objc)
pub const kIMKSubList: c_uint = 2;

/// [Apple's documentation](https://developer.apple.com/documentation/inputmethodkit/imkstyletype?language=objc)
pub type IMKStyleType = NSUInteger;

/// [Apple's documentation](https://developer.apple.com/documentation/inputmethodkit/kimklocatecandidatesabovehint?language=objc)
pub const kIMKLocateCandidatesAboveHint: c_uint = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/inputmethodkit/kimklocatecandidatesbelowhint?language=objc)
pub const kIMKLocateCandidatesBelowHint: c_uint = 2;
/// [Apple's documentation](https://developer.apple.com/documentation/inputmethodkit/kimklocatecandidateslefthint?language=objc)
pub const kIMKLocateCandidatesLeftHint: c_uint = 3;
/// [Apple's documentation](https://developer.apple.com/documentation/inputmethodkit/kimklocatecandidatesrighthint?language=objc)
pub const kIMKLocateCandidatesRightHint: c_uint = 4;

/// [Apple's documentation](https://developer.apple.com/documentation/inputmethodkit/imkcandidateslocationhint?language=objc)
pub type IMKCandidatesLocationHint = NSUInteger;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/inputmethodkit/imkcandidates?language=objc)
    #[unsafe(super(NSResponder, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "objc2-app-kit")]
    pub struct IMKCandidates;
);

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSCoding for IMKCandidates {}

#[cfg(feature = "objc2-app-kit")]
unsafe impl NSObjectProtocol for IMKCandidates {}

extern_methods!(
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl IMKCandidates {
        #[cfg(feature = "IMKServer")]
        /// Default initializer for the class.
        ///
        /// When an input method allocates an IMKCandidate object it should initialize that object by calling this method passing the IMKServer that will manage the candidates and the initial panel type.
        #[method_id(@__retain_semantics Init initWithServer:panelType:)]
        pub unsafe fn initWithServer_panelType(
            this: Allocated<Self>,
            server: Option<&IMKServer>,
            panel_type: IMKCandidatePanelType,
        ) -> Option<Retained<Self>>;

        #[cfg(feature = "IMKServer")]
        #[method_id(@__retain_semantics Init initWithServer:panelType:styleType:)]
        pub unsafe fn initWithServer_panelType_styleType(
            this: Allocated<Self>,
            server: Option<&IMKServer>,
            panel_type: IMKCandidatePanelType,
            style: IMKStyleType,
        ) -> Option<Retained<Self>>;

        /// Return the panel type.
        #[method(panelType)]
        pub unsafe fn panelType(&self) -> IMKCandidatePanelType;

        /// Change the panel type.
        #[method(setPanelType:)]
        pub unsafe fn setPanelType(&self, panel_type: IMKCandidatePanelType);

        /// If a candidate window type has been provided, show the candidate window. The caller provides a location hint that is used to position the window.
        ///
        /// Input methods call this method when it is appropriate, during text conversion, to display a list of candidates.
        #[method(show:)]
        pub unsafe fn show(&self, location_hint: IMKCandidatesLocationHint);

        /// If the candidate window is visible, hide it.
        #[method(hide)]
        pub unsafe fn hide(&self);

        /// Utility method returns YES if a candidate display is visible.
        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;

        /// Call this method to update the candidates displayed in the candidate window.
        ///
        /// Calling this method will result in a call being made to the IMKInputController's candidates method. Note that the candidate list will be updated, but the window's visible state will not change; that is to say, if the window is hidden it will remain hidden, and vice versa.
        #[method(updateCandidates)]
        pub unsafe fn updateCandidates(&self);

        /// Displays an annotation window whose contents are the annotationString.
        ///
        /// An annotation is additional text that explains or somehow adds to the candidate string in a candidate window. Annotations are displayed in a small borderless window that is aligned with the current candidate panel. An input method calls showAnnotation: when the method [IMKInputController candidateSelectionChanged:] is called, and the candidateString has annotations.
        #[method(showAnnotation:)]
        pub unsafe fn showAnnotation(&self, annotation_string: Option<&NSAttributedString>);

        #[method(showSublist:subListDelegate:)]
        pub unsafe fn showSublist_subListDelegate(
            &self,
            candidates: Option<&NSArray>,
            delegate: Option<&AnyObject>,
        );

        #[cfg(feature = "objc2-core-foundation")]
        #[method(candidateFrame)]
        pub unsafe fn candidateFrame(&self) -> NSRect;

        /// Set the selection keys for the candidates.
        ///
        /// Selection keys are an array of NSNumbers where each NSNumber is a virtual key code that the controller will map to characters that are displayed either across the top of the candidates, if the candidates are laid out horizontally, or along the left edge of the candidates, if they are aligned vertically.
        ///
        /// The number of selection keys determines how many candidates are displayed per page.  For example, if you
        /// passed an array of 4 key codes, then 4 candidates are displayed per page.  If you passed 11 key codes, then 11 candidates would be displayed.
        ///
        /// By default the key codes are mapped using the keyboard layout whose source id is com.apple.keylayout.US.  The default layout can be replaced by calling
        /// setSelectionKeysKeylayout (see below).
        ///
        /// The default selection keys are the digits 1 through 9, or in terms of key codes: 18-21,23,22, 26, 28, 25.
        #[method(setSelectionKeys:)]
        pub unsafe fn setSelectionKeys(&self, key_codes: Option<&NSArray>);

        /// Returns an NSArray of NSNumbers where each NSNumber is a virtual key code.
        ///
        /// The NSArray is an autoreleased object. Do not release unless it is first retained.
        #[method_id(@__retain_semantics Other selectionKeys)]
        pub unsafe fn selectionKeys(&self) -> Option<Retained<NSArray>>;

        /// Sets the "style" attributes for the candidates window.  The keys for the attributes dictionary and the values are:
        ///
        /// NSFontAttributeName (value = NSFont)  Setting the font attribute sets the font that is used to draw Candidates.  It does not effect the selection keys which are always drawn in the same font.  Note that to set the font size you should use this key/value pair.
        ///
        /// IMKCandidatesOpacityAttributeName (value = NSNumber with a float value between 0 and 1).  Sets the opacity level to transparent (0.0) to completely opaque (1.0). The default opacity is 1.0.  This constant is declared above.
        ///
        /// NSForegroundColorAttributeName (value = NSColor) Sets the text color used for the candidate text.  By default it is black.
        ///
        /// NSBackgroundColorDocumentAttribute (value = NSColor).  Set the background color that is drawn behind the candidate text.
        ///
        /// IMKCandidatesSendServerKeyEventFirst (value = NSNumber).  NO (default) gives the candidate window first chance at key events.  YES causes events to first be routed to the current IMKInputController.  In that case, if the event is not handled, it will then be sent to the candidate window.
        #[method(setAttributes:)]
        pub unsafe fn setAttributes(&self, attributes: Option<&NSDictionary>);

        /// Returns the attributes dictionary.
        #[method_id(@__retain_semantics Other attributes)]
        pub unsafe fn attributes(&self) -> Option<Retained<NSDictionary>>;

        /// Setting the dismissesAutomatically flag determines what happens to displayed candidates when the return key or enter key is typed.
        ///
        /// By default, if a return or enter key is typed, the candidates are dismissed and a candidateSelected: message is sent to the input controller.  However  if setDismissesAutomatically is passed a NO flag  the candidate display will not be dismissed when a return or enter key is typed.  The input controller will still be sent the candidatesSelected: message, but, as stated, the candidates display will not be dismissed.
        ///
        /// Setting this flag to NO lets an input method process text input while keeping a dynamically changing candidates display in view throughout the text input process.
        ///
        /// When you set this to NO the candidate display will still be hidden when when a session deactivates.
        #[method(setDismissesAutomatically:)]
        pub unsafe fn setDismissesAutomatically(&self, flag: bool);

        /// Returns the dismissesAutomatically flag.
        #[method(dismissesAutomatically)]
        pub unsafe fn dismissesAutomatically(&self) -> bool;

        /// Returns the currently selected candidate identifer.
        ///
        /// Attempts to determine the identifier for the selected candidate.  If there is no selection the return value will be NSNotFound.
        #[method(selectedCandidate)]
        pub unsafe fn selectedCandidate(&self) -> NSInteger;

        #[cfg(feature = "objc2-core-foundation")]
        /// Positions the top-left corner of the candidate window’s frame rectangle at a given point in screen coordinates.
        #[method(setCandidateFrameTopLeft:)]
        pub unsafe fn setCandidateFrameTopLeft(&self, point: NSPoint);

        /// If the current selection has a child IMKCandidates object that will be shown.
        ///
        /// If there is a failure in showing the child this method will throw an exception.
        #[method(showChild)]
        pub unsafe fn showChild(&self);

        /// If the current selection has a child IMKCandidates that is being shown hide it.
        ///
        /// Typically a client will not need to call this as IMKCandidates automatically hides and shows children.
        #[method(hideChild)]
        pub unsafe fn hideChild(&self);

        /// Attach an IMKCandidates object to the specified selection.
        ///
        /// The IMKCandidate can be a sublist or an annotation.
        #[method(attachChild:toCandidate:type:)]
        pub unsafe fn attachChild_toCandidate_type(
            &self,
            child: Option<&IMKCandidates>,
            candidate_identifier: NSInteger,
            the_type: IMKStyleType,
        );

        /// Detach the IMKCandidates object attached to candidate
        #[method(detachChild:)]
        pub unsafe fn detachChild(&self, candidate_identifier: NSInteger);

        /// Set the candidates data directly rather than supplying data via [IMKInputContoller candidates:].
        ///
        /// The elements of the array can be strings or attributed strings.
        #[method(setCandidateData:)]
        pub unsafe fn setCandidateData(&self, candidates_array: Option<&NSArray>);

        /// Select the candidate whose identifier matches the identifier parameter.
        ///
        /// Returns: YES if the candidateIdentifier is valid an the selection was made.  NO if canidateIdentifier is invalid or it was not possible make the selection.
        ///
        /// Parameter `An`: identifier for a candidate.  You can obtain an identifier by mapping a candidate to an identifier via the [IMKCandidates candidateStringIdentifier:].
        #[method(selectCandidateWithIdentifier:)]
        pub unsafe fn selectCandidateWithIdentifier(&self, candidate_identifier: NSInteger)
            -> bool;

        #[method(selectCandidate:)]
        pub unsafe fn selectCandidate(&self, candidate_identifier: NSInteger);

        /// Show the candidate window.
        ///
        /// This simply shows the candidates.  No effort is made to position the candidate.  The caller should move the candidate window to an appropriate location prior to showing.
        #[method(showCandidates)]
        pub unsafe fn showCandidates(&self);

        /// Map a candidateString to an identifier.
        ///
        /// Beginning with MacOS 10.7, candidate strings are mapped internally to an unique identifier of type NSInteger.  Using identifiers to identify a particular candidate is the first stage of enabling data types other than NSString and NSAttributedString for containing the contents of a candidate.
        #[method(candidateStringIdentifier:)]
        pub unsafe fn candidateStringIdentifier(
            &self,
            candidate_string: Option<&AnyObject>,
        ) -> NSInteger;

        /// Returns the currently selected candidate string.
        ///
        /// Attempts to determine the string for the selected candidate.  If there is no selection the return value can be nil.  The attributed string is an autoreleased object.
        #[method_id(@__retain_semantics Other selectedCandidateString)]
        pub unsafe fn selectedCandidateString(&self) -> Option<Retained<NSAttributedString>>;

        /// Returns the candidate identifier for a given line in the candidate window display.
        ///
        /// Maps the lineNumber to a candidate identifier.  Line number 0 corresponds to the candidate in the cell currently in the first (top for vertical) line of the candidate window.  This is convienient for input methods that support selecting a candidate by a number key. Line Number values depend on the column arrangement of your candidate.  If you are displaying a single column candidate window, lines that have been scrolled out of view will have negative values.  For a single row grid line, numbers will correspond to the cell's position in the row (i.e. the first cell will be 0, the second 1, etc).  Finally, for a grid, the line numbers correspond to the grid row.  If the line number is invalid, NSNotFound is returned.
        ///
        /// Parameter `lineNumber`: a number representing a cells position in the candidate window.
        #[method(candidateIdentifierAtLineNumber:)]
        pub unsafe fn candidateIdentifierAtLineNumber(&self, line_number: NSInteger) -> NSInteger;

        /// Returns the line number for a given CandidateID.
        ///
        /// Returns: The line number.  NSNotFound if the candidateID is invalid.
        ///
        /// Parameter `candidateIdentifier`: - A valid identifier for a candidate.
        ///
        /// If the cell that contains the candidate is at the top line of the candidate window, the return value will be 0.
        #[method(lineNumberForCandidateWithIdentifier:)]
        pub unsafe fn lineNumberForCandidateWithIdentifier(
            &self,
            candidate_identifier: NSInteger,
        ) -> NSInteger;

        /// Clears the current selection.
        #[method(clearSelection)]
        pub unsafe fn clearSelection(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl IMKCandidates {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "objc2-app-kit")]
    unsafe impl IMKCandidates {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);
