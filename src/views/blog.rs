use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }
            // img {src : asset!("zapisi/jpgji/GZL I-1 (1243 april 13)-1.jpg")}

            embed {
                src: asset!("zapisi/pdfji/GZL I-1 (1243 april 13)-1.pdf"),
                width: "425",
                height: "425"
            }
            
            //             <embed
//   src="http://URL_TO_PDF.com/pdf.pdf#toolbar=0&navpanes=0&scrollbar=0"
//   width="425" height="425" />

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}
