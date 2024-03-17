use serde::{Deserialize, Serialize};

/// Formatting settings for all documents.
#[derive(Serialize, Deserialize, Default)]
pub struct Format {
	native: Native,
	native_foreign: NativeForeign,
	foreign_native: ForeignNative,
}

/// Formatting settings for the monolingual dictionary and for native text in
/// all documents.
#[derive(Serialize, Deserialize, Default)]
pub struct Native {
	/// Title of the monolingual, native dictionary.
	title: String,
	/// Preceeds every block of native text.
	pub prefix: String,
	/// Follows every block of native text.
	pub suffix: String,
}

/// Formatting settings for the native-foreign translation dictionary.
#[derive(Serialize, Deserialize)]
pub struct NativeForeign {
	title: String,
	/// Separates a native word from its foreign glosses.
	pub delimiter: String,
	/// Separates each foreign gloss for a native word.
	pub gloss_delimiter: String,
}

impl Default for NativeForeign {
	fn default() -> Self {
		Self {
			title: Default::default(),
			delimiter: " · ".into(),
			gloss_delimiter: "; ".into(),
		}
	}
}

/// Formatting settings for the foreign-native translation dictionary.
#[derive(Serialize, Deserialize)]
pub struct ForeignNative {
	title: String,
	/// Separates a foreign word from its native glosses.
	pub delimiter: String,
	/// Separates each native gloss for a foreign word.
	pub gloss_delimiter: String,
}

impl Default for ForeignNative {
	fn default() -> Self {
		Self {
			title: Default::default(),
			delimiter: " · ".into(),
			gloss_delimiter: "; ".into(),
		}
	}
}
