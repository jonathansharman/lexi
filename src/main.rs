mod format;
mod lexicon;

use std::{collections::HashMap, fs::File, io::BufReader};

use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
use tera::{Context, Tera};

use crate::lexicon::Lexicon;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Path to the lexicon YAML file to render.
	#[arg(index = 1)]
	input: String,
	/// Monolingual dictionary output path.
	#[arg(short = 'd', long)]
	dictionary: Option<String>,
	/// Native-foreign translation dictionary output path.
	#[arg(short = 'n', long)]
	native_foreign: Option<String>,
	/// Foreign-native translation dictionary output path.
	#[arg(short = 'f', long)]
	foreign_native: Option<String>,
}

fn main() -> Result<()> {
	env_logger::builder()
		.filter_level(log::LevelFilter::Debug)
		.format_target(false)
		.format_timestamp(None)
		.init();

	let args = Args::parse();

	// Load and validate lexicon settings.
	let file = File::open(args.input)?;
	let reader = BufReader::new(file);
	let lexicon = Lexicon::open(reader)?;

	// Load text templates.
	let tera = load_templates()?;
	let mut context = Context::new();

	// Add the lexicon settings to the text template context.
	context.insert("lexicon", &lexicon);

	// Add a reverse index of foreign-to-native glosses to the context.
	let foreign_to_native: Vec<(String, Vec<String>)> = lexicon
		.lexemes
		.iter()
		.flat_map(|lexeme| {
			lexeme
				.glosses
				.iter()
				.cloned()
				.map(|gloss| (gloss, lexeme.lemma.clone()))
		})
		.fold(
			HashMap::<String, Vec<String>>::new(),
			|mut acc, (gloss, lemma)| {
				acc.entry(gloss).or_default().push(lemma);
				acc
			},
		)
		.into_iter()
		.sorted()
		.collect();
	context.insert("foreign_to_native", &foreign_to_native);

	// Render the requested documents.
	if let Some(dictionary) = args.dictionary {
		let output = File::create(dictionary)?;
		tera.render_to("dictionary.html", &context, output)?;
	}
	if let Some(native_foreign) = args.native_foreign {
		let output = File::create(native_foreign)?;
		tera.render_to("native-foreign.html", &context, output)?;
	}
	if let Some(foreign_native) = args.foreign_native {
		let output = File::create(foreign_native)?;
		tera.render_to("foreign-native.html", &context, output)?;
	}

	Ok(())
}

fn load_templates() -> Result<Tera> {
	let mut tera = Tera::default();
	tera.add_raw_template(
		"dictionary.html",
		include_str!("templates/dictionary.html.tmpl"),
	)?;
	tera.add_raw_template(
		"native-foreign.html",
		include_str!("templates/native-foreign.html.tmpl"),
	)?;
	tera.add_raw_template(
		"foreign-native.html",
		include_str!("templates/foreign-native.html.tmpl"),
	)?;
	// Disable autoescape to allow HTML in definitions, etc.
	tera.autoescape_on(vec![]);
	Ok(tera)
}
