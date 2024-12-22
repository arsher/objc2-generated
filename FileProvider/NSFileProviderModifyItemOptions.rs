//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/fileprovider/nsfileprovidermodifyitemoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSFileProviderModifyItemOptions(pub NSUInteger);
bitflags::bitflags! {
    impl NSFileProviderModifyItemOptions: NSUInteger {
/// We're moving the item to a location where it may refer to an item that already exists. This may happen
/// when two directories are being merged together. When this happens some items may be merged to the
/// same directory and we end up in a situation where the merged contains may also exist.
///
/// This is similar to NSFileProviderCreateItemMayAlreadyExist
        const NSFileProviderModifyItemMayAlreadyExist = 1<<0;
    }
}

unsafe impl Encode for NSFileProviderModifyItemOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NSFileProviderModifyItemOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
