use std::error::Error;

use rome_analyze::{AnalysisHost, FileId};

pub fn main() -> Result<(), Box<dyn Error + 'static + Send + Sync>> {
	let path = std::env::args().skip(1).next().ok_or("Missing filename")?;

	let src = std::fs::read_to_string(path)?;
	let mut host = AnalysisHost::new();
	host.set_file_text(FileId(0), src);
	let signal = host.analyze(FileId(0), None)?;

	dbg!(signal);

	Ok(())
}
