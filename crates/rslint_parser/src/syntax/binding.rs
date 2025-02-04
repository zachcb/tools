use crate::parser::{expected_any, ToDiagnostic};
use crate::syntax::class::parse_equal_value_clause;
use crate::syntax::expr::{is_at_identifier, parse_identifier};
use crate::syntax::js_parse_error::{
	expected_binding, expected_identifier, expected_object_member_name,
};
use crate::syntax::object::{is_at_object_member_name, parse_object_member_name};
use crate::syntax::pattern::{ParseArrayPattern, ParseObjectPattern, ParseWithDefaultPattern};
use crate::ConditionalParsedSyntax::{Invalid, Valid};
use crate::JsSyntaxFeature::StrictMode;
use crate::ParsedSyntax::{Absent, Present};
use crate::{SyntaxKind::*, *};
use rslint_errors::Span;

pub(crate) fn parse_binding(p: &mut Parser) -> ParsedSyntax {
	match p.cur() {
		T!['['] => ArrayBinding.parse_array_pattern(p),
		T!['{'] if p.state.allow_object_expr => ObjectBindingPattern.parse_object_pattern(p),
		T![ident] | T![yield] | T![await] => {
			parse_identifier_binding(p).or_invalid_to_unknown(p, JS_UNKNOWN_BINDING)
		}
		_ => Absent,
	}
}

pub(crate) fn parse_binding_with_optional_default(p: &mut Parser) -> ParsedSyntax {
	BindingWithDefault.parse_pattern_with_optional_default(p)
}

fn is_at_identifier_binding(p: &Parser) -> bool {
	is_at_identifier(p)
}

// test_err binding_identifier_invalid
// async () => { let await = 5; }
// function *foo() {
//    let yield = 5;
// }
// let eval = 5;
// let let = 5;
// const let = 5;
// let a, a;
//
// test_err binding_identifier_invalid_script
// // SCRIPT
// let let = 5;
// const let = 5;
/// Parses an identifier binding or returns an invalid syntax if the identifier isn't valid in this context.
/// An identifier may not be valid if:
/// * it is named "eval" or "arguments" inside of strict mode
/// * it is named "let" inside of a "let" or "const" declaration
/// * the same identifier is bound multiple times inside of a `let` or const` declaration
/// * it is named "yield" inside of a generator function or in strict mode
/// * it is named "await" inside of an async function
pub(crate) fn parse_identifier_binding(p: &mut Parser) -> ConditionalParsedSyntax {
	let parsed = parse_identifier(p, JS_IDENTIFIER_BINDING);

	if let Valid(Present(identifier)) = parsed {
		let identifier_name = identifier.text(p);

		if StrictMode.is_supported(p) && matches!(identifier_name, "eval" | "arguments") {
			let err = p
				.err_builder(&format!(
					"Illegal use of `{}` as an identifier in strict mode",
					identifier_name
				))
				.primary(identifier.range(p), "");
			p.error(err);

			return Invalid(identifier.into());
		}

		if p.state.should_record_names {
			if identifier_name == "let" {
				let err = p
					.err_builder(
						"`let` cannot be declared as a variable name inside of a let or const declaration",
					)
					.primary(identifier.range(p), "Rename the let variable here");

				p.error(err);
				return Invalid(identifier.into());
			}

			if let Some(existing) = p.state.name_map.get(identifier_name) {
				let err = p
					.err_builder(
						"Declarations inside of a `let` or `const` declaration may not have duplicates",
					)
					.secondary(
						existing.to_owned(),
						&format!("{} is first declared here", identifier_name),
					)
					.primary(
						identifier.range(p),
						&format!("a second declaration of {} is not allowed", identifier_name),
					);
				p.error(err);
				return Invalid(identifier.into());
			}

			let identifier_name = String::from(identifier_name);
			p.state
				.name_map
				.insert(identifier_name, identifier.range(p).as_range());
		}

		Valid(identifier.into())
	} else {
		parsed
	}
}

struct BindingWithDefault;

impl ParseWithDefaultPattern for BindingWithDefault {
	#[inline]
	fn pattern_with_default_kind() -> SyntaxKind {
		JS_BINDING_WITH_DEFAULT
	}

	#[inline]
	fn expected_pattern_error(p: &Parser, range: Range<usize>) -> Diagnostic {
		expected_binding(p, range)
	}

	#[inline]
	fn parse_pattern(&self, p: &mut Parser) -> ParsedSyntax {
		parse_binding(p)
	}
}

struct ArrayBinding;

// test array_binding
// let a = "b";
// let [a, b] = [1, 2];
// let [a, ...abcd] = [1];
// let [a = "default", b] = []
// let [, a, ...rest] = []
// let [[...rest], { a }] = []
//
// test_err array_binding_err
// let [a b] = [1, 2];
// let [="default"] = [1, 2];
// let ["default"] = [1, 2];
// let [[a ] = [];
//
// test array_binding_rest
// let [ ...abcd ] = a;
// let [ ...[x, y] ] = b;
// let [ ...[ ...a ] ] = c;
//
// test_err array_binding_rest_err
// let [ ... ] = a;
// let [ ...c = "default" ] = a;
// let [ ...rest, other_assignment ] = a;
impl ParseArrayPattern<BindingWithDefault> for ArrayBinding {
	#[inline]
	fn unknown_pattern_kind() -> SyntaxKind {
		JS_UNKNOWN_BINDING
	}

	#[inline]
	fn array_pattern_kind() -> SyntaxKind {
		JS_ARRAY_BINDING
	}

	#[inline]
	fn rest_pattern_kind() -> SyntaxKind {
		JS_ARRAY_REST_BINDING
	}

	#[inline]
	fn expected_element_error(p: &Parser, range: Range<usize>) -> Diagnostic {
		expected_any(
			&[
				"identifier",
				"object pattern",
				"array pattern",
				"rest pattern",
			],
			range,
		)
		.to_diagnostic(p)
	}

	#[inline]
	fn pattern_with_default(&self) -> BindingWithDefault {
		BindingWithDefault
	}
}

// test_err object_binding_pattern
// let { 5 } } = { eval: "foo" };
// let { eval } = { eval: "foo" };
// let { 5, 6 } = { eval: "foo" };
// let { default: , bar } = {};
struct ObjectBindingPattern;

impl ParseObjectPattern for ObjectBindingPattern {
	#[inline]
	fn unknown_pattern_kind() -> SyntaxKind {
		JS_UNKNOWN_BINDING
	}

	#[inline]
	fn object_pattern_kind() -> SyntaxKind {
		JS_OBJECT_BINDING
	}

	#[inline]
	fn expected_property_pattern_error(p: &Parser, range: Range<usize>) -> Diagnostic {
		expected_any(&["identifier", "member name", "rest pattern"], range).to_diagnostic(p)
	}

	// test object_property_binding
	// let { foo: bar  } = {}
	// let { foo: bar = baz } = {}
	//
	// test_err object_property_binding_err
	// let { foo: , bar } = {}
	// let { : bar = "test" } = {}
	// let { , foo: bar } = {}
	//
	// test object_shorthand_property
	// let { a, b } = c
	// let { a = "default", b = call() } = c
	//
	// test_err object_shorthand_property_err
	// let { a b } = c
	// let { = "test" } = c
	// let { , a } = c
	fn parse_property_pattern(&self, p: &mut Parser) -> ParsedSyntax {
		if !is_at_object_member_name(p) && !p.at_ts(token_set![T![:], T![=]]) {
			return Absent;
		}

		let m = p.start();

		let kind = if p.at(T![=]) || (is_at_identifier_binding(p) && !p.nth_at(1, T![:])) {
			match parse_identifier_binding(p) {
				Valid(identifier) => {
					identifier.or_missing_with_error(p, expected_identifier);
					JS_SHORTHAND_PROPERTY_BINDING
				}
				Invalid(identifier) => {
					identifier.abandon(p);
					JS_UNKNOWN_BINDING
				}
			}
		} else {
			parse_object_member_name(p).or_missing_with_error(p, expected_object_member_name);
			if p.expect_required(T![:]) {
				parse_binding(p).or_missing_with_error(p, expected_binding);
			} else {
				p.missing();
			}
			JS_PROPERTY_BINDING
		};

		parse_equal_value_clause(p).or_missing(p);
		Present(m.complete(p, kind))
	}

	// test rest_property_binding
	// let { ...abcd } = a;
	// let { b: { ...a } } = c;
	//
	// test_err rest_property_binding_err
	// let { ... } = a;
	// let { ...c = "default" } = a;
	// let { ...{a} } = b;
	// let { ...rest, other_assignment } = a;
	// let { ...rest, } = a;
	// async function test() {
	//   let { ...await } = a;
	// }
	fn parse_rest_property_pattern(&self, p: &mut Parser) -> ParsedSyntax {
		if p.at(T![...]) {
			let m = p.start();
			p.bump(T![...]);

			let inner = parse_binding(p);

			if let Present(inner) = inner {
				if inner.kind() != JS_IDENTIFIER_BINDING {
					let inner_range = inner.range(p);
					inner.undo_completion(p).abandon(p);
					let completed = m.complete(p, JS_UNKNOWN_BINDING);

					// Don't add multiple errors
					if inner.kind() != JS_UNKNOWN_BINDING {
						p.error(p.err_builder("Expected identifier binding").primary(inner_range, "Object rest patterns must bind to an identifier, other patterns are not allowed."));
					}
					return Present(completed);
				}
			}

			inner.or_missing_with_error(p, expected_identifier);

			Present(m.complete(p, JS_OBJECT_REST_BINDING))
		} else {
			Absent
		}
	}
}
