//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Accessibility::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait AXChart: NSObjectProtocol {
        #[cfg(feature = "Accessibility_AXChartDescriptor")]
        #[method_id(@__retain_semantics Other accessibilityChartDescriptor)]
        unsafe fn accessibilityChartDescriptor(&self) -> Option<Id<AXChartDescriptor>>;

        #[cfg(feature = "Accessibility_AXChartDescriptor")]
        #[method(setAccessibilityChartDescriptor:)]
        unsafe fn setAccessibilityChartDescriptor(
            &self,
            accessibility_chart_descriptor: Option<&AXChartDescriptor>,
        );
    }

    unsafe impl ProtocolType for dyn AXChart {}
);

extern_protocol!(
    pub unsafe trait AXDataAxisDescriptor {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedTitle)]
        unsafe fn attributedTitle(&self) -> Id<NSAttributedString>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedTitle:)]
        unsafe fn setAttributedTitle(&self, attributed_title: &NSAttributedString);
    }

    unsafe impl ProtocolType for dyn AXDataAxisDescriptor {}
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum AXNumericDataAxisDescriptorScale {
        AXScaleTypeLinear = 0,
        AXScaleTypeLog10 = 1,
        AXScaleTypeLn = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Accessibility_AXNumericDataAxisDescriptor")]
    pub struct AXNumericDataAxisDescriptor;

    #[cfg(feature = "Accessibility_AXNumericDataAxisDescriptor")]
    unsafe impl ClassType for AXNumericDataAxisDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Accessibility_AXNumericDataAxisDescriptor")]
unsafe impl AXDataAxisDescriptor for AXNumericDataAxisDescriptor {}

#[cfg(feature = "Accessibility_AXNumericDataAxisDescriptor")]
unsafe impl NSCopying for AXNumericDataAxisDescriptor {}

#[cfg(feature = "Accessibility_AXNumericDataAxisDescriptor")]
unsafe impl NSObjectProtocol for AXNumericDataAxisDescriptor {}

extern_methods!(
    #[cfg(feature = "Accessibility_AXNumericDataAxisDescriptor")]
    unsafe impl AXNumericDataAxisDescriptor {
        #[method(scaleType)]
        pub unsafe fn scaleType(&self) -> AXNumericDataAxisDescriptorScale;

        #[method(setScaleType:)]
        pub unsafe fn setScaleType(&self, scale_type: AXNumericDataAxisDescriptorScale);

        #[method(lowerBound)]
        pub unsafe fn lowerBound(&self) -> c_double;

        #[method(setLowerBound:)]
        pub unsafe fn setLowerBound(&self, lower_bound: c_double);

        #[method(upperBound)]
        pub unsafe fn upperBound(&self) -> c_double;

        #[method(setUpperBound:)]
        pub unsafe fn setUpperBound(&self, upper_bound: c_double);

        #[cfg(feature = "Foundation_NSString")]
        #[method(valueDescriptionProvider)]
        pub unsafe fn valueDescriptionProvider(
            &self,
        ) -> NonNull<Block<(c_double,), NonNull<NSString>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setValueDescriptionProvider:)]
        pub unsafe fn setValueDescriptionProvider(
            &self,
            value_description_provider: &Block<(c_double,), NonNull<NSString>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other gridlinePositions)]
        pub unsafe fn gridlinePositions(&self) -> Id<NSArray<NSNumber>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method(setGridlinePositions:)]
        pub unsafe fn setGridlinePositions(&self, gridline_positions: &NSArray<NSNumber>);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSNumber",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithTitle:lowerBound:upperBound:gridlinePositions:valueDescriptionProvider:)]
        pub unsafe fn initWithTitle_lowerBound_upperBound_gridlinePositions_valueDescriptionProvider(
            this: Option<Allocated<Self>>,
            title: &NSString,
            lowerbound: c_double,
            upper_bound: c_double,
            gridline_positions: Option<&NSArray<NSNumber>>,
            value_description_provider: &Block<(c_double,), NonNull<NSString>>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSNumber",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithAttributedTitle:lowerBound:upperBound:gridlinePositions:valueDescriptionProvider:)]
        pub unsafe fn initWithAttributedTitle_lowerBound_upperBound_gridlinePositions_valueDescriptionProvider(
            this: Option<Allocated<Self>>,
            attributed_title: &NSAttributedString,
            lowerbound: c_double,
            upper_bound: c_double,
            gridline_positions: Option<&NSArray<NSNumber>>,
            value_description_provider: &Block<(c_double,), NonNull<NSString>>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Accessibility_AXCategoricalDataAxisDescriptor")]
    pub struct AXCategoricalDataAxisDescriptor;

    #[cfg(feature = "Accessibility_AXCategoricalDataAxisDescriptor")]
    unsafe impl ClassType for AXCategoricalDataAxisDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Accessibility_AXCategoricalDataAxisDescriptor")]
unsafe impl AXDataAxisDescriptor for AXCategoricalDataAxisDescriptor {}

#[cfg(feature = "Accessibility_AXCategoricalDataAxisDescriptor")]
unsafe impl NSCopying for AXCategoricalDataAxisDescriptor {}

#[cfg(feature = "Accessibility_AXCategoricalDataAxisDescriptor")]
unsafe impl NSObjectProtocol for AXCategoricalDataAxisDescriptor {}

extern_methods!(
    #[cfg(feature = "Accessibility_AXCategoricalDataAxisDescriptor")]
    unsafe impl AXCategoricalDataAxisDescriptor {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other categoryOrder)]
        pub unsafe fn categoryOrder(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setCategoryOrder:)]
        pub unsafe fn setCategoryOrder(&self, category_order: &NSArray<NSString>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithTitle:categoryOrder:)]
        pub unsafe fn initWithTitle_categoryOrder(
            this: Option<Allocated<Self>>,
            title: &NSString,
            category_order: &NSArray<NSString>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithAttributedTitle:categoryOrder:)]
        pub unsafe fn initWithAttributedTitle_categoryOrder(
            this: Option<Allocated<Self>>,
            attributed_title: &NSAttributedString,
            category_order: &NSArray<NSString>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Accessibility_AXDataPointValue")]
    pub struct AXDataPointValue;

    #[cfg(feature = "Accessibility_AXDataPointValue")]
    unsafe impl ClassType for AXDataPointValue {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Accessibility_AXDataPointValue")]
unsafe impl NSCopying for AXDataPointValue {}

#[cfg(feature = "Accessibility_AXDataPointValue")]
unsafe impl NSObjectProtocol for AXDataPointValue {}

extern_methods!(
    #[cfg(feature = "Accessibility_AXDataPointValue")]
    unsafe impl AXDataPointValue {
        #[method(number)]
        pub unsafe fn number(&self) -> c_double;

        #[method(setNumber:)]
        pub unsafe fn setNumber(&self, number: c_double);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other category)]
        pub unsafe fn category(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCategory:)]
        pub unsafe fn setCategory(&self, category: &NSString);

        #[method_id(@__retain_semantics Other valueWithNumber:)]
        pub unsafe fn valueWithNumber(number: c_double) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other valueWithCategory:)]
        pub unsafe fn valueWithCategory(category: &NSString) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Accessibility_AXDataPoint")]
    pub struct AXDataPoint;

    #[cfg(feature = "Accessibility_AXDataPoint")]
    unsafe impl ClassType for AXDataPoint {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Accessibility_AXDataPoint")]
unsafe impl NSCopying for AXDataPoint {}

#[cfg(feature = "Accessibility_AXDataPoint")]
unsafe impl NSObjectProtocol for AXDataPoint {}

extern_methods!(
    #[cfg(feature = "Accessibility_AXDataPoint")]
    unsafe impl AXDataPoint {
        #[cfg(feature = "Accessibility_AXDataPointValue")]
        #[method_id(@__retain_semantics Other xValue)]
        pub unsafe fn xValue(&self) -> Id<AXDataPointValue>;

        #[cfg(feature = "Accessibility_AXDataPointValue")]
        #[method(setXValue:)]
        pub unsafe fn setXValue(&self, x_value: &AXDataPointValue);

        #[cfg(feature = "Accessibility_AXDataPointValue")]
        #[method_id(@__retain_semantics Other yValue)]
        pub unsafe fn yValue(&self) -> Option<Id<AXDataPointValue>>;

        #[cfg(feature = "Accessibility_AXDataPointValue")]
        #[method(setYValue:)]
        pub unsafe fn setYValue(&self, y_value: Option<&AXDataPointValue>);

        #[cfg(all(
            feature = "Accessibility_AXDataPointValue",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other additionalValues)]
        pub unsafe fn additionalValues(&self) -> Id<NSArray<AXDataPointValue>>;

        #[cfg(all(
            feature = "Accessibility_AXDataPointValue",
            feature = "Foundation_NSArray"
        ))]
        #[method(setAdditionalValues:)]
        pub unsafe fn setAdditionalValues(&self, additional_values: &NSArray<AXDataPointValue>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: Option<&NSString>);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedLabel)]
        pub unsafe fn attributedLabel(&self) -> Option<Id<NSAttributedString>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedLabel:)]
        pub unsafe fn setAttributedLabel(&self, attributed_label: Option<&NSAttributedString>);

        #[cfg(feature = "Accessibility_AXDataPointValue")]
        #[method_id(@__retain_semantics Init initWithX:y:)]
        pub unsafe fn initWithX_y(
            this: Option<Allocated<Self>>,
            x_value: &AXDataPointValue,
            y_value: Option<&AXDataPointValue>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Accessibility_AXDataPointValue",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Init initWithX:y:additionalValues:)]
        pub unsafe fn initWithX_y_additionalValues(
            this: Option<Allocated<Self>>,
            x_value: &AXDataPointValue,
            y_value: Option<&AXDataPointValue>,
            additional_values: Option<&NSArray<AXDataPointValue>>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Accessibility_AXDataPointValue",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithX:y:additionalValues:label:)]
        pub unsafe fn initWithX_y_additionalValues_label(
            this: Option<Allocated<Self>>,
            x_value: &AXDataPointValue,
            y_value: Option<&AXDataPointValue>,
            additional_values: Option<&NSArray<AXDataPointValue>>,
            label: Option<&NSString>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Accessibility_AXDataSeriesDescriptor")]
    pub struct AXDataSeriesDescriptor;

    #[cfg(feature = "Accessibility_AXDataSeriesDescriptor")]
    unsafe impl ClassType for AXDataSeriesDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Accessibility_AXDataSeriesDescriptor")]
unsafe impl NSCopying for AXDataSeriesDescriptor {}

#[cfg(feature = "Accessibility_AXDataSeriesDescriptor")]
unsafe impl NSObjectProtocol for AXDataSeriesDescriptor {}

extern_methods!(
    #[cfg(feature = "Accessibility_AXDataSeriesDescriptor")]
    unsafe impl AXDataSeriesDescriptor {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedName)]
        pub unsafe fn attributedName(&self) -> Id<NSAttributedString>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedName:)]
        pub unsafe fn setAttributedName(&self, attributed_name: &NSAttributedString);

        #[method(isContinuous)]
        pub unsafe fn isContinuous(&self) -> bool;

        #[method(setIsContinuous:)]
        pub unsafe fn setIsContinuous(&self, is_continuous: bool);

        #[cfg(all(feature = "Accessibility_AXDataPoint", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other dataPoints)]
        pub unsafe fn dataPoints(&self) -> Id<NSArray<AXDataPoint>>;

        #[cfg(all(feature = "Accessibility_AXDataPoint", feature = "Foundation_NSArray"))]
        #[method(setDataPoints:)]
        pub unsafe fn setDataPoints(&self, data_points: &NSArray<AXDataPoint>);

        #[cfg(all(
            feature = "Accessibility_AXDataPoint",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithName:isContinuous:dataPoints:)]
        pub unsafe fn initWithName_isContinuous_dataPoints(
            this: Option<Allocated<Self>>,
            name: &NSString,
            is_continuous: bool,
            data_points: &NSArray<AXDataPoint>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Accessibility_AXDataPoint",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSAttributedString"
        ))]
        #[method_id(@__retain_semantics Init initWithAttributedName:isContinuous:dataPoints:)]
        pub unsafe fn initWithAttributedName_isContinuous_dataPoints(
            this: Option<Allocated<Self>>,
            attributed_name: &NSAttributedString,
            is_continuous: bool,
            data_points: &NSArray<AXDataPoint>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum AXChartDescriptorContentDirection {
        AXChartContentDirectionLeftToRight = 0,
        AXChartContentDirectionRightToLeft = 1,
        AXChartContentDirectionTopToBottom = 2,
        AXChartContentDirectionBottomToTop = 3,
        AXChartContentDirectionRadialClockwise = 4,
        AXChartContentDirectionRadialCounterClockwise = 5,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Accessibility_AXChartDescriptor")]
    pub struct AXChartDescriptor;

    #[cfg(feature = "Accessibility_AXChartDescriptor")]
    unsafe impl ClassType for AXChartDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Accessibility_AXChartDescriptor")]
unsafe impl NSCopying for AXChartDescriptor {}

#[cfg(feature = "Accessibility_AXChartDescriptor")]
unsafe impl NSObjectProtocol for AXChartDescriptor {}

extern_methods!(
    #[cfg(feature = "Accessibility_AXChartDescriptor")]
    unsafe impl AXChartDescriptor {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Option<Id<NSAttributedString>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributed_title: Option<&NSAttributedString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other summary)]
        pub unsafe fn summary(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSummary:)]
        pub unsafe fn setSummary(&self, summary: Option<&NSString>);

        #[method(contentDirection)]
        pub unsafe fn contentDirection(&self) -> AXChartDescriptorContentDirection;

        #[method(setContentDirection:)]
        pub unsafe fn setContentDirection(
            &self,
            content_direction: AXChartDescriptorContentDirection,
        );

        #[method(contentFrame)]
        pub unsafe fn contentFrame(&self) -> CGRect;

        #[method(setContentFrame:)]
        pub unsafe fn setContentFrame(&self, content_frame: CGRect);

        #[cfg(all(
            feature = "Accessibility_AXDataSeriesDescriptor",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other series)]
        pub unsafe fn series(&self) -> Id<NSArray<AXDataSeriesDescriptor>>;

        #[cfg(all(
            feature = "Accessibility_AXDataSeriesDescriptor",
            feature = "Foundation_NSArray"
        ))]
        #[method(setSeries:)]
        pub unsafe fn setSeries(&self, series: &NSArray<AXDataSeriesDescriptor>);

        #[method_id(@__retain_semantics Other xAxis)]
        pub unsafe fn xAxis(&self) -> Id<ProtocolObject<dyn AXDataAxisDescriptor>>;

        #[method(setXAxis:)]
        pub unsafe fn setXAxis(&self, x_axis: &ProtocolObject<dyn AXDataAxisDescriptor>);

        #[cfg(feature = "Accessibility_AXNumericDataAxisDescriptor")]
        #[method_id(@__retain_semantics Other yAxis)]
        pub unsafe fn yAxis(&self) -> Option<Id<AXNumericDataAxisDescriptor>>;

        #[cfg(feature = "Accessibility_AXNumericDataAxisDescriptor")]
        #[method(setYAxis:)]
        pub unsafe fn setYAxis(&self, y_axis: Option<&AXNumericDataAxisDescriptor>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other additionalAxes)]
        pub unsafe fn additionalAxes(
            &self,
        ) -> Option<Id<NSArray<ProtocolObject<dyn AXDataAxisDescriptor>>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setAdditionalAxes:)]
        pub unsafe fn setAdditionalAxes(
            &self,
            additional_axes: Option<&NSArray<ProtocolObject<dyn AXDataAxisDescriptor>>>,
        );

        #[cfg(all(
            feature = "Accessibility_AXDataSeriesDescriptor",
            feature = "Accessibility_AXNumericDataAxisDescriptor",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithTitle:summary:xAxisDescriptor:yAxisDescriptor:series:)]
        pub unsafe fn initWithTitle_summary_xAxisDescriptor_yAxisDescriptor_series(
            this: Option<Allocated<Self>>,
            title: Option<&NSString>,
            summary: Option<&NSString>,
            x_axis: &ProtocolObject<dyn AXDataAxisDescriptor>,
            y_axis: Option<&AXNumericDataAxisDescriptor>,
            series: &NSArray<AXDataSeriesDescriptor>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Accessibility_AXDataSeriesDescriptor",
            feature = "Accessibility_AXNumericDataAxisDescriptor",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithAttributedTitle:summary:xAxisDescriptor:yAxisDescriptor:series:)]
        pub unsafe fn initWithAttributedTitle_summary_xAxisDescriptor_yAxisDescriptor_series(
            this: Option<Allocated<Self>>,
            attributed_title: Option<&NSAttributedString>,
            summary: Option<&NSString>,
            x_axis: &ProtocolObject<dyn AXDataAxisDescriptor>,
            y_axis: &AXNumericDataAxisDescriptor,
            series: &NSArray<AXDataSeriesDescriptor>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Accessibility_AXDataSeriesDescriptor",
            feature = "Accessibility_AXNumericDataAxisDescriptor",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithTitle:summary:xAxisDescriptor:yAxisDescriptor:additionalAxes:series:)]
        pub unsafe fn initWithTitle_summary_xAxisDescriptor_yAxisDescriptor_additionalAxes_series(
            this: Option<Allocated<Self>>,
            title: Option<&NSString>,
            summary: Option<&NSString>,
            x_axis: &ProtocolObject<dyn AXDataAxisDescriptor>,
            y_axis: Option<&AXNumericDataAxisDescriptor>,
            additional_axes: Option<&NSArray<ProtocolObject<dyn AXDataAxisDescriptor>>>,
            series: &NSArray<AXDataSeriesDescriptor>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Accessibility_AXDataSeriesDescriptor",
            feature = "Accessibility_AXNumericDataAxisDescriptor",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithAttributedTitle:summary:xAxisDescriptor:yAxisDescriptor:additionalAxes:series:)]
        pub unsafe fn initWithAttributedTitle_summary_xAxisDescriptor_yAxisDescriptor_additionalAxes_series(
            this: Option<Allocated<Self>>,
            attributed_title: Option<&NSAttributedString>,
            summary: Option<&NSString>,
            x_axis: &ProtocolObject<dyn AXDataAxisDescriptor>,
            y_axis: Option<&AXNumericDataAxisDescriptor>,
            additional_axes: Option<&NSArray<ProtocolObject<dyn AXDataAxisDescriptor>>>,
            series: &NSArray<AXDataSeriesDescriptor>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Accessibility_AXLiveAudioGraph")]
    pub struct AXLiveAudioGraph;

    #[cfg(feature = "Accessibility_AXLiveAudioGraph")]
    unsafe impl ClassType for AXLiveAudioGraph {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Accessibility_AXLiveAudioGraph")]
unsafe impl NSObjectProtocol for AXLiveAudioGraph {}

extern_methods!(
    #[cfg(feature = "Accessibility_AXLiveAudioGraph")]
    unsafe impl AXLiveAudioGraph {
        #[method(start)]
        pub unsafe fn start();

        #[method(updateValue:)]
        pub unsafe fn updateValue(value: c_double);

        #[method(stop)]
        pub unsafe fn stop();
    }
);
