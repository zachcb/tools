//! Temporary interner to turn LSP URLs into unique file ids.
//! Should be replaced with a `rome_vfs` (virtual file system) at some point.
//! Based on `path_interner` from `rust-analyzer`

use indexmap::IndexSet;
use lsp_types::Url;
use rome_analyze::FileId;

/// Structure to map between [`Url`] and [`FileId`].
pub(crate) struct UrlInterner {
	map: IndexSet<Url>,
}

impl Default for UrlInterner {
	fn default() -> Self {
		Self {
			map: IndexSet::default(),
		}
	}
}

impl UrlInterner {
	/// Get the id corresponding to `path`.
	///
	/// If `path` does not exists in `self`, returns [`None`].
	pub(crate) fn get(&self, path: &Url) -> Option<FileId> {
		self.map.get_index_of(path).map(|i| FileId(i))
	}

	/// Insert `path` in `self`.
	///
	/// - If `path` already exists in `self`, returns its associated id;
	/// - Else, returns a newly allocated id.
	pub(crate) fn intern(&mut self, path: Url) -> FileId {
		let (id, _added) = self.map.insert_full(path);
		assert!(id < usize::MAX);
		FileId(id)
	}

	/// Returns the path corresponding to `id`.
	///
	/// # Panics
	///
	/// Panics if `id` does not exists in `self`.
	#[allow(dead_code)]
	pub(crate) fn lookup(&self, id: FileId) -> &Url {
		self.map.get_index(id.0 as usize).unwrap()
	}
}
