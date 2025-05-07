use dioxus::logger::tracing::info;
use dioxus::prelude::*;

// Želje: https://stackoverflow.com/questions/391440/how-to-make-div-resizeable/76461448#76461448
// Pfection: https://stackoverflow.com/questions/46044589/dynamically-resize-columns-in-css-grid-layout-with-mouse/77857312#77857312
use crate::Route;

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        document::Script { src: asset!("/assets/scripts/grid.js") },
        // script { src: "/assets/scripts/grid.js"}
        div { class: "trije_divi panes pane h-full",
            div { class: "leva_stran pane", "leva stran" }
            div { class: "srednja_stran pane",
                form { onsubmit: move |event| { info!("Submitted! {event:?}") },
                    textarea {
                        width: "100%",
                        autocomplete: "off",
                        spellcheck: "off",
                        name: "zapis",
                        value: "Hehehehehehhe, nekaj \n nekaj drugega"
                    }
                    input { r#type: "submit" }
                }
            }


            div { class: "desna_stran pane",
                embed {
                    src: asset!("zapisi/pdfji/GZL I-1 (1243 april 13)-1.pdf"),
                    width: "100%"
                    // height: "100%"
                }

        }

        }
        // document::Script { src: asset!("/assets/scripts/grid.js") }
    }

    // rsx! {
    //         div {
    //             id: "blog",
    //             onmouseup: |event_data| println!("unclicker!  {event_data:?}"),
    //             // background_color: "red",
    //             // width: "50%",

    //             // Content
    //             h1 { "This is blog #{id}!" }
    //             p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }
    //             // img {src : asset!("zapisi/jpgji/GZL I-1 (1243 april 13)-1.jpg")}
    //             div { id: "container",
    //                 // grid-template-columns: "10%" "auto" "90%",

    //                 div {
    //                     id: "leva_stran",

    //                     p { "leva stran" }
    //                 }

    //                 div {id: "drsnik",
    //                     // width: "20px",

    //                     onmousemove: |event_data| {println!("{:?}", event_data.screen_coordinates().y);},
    //                     onmousedown: |event_data| {println!("{:?}", event_data.screen_coordinates().y);},

    //                     p {"{y_koordinata}"}

    //                 }

    //                 div {
    //                     id: "desna_stran",

    //                     // embed {
    //                     //     src: asset!("zapisi/pdfji/GZL I-1 (1243 april 13)-1.pdf"),
    //                     //     // width: "425",
    //                     //     height: "100%"
    //                     // }
    //                 }
    //             }

    //             //             <embed
    // //   src="http://URL_TO_PDF.com/pdf.pdf#toolbar=0&navpanes=0&scrollbar=0"
    // //   width="425" height="425" />

    //             // Navigation links
    //             Link {
    //                 to: Route::Blog { id: id - 1 },
    //                 "Previous"
    //             }
    //             span { " <---> " }
    //             Link {
    //                 to: Route::Blog { id: id + 1 },
    //                 "Next"
    //             }
    //         }
    //     }
}
