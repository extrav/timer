// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement, HtmlInputElement, KeyboardEvent, HtmlBodyElement, EventTarget, Event};
//use js-sys::prelude::*;
// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
	editable_right_ica: String::new(),
        report_right_ica: String::from(""),
    }
}

// const initial_ica: String = String::from("Right ICA was");

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
//#[derive(Clone)]
struct Model {
	editable_right_ica: String,		
    report_right_ica: String,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    RICA,
    RicaEdit(String),
	RefreshRICA,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {

    match msg {
	/////////////////////////////////////
	Msg::RefreshRICA => {model.editable_right_ica = String::from("Babbbb");}

	
        /////////////////////////////////////
        Msg::RicaEdit(editable_right_ica) => {
            model.report_right_ica = editable_right_ica;
        }
        ///////////////////////////////////////
        Msg::RICA => {
//			let target = event.current_target().unwrap();
//let content = target.dyn_ref::<web_sys::HtmlElement>().unwrap().text_content();
		model.editable_right_ica = model.report_right_ica.to_string();


            let initial_rica: String = String::from(
                "The right internal carotid artery was selected and injected with cranial views
The right internal carotid artery appears normal without any areas of irregularity or stenosis.
The right middle cerebral artery appears normal without any areas of irregularity or stenosis.
The right anterior cerebral artery appears normal without any areas of irregularity or stenosis. 
",
            );

            if model.editable_right_ica.is_empty() {
                model.editable_right_ica = initial_rica;
		model.report_right_ica = String::from("The right internal carotid artery was selected and injected with cranial views
The right internal carotid artery appears normal without any areas of irregularity or stenosis.
The right middle cerebral artery appears normal without any areas of irregularity or stenosis.
The right anterior cerebral artery appears normal without any areas of irregularity or stenosis.
");
            } else {
		model.report_right_ica = String::new();
               
		 model.editable_right_ica = model.report_right_ica.to_string();
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        C!["columns"],
        div![
            C!["column"],
            button![
                C!["button", "is-primary"],
                "RICA",
                ev(Ev::Click,|_| Msg::RICA),
            ]
        ],
        div![
            C!["column"],
            textarea![
                C!["has-background-warning "],
                input_ev(Ev::Change, Msg::RicaEdit),
                attrs!{At::Value => &model.report_right_ica;},

            ],
        ],
//	div![button![C!["button"], "Refresh", ev(Ev::Click,|_| Msg::RefreshRICA),]],
        div![C!["column"], textarea![attrs!{At::Value => &model.report_right_ica;}]],
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
