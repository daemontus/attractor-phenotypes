use biodivine_lib_param_bn::{VariableId, BinaryOp};
use biodivine_lib_param_bn::symbolic_async_graph::{GraphColoredVertices, GraphColors, SymbolicContext};
use std::collections::HashSet;

/// A copy of aeon attractor algorithms.
///
/// We copy the algorithms instead of declaring Aeon as dependency to avoid
/// adding additional transitive dependencies on HTTP server and Json libraries used
/// in Aeon, which are not needed here and would slow down the build process.
pub mod aeon;

/*
    This file is the root of the library. Here, we can implement the actual functionality.
    (Although, it would be best to eventually separate the implementation into smaller sub-modules)

    Note that the underscore (_) prefix on all function parameters is just to avoid warnings
    about unused parameters.
 */

enum PhenotypeFormula {
    // The recursive data-type for the formula will have a similar declaration
    VarTrue(VariableId),
    Negation(Box<PhenotypeFormula>),
    Binary(BinaryOp, Box<PhenotypeFormula>, Box<PhenotypeFormula>),
}

/// This function reads a phenotype specification from the given string into the
/// `PhenotypeFormula` data type. In case of error, a string with human readable
/// error description is returned instead.
///
/// The function also takes a hash set of variable names which are currently declared in the
/// corresponding model.
pub fn read_phenotype_formula(_formula: &str, _model_variables: &HashSet<String>) -> Result<PhenotypeFormula, String> {
    unimplemented!("TODO");
}

/// This function takes a phenotype specification and an attractor (given as a parametrised
/// set of vertices) and produces a set of colors (parametrisation set) for which the
/// attractor satisfies given specification.
///
/// The function will also probably need either a reference to `SymbolicContext` (or
/// `SymbolicAsyncGraph`) for mapping between Boolean network variables and Bdd variables
/// (or symbolic sets). But we will add this when we have a better idea how the algorithm
/// will look.
pub fn test_phenotype(_phenotype: &PhenotypeFormula, _attractor: GraphColoredVertices) -> GraphColors {
    unimplemented!("TODO");
}

/*
    You can use this to include unit-tests in the module file.
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xyz() {
        assert!(true);  // Some condition
    }
}
