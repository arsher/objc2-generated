//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    /// A data class that encapsulates call stack trees vended by MetricKit.
    ///
    /// You should use the JSONRepresentation API to generate human readable call stack trees for symbolication off device.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metrickit/mxcallstacktree?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MXCallStackTree;
);

unsafe impl NSCoding for MXCallStackTree {}

unsafe impl NSObjectProtocol for MXCallStackTree {}

unsafe impl NSSecureCoding for MXCallStackTree {}

extern_methods!(
    unsafe impl MXCallStackTree {
        /// Convenience method to return a JSON representation of this callstack tree.
        ///
        /// The JSON structure of MXCallStackTree is organized into individual groups of call stacks. Individual call stacks contain stack frames, which consist of information needed to symbolicate the frame off device. This includes binary image name, binary UUID, offset in binary text segment, address, and sample count (for stack trees that contain temporally sampled data.)
        ///
        /// MXCallStackTrees can be organized into a single callstack for the entire application, or broken up into callstacks associated with individual threads.
        ///
        /// Returns: An NSData object containing the JSON representation
        #[method_id(@__retain_semantics Other JSONRepresentation)]
        pub unsafe fn JSONRepresentation(&self) -> Retained<NSData>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl MXCallStackTree {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);
