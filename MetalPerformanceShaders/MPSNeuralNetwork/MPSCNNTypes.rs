//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnlosstype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSCNNLossType(pub u32);
impl MPSCNNLossType {
    #[doc(alias = "MPSCNNLossTypeMeanAbsoluteError")]
    pub const MeanAbsoluteError: Self = Self(0);
    #[doc(alias = "MPSCNNLossTypeMeanSquaredError")]
    pub const MeanSquaredError: Self = Self(1);
    #[doc(alias = "MPSCNNLossTypeSoftMaxCrossEntropy")]
    pub const SoftMaxCrossEntropy: Self = Self(2);
    #[doc(alias = "MPSCNNLossTypeSigmoidCrossEntropy")]
    pub const SigmoidCrossEntropy: Self = Self(3);
    #[doc(alias = "MPSCNNLossTypeCategoricalCrossEntropy")]
    pub const CategoricalCrossEntropy: Self = Self(4);
    #[doc(alias = "MPSCNNLossTypeHinge")]
    pub const Hinge: Self = Self(5);
    #[doc(alias = "MPSCNNLossTypeHuber")]
    pub const Huber: Self = Self(6);
    #[doc(alias = "MPSCNNLossTypeCosineDistance")]
    pub const CosineDistance: Self = Self(7);
    #[doc(alias = "MPSCNNLossTypeLog")]
    pub const Log: Self = Self(8);
    #[doc(alias = "MPSCNNLossTypeKullbackLeiblerDivergence")]
    pub const KullbackLeiblerDivergence: Self = Self(9);
    #[doc(alias = "MPSCNNLossTypeCount")]
    pub const Count: Self = Self(10);
}

unsafe impl Encode for MPSCNNLossType {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for MPSCNNLossType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpscnnreductiontype?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MPSCNNReductionType(pub i32);
impl MPSCNNReductionType {
    #[doc(alias = "MPSCNNReductionTypeNone")]
    pub const None: Self = Self(0);
    #[doc(alias = "MPSCNNReductionTypeSum")]
    pub const Sum: Self = Self(1);
    #[doc(alias = "MPSCNNReductionTypeMean")]
    pub const Mean: Self = Self(2);
    #[doc(alias = "MPSCNNReductionTypeSumByNonZeroWeights")]
    pub const SumByNonZeroWeights: Self = Self(3);
    #[doc(alias = "MPSCNNReductionTypeCount")]
    pub const Count: Self = Self(4);
}

unsafe impl Encode for MPSCNNReductionType {
    const ENCODING: Encoding = i32::ENCODING;
}

unsafe impl RefEncode for MPSCNNReductionType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}