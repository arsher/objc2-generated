//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[deprecated = "UISearchDisplayController has been replaced with UISearchController"]
    pub struct UISearchDisplayController;

    unsafe impl ClassType for UISearchDisplayController {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

unsafe impl NSObjectProtocol for UISearchDisplayController {}

extern_methods!(
    unsafe impl UISearchDisplayController {
        #[cfg(all(
            feature = "UIResponder",
            feature = "UISearchBar",
            feature = "UIView",
            feature = "UIViewController"
        ))]
        #[deprecated = "UISearchDisplayController has been replaced with UISearchController"]
        #[method_id(@__retain_semantics Init initWithSearchBar:contentsController:)]
        pub unsafe fn initWithSearchBar_contentsController(
            this: Allocated<Self>,
            search_bar: &UISearchBar,
            view_controller: &UIViewController,
        ) -> Id<Self>;

        #[deprecated = "UISearchDisplayController has been replaced with UISearchController"]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn UISearchDisplayDelegate>>>;

        #[deprecated = "UISearchDisplayController has been replaced with UISearchController"]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn UISearchDisplayDelegate>>,
        );

        #[deprecated = "UISearchDisplayController has been replaced with UISearchController"]
        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[deprecated = "UISearchDisplayController has been replaced with UISearchController"]
        #[method(setActive:)]
        pub unsafe fn setActive(&self, active: bool);

        #[deprecated = "UISearchDisplayController has been replaced with UISearchController"]
        #[method(setActive:animated:)]
        pub unsafe fn setActive_animated(&self, visible: bool, animated: bool);

        #[cfg(all(feature = "UIResponder", feature = "UISearchBar", feature = "UIView"))]
        #[deprecated = "UISearchDisplayController has been replaced with UISearchController"]
        #[method_id(@__retain_semantics Other searchBar)]
        pub unsafe fn searchBar(&self) -> Id<UISearchBar>;

        #[cfg(all(feature = "UIResponder", feature = "UIViewController"))]
        #[deprecated = "UISearchDisplayController has been replaced with UISearchController"]
        #[method_id(@__retain_semantics Other searchContentsController)]
        pub unsafe fn searchContentsController(&self) -> Id<UIViewController>;

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScrollView",
            feature = "UITableView",
            feature = "UIView"
        ))]
        #[deprecated = "UISearchDisplayController has been replaced with UISearchController"]
        #[method_id(@__retain_semantics Other searchResultsTableView)]
        pub unsafe fn searchResultsTableView(&self) -> Id<UITableView>;

        #[cfg(feature = "UITableView")]
        #[deprecated = "UISearchDisplayController has been replaced with UISearchController"]
        #[method_id(@__retain_semantics Other searchResultsDataSource)]
        pub unsafe fn searchResultsDataSource(
            &self,
        ) -> Option<Id<ProtocolObject<dyn UITableViewDataSource>>>;

        #[cfg(feature = "UITableView")]
        #[deprecated = "UISearchDisplayController has been replaced with UISearchController"]
        #[method(setSearchResultsDataSource:)]
        pub unsafe fn setSearchResultsDataSource(
            &self,
            search_results_data_source: Option<&ProtocolObject<dyn UITableViewDataSource>>,
        );

        #[cfg(all(feature = "UIScrollView", feature = "UITableView"))]
        #[deprecated = "UISearchDisplayController has been replaced with UISearchController"]
        #[method_id(@__retain_semantics Other searchResultsDelegate)]
        pub unsafe fn searchResultsDelegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn UITableViewDelegate>>>;

        #[cfg(all(feature = "UIScrollView", feature = "UITableView"))]
        #[deprecated = "UISearchDisplayController has been replaced with UISearchController"]
        #[method(setSearchResultsDelegate:)]
        pub unsafe fn setSearchResultsDelegate(
            &self,
            search_results_delegate: Option<&ProtocolObject<dyn UITableViewDelegate>>,
        );

        #[method_id(@__retain_semantics Other searchResultsTitle)]
        pub unsafe fn searchResultsTitle(&self) -> Option<Id<NSString>>;

        #[method(setSearchResultsTitle:)]
        pub unsafe fn setSearchResultsTitle(&self, search_results_title: Option<&NSString>);

        #[method(displaysSearchBarInNavigationBar)]
        pub unsafe fn displaysSearchBarInNavigationBar(&self) -> bool;

        #[method(setDisplaysSearchBarInNavigationBar:)]
        pub unsafe fn setDisplaysSearchBarInNavigationBar(
            &self,
            displays_search_bar_in_navigation_bar: bool,
        );

        #[cfg(feature = "UINavigationItem")]
        #[method_id(@__retain_semantics Other navigationItem)]
        pub unsafe fn navigationItem(&self) -> Option<Id<UINavigationItem>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl UISearchDisplayController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait UISearchDisplayDelegate: NSObjectProtocol + IsMainThreadOnly {
        #[deprecated]
        #[optional]
        #[method(searchDisplayControllerWillBeginSearch:)]
        unsafe fn searchDisplayControllerWillBeginSearch(
            &self,
            controller: &UISearchDisplayController,
        );

        #[deprecated]
        #[optional]
        #[method(searchDisplayControllerDidBeginSearch:)]
        unsafe fn searchDisplayControllerDidBeginSearch(
            &self,
            controller: &UISearchDisplayController,
        );

        #[deprecated]
        #[optional]
        #[method(searchDisplayControllerWillEndSearch:)]
        unsafe fn searchDisplayControllerWillEndSearch(
            &self,
            controller: &UISearchDisplayController,
        );

        #[deprecated]
        #[optional]
        #[method(searchDisplayControllerDidEndSearch:)]
        unsafe fn searchDisplayControllerDidEndSearch(
            &self,
            controller: &UISearchDisplayController,
        );

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScrollView",
            feature = "UITableView",
            feature = "UIView"
        ))]
        #[deprecated]
        #[optional]
        #[method(searchDisplayController:didLoadSearchResultsTableView:)]
        unsafe fn searchDisplayController_didLoadSearchResultsTableView(
            &self,
            controller: &UISearchDisplayController,
            table_view: &UITableView,
        );

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScrollView",
            feature = "UITableView",
            feature = "UIView"
        ))]
        #[deprecated]
        #[optional]
        #[method(searchDisplayController:willUnloadSearchResultsTableView:)]
        unsafe fn searchDisplayController_willUnloadSearchResultsTableView(
            &self,
            controller: &UISearchDisplayController,
            table_view: &UITableView,
        );

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScrollView",
            feature = "UITableView",
            feature = "UIView"
        ))]
        #[deprecated]
        #[optional]
        #[method(searchDisplayController:willShowSearchResultsTableView:)]
        unsafe fn searchDisplayController_willShowSearchResultsTableView(
            &self,
            controller: &UISearchDisplayController,
            table_view: &UITableView,
        );

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScrollView",
            feature = "UITableView",
            feature = "UIView"
        ))]
        #[deprecated]
        #[optional]
        #[method(searchDisplayController:didShowSearchResultsTableView:)]
        unsafe fn searchDisplayController_didShowSearchResultsTableView(
            &self,
            controller: &UISearchDisplayController,
            table_view: &UITableView,
        );

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScrollView",
            feature = "UITableView",
            feature = "UIView"
        ))]
        #[deprecated]
        #[optional]
        #[method(searchDisplayController:willHideSearchResultsTableView:)]
        unsafe fn searchDisplayController_willHideSearchResultsTableView(
            &self,
            controller: &UISearchDisplayController,
            table_view: &UITableView,
        );

        #[cfg(all(
            feature = "UIResponder",
            feature = "UIScrollView",
            feature = "UITableView",
            feature = "UIView"
        ))]
        #[deprecated]
        #[optional]
        #[method(searchDisplayController:didHideSearchResultsTableView:)]
        unsafe fn searchDisplayController_didHideSearchResultsTableView(
            &self,
            controller: &UISearchDisplayController,
            table_view: &UITableView,
        );

        #[deprecated]
        #[optional]
        #[method(searchDisplayController:shouldReloadTableForSearchString:)]
        unsafe fn searchDisplayController_shouldReloadTableForSearchString(
            &self,
            controller: &UISearchDisplayController,
            search_string: Option<&NSString>,
        ) -> bool;

        #[deprecated]
        #[optional]
        #[method(searchDisplayController:shouldReloadTableForSearchScope:)]
        unsafe fn searchDisplayController_shouldReloadTableForSearchScope(
            &self,
            controller: &UISearchDisplayController,
            search_option: NSInteger,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn UISearchDisplayDelegate {}
);
