use serde::{Deserialize, Serialize};

/// Formatting settings for all documents.
#[derive(Serialize, Deserialize, Default)]
pub struct Format {
	#[serde(default)]
	pub native: Native,
	#[serde(default)]
	pub native_foreign: TranslationDictionary,
	#[serde(default)]
	pub foreign_native: TranslationDictionary,
}

/// Formatting settings for the monolingual native dictionary and for native
/// text in all documents.
#[derive(Serialize, Deserialize, Default)]
pub struct Native {
	/// Title of the monolingual native dictionary.
	#[serde(default)]
	pub title: String,
	/// Optional override of the standard lexicographical sort for native text,
	/// specified as an ascending list of graphemes.
	pub sort: Option<Vec<String>>,
	/// Preceeds every block of native text.
	#[serde(default)]
	pub prefix: String,
	/// Follows every block of native text.
	#[serde(default)]
	pub suffix: String,
}

/// Formatting settings for the native-foreign translation dictionary.
#[derive(Serialize, Deserialize, Default)]
pub struct TranslationDictionary {
	#[serde(default)]
	pub title: String,
}
