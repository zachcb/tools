use anyhow::Result;
use rslint_parser::{ast::JsBinaryExpression, make, SyntaxToken};

use crate::{Action, Analyzer, AnalyzerContext, AnalyzerSignal, Diagnostic, Replacement};

#[derive(Default)]
pub struct DoubleEq;

impl Analyzer for DoubleEq {
	fn analyze(&self, ctx: &AnalyzerContext) -> Result<AnalyzerSignal> {
		let mut signal = AnalyzerSignal::default();

		for node in ctx
			.query_nodes::<JsBinaryExpression>()
			.filter(|n| n.operator().ok().filter(|op| op.text() == "==").is_some())
		{
			if let Ok(op) = node.operator() {
				let range = op.text_range();
				let message = format!("rome: do not use == operator");
				let diag = Diagnostic { range, message };
				signal.add_diagnostic(diag.clone());

				if ctx.cursor_range.map(|r| range.contains_range(r)).is_some() {
					let token: SyntaxToken = make::token_from_text(op.kind(), "===");

					let replacements = vec![Replacement {
						old: node.operator().unwrap().into(),
						new: token.into(),
					}];
					let action = Action {
						title: "rome: Change to ===".into(),
						replacements,
						diagnostics: vec![diag],
					};
					signal.add_action(action)
				}
			}
		}

		Ok(signal)
	}
}
