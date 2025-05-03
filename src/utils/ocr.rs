use std::path::Path;

use anyhow::{Context, Result};
use pdf2image::PDF;
use rusty_tesseract::{image_to_string, Args as TesseractArgs, Image as TesseractImage};

pub fn extract(path: &Path, languages: &[&str]) -> Result<String> {
    // Load and render the document as PDF
    let pdf = PDF::from_file(path).context("Failed to load PDF")?;
    let pages = pdf
        .render(pdf2image::Pages::Range(1..=pdf.page_count()), pdf2image::RenderOptions::default())
        .context("Failed to render PDF")?;

    let mut results = vec![];

    for page in pages.iter() {
        let filename = tempfile::Builder::new().prefix("ocr-").suffix(".jpg").tempfile()?;

        // Save the page as an image
        page.save(filename.path()).context("Failed to save page")?;

        // Load the image and extract text
        let args = TesseractArgs { lang: languages.join("+"), ..Default::default() };
        let img = TesseractImage::from_path(filename.path()).context("Failed to load image")?;
        let result = image_to_string(&img, &args).context("Failed to extract text")?;

        results.push(result);
    }

    // Join all outputs into a single string
    let text = results.join("\n\n").replace("\r\n", "\n");

    Ok(text)
}
