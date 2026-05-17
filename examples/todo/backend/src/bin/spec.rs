//! This binary is used to generate the OpenAPI spec for the API.
//! It is used by the `generate-api-spec` task in the `justfile` to generate the spec before
//! generating the API client.
use std::io::Write;

use color_eyre::eyre::Result;

use groom_example_todo_backend::controller::make_spec;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut stdout = std::io::stdout();
    stdout.write_all(make_spec()?.get().as_bytes())?;
    stdout.flush()?;
    
    Ok(())
}
