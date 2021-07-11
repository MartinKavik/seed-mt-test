#![allow(clippy::wildcard_imports)]
#![feature(async_closure)]

use seed::{prelude::*, *};
use wasm_mt_pool::prelude::*;
use wasm_mt::utils::{console_ln, sleep};
use wasm_mt::prelude::*;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async {
        let pkg_js = "./pkg/package.js"; // path to `wasm-bindgen`'s JS binding
        let mt = WasmMt::new(pkg_js).and_init().await.unwrap();

        let th = mt.thread().and_init().await.unwrap();
    });

    Model
}

// ------ ------
//     Model
// ------ ------

struct Model;

// ------ ------
//    Update
// ------ ------

#[derive(Copy, Clone)]
enum Msg {}

fn update(_: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {
}

// ------ ------
//     View
// ------ ------

fn view(_: &Model) -> Node<Msg> {
    div![
        "Seed MT test",
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
