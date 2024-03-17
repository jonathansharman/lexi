use serde::{Deserialize, Serialize};

/// Formatting settings for all documents.
#[derive(Serialize, Deserialize)]
pub struct Format {
	#[serde(default)]
	native: NativeDictionary,
	#[serde(default)]
	native_foreign: TranslationDictionary,
	#[serde(default)]
	foreign_native: TranslationDictionary,
	#[serde(default = "default_indent")]
	indent: String,
}

impl Default for Format {
	fn default() -> Self {
		Self {
			native: Default::default(),
			native_foreign: Default::default(),
			foreign_native: Default::default(),
			indent: default_indent(),
		}
	}
}

fn default_indent() -> String {
	"20px".into()
}

/// Formatting settings for the monolingual dictionary and for native text in
/// all documents.
#[derive(Serialize, Deserialize, Default)]
pub struct NativeDictionary {
	/// Title of the monolingual, native dictionary.
	#[serde(default)]
	title: String,
	/// Preceeds every block of native text.
	#[serde(default)]
	pub prefix: String,
	/// Follows every block of native text.
	#[serde(default)]
	pub suffix: String,
}

/// Formatting settings for the native-foreign translation dictionary.
#[derive(Serialize, Deserialize)]
pub struct TranslationDictionary {
	#[serde(default)]
	title: String,
	/// Delimiter between the lemma and its list of glosses.
	#[serde(default = "default_delimiter")]
	pub delimiter: String,
	/// Delimiter between glosses.
	#[serde(default = "default_gloss_delimiter")]
	pub gloss_delimiter: String,
}

impl Default for TranslationDictionary {
	fn default() -> Self {
		Self {
			title: Default::default(),
			delimiter: default_delimiter(),
			gloss_delimiter: default_gloss_delimiter(),
		}
	}
}

fn default_delimiter() -> String {
	" · ".into()
}

fn default_gloss_delimiter() -> String {
	"; ".into()
}
