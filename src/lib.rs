#![allow(clippy::wildcard_imports)]
#![feature(async_closure)]

use seed::{prelude::*, *};
use wasm_mt_pool::prelude::*;
use wasm_mt::utils::sleep;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(example());
    Model
}

// https://github.com/w3reality/wasm-mt/tree/43835ec0962720cc638deda658b93c7bf1f10be2/crates/pool#usage
async fn example() {
    let size = 2;
    let pkg_js = "./pkg/package.js";
    let pool = ThreadPool::new(size, pkg_js).and_init().await.unwrap();

    let num = 4;

    log!("a) 💦 pool_exec! {} closures:", num);
    for _ in 0..num {
        pool_exec!(pool, move || {
            log!("a) closure: done.");
            Ok(JsValue::NULL)
        });
    }

    log!("b) 💦 pool_exec! {} async closures:", num);
    for _ in 0..num {
        pool_exec!(pool, async move || {
            sleep(1000).await;
            log!("b) async closure: done.");
            Ok(JsValue::NULL)
        });
    }

    let cb = move |result| {
        log!("callback: result: {:?}", result);
    };

    log!("c) 💦 pool_exec! {} closures with callback:", num);
    for _ in 0..num {
        pool_exec!(pool, move || {
            log!("c) closure: done.");
            Ok(JsValue::from("C"))
        }, cb);
    }

    log!("d) 💦 pool_exec! {} async closures with callback:", num);
    for _ in 0..num {
        pool_exec!(pool, async move || {
            sleep(1000).await;
            log!("d) async closure: done.");
            Ok(JsValue::from("D"))
        }, cb);
    }

    sleep(6_000).await; // Do sleep long enough to ensure all jobs are completed.
    assert_eq!(pool.count_pending_jobs(), 0);
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
        "Seed MT test. Read the dev console.",
    ]
}

// ------ ------
//     Start
// ------ ------

// Without the `start` tag (`#[wasm_bindgen(start)]`),
// because we don't want to automatically start the `App` inside the worker,
// otherwise it would fail because of missing `Window` and other problems.
#[wasm_bindgen]
pub fn start() {
    App::start("app", init, update, view);
}
