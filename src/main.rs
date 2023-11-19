mod lexicon;

use std::{fs::File, io::Write};

use anyhow::Result;
use clap::Parser;
use tera::{Context, Tera};

use crate::lexicon::Lexicon;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Path to the lexicon YAML file to render.
	#[arg(index = 1)]
	lexicon: String,
	/// Path to a custom font file.
	#[arg(short, long)]
	font: Option<String>,
	/// Output path for the generated HTML.
	#[arg(short = 'o', long, default_value = "index.html")]
	html: String,
}

fn main() -> Result<()> {
	let args = Args::parse();

	// Load and validate the lexicon.
	let lexicon = Lexicon::load(args.lexicon)?;

	// Load templates.
	let tera = Tera::new("templates/**/*")?;
	let mut context = Context::new();
	context.insert("lexicon", &lexicon);
	context.insert("font", &args.font);

	// Generate HTML.
	let html = tera.render("index.html.tmpl", &context)?;
	let mut html_file = File::create(args.html)?;
	write!(html_file, "{html}")?;
	html_file.flush()?;

	Ok(())
}
