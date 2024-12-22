//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use core::ffi::*;
use core::ptr::NonNull;
use objc2::__framework_prelude::*;
#[cfg(feature = "objc2-core-audio-types")]
use objc2_core_audio_types::*;

use crate::*;

/// A reference to an AUGraph object.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/augraph?language=objc)
pub type AUGraph = *mut c_void;

/// Used to represent a member of an AUGraph
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/aunode?language=objc)
pub type AUNode = i32;

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaugrapherr_nodenotfound?language=objc)
pub const kAUGraphErr_NodeNotFound: OSStatus = -10860;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaugrapherr_invalidconnection?language=objc)
pub const kAUGraphErr_InvalidConnection: OSStatus = -10861;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaugrapherr_outputnodeerr?language=objc)
pub const kAUGraphErr_OutputNodeErr: OSStatus = -10862;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaugrapherr_cannotdoincurrentcontext?language=objc)
pub const kAUGraphErr_CannotDoInCurrentContext: OSStatus = -10863;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaugrapherr_invalidaudiounit?language=objc)
pub const kAUGraphErr_InvalidAudioUnit: OSStatus = -10864;

extern "C-unwind" {
    /// Create a new AUGraph
    ///
    ///
    /// Parameter `outGraph`: the new AUGraph object
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn NewAUGraph(out_graph: NonNull<AUGraph>) -> OSStatus;
}

extern "C-unwind" {
    /// Dispose an AUGraph
    ///
    /// Deallocates the AUGraph along with its nodes and their resources.
    ///
    ///
    /// Parameter `inGraph`: the AUGraph object to be disposed
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn DisposeAUGraph(in_graph: AUGraph) -> OSStatus;
}

extern "C-unwind" {
    /// Add a node to an AUGraph
    ///
    /// Creates a node in the graph that is an AudioUnit, using the supplied
    /// AudioComponentDescription to find and open that unit.
    ///
    ///
    /// Parameter `inGraph`: the AUGraph object
    ///
    /// Parameter `inDescription`: the AudioComponentDescription used to find and open the AudioUnit
    ///
    /// Parameter `outNode`: the newly added node
    #[cfg(feature = "AudioComponent")]
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphAddNode(
        in_graph: AUGraph,
        in_description: NonNull<AudioComponentDescription>,
        out_node: NonNull<AUNode>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Remove a node from an AUGraph
    ///
    /// Nodes can be removed from any thread context. The output node of
    /// the AUGraph cannot be removed while the graph is running.
    ///
    ///
    /// Parameter `inGraph`: the AUGraph object
    ///
    /// Parameter `inNode`: the node to be removed
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphRemoveNode(in_graph: AUGraph, in_node: AUNode) -> OSStatus;
}

extern "C-unwind" {
    /// The number of nodes in an AUGraph
    ///
    ///
    /// Parameter `inGraph`: the AUGraph object
    ///
    /// Parameter `outNumberOfNodes`: the number of nodes
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphGetNodeCount(in_graph: AUGraph, out_number_of_nodes: NonNull<u32>) -> OSStatus;
}

extern "C-unwind" {
    /// Returns the node at a given index
    ///
    /// By using AUGraphGetNodeCount in conjunction with this call, you can
    /// iterate through the nodes of an AUGraph.
    ///
    ///
    /// Parameter `inGraph`: the AUGraph object
    ///
    /// Parameter `inIndex`: the index of the node to retrieve
    ///
    /// Parameter `outNode`: the node at that index
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphGetIndNode(
        in_graph: AUGraph,
        in_index: u32,
        out_node: NonNull<AUNode>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Returns information about a particular AUNode
    ///
    /// You can pass in NULL for any of the out parameters if you're not interested
    /// in that value.
    ///
    ///
    /// Parameter `inGraph`: the AUGraph object
    ///
    /// Parameter `inNode`: the node to query
    ///
    /// Parameter `outDescription`: the component description that would describe the AudioUnit of this node
    ///
    /// Parameter `outAudioUnit`: the AudioUnit of this node
    #[cfg(all(feature = "AUComponent", feature = "AudioComponent"))]
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphNodeInfo(
        in_graph: AUGraph,
        in_node: AUNode,
        out_description: *mut AudioComponentDescription,
        out_audio_unit: *mut AudioUnit,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Create a node that will represent a sub graph
    ///
    /// This will create a node that represents a contained or member AUGraph.
    /// The AUGraph can be retrieved through the GetNodeInfoSubGraph call.
    /// The member AUGraph is owned by the parent graph and will be disposed when
    /// either:
    /// 1. The parent graph is disposed
    /// 2. The node is removed from the parent AUGraph
    ///
    ///
    /// Parameter `inGraph`: the AUGraph object
    ///
    /// Parameter `outNode`: the node that is used to refer to the sub-graph
    #[deprecated = "no longer supported"]
    pub fn AUGraphNewNodeSubGraph(in_graph: AUGraph, out_node: NonNull<AUNode>) -> OSStatus;
}

extern "C-unwind" {
    /// Return an AUGraph represented by this node
    ///
    /// This will return the sub graph represented by this AUNode.
    ///
    ///
    /// Parameter `inGraph`: the AUGraph object
    ///
    /// Parameter `inNode`: the node to query
    ///
    /// Parameter `outSubGraph`: the sub-graph
    #[deprecated = "no longer supported"]
    pub fn AUGraphGetNodeInfoSubGraph(
        in_graph: AUGraph,
        in_node: AUNode,
        out_sub_graph: NonNull<AUGraph>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Determine whether the node represents a sub graph
    ///
    /// This will return true if the specified node represents a subgraph, false if not.
    ///
    ///
    /// Parameter `inGraph`: the AUGraph object
    ///
    /// Parameter `inNode`: the node to query
    ///
    /// Parameter `outFlag`: true if the node is a subgraph, false if not
    #[deprecated = "no longer supported"]
    pub fn AUGraphIsNodeSubGraph(
        in_graph: AUGraph,
        in_node: AUNode,
        out_flag: NonNull<Boolean>,
    ) -> OSStatus;
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaunodeinteraction_connection?language=objc)
pub const kAUNodeInteraction_Connection: u32 = 1;
/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/kaunodeinteraction_inputcallback?language=objc)
pub const kAUNodeInteraction_InputCallback: u32 = 2;

/// A connection between two nodes
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/audiounitnodeconnection?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioUnitNodeConnection {
    pub sourceNode: AUNode,
    pub sourceOutputNumber: u32,
    pub destNode: AUNode,
    pub destInputNumber: u32,
}

unsafe impl Encode for AudioUnitNodeConnection {
    const ENCODING: Encoding = Encoding::Struct(
        "AudioUnitNodeConnection",
        &[
            <AUNode>::ENCODING,
            <u32>::ENCODING,
            <AUNode>::ENCODING,
            <u32>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for AudioUnitNodeConnection {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/aunodeconnection?language=objc)
pub type AUNodeConnection = AudioUnitNodeConnection;

/// A callback used to provide input to an audio unit
///
/// Used to contain information when a callback is used
/// to provide input to the specific node's specified input
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/audiotoolbox/aunoderendercallback?language=objc)
#[cfg(all(
    feature = "AUComponent",
    feature = "AudioUnitProperties",
    feature = "objc2-core-audio-types"
))]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUNodeRenderCallback {
    pub destNode: AUNode,
    pub destInputNumber: AudioUnitElement,
    pub cback: AURenderCallbackStruct,
}

#[cfg(all(
    feature = "AUComponent",
    feature = "AudioUnitProperties",
    feature = "objc2-core-audio-types"
))]
unsafe impl Encode for AUNodeRenderCallback {
    const ENCODING: Encoding = Encoding::Struct(
        "AUNodeRenderCallback",
        &[
            <AUNode>::ENCODING,
            <AudioUnitElement>::ENCODING,
            <AURenderCallbackStruct>::ENCODING,
        ],
    );
}

#[cfg(all(
    feature = "AUComponent",
    feature = "AudioUnitProperties",
    feature = "objc2-core-audio-types"
))]
unsafe impl RefEncode for AUNodeRenderCallback {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern "C-unwind" {
    /// connect a node's output to a node's input
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphConnectNodeInput(
        in_graph: AUGraph,
        in_source_node: AUNode,
        in_source_output_number: u32,
        in_dest_node: AUNode,
        in_dest_input_number: u32,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Set a callback for the specified node's specified input.
    ///
    /// Parameter `inInputCallback`: The callback that will provide input data to the node
    #[cfg(all(
        feature = "AUComponent",
        feature = "AudioUnitProperties",
        feature = "objc2-core-audio-types"
    ))]
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphSetNodeInputCallback(
        in_graph: AUGraph,
        in_dest_node: AUNode,
        in_dest_input_number: u32,
        in_input_callback: NonNull<AURenderCallbackStruct>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// disconnect a node's input
    ///
    /// This can be used to disconnect either a connection or callback interaction
    /// to the specified node input
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphDisconnectNodeInput(
        in_graph: AUGraph,
        in_dest_node: AUNode,
        in_dest_input_number: u32,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// clear all of the interactions in a graph
    ///
    /// This will clear all connections and callback interactions of the nodes of a graph
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphClearConnections(in_graph: AUGraph) -> OSStatus;
}

extern "C-unwind" {
    /// Retrieve the number of interactions of a graph
    ///
    /// The number of node interactions currently being managed by the graph
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphGetNumberOfInteractions(
        in_graph: AUGraph,
        out_num_interactions: NonNull<u32>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Retrieve the number of interactions of a graph's node
    ///
    /// The number of node interactions currently being managed by the graph for the specified node.
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphCountNodeInteractions(
        in_graph: AUGraph,
        in_node: AUNode,
        out_num_interactions: NonNull<u32>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Updates the state of a running AUGraph
    ///
    /// Call this after performing a series of "edits" on the AUGraph with calls
    /// such as AUGraphConnectNodeInput() to finalize those edits.
    /// The call will be synchronous if outIsUpdated is NULL,
    /// meaning it will block until the changes are incorporated
    /// into the graph OR an error is returned from the update process
    /// if outIsUpdated is non-NULL, then AUGraphUpdate() will return
    /// immediately and outIsUpdated will equal "true" if the changes
    /// were already made (no more changes to make) or "false" if changes are still
    /// outstanding.
    ///
    /// Calling from the AUGraphRenderNotification callback:
    /// Connection and Disconnection events can be completely processed
    /// in the render notification callback of the AUGraph
    /// Nodes can also be removed (except for the output node or a sub-graph of the AUGraph)
    /// as well.
    ///
    /// Getting kAUGraphErr_CannotDoInCurrentContext as a result code:
    /// If AUGraphUpdate returns this result, then it means it was
    /// unable to process the update, due to an inability to safely
    /// alter the state of the graph (because another thread was accessing
    /// a call that relies on the graph's state having integrity).
    /// This result code is only a transitory state, which will pass as soon
    /// as your other thread's call to AUGraph (that has the lock) completes.
    ///
    /// If an error is encountered in the process of an update (say an invalid connection
    /// is attempted, a disconnection between nodes that are not connected, etc) on a running graph,
    /// then the call will return that error code. It only process events whilst it receives
    /// no error results. Thus, if an error is encountered, other events will not be
    /// processed until AUGraphUpdate is called again. This is done, in cases where
    /// the state of the graph could become inconsistent if further events were processed, so
    /// this decision is left up to you. The same applies to the "cant do" error - you have
    /// to explicitly call AUGraphUpdate again to have the processing of the events occur.
    ///
    ///
    /// Parameter `outIsUpdated`: if specified returns true if all of the edits were applied to the graph
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphUpdate(in_graph: AUGraph, out_is_updated: *mut Boolean) -> OSStatus;
}

extern "C-unwind" {
    /// Open a graph
    ///
    /// AudioUnits are open but not initialized (no resource allocation occurs here)
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphOpen(in_graph: AUGraph) -> OSStatus;
}

extern "C-unwind" {
    /// Close a graph
    ///
    /// All AudioUnits are closed - leaving only its nodal representation
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphClose(in_graph: AUGraph) -> OSStatus;
}

extern "C-unwind" {
    /// Initialise a graph
    ///
    /// AudioUnitInitialize() is called on each opened node/AudioUnit
    /// (get ready to render) and SubGraph that are involved in a
    /// interaction. If the node is not involved, it is initialised
    /// after it becomes involved in an interaction.
    ///
    /// A graph must be opened before it can be initialised.
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphInitialize(in_graph: AUGraph) -> OSStatus;
}

extern "C-unwind" {
    /// Uninitialise a graph
    ///
    /// The member of the graph are uninitialised
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphUninitialize(in_graph: AUGraph) -> OSStatus;
}

extern "C-unwind" {
    /// Start a graph
    ///
    /// Start() is called on the "head" node(s) of the AUGraph (now rendering starts)
    ///
    /// The graph must be initialised before it can be started.
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphStart(in_graph: AUGraph) -> OSStatus;
}

extern "C-unwind" {
    /// Stop a graph
    ///
    /// Stop() is called on the "head" node(s) of the AUGraph    (rendering is stopped)
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphStop(in_graph: AUGraph) -> OSStatus;
}

extern "C-unwind" {
    /// Is the graph open
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphIsOpen(in_graph: AUGraph, out_is_open: NonNull<Boolean>) -> OSStatus;
}

extern "C-unwind" {
    /// Is the graph initialised
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphIsInitialized(
        in_graph: AUGraph,
        out_is_initialized: NonNull<Boolean>,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Is the graph running (has it been started)
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphIsRunning(in_graph: AUGraph, out_is_running: NonNull<Boolean>) -> OSStatus;
}

extern "C-unwind" {
    /// The CPU load of the graph
    ///
    /// Returns a short-term running average of the current CPU load of the graph.
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphGetCPULoad(in_graph: AUGraph, out_average_cpu_load: NonNull<f32>) -> OSStatus;
}

extern "C-unwind" {
    /// The Maximum CPU load of the graph
    ///
    /// Returns the max CPU load of the graph since this call was last made or the graph was last
    /// started.
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphGetMaxCPULoad(in_graph: AUGraph, out_max_load: NonNull<f32>) -> OSStatus;
}

extern "C-unwind" {
    /// Add a notification callback
    ///
    /// Add a callback that the graph will call every time the graph renders. The callback will be
    /// called once before the graph's render operation, and once after the render operation is
    /// complete.
    #[cfg(all(feature = "AUComponent", feature = "objc2-core-audio-types"))]
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphAddRenderNotify(
        in_graph: AUGraph,
        in_callback: AURenderCallback,
        in_ref_con: *mut c_void,
    ) -> OSStatus;
}

extern "C-unwind" {
    /// Remove a notification callback
    ///
    /// Remove a previously added callback. You must provide both the callback and the refCon that was
    /// used previously to add the callback.
    #[cfg(all(feature = "AUComponent", feature = "objc2-core-audio-types"))]
    #[deprecated = "AUGraph is deprecated in favor of AVAudioEngine"]
    pub fn AUGraphRemoveRenderNotify(
        in_graph: AUGraph,
        in_callback: AURenderCallback,
        in_ref_con: *mut c_void,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[deprecated = "no longer supported"]
    pub fn AUGraphGetNumberOfConnections(
        in_graph: AUGraph,
        out_num_connections: NonNull<u32>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[deprecated = "no longer supported"]
    pub fn AUGraphGetConnectionInfo(
        in_graph: AUGraph,
        in_connection_index: u32,
        out_source_node: NonNull<AUNode>,
        out_source_output_number: NonNull<u32>,
        out_dest_node: NonNull<AUNode>,
        out_dest_input_number: NonNull<u32>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[deprecated = "no longer supported"]
    pub fn AUGraphCountNodeConnections(
        in_graph: AUGraph,
        in_node: AUNode,
        out_num_connections: NonNull<u32>,
    ) -> OSStatus;
}

extern "C-unwind" {
    #[deprecated = "no longer supported"]
    pub fn AUGraphGetNodeConnections(
        in_graph: AUGraph,
        in_node: AUNode,
        out_connections: NonNull<AudioUnitNodeConnection>,
        io_num_connections: NonNull<u32>,
    ) -> OSStatus;
}
