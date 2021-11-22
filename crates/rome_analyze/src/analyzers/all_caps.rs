use anyhow::Result;
use rslint_parser::{ast, AstNode};

use crate::{Analyzer, AnalyzerContext, AnalyzerSignal, Diagnostic, Signal};

#[derive(Default)]
pub struct AllCaps;

impl Analyzer for AllCaps {
	fn analyze(&self, ctx: &AnalyzerContext) -> Result<AnalyzerSignal> {
		ctx.query_nodes::<ast::JsIdentifierBinding>()
			.filter(|n| n.text().to_uppercase() == n.text())
			.map(|n| -> Signal {
				let message = format!("rome: the name {} is in all caps.", n.text());
				let range = n.syntax().text_range();
				Diagnostic { range, message }.into()
			})
			.collect()
	}
}
