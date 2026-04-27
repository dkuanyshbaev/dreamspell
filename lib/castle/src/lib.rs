//! Castle of Destiny library — per-kin blank generation.
//!
//! Domain logic for the Замок Судьбы layout: family/tone/seal derivation,
//! age placement across 13-year cycles and 4-year groups, and PDF rendering.

use chrono::NaiveDate;
use svg2pdf::{ConversionOptions, PageOptions};

const BLANK_SVG: &str = include_str!("../templates/blank.svg");

#[derive(Debug)]
pub enum Error {
    Svg(svg2pdf::usvg::Error),
    Pdf(svg2pdf::ConversionError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Svg(e) => write!(f, "SVG parse error: {e}"),
            Error::Pdf(e) => write!(f, "PDF conversion error: {e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<svg2pdf::usvg::Error> for Error {
    fn from(e: svg2pdf::usvg::Error) -> Self {
        Error::Svg(e)
    }
}

impl From<svg2pdf::ConversionError> for Error {
    fn from(e: svg2pdf::ConversionError) -> Self {
        Error::Pdf(e)
    }
}

pub fn render_pdf(_birth_date: NaiveDate) -> Result<Vec<u8>, Error> {
    let mut options = svg2pdf::usvg::Options::default();
    options.fontdb_mut().load_system_fonts();
    let tree = svg2pdf::usvg::Tree::from_str(BLANK_SVG, &options)?;
    let pdf = svg2pdf::to_pdf(&tree, ConversionOptions::default(), PageOptions::default())?;
    Ok(pdf)
}
