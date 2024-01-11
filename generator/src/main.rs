use std::ffi::OsString;

mod openapi;

fn main() -> eyre::Result<()> {
    let spec = get_spec()?;
    dbg!(spec);
    Ok(())
}

fn get_spec() -> eyre::Result<openapi::OpenApiV2> {
    let path = std::env::var_os("FORGEJO_API_SPEC_PATH")
        .unwrap_or_else(|| OsString::from("./api_spec.json"));
    let file = std::fs::read(path)?;
    let spec = serde_json::from_slice::<openapi::OpenApiV2>(&file)?;
    Ok(spec)
}
