//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-app-kit")]
use objc2_app_kit::*;
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

        #[method(panelType)]
        pub unsafe fn panelType(&self) -> IMKCandidatePanelType;

        #[method(setPanelType:)]
        pub unsafe fn setPanelType(&self, panel_type: IMKCandidatePanelType);

        #[method(show:)]
        pub unsafe fn show(&self, location_hint: IMKCandidatesLocationHint);

        #[method(hide)]
        pub unsafe fn hide(&self);

        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;

        #[method(updateCandidates)]
        pub unsafe fn updateCandidates(&self);

        #[method(showAnnotation:)]
        pub unsafe fn showAnnotation(&self, annotation_string: Option<&NSAttributedString>);

        #[method(showSublist:subListDelegate:)]
        pub unsafe fn showSublist_subListDelegate(
            &self,
            candidates: Option<&NSArray>,
            delegate: Option<&AnyObject>,
        );

        #[method(candidateFrame)]
        pub unsafe fn candidateFrame(&self) -> NSRect;

        #[method(setSelectionKeys:)]
        pub unsafe fn setSelectionKeys(&self, key_codes: Option<&NSArray>);

        #[method_id(@__retain_semantics Other selectionKeys)]
        pub unsafe fn selectionKeys(&self) -> Option<Retained<NSArray>>;

        #[method(setAttributes:)]
        pub unsafe fn setAttributes(&self, attributes: Option<&NSDictionary>);

        #[method_id(@__retain_semantics Other attributes)]
        pub unsafe fn attributes(&self) -> Option<Retained<NSDictionary>>;

        #[method(setDismissesAutomatically:)]
        pub unsafe fn setDismissesAutomatically(&self, flag: bool);

        #[method(dismissesAutomatically)]
        pub unsafe fn dismissesAutomatically(&self) -> bool;

        #[method(selectedCandidate)]
        pub unsafe fn selectedCandidate(&self) -> NSInteger;

        #[method(setCandidateFrameTopLeft:)]
        pub unsafe fn setCandidateFrameTopLeft(&self, point: NSPoint);

        #[method(showChild)]
        pub unsafe fn showChild(&self);

        #[method(hideChild)]
        pub unsafe fn hideChild(&self);

        #[method(attachChild:toCandidate:type:)]
        pub unsafe fn attachChild_toCandidate_type(
            &self,
            child: Option<&IMKCandidates>,
            candidate_identifier: NSInteger,
            the_type: IMKStyleType,
        );

        #[method(detachChild:)]
        pub unsafe fn detachChild(&self, candidate_identifier: NSInteger);

        #[method(setCandidateData:)]
        pub unsafe fn setCandidateData(&self, candidates_array: Option<&NSArray>);

        #[method(selectCandidateWithIdentifier:)]
        pub unsafe fn selectCandidateWithIdentifier(&self, candidate_identifier: NSInteger)
            -> bool;

        #[method(selectCandidate:)]
        pub unsafe fn selectCandidate(&self, candidate_identifier: NSInteger);

        #[method(showCandidates)]
        pub unsafe fn showCandidates(&self);

        #[method(candidateStringIdentifier:)]
        pub unsafe fn candidateStringIdentifier(
            &self,
            candidate_string: Option<&AnyObject>,
        ) -> NSInteger;

        #[method_id(@__retain_semantics Other selectedCandidateString)]
        pub unsafe fn selectedCandidateString(&self) -> Option<Retained<NSAttributedString>>;

        #[method(candidateIdentifierAtLineNumber:)]
        pub unsafe fn candidateIdentifierAtLineNumber(&self, line_number: NSInteger) -> NSInteger;

        #[method(lineNumberForCandidateWithIdentifier:)]
        pub unsafe fn lineNumberForCandidateWithIdentifier(
            &self,
            candidate_identifier: NSInteger,
        ) -> NSInteger;

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
