use std::path::Path;

use pdf2image::{PDF2ImageError, RenderOptionsBuilder, PDF};

pub fn pfd_to_img() {
    let mapa_s_pdfji = Path::new("zapisi").join("pdfji"); //mapa
    let output_folder = Path::new("zapisi").join("jpgji"); //mapa

    for entry in std::fs::read_dir(mapa_s_pdfji).unwrap() {
        match entry {
            Ok(entry) => {
                let file_path = entry.path();
                let file_name = file_path.file_stem().unwrap().to_string_lossy(); // Get the file name without extension

                let pdf = PDF::from_file(&file_path).unwrap();
                let pages = pdf
                    .render(
                        pdf2image::Pages::Range(1..=8),
                        RenderOptionsBuilder::default().pdftocairo(true).build().unwrap(),
                    )
                    .unwrap();

                std::fs::create_dir_all(&output_folder).unwrap();
                // let neki = image::ImageFormat::Jpeg;
                for (i, page) in pages.iter().enumerate() {
                    let mut output_path = output_folder.join(format!("{}", &file_name));
                    std::fs::create_dir_all(&output_path).unwrap();
                    output_path = output_path.join(format!("{}.jpg", i + 1));
                    page.save_with_format(output_path, image::ImageFormat::Jpeg).unwrap();
                }
            }
            Err(er) => print!("{:?}", er),
        }
    }
}

// pub fn img_to_text() {
//     let mapa_s_jpgji = Path::new("zapisi").join("jpgji"); //mapa
//     let output_folder = Path::new("zapisi").join("txtji"); //mapa
//
//     for entry in std::fs::read_dir(mapa_s_jpgji).unwrap() {
//         match entry {
//             Ok(entry) => {
//                 for slika_entry in std::fs::read_dir(entry.path()).unwrap() {
//                     match slika_entry {
//                         Ok(slika) => {
//                             println!("{:?}", slika.path());
//                             let text: Result<_, tesseract::InitializeError> = Tesseract::new(None, &"lat")
//                                 .and_then(|mut tess| tess.set_image("path/to/image.jpg"));
//                                 // .and_then(|_| tess.get_text()))
//                                 // .expect("Failed to perform OCR");
//
//                             println!("Extracted text: {}", text);
//                         },
//                         Err(er) => print!("{:?}", er)
//                     }
//                 }
//             }
//             Err(er) => print!("{:?}", er)
//         }
//     }
// }
