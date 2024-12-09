//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-foundation")]
use objc2_core_foundation::*;
use objc2_foundation::*;
#[cfg(feature = "objc2-quartz-core")]
#[cfg(not(target_os = "watchos"))]
use objc2_quartz_core::*;

use crate::*;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipickerview?language=objc)
    #[unsafe(super(UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    pub struct UIPickerView;
);

#[cfg(all(
    feature = "UIResponder",
    feature = "UIView",
    feature = "objc2-quartz-core"
))]
#[cfg(not(target_os = "watchos"))]
unsafe impl CALayerDelegate for UIPickerView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSCoding for UIPickerView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl NSObjectProtocol for UIPickerView {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearance for UIPickerView {}

#[cfg(all(feature = "UIAppearance", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIAppearanceContainer for UIPickerView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UICoordinateSpace for UIPickerView {}

#[cfg(all(
    feature = "UIDynamicBehavior",
    feature = "UIResponder",
    feature = "UIView"
))]
unsafe impl UIDynamicItem for UIPickerView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusEnvironment for UIPickerView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItem for UIPickerView {}

#[cfg(all(feature = "UIFocus", feature = "UIResponder", feature = "UIView"))]
unsafe impl UIFocusItemContainer for UIPickerView {}

#[cfg(all(feature = "UIResponder", feature = "UIView"))]
unsafe impl UIResponderStandardEditActions for UIPickerView {}

#[cfg(all(
    feature = "UIResponder",
    feature = "UITraitCollection",
    feature = "UIView"
))]
unsafe impl UITraitEnvironment for UIPickerView {}

extern_methods!(
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIPickerView {
        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn UIPickerViewDataSource>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDataSource:)]
        pub unsafe fn setDataSource(
            &self,
            data_source: Option<&ProtocolObject<dyn UIPickerViewDataSource>>,
        );

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self)
            -> Option<Retained<ProtocolObject<dyn UIPickerViewDelegate>>>;

        /// This is a [weak property][objc2::topics::weak_property].
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UIPickerViewDelegate>>,
        );

        #[deprecated = "This property has no effect on iOS 7 and later."]
        #[method(showsSelectionIndicator)]
        pub unsafe fn showsSelectionIndicator(&self) -> bool;

        #[deprecated = "This property has no effect on iOS 7 and later."]
        #[method(setShowsSelectionIndicator:)]
        pub unsafe fn setShowsSelectionIndicator(&self, shows_selection_indicator: bool);

        #[method(numberOfComponents)]
        pub unsafe fn numberOfComponents(&self) -> NSInteger;

        #[method(numberOfRowsInComponent:)]
        pub unsafe fn numberOfRowsInComponent(&self, component: NSInteger) -> NSInteger;

        #[cfg(feature = "objc2-core-foundation")]
        #[method(rowSizeForComponent:)]
        pub unsafe fn rowSizeForComponent(&self, component: NSInteger) -> CGSize;

        #[method_id(@__retain_semantics Other viewForRow:forComponent:)]
        pub unsafe fn viewForRow_forComponent(
            &self,
            row: NSInteger,
            component: NSInteger,
        ) -> Option<Retained<UIView>>;

        #[method(reloadAllComponents)]
        pub unsafe fn reloadAllComponents(&self);

        #[method(reloadComponent:)]
        pub unsafe fn reloadComponent(&self, component: NSInteger);

        #[method(selectRow:inComponent:animated:)]
        pub unsafe fn selectRow_inComponent_animated(
            &self,
            row: NSInteger,
            component: NSInteger,
            animated: bool,
        );

        #[method(selectedRowInComponent:)]
        pub unsafe fn selectedRowInComponent(&self, component: NSInteger) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `UIView`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIPickerView {
        #[cfg(feature = "objc2-core-foundation")]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame: CGRect) -> Retained<Self>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Allocated<Self>,
            coder: &NSCoder,
        ) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(all(feature = "UIResponder", feature = "UIView"))]
    unsafe impl UIPickerView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipickerviewdatasource?language=objc)
    pub unsafe trait UIPickerViewDataSource: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(numberOfComponentsInPickerView:)]
        unsafe fn numberOfComponentsInPickerView(&self, picker_view: &UIPickerView) -> NSInteger;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[method(pickerView:numberOfRowsInComponent:)]
        unsafe fn pickerView_numberOfRowsInComponent(
            &self,
            picker_view: &UIPickerView,
            component: NSInteger,
        ) -> NSInteger;
    }

    unsafe impl ProtocolType for dyn UIPickerViewDataSource {}
);

extern_protocol!(
    /// [Apple's documentation](https://developer.apple.com/documentation/uikit/uipickerviewdelegate?language=objc)
    pub unsafe trait UIPickerViewDelegate: NSObjectProtocol + MainThreadOnly {
        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[optional]
        #[method(pickerView:widthForComponent:)]
        unsafe fn pickerView_widthForComponent(
            &self,
            picker_view: &UIPickerView,
            component: NSInteger,
        ) -> CGFloat;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIView",
            feature = "objc2-core-foundation"
        ))]
        #[optional]
        #[method(pickerView:rowHeightForComponent:)]
        unsafe fn pickerView_rowHeightForComponent(
            &self,
            picker_view: &UIPickerView,
            component: NSInteger,
        ) -> CGFloat;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method_id(@__retain_semantics Other pickerView:titleForRow:forComponent:)]
        unsafe fn pickerView_titleForRow_forComponent(
            &self,
            picker_view: &UIPickerView,
            row: NSInteger,
            component: NSInteger,
        ) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method_id(@__retain_semantics Other pickerView:attributedTitleForRow:forComponent:)]
        unsafe fn pickerView_attributedTitleForRow_forComponent(
            &self,
            picker_view: &UIPickerView,
            row: NSInteger,
            component: NSInteger,
        ) -> Option<Retained<NSAttributedString>>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method_id(@__retain_semantics Other pickerView:viewForRow:forComponent:reusingView:)]
        unsafe fn pickerView_viewForRow_forComponent_reusingView(
            &self,
            picker_view: &UIPickerView,
            row: NSInteger,
            component: NSInteger,
            view: Option<&UIView>,
        ) -> Retained<UIView>;

        #[cfg(all(feature = "UIResponder", feature = "UIView"))]
        #[optional]
        #[method(pickerView:didSelectRow:inComponent:)]
        unsafe fn pickerView_didSelectRow_inComponent(
            &self,
            picker_view: &UIPickerView,
            row: NSInteger,
            component: NSInteger,
        );
    }

    unsafe impl ProtocolType for dyn UIPickerViewDelegate {}
);
