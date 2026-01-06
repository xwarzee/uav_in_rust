// MBSE Test Suite Entry Point
// This module loads all MBSE traceability tests from the mbse/ subdirectory

#[path = "mbse/component_mapping_tests.rs"]
mod component_mapping_tests;

#[path = "mbse/requirements_validation_tests.rs"]
mod requirements_validation_tests;

#[path = "mbse/safety_constraints_tests.rs"]
mod safety_constraints_tests;

#[path = "mbse/traceability_matrix_tests.rs"]
mod traceability_matrix_tests;
