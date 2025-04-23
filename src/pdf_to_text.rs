use pdf2image::{PDF2ImageError, RenderOptionsBuilder, PDF};
use std::path::Path;
use std::collections::VecDeque;
use std::error::Error;
use std::path::PathBuf;

use dioxus::prelude::asset;
use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
#[allow(unused)]
// use rten_tensor::prelude::*;
use dioxus::prelude::*;


pub fn pfd_to_img(){
    let mapa_s_pdfji = Path::new("zapisi").join("pdfji"); //mapa
    let output_folder = Path::new("zapisi").join("jpgji"); //mapa

    for entry in std::fs::read_dir(mapa_s_pdfji).unwrap() {
        match entry {
            Ok(entry) => {
                let file_path = entry.path();
                let file_name = file_path.file_stem().unwrap().to_string_lossy(); // Get the file name without extension

                let pdf = PDF::from_file(&file_path).unwrap();
                let pages = pdf.render(
                    pdf2image::Pages::Range(1..=8),
                    RenderOptionsBuilder::default().pdftocairo(true).build().unwrap(),
                ).unwrap();

                std::fs::create_dir_all(&output_folder).unwrap();
                // let neki = image::ImageFormat::Jpeg;
                for (i, page) in pages.iter().enumerate() {
                    let mut output_path = output_folder.join(format!("{}", &file_name));
                    std::fs::create_dir_all(&output_path).unwrap();
                    output_path = output_path.join(format!("{}.jpg", i + 1));
                    page.save_with_format(output_path, image::ImageFormat::Jpeg).unwrap();
                }


            }
            Err(er) => print!("{:?}", er)
        }
    }
}

fn file_path(path: &str) -> PathBuf {
    let mut abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    abs_path.push(path);
    abs_path
}

fn img_to_text(img_pot:&Path) -> Result<(), Box<dyn Error>> {
    // let img_pot = "zapisi/jpgji/GZL I-1 (1243 april 13)-1";
    // let args = parse_args()?;

    // Use the `download-models.sh` script to download the models.
    let detection_model_path = file_path("assets/models/text-detection.rten");
    let rec_model_path = file_path("assets/models/text-recognition.rten");

    let detection_model = Model::load_file(detection_model_path)?;
    let recognition_model = Model::load_file(rec_model_path)?;

    let engine = OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    })?;

    // Read image using image-rs library, and convert to RGB if not already
    // in that format.
    let img = image::open(img_pot).map(|image| image.into_rgb8())?;

    // Apply standard image pre-processing expected by this library (convert
    // to greyscale, map range to [-0.5, 0.5]).
    let img_source = ImageSource::from_bytes(img.as_raw(), img.dimensions())?;
    let ocr_input = engine.prepare_input(img_source)?;

    // Detect and recognize text. If you only need the text and don't need any
    // layout information, you can also use `engine.get_text(&ocr_input)`,
    // which returns all the text in an image as a single string.

    // Get oriented bounding boxes of text words in input image.
    let word_rects = engine.detect_words(&ocr_input)?;

    // Group words into lines. Each line is represented by a list of word
    // bounding boxes.
    let line_rects = engine.find_text_lines(&ocr_input, &word_rects);

    // Recognize the characters in each line.
    let line_texts = engine.recognize_text(&ocr_input, &line_rects)?;

    println!("izpisovanje");
    for line in line_texts
        .iter()
        .flatten()
        // Filter likely spurious detections. With future model improvements
        // this should become unnecessary.
        .filter(|l| l.to_string().len() > 1)
    {
        println!("{}", line);
    }

    Ok(())
}

pub fn imgs_to_text() {
    let mapa_s_jpgji = Path::new("zapisi").join("jpgji"); //mapa
    let output_folder = Path::new("zapisi").join("txtji"); //mapa

    for entry in std::fs::read_dir(mapa_s_jpgji).unwrap() {
        match entry {
            Ok(entry) => {
                for slika_entry in std::fs::read_dir(entry.path()).unwrap() {
                    match slika_entry {
                        Ok(slika) => {
                            println!("{:?}", slika.path());
                            let rezultat = img_to_text(&slika.path());
                            print!("{:?}", rezultat);
                            let text = "hgejhwlqlh";

                            println!("Extracted text: {}", text);
                        },
                        Err(er) => print!("{:?}", er)
                    }
                }
            }
            Err(er) => print!("{:?}", er)
        }
    }
}
