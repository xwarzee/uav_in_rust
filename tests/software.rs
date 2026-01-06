// Software Test Suite Entry Point
// This module loads all software tests from the software/ subdirectory

#[path = "software/unit_tests.rs"]
mod unit_tests;

#[path = "software/integration_tests.rs"]
mod integration_tests;
