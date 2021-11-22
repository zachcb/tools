use lsp_types::TextEdit;
use rome_analyze::Replacement;
use rslint_parser::{TextRange, TextSize};

use crate::line_index::{LineCol, LineIndex};

pub(crate) fn position(line_index: &LineIndex, offset: TextSize) -> lsp_types::Position {
	let line_col = line_index.line_col(offset);
	lsp_types::Position::new(line_col.line, line_col.col)
}

pub(crate) fn range(line_index: &LineIndex, range: TextRange) -> lsp_types::Range {
	let start = position(line_index, range.start());
	let end = position(line_index, range.end());
	lsp_types::Range::new(start, end)
}

pub(crate) fn offset(line_index: &LineIndex, position: lsp_types::Position) -> TextSize {
	let line_col = LineCol {
		line: position.line as u32,
		col: position.character as u32,
	};
	line_index.offset(line_col)
}

pub(crate) fn text_range(line_index: &LineIndex, range: lsp_types::Range) -> TextRange {
	let start = offset(line_index, range.start);
	let end = offset(line_index, range.end);
	TextRange::new(start, end)
}

pub(crate) fn text_edit(line_index: &LineIndex, replacement: &Replacement) -> TextEdit {
	let text_range = replacement.old.text_range();
	let range = range(line_index, text_range);
	let new_text = match &replacement.new {
		rslint_parser::NodeOrToken::Node(it) => it.text().into(),
		rslint_parser::NodeOrToken::Token(it) => it.text().into(),
	};
	TextEdit { range, new_text }
}
