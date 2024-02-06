// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `Accessibility` framework
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

#[path = "AXAudiograph.rs"]
mod __AXAudiograph;
#[path = "AXBrailleMap.rs"]
mod __AXBrailleMap;
#[path = "AXColorUtilities.rs"]
mod __AXColorUtilities;
#[path = "AXCustomContent.rs"]
mod __AXCustomContent;
#[path = "AXFoundation.rs"]
mod __AXFoundation;
#[path = "AXHearingUtilities.rs"]
mod __AXHearingUtilities;
#[path = "AXSettings.rs"]
mod __AXSettings;

#[cfg(feature = "Accessibility_AXCategoricalDataAxisDescriptor")]
pub use self::__AXAudiograph::AXCategoricalDataAxisDescriptor;
pub use self::__AXAudiograph::AXChart;
#[cfg(feature = "Accessibility_AXChartDescriptor")]
pub use self::__AXAudiograph::AXChartDescriptor;
pub use self::__AXAudiograph::AXChartDescriptorContentDirection;
pub use self::__AXAudiograph::AXDataAxisDescriptor;
#[cfg(feature = "Accessibility_AXDataPoint")]
pub use self::__AXAudiograph::AXDataPoint;
#[cfg(feature = "Accessibility_AXDataPointValue")]
pub use self::__AXAudiograph::AXDataPointValue;
#[cfg(feature = "Accessibility_AXDataSeriesDescriptor")]
pub use self::__AXAudiograph::AXDataSeriesDescriptor;
#[cfg(feature = "Accessibility_AXLiveAudioGraph")]
pub use self::__AXAudiograph::AXLiveAudioGraph;
#[cfg(feature = "Accessibility_AXNumericDataAxisDescriptor")]
pub use self::__AXAudiograph::AXNumericDataAxisDescriptor;
pub use self::__AXAudiograph::AXNumericDataAxisDescriptorScale;
pub use self::__AXAudiograph::{
    AXChartContentDirectionBottomToTop, AXChartContentDirectionLeftToRight,
    AXChartContentDirectionRadialClockwise, AXChartContentDirectionRadialCounterClockwise,
    AXChartContentDirectionRightToLeft, AXChartContentDirectionTopToBottom,
};
pub use self::__AXAudiograph::{AXScaleTypeLinear, AXScaleTypeLn, AXScaleTypeLog10};
#[cfg(feature = "Accessibility_AXBrailleMap")]
pub use self::__AXBrailleMap::AXBrailleMap;
pub use self::__AXBrailleMap::AXBrailleMapRenderer;
#[cfg(feature = "Accessibility_AXCustomContent")]
pub use self::__AXCustomContent::AXCustomContent;
pub use self::__AXCustomContent::AXCustomContentImportance;
pub use self::__AXCustomContent::AXCustomContentProvider;
pub use self::__AXCustomContent::AXCustomContentReturnBlock;
pub use self::__AXCustomContent::{
    AXCustomContentImportanceDefault, AXCustomContentImportanceHigh,
};
pub use self::__AXHearingUtilities::AXHearingDeviceEar;
#[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSUUID"))]
pub use self::__AXHearingUtilities::AXMFiHearingDevicePairedUUIDs;
pub use self::__AXHearingUtilities::AXMFiHearingDevicePairedUUIDsDidChangeNotification;
pub use self::__AXHearingUtilities::AXMFiHearingDeviceStreamingEar;
pub use self::__AXHearingUtilities::AXMFiHearingDeviceStreamingEarDidChangeNotification;
pub use self::__AXHearingUtilities::AXSupportsBidirectionalAXMFiHearingDeviceStreaming;
pub use self::__AXHearingUtilities::{
    AXHearingDeviceEarBoth, AXHearingDeviceEarLeft, AXHearingDeviceEarNone, AXHearingDeviceEarRight,
};
pub use self::__AXSettings::AXAnimatedImagesEnabled;
pub use self::__AXSettings::AXAnimatedImagesEnabledDidChangeNotification;
pub use self::__AXSettings::AXPrefersHorizontalTextLayout;
pub use self::__AXSettings::AXPrefersHorizontalTextLayoutDidChangeNotification;
