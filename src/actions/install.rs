
use eyre::Result;

#[cfg(feature = "install")]
use rust_embed::RustEmbed;

#[cfg(feature = "install")]
use std::io::Write;

#[cfg(feature = "install")]
#[derive(RustEmbed)]
#[folder = "workflow"]
struct Workflow;

#[cfg(feature = "install")]
pub fn run() -> Result<()> {
    // create a zip file of the workflow

    use std::fs::remove_file;

    let mut zip = zip::ZipWriter::new(std::fs::File::create("github.alfredworkflow")?);
    for file in Workflow::iter() {
        let path = file.as_ref();
        let file = Workflow::get(path).unwrap();
        zip.start_file(path, zip::write::FileOptions::default())?;
        zip.write_all(file.data.as_ref())?;
    }

    open::that("github.alfredworkflow")?;
    remove_file("github.alfredworkflow")?;

    Ok(())
}

#[cfg(not(feature = "install"))]
pub async fn run() -> Result<()> {
    println!("install feature not enabled");
    Ok(())
}
