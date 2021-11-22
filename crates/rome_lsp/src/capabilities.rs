use lsp_types::{
	CodeActionProviderCapability, OneOf, ServerCapabilities, TextDocumentSyncCapability,
	TextDocumentSyncKind,
};

pub(crate) fn server_capabilities() -> ServerCapabilities {
	ServerCapabilities {
		definition_provider: Some(OneOf::Left(true)),
		text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::Full)),
		code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
		document_formatting_provider: Some(OneOf::Left(true)),
		..Default::default()
	}
}
