mod analyzers;
mod salsa_db;

use std::sync::Arc;

use anyhow::Result;
use rslint_parser::parse_text;
use rslint_parser::{AstNode, SyntaxElement, SyntaxNode, TextRange};
pub use salsa_db::RootDatabase;
pub use salsa_db::SourceDatabase;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub struct FileId(pub usize);

#[derive(Debug, Clone)]
pub struct Diagnostic {
	pub range: TextRange,
	pub message: String,
}

#[derive(Debug, Clone)]
pub struct Action {
	pub title: String,
	pub replacements: Vec<Replacement>,
	pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone)]
pub struct Replacement {
	pub old: SyntaxElement,
	pub new: SyntaxElement,
}

trait Analyzer {
	fn analyze(&self, ctx: &AnalyzerContext) -> AnalyzerResult;
}

#[derive(Default, Debug)]
pub struct AnalyzerSignal {
	pub diagnostics: Vec<Diagnostic>,
	pub actions: Vec<Action>,
}

impl AnalyzerSignal {
	fn add_diagnostic(&mut self, d: Diagnostic) {
		self.diagnostics.push(d);
	}

	// fn add_simple_diagnostic(&mut self, range: TextRange, message: impl Into<String>) {
	// 	self.diagnostics.push(Diagnostic {
	// 		range,
	// 		message: message.into(),
	// 	});
	// }

	fn add_action(&mut self, a: Action) {
		self.actions.push(a);
	}
}

type AnalyzerResult = Result<AnalyzerSignal>;

impl FromIterator<Signal> for Result<AnalyzerSignal> {
	fn from_iter<T: IntoIterator<Item = Signal>>(iter: T) -> Self {
		let mut diagnostics = Vec::new();
		let mut actions = Vec::new();
		for signal in iter {
			match signal {
				Signal::Diagnostic(it) => diagnostics.push(it),
				Signal::Action(it) => actions.push(it),
			}
		}
		Ok(AnalyzerSignal {
			diagnostics,
			actions,
		})
	}
}

impl FromIterator<AnalyzerSignal> for AnalyzerSignal {
	fn from_iter<T: IntoIterator<Item = AnalyzerSignal>>(iter: T) -> Self {
		let mut diagnostics = Vec::new();
		let mut actions = Vec::new();
		for signal in iter {
			diagnostics.extend(signal.diagnostics);
			actions.extend(signal.actions);
		}
		AnalyzerSignal {
			diagnostics,
			actions,
		}
	}
}

struct AnalyzerContext<'a> {
	file_id: FileId,
	cursor_range: Option<TextRange>,
	host: &'a AnalysisHost,
}

impl<'a> AnalyzerContext<'a> {
	fn new(host: &'a AnalysisHost, cursor_range: Option<TextRange>, file_id: FileId) -> Self {
		Self {
			cursor_range,
			host,
			file_id,
		}
	}

	fn query_nodes<T: AstNode>(&self) -> impl Iterator<Item = T> {
		self.host.query_nodes(self.file_id)
	}

	fn query_nodes_in_range<T: AstNode>(&self, range: TextRange) -> impl Iterator<Item = T> {
		self.host.query_nodes_in_range(self.file_id, range)
	}
}

#[derive(Debug)]
pub enum Signal {
	Diagnostic(Diagnostic),
	Action(Action),
}

impl From<Diagnostic> for Signal {
	fn from(d: Diagnostic) -> Self {
		Self::Diagnostic(d)
	}
}

impl From<Action> for Signal {
	fn from(a: Action) -> Self {
		Self::Action(a)
	}
}

#[derive(Default)]
pub struct AnalysisHost {
	db: RootDatabase,
}

impl AnalysisHost {
	pub fn new() -> Self {
		Self {
			db: Default::default(),
		}
	}

	pub fn set_file_text(&mut self, file_id: FileId, text: impl Into<Arc<String>>) {
		self.db.set_file_text(file_id, text.into())
	}
}

// These should be on a database snapshot, but the current database contains
// non-Send SyntaxNodes. Should either make SyntaxNodes Send (using atomics)
// or remove SyntaxNode as a possible query parameter/return type.
impl AnalysisHost {
	pub fn get_file_text(&self, file_id: FileId) -> Arc<String> {
		self.db.file_text(file_id)
	}

	pub fn parse(&self, file_id: FileId) -> SyntaxNode {
		let text = self.db.file_text(file_id);
		parse_text(&text, file_id.0).syntax()
	}

	pub fn query_nodes<T: AstNode>(&self, file_id: FileId) -> impl Iterator<Item = T> {
		// eprintln!("Querying Nodes!, {:?}", std::any::type_name::<T>());
		let nodes = self.db.nodes(file_id, T::can_cast);
		let len = nodes.len();
		(0..len).filter_map(move |i| T::cast(nodes[i].clone()))
	}

	pub fn query_nodes_in_range<T: AstNode>(
		&self,
		file_id: FileId,
		range: TextRange,
	) -> impl Iterator<Item = T> {
		// eprintln!("Querying Nodes in range!, {:?}", std::any::type_name::<T>());
		let nodes = self.db.nodes_in_range(file_id, T::can_cast, range);
		let len = nodes.len();
		(0..len).filter_map(move |i| T::cast(nodes[i].clone()))
	}

	pub fn analyze(&self, file_id: FileId, cursor_range: Option<TextRange>) -> AnalyzerResult {
		let all_caps = analyzers::all_caps::AllCaps;
		let double_eq = analyzers::double_eq::DoubleEq::default();
		let swap_cond = analyzers::swap_cond::SwapCond::default();
		let analyzers: Vec<&dyn Analyzer> = vec![&all_caps, &double_eq, &swap_cond];

		let ctx = AnalyzerContext::new(self, cursor_range, file_id);

		analyzers.iter().map(|a| a.analyze(&ctx)).collect()
	}
}
