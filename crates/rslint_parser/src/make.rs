use crate::SyntaxKind;

use crate::ast;
use crate::AstNode;
use crate::SyntaxToken;
use crate::SyntaxTreeBuilder;

pub fn ast_from_text<N: AstNode>(text: &str) -> N {
	let parse = crate::parse_text(text, 0);
	let node = match parse.tree().syntax().descendants().find_map(N::cast) {
		Some(it) => it,
		None => {
			panic!(
				"Failed to make ast node `{}` from text {}",
				std::any::type_name::<N>(),
				text
			)
		}
	};
	let node = node.clone_subtree();
	assert_eq!(node.syntax().text_range().start(), 0.into());
	node
}

pub fn token_from_text(kind: SyntaxKind, text: &str) -> SyntaxToken {
	let mut builder = SyntaxTreeBuilder::new();
	builder.start_node(SyntaxKind::JS_ROOT);
	builder.token(kind, text);
	builder.finish_node();
	let node = builder.finish();
	node.first_token().unwrap()
}

pub fn name(text: &str) -> ast::Name {
	ast_from_text(&format!("function {}(){{}}", text))
}
