use std::collections::HashMap;

use anyhow::{anyhow, Result};
use lsp_server::{Connection, Message, Response};
use lsp_types::{
	notification::{DidChangeTextDocument, DidOpenTextDocument, Notification, PublishDiagnostics},
	request::{CodeActionRequest, Formatting, Request},
	CodeAction, CodeActionKind, CodeActionOrCommand, CodeActionParams, CodeActionResponse,
	Diagnostic, DidChangeTextDocumentParams, DidOpenTextDocumentParams, DocumentFormattingParams,
	InitializeParams, Position, PublishDiagnosticsParams, Range, TextEdit, Url, WorkspaceEdit,
};
use rome_analyze::AnalysisHost;
use rome_formatter::{FormatOptions, Formatter, IndentStyle};
use rslint_parser::TextRange;

use crate::{
	line_index::{self, LineIndex},
	url_interner::UrlInterner,
	utils::text_edit,
};

pub fn run_server() -> Result<()> {
	eprintln!("Starting Rome LSP server");

	let (connection, io_threads) = Connection::stdio();

	let server_capabilities = crate::capabilities::server_capabilities();
	let server_capabilities = serde_json::to_value(server_capabilities)?;

	let params = connection.initialize(server_capabilities)?;
	let params: InitializeParams = serde_json::from_value(params)?;

	let mut server = LSPServer::new(connection);

	server.main_loop(params)?;
	io_threads.join()?;

	eprintln!("Shutting down Rome LSP server");
	Ok(())
}

pub(crate) struct LSPServer {
	connection: Connection,
	// text_documents: HashMap<Url, TextDocumentItem>,
	url_interner: UrlInterner,
	analysis_host: AnalysisHost,
}

impl LSPServer {
	fn new(connection: Connection) -> Self {
		Self {
			connection,
			// text_documents: Default::default(),
			url_interner: Default::default(),
			analysis_host: Default::default(),
		}
	}

	fn main_loop(&mut self, _params: InitializeParams) -> Result<()> {
		for msg in &self.connection.receiver {
			// eprintln!("Message: {:?}", msg);
			match msg {
				Message::Request(req) => {
					// eprintln!("Request: {:?}", req);

					if self.connection.handle_shutdown(&req)? {
						return Ok(());
					}
					match req.method.as_str() {
						CodeActionRequest::METHOD => {
							eprintln!("Code action request: {:?}", req.id);

							let params: CodeActionParams = serde_json::from_value(req.params)?;

							let uri = params.text_document.uri;

							let file_id = self.url_interner.get(&uri).ok_or_else(|| {
								anyhow!("FileId not found  while getting code actions: {}", uri)
							})?;
							let text = self.analysis_host.get_file_text(file_id);

							// let text_document = self.text_documents.get(&uri).ok_or_else(|| {
							// 	anyhow!("Contents missing while getting code actions: {}", uri)
							// })?;

							let line_index = LineIndex::new(&text);
							let cursor_range = crate::utils::text_range(&line_index, params.range);

							let (_, actions) = self.compute_signals(&uri, Some(cursor_range))?;

							let result: CodeActionResponse = actions
								.into_iter()
								.map(CodeActionOrCommand::CodeAction)
								.collect();

							let result = serde_json::to_value(&result)?;

							let resp = Response {
								id: req.id,
								result: Some(result),
								error: None,
							};
							self.connection.sender.send(Message::Response(resp))?;
						}
						Formatting::METHOD => {
							let params: DocumentFormattingParams =
								serde_json::from_value(req.params)?;

							let uri = params.text_document.uri;
							let file_id = self.url_interner.get(&uri).ok_or_else(|| {
								anyhow!("FileId not found  while formatting: {}", uri)
							})?;
							let text = self.analysis_host.get_file_text(file_id);
							let tree = self.analysis_host.parse(file_id);

							let options = FormatOptions {
								indent_style: IndentStyle::Tab,
								line_width: 80,
							};

							let new_text = Formatter::new(options)
								.format_root(&tree)
								// TODO: impl Error for FormatError
								.unwrap()
								.code()
								.to_string();

							let num_lines: u32 = line_index::LineIndex::new(&text)
								.newlines
								.len()
								.try_into()?;

							let range = Range {
								start: Position::default(),
								end: Position {
									line: num_lines,
									character: 0,
								},
							};

							let edits = &[TextEdit { range, new_text }];
							let result = serde_json::to_value(edits)?;

							let response = Response {
								id: req.id,
								result: Some(result),
								error: None,
							};

							self.connection.sender.send(Message::Response(response))?;
						}

						_ => (),
					};
				}
				Message::Response(resp) => {
					eprintln!("Response: {:?}", resp);
				}
				Message::Notification(not) => {
					match not.method.as_str() {
						DidOpenTextDocument::METHOD => {
							let params: DidOpenTextDocumentParams =
								serde_json::from_value(not.params)?;
							let url = params.text_document.uri.clone();
							// self.text_documents
							// 	.insert(url.clone(), params.text_document);
							let file_id = self.url_interner.intern(url.clone());
							let text = params.text_document.text;
							self.analysis_host.set_file_text(file_id, text);
							let (diagnostics, _) = self.compute_signals(&url, None)?;
							self.publish_diagnostics(&url, diagnostics)?;
						}
						DidChangeTextDocument::METHOD => {
							let mut params: DidChangeTextDocumentParams =
								serde_json::from_value(not.params)?;

							let url = params.text_document.uri.clone();

							// Because of TextDocumentSyncKind::Full, there should only be one change.
							let change = params.content_changes.pop().ok_or_else(|| {
								anyhow!("Content change missing in textDocument/didChange")
							})?;
							let text = change.text;

							// self.text_documents
							// 	.entry(params.text_document.uri)
							// 	.and_modify(|td| {
							// 		td.version = params.text_document.version;
							// 		td.text = change.text;
							// 	});
							// let (diagnostics, _) = self.compute_signals(&url, None)?;
							// self.publish_diagnostics(&url, diagnostics)?;

							let file_id = self.url_interner.intern(url.clone());
							self.analysis_host.set_file_text(file_id, text);
							let (diagnostics, _) = self.compute_signals(&url, None)?;
							self.publish_diagnostics(&url, diagnostics)?;
						}
						_ => {}
					}
				}
			}
		}
		Ok(())
	}

	fn publish_diagnostics(&self, uri: &Url, diagnostics: Vec<Diagnostic>) -> Result<()> {
		// let text_document = self
		// 	.text_documents
		// 	.get(uri)
		// 	.ok_or_else(|| anyhow!("Contents missing while publishing diagnostics: {}", uri))?;

		let params = PublishDiagnosticsParams {
			uri: uri.to_owned(),
			diagnostics,
			// version: Some(text_document.version),
			version: None,
		};
		let message =
			lsp_server::Notification::new(PublishDiagnostics::METHOD.into(), params).into();
		self.connection.sender.send(message)?;

		Ok(())
	}

	fn compute_signals(
		&self,
		uri: &Url,
		cursor_range: Option<TextRange>,
	) -> Result<(Vec<Diagnostic>, Vec<CodeAction>)> {
		let file_id = self
			.url_interner
			.get(uri)
			.ok_or_else(|| anyhow!("Missing URL  while computing signals: {}", uri))?;

		let text = self.analysis_host.get_file_text(file_id);
		let signal = self.analysis_host.analyze(file_id, cursor_range)?;
		let line_index = LineIndex::new(&text);

		let diagnostics = signal
			.diagnostics
			.into_iter()
			.map(|d| {
				let range = crate::utils::range(&line_index, d.range);

				Diagnostic::new_simple(range, d.message)
			})
			.collect();

		let code_actions: Vec<CodeAction> = signal
			.actions
			.into_iter()
			.map(|a| {
				let edits = a
					.replacements
					.iter()
					.map(|r| text_edit(&line_index, r))
					.collect();

				let mut text_edits = HashMap::new();
				text_edits.insert(uri.clone(), edits);
				let changes = Some(text_edits);
				let edit = WorkspaceEdit {
					changes,
					document_changes: None,
				};
				let diagnostics = a
					.diagnostics
					.iter()
					.map(|d| {
						let range = crate::utils::range(&line_index, d.range);
						Diagnostic::new_simple(range, d.message.clone())
					})
					.collect();
				CodeAction {
					title: a.title,
					kind: Some(CodeActionKind::QUICKFIX),
					diagnostics: Some(diagnostics),
					edit: Some(edit),
					command: None,
					is_preferred: None,
				}
			})
			.collect();

		Ok((diagnostics, code_actions))
	}
}
