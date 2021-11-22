use anyhow::Result;
use rslint_parser::{ast::JsBinaryExpression, AstNode};

use crate::{Action, Analyzer, AnalyzerContext, AnalyzerSignal, Replacement, Signal};

#[derive(Default)]
pub struct SwapCond {}

impl Analyzer for SwapCond {
	fn analyze(&self, ctx: &AnalyzerContext) -> Result<AnalyzerSignal> {
		let range = ctx.cursor_range.unwrap_or_default();
		ctx.query_nodes_in_range::<JsBinaryExpression>(range)
			.filter_map(|node| {
				let lhs = node.left().ok()?;
				let rhs = node.right()?;
				let replacements = vec![
					Replacement {
						old: lhs.syntax().clone().into(),
						new: rhs.syntax().clone().into(),
					},
					Replacement {
						old: rhs.syntax().clone().into(),
						new: lhs.syntax().clone().into(),
					},
				];
				let signal = Signal::Action(Action {
					title: "rome: swap BinExp".into(),
					replacements,
					diagnostics: vec![],
				});
				Some(signal)
			})
			.collect()
	}
}
