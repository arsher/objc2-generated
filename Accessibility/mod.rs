// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "Accessibility", kind = "framework")]
extern "C" {}

#[cfg(feature = "AXAudiograph")]
#[path = "AXAudiograph.rs"]
mod __AXAudiograph;
#[cfg(feature = "AXBrailleMap")]
#[path = "AXBrailleMap.rs"]
mod __AXBrailleMap;
#[cfg(feature = "AXColorUtilities")]
#[path = "AXColorUtilities.rs"]
mod __AXColorUtilities;
#[cfg(feature = "AXCustomContent")]
#[path = "AXCustomContent.rs"]
mod __AXCustomContent;
#[cfg(feature = "AXFeatureOverrideSessionManager")]
#[path = "AXFeatureOverrideSessionManager.rs"]
mod __AXFeatureOverrideSessionManager;
#[cfg(feature = "AXFoundation")]
#[path = "AXFoundation.rs"]
mod __AXFoundation;
#[cfg(feature = "AXHearingUtilities")]
#[path = "AXHearingUtilities.rs"]
mod __AXHearingUtilities;
#[cfg(feature = "AXMathExpression")]
#[path = "AXMathExpression.rs"]
mod __AXMathExpression;
#[cfg(feature = "AXRequest")]
#[path = "AXRequest.rs"]
mod __AXRequest;
#[cfg(feature = "AXSettings")]
#[path = "AXSettings.rs"]
mod __AXSettings;
#[cfg(feature = "AXTechnology")]
#[path = "AXTechnology.rs"]
mod __AXTechnology;

#[cfg(feature = "AXAudiograph")]
pub use self::__AXAudiograph::AXCategoricalDataAxisDescriptor;
#[cfg(feature = "AXAudiograph")]
pub use self::__AXAudiograph::AXChart;
#[cfg(feature = "AXAudiograph")]
pub use self::__AXAudiograph::AXChartDescriptor;
#[cfg(feature = "AXAudiograph")]
pub use self::__AXAudiograph::AXChartDescriptorContentDirection;
#[cfg(feature = "AXAudiograph")]
pub use self::__AXAudiograph::AXDataAxisDescriptor;
#[cfg(feature = "AXAudiograph")]
pub use self::__AXAudiograph::AXDataPoint;
#[cfg(feature = "AXAudiograph")]
pub use self::__AXAudiograph::AXDataPointValue;
#[cfg(feature = "AXAudiograph")]
pub use self::__AXAudiograph::AXDataSeriesDescriptor;
#[cfg(feature = "AXAudiograph")]
pub use self::__AXAudiograph::AXLiveAudioGraph;
#[cfg(feature = "AXAudiograph")]
pub use self::__AXAudiograph::AXNumericDataAxisDescriptor;
#[cfg(feature = "AXAudiograph")]
pub use self::__AXAudiograph::AXNumericDataAxisDescriptorScale;
#[cfg(feature = "AXBrailleMap")]
pub use self::__AXBrailleMap::AXBrailleMap;
#[cfg(feature = "AXBrailleMap")]
pub use self::__AXBrailleMap::AXBrailleMapRenderer;
#[cfg(all(feature = "AXColorUtilities", feature = "objc2-core-graphics"))]
pub use self::__AXColorUtilities::AXNameFromColor;
#[cfg(feature = "AXCustomContent")]
pub use self::__AXCustomContent::AXCustomContent;
#[cfg(feature = "AXCustomContent")]
pub use self::__AXCustomContent::AXCustomContentImportance;
#[cfg(feature = "AXCustomContent")]
pub use self::__AXCustomContent::AXCustomContentProvider;
#[cfg(all(feature = "AXCustomContent", feature = "block2"))]
pub use self::__AXCustomContent::AXCustomContentReturnBlock;
#[cfg(feature = "AXHearingUtilities")]
pub use self::__AXHearingUtilities::AXHearingDeviceEar;
#[cfg(feature = "AXHearingUtilities")]
pub use self::__AXHearingUtilities::AXMFiHearingDevicePairedUUIDs;
#[cfg(feature = "AXHearingUtilities")]
pub use self::__AXHearingUtilities::AXMFiHearingDevicePairedUUIDsDidChangeNotification;
#[cfg(feature = "AXHearingUtilities")]
pub use self::__AXHearingUtilities::AXMFiHearingDeviceStreamingEar;
#[cfg(feature = "AXHearingUtilities")]
pub use self::__AXHearingUtilities::AXMFiHearingDeviceStreamingEarDidChangeNotification;
#[cfg(feature = "AXHearingUtilities")]
pub use self::__AXHearingUtilities::AXSupportsBidirectionalAXMFiHearingDeviceStreaming;
#[cfg(feature = "AXMathExpression")]
pub use self::__AXMathExpression::AXMathExpression;
#[cfg(feature = "AXMathExpression")]
pub use self::__AXMathExpression::AXMathExpressionFenced;
#[cfg(feature = "AXMathExpression")]
pub use self::__AXMathExpression::AXMathExpressionFraction;
#[cfg(feature = "AXMathExpression")]
pub use self::__AXMathExpression::AXMathExpressionIdentifier;
#[cfg(feature = "AXMathExpression")]
pub use self::__AXMathExpression::AXMathExpressionMultiscript;
#[cfg(feature = "AXMathExpression")]
pub use self::__AXMathExpression::AXMathExpressionNumber;
#[cfg(feature = "AXMathExpression")]
pub use self::__AXMathExpression::AXMathExpressionOperator;
#[cfg(feature = "AXMathExpression")]
pub use self::__AXMathExpression::AXMathExpressionProvider;
#[cfg(feature = "AXMathExpression")]
pub use self::__AXMathExpression::AXMathExpressionRoot;
#[cfg(feature = "AXMathExpression")]
pub use self::__AXMathExpression::AXMathExpressionRow;
#[cfg(feature = "AXMathExpression")]
pub use self::__AXMathExpression::AXMathExpressionSubSuperscript;
#[cfg(feature = "AXMathExpression")]
pub use self::__AXMathExpression::AXMathExpressionTable;
#[cfg(feature = "AXMathExpression")]
pub use self::__AXMathExpression::AXMathExpressionTableCell;
#[cfg(feature = "AXMathExpression")]
pub use self::__AXMathExpression::AXMathExpressionTableRow;
#[cfg(feature = "AXMathExpression")]
pub use self::__AXMathExpression::AXMathExpressionText;
#[cfg(feature = "AXMathExpression")]
pub use self::__AXMathExpression::AXMathExpressionUnderOver;
#[cfg(feature = "AXRequest")]
pub use self::__AXRequest::AXRequest;
#[cfg(feature = "AXSettings")]
pub use self::__AXSettings::AXAnimatedImagesEnabled;
#[cfg(feature = "AXSettings")]
pub use self::__AXSettings::AXAnimatedImagesEnabledDidChangeNotification;
#[cfg(feature = "AXSettings")]
pub use self::__AXSettings::AXAssistiveAccessEnabled;
#[cfg(all(feature = "AXSettings", feature = "block2"))]
pub use self::__AXSettings::AXOpenSettingsFeature;
#[cfg(feature = "AXSettings")]
pub use self::__AXSettings::AXPrefersHorizontalTextLayout;
#[cfg(feature = "AXSettings")]
pub use self::__AXSettings::AXPrefersHorizontalTextLayoutDidChangeNotification;
#[cfg(feature = "AXSettings")]
pub use self::__AXSettings::AXPrefersNonBlinkingTextInsertionIndicator;
#[cfg(feature = "AXSettings")]
pub use self::__AXSettings::AXPrefersNonBlinkingTextInsertionIndicatorDidChangeNotification;
#[cfg(feature = "AXSettings")]
pub use self::__AXSettings::AXSettingsFeature;
#[cfg(feature = "AXTechnology")]
pub use self::__AXTechnology::AXTechnology;
#[cfg(feature = "AXTechnology")]
pub use self::__AXTechnology::AXTechnologyAutomation;
#[cfg(feature = "AXTechnology")]
pub use self::__AXTechnology::AXTechnologyFullKeyboardAccess;
#[cfg(feature = "AXTechnology")]
pub use self::__AXTechnology::AXTechnologyHoverText;
#[cfg(feature = "AXTechnology")]
pub use self::__AXTechnology::AXTechnologySpeakScreen;
#[cfg(feature = "AXTechnology")]
pub use self::__AXTechnology::AXTechnologySwitchControl;
#[cfg(feature = "AXTechnology")]
pub use self::__AXTechnology::AXTechnologyVoiceControl;
#[cfg(feature = "AXTechnology")]
pub use self::__AXTechnology::AXTechnologyVoiceOver;
#[cfg(feature = "AXTechnology")]
pub use self::__AXTechnology::AXTechnologyZoom;
