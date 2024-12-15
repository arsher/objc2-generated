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

#[link(name = "MetalPerformanceShadersGraph", kind = "framework")]
extern "C" {}

#[cfg(feature = "MPSGraph")]
#[path = "MPSGraph.rs"]
mod __MPSGraph;
#[cfg(feature = "MPSGraphActivationOps")]
#[path = "MPSGraphActivationOps.rs"]
mod __MPSGraphActivationOps;
#[cfg(feature = "MPSGraphArithmeticOps")]
#[path = "MPSGraphArithmeticOps.rs"]
mod __MPSGraphArithmeticOps;
#[cfg(feature = "MPSGraphAutomaticDifferentiation")]
#[path = "MPSGraphAutomaticDifferentiation.rs"]
mod __MPSGraphAutomaticDifferentiation;
#[cfg(feature = "MPSGraphCallOps")]
#[path = "MPSGraphCallOps.rs"]
mod __MPSGraphCallOps;
#[cfg(feature = "MPSGraphControlFlowOps")]
#[path = "MPSGraphControlFlowOps.rs"]
mod __MPSGraphControlFlowOps;
#[cfg(feature = "MPSGraphConvolutionOps")]
#[path = "MPSGraphConvolutionOps.rs"]
mod __MPSGraphConvolutionOps;
#[cfg(feature = "MPSGraphConvolutionTransposeOps")]
#[path = "MPSGraphConvolutionTransposeOps.rs"]
mod __MPSGraphConvolutionTransposeOps;
#[cfg(feature = "MPSGraphCore")]
#[path = "MPSGraphCore.rs"]
mod __MPSGraphCore;
#[cfg(feature = "MPSGraphCumulativeOps")]
#[path = "MPSGraphCumulativeOps.rs"]
mod __MPSGraphCumulativeOps;
#[cfg(feature = "MPSGraphDepthwiseConvolutionOps")]
#[path = "MPSGraphDepthwiseConvolutionOps.rs"]
mod __MPSGraphDepthwiseConvolutionOps;
#[cfg(feature = "MPSGraphDevice")]
#[path = "MPSGraphDevice.rs"]
mod __MPSGraphDevice;
#[cfg(feature = "MPSGraphExecutable")]
#[path = "MPSGraphExecutable.rs"]
mod __MPSGraphExecutable;
#[cfg(feature = "MPSGraphFourierTransformOps")]
#[path = "MPSGraphFourierTransformOps.rs"]
mod __MPSGraphFourierTransformOps;
#[cfg(feature = "MPSGraphGatherOps")]
#[path = "MPSGraphGatherOps.rs"]
mod __MPSGraphGatherOps;
#[cfg(feature = "MPSGraphImToColOps")]
#[path = "MPSGraphImToColOps.rs"]
mod __MPSGraphImToColOps;
#[cfg(feature = "MPSGraphLinearAlgebraOps")]
#[path = "MPSGraphLinearAlgebraOps.rs"]
mod __MPSGraphLinearAlgebraOps;
#[cfg(feature = "MPSGraphLossOps")]
#[path = "MPSGraphLossOps.rs"]
mod __MPSGraphLossOps;
#[cfg(feature = "MPSGraphMatrixInverseOps")]
#[path = "MPSGraphMatrixInverseOps.rs"]
mod __MPSGraphMatrixInverseOps;
#[cfg(feature = "MPSGraphMatrixMultiplicationOps")]
#[path = "MPSGraphMatrixMultiplicationOps.rs"]
mod __MPSGraphMatrixMultiplicationOps;
#[cfg(feature = "MPSGraphMemoryOps")]
#[path = "MPSGraphMemoryOps.rs"]
mod __MPSGraphMemoryOps;
#[cfg(feature = "MPSGraphNonMaximumSuppressionOps")]
#[path = "MPSGraphNonMaximumSuppressionOps.rs"]
mod __MPSGraphNonMaximumSuppressionOps;
#[cfg(feature = "MPSGraphNonZeroOps")]
#[path = "MPSGraphNonZeroOps.rs"]
mod __MPSGraphNonZeroOps;
#[cfg(feature = "MPSGraphNormalizationOps")]
#[path = "MPSGraphNormalizationOps.rs"]
mod __MPSGraphNormalizationOps;
#[cfg(feature = "MPSGraphOneHotOps")]
#[path = "MPSGraphOneHotOps.rs"]
mod __MPSGraphOneHotOps;
#[cfg(feature = "MPSGraphOperation")]
#[path = "MPSGraphOperation.rs"]
mod __MPSGraphOperation;
#[cfg(feature = "MPSGraphOptimizerOps")]
#[path = "MPSGraphOptimizerOps.rs"]
mod __MPSGraphOptimizerOps;
#[cfg(feature = "MPSGraphPoolingOps")]
#[path = "MPSGraphPoolingOps.rs"]
mod __MPSGraphPoolingOps;
#[cfg(feature = "MPSGraphQuantizationOps")]
#[path = "MPSGraphQuantizationOps.rs"]
mod __MPSGraphQuantizationOps;
#[cfg(feature = "MPSGraphRNNOps")]
#[path = "MPSGraphRNNOps.rs"]
mod __MPSGraphRNNOps;
#[cfg(feature = "MPSGraphRandomOps")]
#[path = "MPSGraphRandomOps.rs"]
mod __MPSGraphRandomOps;
#[cfg(feature = "MPSGraphReductionOps")]
#[path = "MPSGraphReductionOps.rs"]
mod __MPSGraphReductionOps;
#[cfg(feature = "MPSGraphResizeOps")]
#[path = "MPSGraphResizeOps.rs"]
mod __MPSGraphResizeOps;
#[cfg(feature = "MPSGraphSampleGridOps")]
#[path = "MPSGraphSampleGridOps.rs"]
mod __MPSGraphSampleGridOps;
#[cfg(feature = "MPSGraphScatterNDOps")]
#[path = "MPSGraphScatterNDOps.rs"]
mod __MPSGraphScatterNDOps;
#[cfg(feature = "MPSGraphSortOps")]
#[path = "MPSGraphSortOps.rs"]
mod __MPSGraphSortOps;
#[cfg(feature = "MPSGraphSparseOps")]
#[path = "MPSGraphSparseOps.rs"]
mod __MPSGraphSparseOps;
#[cfg(feature = "MPSGraphStencilOps")]
#[path = "MPSGraphStencilOps.rs"]
mod __MPSGraphStencilOps;
#[cfg(feature = "MPSGraphTensor")]
#[path = "MPSGraphTensor.rs"]
mod __MPSGraphTensor;
#[cfg(feature = "MPSGraphTensorData")]
#[path = "MPSGraphTensorData.rs"]
mod __MPSGraphTensorData;
#[cfg(feature = "MPSGraphTensorShapeOps")]
#[path = "MPSGraphTensorShapeOps.rs"]
mod __MPSGraphTensorShapeOps;
#[cfg(feature = "MPSGraphTopKOps")]
#[path = "MPSGraphTopKOps.rs"]
mod __MPSGraphTopKOps;

#[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
pub use self::__MPSGraph::MPSGraph;
#[cfg(all(
    feature = "MPSGraph",
    feature = "MPSGraphCore",
    feature = "MPSGraphExecutable"
))]
pub use self::__MPSGraph::MPSGraphCallableMap;
#[cfg(all(
    feature = "MPSGraph",
    feature = "MPSGraphCore",
    feature = "MPSGraphExecutable",
    feature = "block2"
))]
pub use self::__MPSGraph::MPSGraphCompilationCompletionHandler;
#[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
pub use self::__MPSGraph::MPSGraphCompilationDescriptor;
#[cfg(all(
    feature = "MPSGraph",
    feature = "MPSGraphCore",
    feature = "MPSGraphTensor",
    feature = "MPSGraphTensorData",
    feature = "block2"
))]
pub use self::__MPSGraph::MPSGraphCompletionHandler;
#[cfg(all(feature = "MPSGraph", feature = "MPSGraphCore"))]
pub use self::__MPSGraph::MPSGraphExecutionDescriptor;
#[cfg(feature = "MPSGraph")]
pub use self::__MPSGraph::MPSGraphExecutionStage;
#[cfg(feature = "MPSGraph")]
pub use self::__MPSGraph::MPSGraphOptimization;
#[cfg(feature = "MPSGraph")]
pub use self::__MPSGraph::MPSGraphOptimizationProfile;
#[cfg(feature = "MPSGraph")]
pub use self::__MPSGraph::MPSGraphOptions;
#[cfg(all(
    feature = "MPSGraph",
    feature = "MPSGraphCore",
    feature = "MPSGraphTensor",
    feature = "MPSGraphTensorData",
    feature = "block2"
))]
pub use self::__MPSGraph::MPSGraphScheduledHandler;
#[cfg(all(
    feature = "MPSGraph",
    feature = "MPSGraphCore",
    feature = "MPSGraphTensor",
    feature = "MPSGraphTensorData"
))]
pub use self::__MPSGraph::MPSGraphTensorDataDictionary;
#[cfg(all(
    feature = "MPSGraph",
    feature = "MPSGraphCore",
    feature = "MPSGraphTensor"
))]
pub use self::__MPSGraph::MPSGraphTensorShapedTypeDictionary;
#[cfg(all(
    feature = "MPSGraphControlFlowOps",
    feature = "MPSGraphCore",
    feature = "MPSGraphTensor",
    feature = "block2"
))]
pub use self::__MPSGraphControlFlowOps::MPSGraphControlFlowDependencyBlock;
#[cfg(all(
    feature = "MPSGraphControlFlowOps",
    feature = "MPSGraphCore",
    feature = "MPSGraphTensor",
    feature = "block2"
))]
pub use self::__MPSGraphControlFlowOps::MPSGraphForLoopBodyBlock;
#[cfg(all(
    feature = "MPSGraphControlFlowOps",
    feature = "MPSGraphCore",
    feature = "MPSGraphTensor",
    feature = "block2"
))]
pub use self::__MPSGraphControlFlowOps::MPSGraphIfThenElseBlock;
#[cfg(all(
    feature = "MPSGraphControlFlowOps",
    feature = "MPSGraphCore",
    feature = "MPSGraphTensor",
    feature = "block2"
))]
pub use self::__MPSGraphControlFlowOps::MPSGraphWhileAfterBlock;
#[cfg(all(
    feature = "MPSGraphControlFlowOps",
    feature = "MPSGraphCore",
    feature = "MPSGraphTensor",
    feature = "block2"
))]
pub use self::__MPSGraphControlFlowOps::MPSGraphWhileBeforeBlock;
#[cfg(all(feature = "MPSGraphConvolutionOps", feature = "MPSGraphCore"))]
pub use self::__MPSGraphConvolutionOps::MPSGraphConvolution2DOpDescriptor;
#[cfg(all(feature = "MPSGraphConvolutionOps", feature = "MPSGraphCore"))]
pub use self::__MPSGraphConvolutionOps::MPSGraphConvolution3DOpDescriptor;
#[cfg(feature = "MPSGraphCore")]
pub use self::__MPSGraphCore::MPSGraphObject;
#[cfg(feature = "MPSGraphCore")]
pub use self::__MPSGraphCore::MPSGraphPaddingMode;
#[cfg(feature = "MPSGraphCore")]
pub use self::__MPSGraphCore::MPSGraphPaddingStyle;
#[cfg(feature = "MPSGraphCore")]
pub use self::__MPSGraphCore::MPSGraphReductionMode;
#[cfg(feature = "MPSGraphCore")]
pub use self::__MPSGraphCore::MPSGraphShapedType;
#[cfg(feature = "MPSGraphCore")]
pub use self::__MPSGraphCore::MPSGraphTensorNamedDataLayout;
#[cfg(feature = "MPSGraphCore")]
pub use self::__MPSGraphCore::MPSGraphType;
#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphDepthwiseConvolutionOps"))]
pub use self::__MPSGraphDepthwiseConvolutionOps::MPSGraphDepthwiseConvolution2DOpDescriptor;
#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphDepthwiseConvolutionOps"))]
pub use self::__MPSGraphDepthwiseConvolutionOps::MPSGraphDepthwiseConvolution3DOpDescriptor;
#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphDevice"))]
pub use self::__MPSGraphDevice::MPSGraphDevice;
#[cfg(feature = "MPSGraphDevice")]
pub use self::__MPSGraphDevice::MPSGraphDeviceType;
#[cfg(feature = "MPSGraphExecutable")]
pub use self::__MPSGraphExecutable::MPSGraphDeploymentPlatform;
#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphExecutable"))]
pub use self::__MPSGraphExecutable::MPSGraphExecutable;
#[cfg(all(
    feature = "MPSGraphCore",
    feature = "MPSGraphExecutable",
    feature = "MPSGraphTensorData",
    feature = "block2"
))]
pub use self::__MPSGraphExecutable::MPSGraphExecutableCompletionHandler;
#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphExecutable"))]
pub use self::__MPSGraphExecutable::MPSGraphExecutableExecutionDescriptor;
#[cfg(all(
    feature = "MPSGraphCore",
    feature = "MPSGraphExecutable",
    feature = "MPSGraphTensorData",
    feature = "block2"
))]
pub use self::__MPSGraphExecutable::MPSGraphExecutableScheduledHandler;
#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphExecutable"))]
pub use self::__MPSGraphExecutable::MPSGraphExecutableSerializationDescriptor;
#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphFourierTransformOps"))]
pub use self::__MPSGraphFourierTransformOps::MPSGraphFFTDescriptor;
#[cfg(feature = "MPSGraphFourierTransformOps")]
pub use self::__MPSGraphFourierTransformOps::MPSGraphFFTScalingMode;
#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphImToColOps"))]
pub use self::__MPSGraphImToColOps::MPSGraphImToColOpDescriptor;
#[cfg(feature = "MPSGraphLossOps")]
pub use self::__MPSGraphLossOps::MPSGraphLossReductionType;
#[cfg(all(
    feature = "MPSGraphCore",
    feature = "MPSGraphMemoryOps",
    feature = "MPSGraphOperation"
))]
pub use self::__MPSGraphMemoryOps::MPSGraphVariableOp;
#[cfg(feature = "MPSGraphNonMaximumSuppressionOps")]
pub use self::__MPSGraphNonMaximumSuppressionOps::MPSGraphNonMaximumSuppressionCoordinateMode;
#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphOperation"))]
pub use self::__MPSGraphOperation::MPSGraphOperation;
#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphPoolingOps"))]
pub use self::__MPSGraphPoolingOps::MPSGraphPooling2DOpDescriptor;
#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphPoolingOps"))]
pub use self::__MPSGraphPoolingOps::MPSGraphPooling4DOpDescriptor;
#[cfg(feature = "MPSGraphPoolingOps")]
pub use self::__MPSGraphPoolingOps::MPSGraphPoolingReturnIndicesMode;
#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphRNNOps"))]
pub use self::__MPSGraphRNNOps::MPSGraphGRUDescriptor;
#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphRNNOps"))]
pub use self::__MPSGraphRNNOps::MPSGraphLSTMDescriptor;
#[cfg(feature = "MPSGraphRNNOps")]
pub use self::__MPSGraphRNNOps::MPSGraphRNNActivation;
#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphRNNOps"))]
pub use self::__MPSGraphRNNOps::MPSGraphSingleGateRNNDescriptor;
#[cfg(feature = "MPSGraphRandomOps")]
pub use self::__MPSGraphRandomOps::MPSGraphRandomDistribution;
#[cfg(feature = "MPSGraphRandomOps")]
pub use self::__MPSGraphRandomOps::MPSGraphRandomNormalSamplingMethod;
#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphRandomOps"))]
pub use self::__MPSGraphRandomOps::MPSGraphRandomOpDescriptor;
#[cfg(feature = "MPSGraphResizeOps")]
pub use self::__MPSGraphResizeOps::MPSGraphResizeMode;
#[cfg(feature = "MPSGraphResizeOps")]
pub use self::__MPSGraphResizeOps::MPSGraphResizeNearestRoundingMode;
#[cfg(feature = "MPSGraphScatterNDOps")]
pub use self::__MPSGraphScatterNDOps::MPSGraphScatterMode;
#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphSparseOps"))]
pub use self::__MPSGraphSparseOps::MPSGraphCreateSparseOpDescriptor;
#[cfg(feature = "MPSGraphSparseOps")]
pub use self::__MPSGraphSparseOps::MPSGraphSparseStorageType;
#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphStencilOps"))]
pub use self::__MPSGraphStencilOps::MPSGraphStencilOpDescriptor;
#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphTensor"))]
pub use self::__MPSGraphTensor::MPSGraphTensor;
#[cfg(all(feature = "MPSGraphCore", feature = "MPSGraphTensorData"))]
pub use self::__MPSGraphTensorData::MPSGraphTensorData;
