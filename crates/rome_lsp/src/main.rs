use anyhow::Result;

fn main() -> Result<()> {
	rome_lsp::server::run_server()
}
