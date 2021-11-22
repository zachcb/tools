use std::error::Error;

use rslint_parser::parse_text;

pub fn main() -> Result<(), Box<dyn Error + 'static + Send + Sync>> {
	let path = std::env::args().skip(1).next().ok_or("Missing filename")?;

	let src = std::fs::read_to_string(path)?;
	let parse = parse_text(&src, 0);
	let tree = parse.tree();
	dbg!(tree);

	Ok(())
}
