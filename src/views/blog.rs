// use std::println;
use std::rc::Rc;

// use dioxus::desktop::wry::dpi::Position;
use dioxus::prelude::*;
// use dioxus::events::MouseEvent;

use crate::Route;

#[component]
pub fn Blog(id: i32) -> Element {

    // let mut y_delta:f64 = 0.0;
    // static mut y_koordinata:Option<f64> = None;
    let mut y_koordinata = 0.0;
    fn get_mouse_pos(event_data:Event<MouseData>, mut y_koordinata:Option<f64>) {
        let koordinate = event_data.screen_coordinates();
        y_koordinata = Some(koordinate.y);
        println!("{:?}", koordinate);
    }

    fn drag_to_sirina() {

    }

    rsx! {
            div {
                id: "blog",
                onmouseup: |event_data| println!("unclicker!  {event_data:?}"),
                // background_color: "red",
                // width: "50%",

                // Content
                h1 { "This is blog #{id}!" }
                p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }
                // img {src : asset!("zapisi/jpgji/GZL I-1 (1243 april 13)-1.jpg")}
                div { id: "container",

                    div {
                        id: "leva_stran",

                        p { "leva stran" }
                    }

                    div {id: "drsnik",
                        // width: sirina,

                        onmousedown: |event_data| {println!("{:?}", event_data.screen_coordinates().y);},

                        p {"{y_koordinata}"}

                    }



                    div {
                        id: "desna_stran",

                        embed {
                            src: asset!("zapisi/pdfji/GZL I-1 (1243 april 13)-1.pdf"),
                            // width: "425",
                            height: "100%"
                        }
                    }
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
