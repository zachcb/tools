use crate::{
	format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsSetterObjectMember;

impl ToFormatElement for JsSetterObjectMember {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_token(&self.set_token()?)?,
			space_token(),
			formatter.format_node(self.name()?)?,
			formatter.format_token(&self.l_paren_token()?)?,
			formatter.format_node(self.parameter()?)?,
			formatter.format_token(&self.r_paren_token()?)?,
			space_token(),
			formatter.format_node(self.body()?)?
		])
	}
}
