use pdf2image::{PDF2ImageError, RenderOptionsBuilder, PDF};
use std::path::Path;


pub fn pfd_to_img(){
    let mapa_s_pdfji = Path::new("zapisi").join("pdfji"); //mapa

    for entry in std::fs::read_dir(mapa_s_pdfji).unwrap() {
        match entry {
            Ok(entry) => {
                let file_path = entry.path();
                let pdf = PDF::from_file(file_path).unwrap();
                let pages = pdf.render(
                    pdf2image::Pages::Range(1..=8),
                    RenderOptionsBuilder::default().pdftocairo(true).build().unwrap(),
                ).unwrap();

                std::fs::create_dir_all("zapisi/poskusi").unwrap();
                // let neki = image::ImageFormat::Jpeg;
                for (i, page) in pages.iter().enumerate() {
                    page.save_with_format(format!("zapisi/poskusi/{}.jpg", i + 1), image::ImageFormat::Jpeg);
                }


            }
            Err(er) => print!("{:?}", er)
        }
    }


    // let pdf = PDF::from_file("examples/pdfs/ropes.pdf").unwrap();
    // let pages = pdf.render(
    //     pdf2image::Pages::Range(1..=8),
    //     RenderOptionsBuilder::default().pdftocairo(true).build()?,
    // )?;

    // std::fs::create_dir("examples/out").unwrap();
    // for (i, page) in pages.iter().enumerate() {
    //     page.save_with_format(format!("examples/out/{}.jpg", i + 1), image::ImageFormat::Jpeg)?;
    // }

    // Ok(())
}
