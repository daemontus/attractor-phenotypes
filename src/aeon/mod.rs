use crate::aeon::algo_xie_beerel::xie_beerel_attractors;
use algo_interleaved_transition_guided_reduction::interleaved_transition_guided_reduction;
use biodivine_lib_param_bn::symbolic_async_graph::{
    GraphColoredVertices, GraphColors, SymbolicAsyncGraph,
};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::{Arc, Mutex};

/// **(internal)** Utility methods for the behaviour `Class`.
mod _impl_class;
/// **(internal)** Implementation of `Behaviour` classification in `Classifier`.
mod _impl_classifier;
mod _impl_graph_task_context;
mod _impl_progress_tracker;
pub mod algo_interleaved_transition_guided_reduction;
pub mod algo_saturated_reachability;
pub mod algo_xie_beerel;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Behaviour {
    Stability,
    Oscillation,
    Disorder,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Class(Vec<Behaviour>);

pub struct Classifier {
    classes: Mutex<HashMap<Class, GraphColors>>,
    attractors: Mutex<Vec<(GraphColoredVertices, HashMap<Behaviour, GraphColors>)>>,
}

pub struct ProgressTracker {
    total: Mutex<f64>,
    remaining: Mutex<f64>,
    processes: AtomicU32,
}

/// A context object which aggregates all necessary information about a running task working with
/// a symbolic graph.
///
/// We use this to avoid passing each context variable as a (mutable) reference. It is also easier
/// to implement some utility methods this way.
pub struct GraphTaskContext {
    is_cancelled: AtomicBool,
    progress: ProgressTracker,
}

/// A helper function for computing attractors without unnecessary fluff with cancellation
/// and progress tracking.
pub fn compute_attractors(graph: &SymbolicAsyncGraph) -> Vec<GraphColoredVertices> {
    let task = GraphTaskContext::new();
    let (universe, variables) =
        interleaved_transition_guided_reduction(&task, graph, graph.mk_unit_colored_vertices());
    let result = Arc::new(Mutex::new(Vec::new()));
    xie_beerel_attractors(&task, graph, &universe, &variables, |component| {
        result.lock().unwrap().push(component)
    });
    {
        let x = result.lock().unwrap().to_vec();
        x
    } // Safely read result from a locked mutex.
}
