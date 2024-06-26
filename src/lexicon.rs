use std::{
	collections::{HashMap, HashSet},
	io::Read,
};

use anyhow::Result;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_yaml::Deserializer;

use crate::format::Format;

#[derive(Serialize, Deserialize)]
pub struct Lexeme {
	pub lemma: String,
	pub class: String,
	pub definition: Option<String>,
	pub translation: Option<String>,
	#[serde(default)]
	pub glosses: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Lexicon {
	#[serde(default)]
	pub format: Format,
	pub classes: Vec<String>,
	pub lexemes: Vec<Lexeme>,
}

impl Lexicon {
	pub fn open<R: Read>(reader: R) -> Result<Self> {
		let de = Deserializer::from_reader(reader);
		let mut lexicon = Lexicon::deserialize(de)?;

		// Validate lexeme classes and required fields.
		let classes = HashSet::<&String>::from_iter(lexicon.classes.iter());
		for lexeme in &lexicon.lexemes {
			if !classes.contains(&lexeme.class) {
				log::error!(
					"Lexeme '{}' has unrecognized class '{}'",
					lexeme.lemma,
					lexeme.class
				)
			}
			if lexeme.definition.is_none()
				&& lexeme.translation.is_none()
				&& lexeme.glosses.is_empty()
			{
				log::warn!(
					"Lexeme '{}' has no definition, translation, or glosses",
					lexeme.lemma
				)
			}
		}

		// Sort lexemes by lemma, according to the specified sort.
		match &lexicon.format.native.sort {
			Some(sort) => {
				// TODO: Support multigraphs.
				let sort = HashMap::<char, usize>::from_iter(
					sort.iter().enumerate().filter_map(|(i, grapheme)| {
						grapheme.chars().next().map(|c| (c, i))
					}),
				);
				lexicon.lexemes.sort_by(|a, b| {
					a.lemma
						.chars()
						.map(|c| sort.get(&c).unwrap_or(&usize::MAX))
						.cmp(
							b.lemma
								.chars()
								.map(|c| sort.get(&c).unwrap_or(&usize::MAX)),
						)
				});
			}
			None => lexicon.lexemes.sort_by(|a, b| a.lemma.cmp(&b.lemma)),
		};

		// Warn about duplicate lemmas.
		for (first, second) in lexicon
			.lexemes
			.iter()
			.map(|lexeme| &lexeme.lemma)
			.tuple_windows()
		{
			if first == second {
				log::warn!("Duplicate lemma '{}'", first)
			}
		}

		Ok(lexicon)
	}
}
