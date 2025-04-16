use std::path::PathBuf;

use anyhow::Result;
use pyo3::prelude::*;

use pyo3_stub_gen::define_stub_info_gatherer;
use pyo3_stub_gen::derive::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[gen_stub_pyfunction]
#[pyfunction]
fn to_hash_identifier(s: String) -> Result<String> {
    tsg::graph::to_hash_identifier(&s, None)
}

#[gen_stub_pyfunction]
#[pyfunction]
fn summary_graph(path: PathBuf) -> Result<String> {
    use tsg::graph::TSGraphAnalysis;
    let graph = tsg::graph::TSGraph::from_file(path)?;
    let summary = graph.summarize()?;
    Ok(summary.to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn tsgraph(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(to_hash_identifier, m)?)?;
    m.add_function(wrap_pyfunction!(summary_graph, m)?)?;
    Ok(())
}

// Define a function to gather stub information.
define_stub_info_gatherer!(stub_info);
