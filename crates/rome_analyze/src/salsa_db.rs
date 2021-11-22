use std::sync::Arc;

use rslint_parser::{parse_text, SyntaxKind, SyntaxNode, TextRange};

use crate::FileId;

#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase: salsa::Database {
	#[salsa::input]
	fn file_text(&self, file_id: FileId) -> Arc<String>;

	fn parse(&self, file_id: FileId) -> SyntaxNode;

	fn nodes(&self, file_id: FileId, can_cast: fn(SyntaxKind) -> bool) -> Arc<Vec<SyntaxNode>>;

	fn nodes_in_range(
		&self,
		file_id: FileId,
		can_cast: fn(SyntaxKind) -> bool,
		range: TextRange,
	) -> Arc<Vec<SyntaxNode>>;
}

fn parse(db: &dyn SourceDatabase, file_id: FileId) -> SyntaxNode {
	let text = db.file_text(file_id);
	parse_text(&text, file_id.0).syntax()
}

fn nodes(
	db: &dyn SourceDatabase,
	file_id: FileId,
	can_cast: fn(SyntaxKind) -> bool,
) -> Arc<Vec<SyntaxNode>> {
	eprintln!("Querying Salsa for Nodes!");
	let tree = db.parse(file_id);
	let nodes = tree.descendants().filter(|n| can_cast(n.kind())).collect();
	Arc::new(nodes)
}

fn nodes_in_range(
	db: &dyn SourceDatabase,
	file_id: FileId,
	can_cast: fn(SyntaxKind) -> bool,
	range: TextRange,
) -> Arc<Vec<SyntaxNode>> {
	eprintln!("Querying Salsa for Nodes in Range!");
	let nodes = db.nodes(file_id, can_cast);
	let nodes = nodes
		.iter()
		.filter(|n| n.text_range().contains_range(range))
		.cloned()
		.collect();
	Arc::new(nodes)
}

#[salsa::database(SourceDatabaseStorage)]
#[derive(Default)]
pub struct RootDatabase {
	storage: salsa::Storage<Self>,
}

impl salsa::Database for RootDatabase {}

// SyntaxNodes are not `Send`
// impl salsa::ParallelDatabase for RootDatabase {
// 	fn snapshot(&self) -> salsa::Snapshot<Self> {
// 		salsa::Snapshot::new(RootDatabase {
// 			storage: self.storage.snapshot(),
// 		})
// 	}
// }
