mod lexicon;

use std::fs::File;

use anyhow::Result;
use clap::Parser;
use tera::{Context, Tera};

use crate::lexicon::Lexicon;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Path to the lexicon YAML file to render.
	#[arg(index = 1)]
	input: String,
	/// Output path.
	#[arg(short = 'o', long, default_value = "dictionary.md")]
	output: String,
}

fn main() -> Result<()> {
	let args = Args::parse();

	// Load and validate the lexicon.
	let lexicon = Lexicon::load(args.input)?;

	// Load the template.
	let mut tera = Tera::default();
	tera.add_raw_template(
		"dictionary.md",
		include_str!("templates/dictionary.md.tmpl"),
	)?;

	// Render the lexicon.
	let mut context = Context::new();
	context.insert("lexicon", &lexicon);
	let output = File::create(args.output)?;
	tera.render_to("dictionary.md", &context, output)?;

	Ok(())
}
