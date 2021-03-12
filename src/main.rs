use biodivine_attractor_phenotypes::*;
use biodivine_lib_param_bn::symbolic_async_graph::SymbolicAsyncGraph;
use biodivine_lib_param_bn::BooleanNetwork;
use std::collections::HashSet;
use std::convert::TryFrom;

fn main() {
    let mut cmd_arguments = std::env::args();
    cmd_arguments.next().unwrap(); // Skip first binary path argument.
    let model_file_path = cmd_arguments.next().unwrap_or_else(|| {
        panic!("First argument must be a path to .aeon model file.");
    });
    let phenotypes_file_path = cmd_arguments.next().unwrap_or_else(|| {
        panic!("Second argument must be a path to a phenotype specification file.")
    });

    let model_string = std::fs::read_to_string(&model_file_path).unwrap_or_else(|error| {
        panic!("Model file {} is not readable: {}", model_file_path, error);
    });

    let boolean_network = BooleanNetwork::try_from(model_string.as_str()).unwrap_or_else(|error| {
        panic!("Invalid .aeon model: {}", error);
    });

    let network_variable_names = boolean_network
        .variables()
        .map(|id| boolean_network.get_variable_name(id).clone())
        .collect::<HashSet<_>>();

    let asynchronous_transition_graph = SymbolicAsyncGraph::new(boolean_network).unwrap();

    let attractors = aeon::compute_attractors(&asynchronous_transition_graph);

    let phenotypes = std::fs::read_to_string(&phenotypes_file_path)
        .unwrap_or_else(|error| {
            panic!(
                "Phenotype specification file {} is not readable: {}",
                phenotypes_file_path, error
            );
        })
        .lines()
        .map(|line| {
            read_phenotype_formula(line, &network_variable_names).unwrap_or_else(|error| {
                panic!("Invalid phenotype specification: {}", error);
            })
        })
        .collect::<Vec<_>>();

    for attractor in &attractors {
        println!(
            "Analyse attractor with {} colors.",
            attractor.colors().approx_cardinality()
        );

        for specification in &phenotypes {
            let satisfying_colors = test_phenotype(specification, attractor);
            println!(
                "\t - Phenotype satisfied for {} colors.",
                satisfying_colors.approx_cardinality()
            );
        }
    }
}
