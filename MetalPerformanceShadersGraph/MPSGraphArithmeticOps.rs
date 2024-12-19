//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_methods!(
    /// MPSGraphArithmeticOps
    #[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
    unsafe impl MPSGraph {
        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other identityWithTensor:name:)]
        pub unsafe fn identityWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other exponentWithTensor:name:)]
        pub unsafe fn exponentWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other exponentBase2WithTensor:name:)]
        pub unsafe fn exponentBase2WithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other exponentBase10WithTensor:name:)]
        pub unsafe fn exponentBase10WithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other logarithmWithTensor:name:)]
        pub unsafe fn logarithmWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other logarithmBase2WithTensor:name:)]
        pub unsafe fn logarithmBase2WithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other logarithmBase10WithTensor:name:)]
        pub unsafe fn logarithmBase10WithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other squareWithTensor:name:)]
        pub unsafe fn squareWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other squareRootWithTensor:name:)]
        pub unsafe fn squareRootWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reciprocalSquareRootWithTensor:name:)]
        pub unsafe fn reciprocalSquareRootWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[deprecated]
        #[method_id(@__retain_semantics Other reverseSquareRootWithTensor:name:)]
        pub unsafe fn reverseSquareRootWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other reciprocalWithTensor:name:)]
        pub unsafe fn reciprocalWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other absoluteWithTensor:name:)]
        pub unsafe fn absoluteWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other absoluteSquareWithTensor:name:)]
        pub unsafe fn absoluteSquareWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other negativeWithTensor:name:)]
        pub unsafe fn negativeWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other signWithTensor:name:)]
        pub unsafe fn signWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other signbitWithTensor:name:)]
        pub unsafe fn signbitWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other ceilWithTensor:name:)]
        pub unsafe fn ceilWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other floorWithTensor:name:)]
        pub unsafe fn floorWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other roundWithTensor:name:)]
        pub unsafe fn roundWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other rintWithTensor:name:)]
        pub unsafe fn rintWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other sinWithTensor:name:)]
        pub unsafe fn sinWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other cosWithTensor:name:)]
        pub unsafe fn cosWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other tanWithTensor:name:)]
        pub unsafe fn tanWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other sinhWithTensor:name:)]
        pub unsafe fn sinhWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other coshWithTensor:name:)]
        pub unsafe fn coshWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other tanhWithTensor:name:)]
        pub unsafe fn tanhWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other asinWithTensor:name:)]
        pub unsafe fn asinWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other acosWithTensor:name:)]
        pub unsafe fn acosWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other atanWithTensor:name:)]
        pub unsafe fn atanWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other asinhWithTensor:name:)]
        pub unsafe fn asinhWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other acoshWithTensor:name:)]
        pub unsafe fn acoshWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other atanhWithTensor:name:)]
        pub unsafe fn atanhWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other notWithTensor:name:)]
        pub unsafe fn notWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other isInfiniteWithTensor:name:)]
        pub unsafe fn isInfiniteWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other isFiniteWithTensor:name:)]
        pub unsafe fn isFiniteWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other isNaNWithTensor:name:)]
        pub unsafe fn isNaNWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other erfWithTensor:name:)]
        pub unsafe fn erfWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other truncateWithTensor:name:)]
        pub unsafe fn truncateWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other bitwiseNOTWithTensor:name:)]
        pub unsafe fn bitwiseNOTWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other bitwisePopulationCountWithTensor:name:)]
        pub unsafe fn bitwisePopulationCountWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other conjugateWithTensor:name:)]
        pub unsafe fn conjugateWithTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other additionWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn additionWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other subtractionWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn subtractionWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other multiplicationWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn multiplicationWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other divisionWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn divisionWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other moduloWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn moduloWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other powerWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn powerWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other minimumWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn minimumWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other maximumWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn maximumWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other minimumWithNaNPropagationWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn minimumWithNaNPropagationWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other maximumWithNaNPropagationWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn maximumWithNaNPropagationWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other equalWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn equalWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other notEqualWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn notEqualWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other lessThanWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn lessThanWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other lessThanOrEqualToWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn lessThanOrEqualToWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other greaterThanWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn greaterThanWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other greaterThanOrEqualToWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn greaterThanOrEqualToWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other logicalANDWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn logicalANDWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other logicalORWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn logicalORWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other logicalNANDWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn logicalNANDWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other logicalNORWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn logicalNORWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other logicalXORWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn logicalXORWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other logicalXNORWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn logicalXNORWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other atan2WithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn atan2WithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other bitwiseANDWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn bitwiseANDWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other bitwiseORWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn bitwiseORWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other bitwiseXORWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn bitwiseXORWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other bitwiseLeftShiftWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn bitwiseLeftShiftWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other bitwiseRightShiftWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn bitwiseRightShiftWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other selectWithPredicateTensor:truePredicateTensor:falsePredicateTensor:name:)]
        pub unsafe fn selectWithPredicateTensor_truePredicateTensor_falsePredicateTensor_name(
            &self,
            predicate_tensor: &MPSGraphTensor,
            true_predicate_tensor: &MPSGraphTensor,
            false_select_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other clampWithTensor:minValueTensor:maxValueTensor:name:)]
        pub unsafe fn clampWithTensor_minValueTensor_maxValueTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            min_value_tensor: &MPSGraphTensor,
            max_value_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other divisionNoNaNWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn divisionNoNaNWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other floorModuloWithPrimaryTensor:secondaryTensor:name:)]
        pub unsafe fn floorModuloWithPrimaryTensor_secondaryTensor_name(
            &self,
            primary_tensor: &MPSGraphTensor,
            secondary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other realPartOfTensor:name:)]
        pub unsafe fn realPartOfTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other imaginaryPartOfTensor:name:)]
        pub unsafe fn imaginaryPartOfTensor_name(
            &self,
            tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;

        #[cfg(feature = "MPSGraphTensor")]
        #[method_id(@__retain_semantics Other complexTensorWithRealTensor:imaginaryTensor:name:)]
        pub unsafe fn complexTensorWithRealTensor_imaginaryTensor_name(
            &self,
            real_tensor: &MPSGraphTensor,
            imaginary_tensor: &MPSGraphTensor,
            name: Option<&NSString>,
        ) -> Retained<MPSGraphTensor>;
    }
);
