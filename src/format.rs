use serde::{Deserialize, Serialize};

/// Formatting settings for all documents.
#[derive(Serialize, Deserialize, Default)]
pub struct Format {
	native: NativeDictionary,
	native_foreign: TranslationDictionary,
	foreign_native: TranslationDictionary,
}

/// Formatting settings for the monolingual dictionary and for native text in
/// all documents.
#[derive(Serialize, Deserialize, Default)]
pub struct NativeDictionary {
	/// Title of the monolingual, native dictionary.
	title: String,
	/// Preceeds every block of native text.
	pub prefix: String,
	/// Follows every block of native text.
	pub suffix: String,
}

/// Formatting settings for the native-foreign translation dictionary.
#[derive(Serialize, Deserialize)]
pub struct TranslationDictionary {
	title: String,
	/// Delimiter between the lemma and its list of glosses.
	pub delimiter: String,
	/// Delimiter between glosses.
	pub gloss_delimiter: String,
}

impl Default for TranslationDictionary {
	fn default() -> Self {
		Self {
			title: Default::default(),
			delimiter: " · ".into(),
			gloss_delimiter: "; ".into(),
		}
	}
}
