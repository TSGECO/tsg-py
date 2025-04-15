use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    let stub = tsgraph::stub_info()?;
    stub.generate()?;
    Ok(())
}
