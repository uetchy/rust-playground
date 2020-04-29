extern crate cfg_if;
extern crate tensorflow;
extern crate wasm_bindgen;

use cfg_if::cfg_if;

use std::error::Error;
use std::fs::File;
use std::io::Read;

use std::path::Path;
use std::process::exit;
use std::result::Result;
use tensorflow::Graph;
use tensorflow::ImportGraphDefOptions;
use tensorflow::Session;
use tensorflow::SessionOptions;
use tensorflow::SessionRunArgs;
use tensorflow::Status;
use tensorflow::Tensor;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator];
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn jsSum(_: i32, _: i32) -> i32;
}

#[wasm_bindgen]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn sum_with_js(a: i32, b: i32) -> i32 {
    jsSum(a, b)
}

#[wasm_bindgen]
pub fn tfPredict() -> Result<(), Box<dyn Error>> {
    let filename = "src/models/model.pb"; // z = x + y
    if !Path::new(filename).exists() {
        return Err(Box::new(
            Status::new_set(
                Code::NotFound,
                &format!(
                    "Run 'python addition.py' to generate {} \
                     and try again.",
                    filename
                ),
            )
            .unwrap(),
        ));
    }

    // Create input variables for our addition
    let mut x = Tensor::new(&[1]);
    x[0] = 2i32;
    let mut y = Tensor::new(&[1]);
    y[0] = 40i32;

    // Load the computation graph defined by addition.py.
    let mut graph = Graph::new();
    let mut proto = Vec::new();
    File::open(filename)?.read_to_end(&mut proto)?;
    graph.import_graph_def(&proto, &ImportGraphDefOptions::new())?;
    let session = Session::new(&SessionOptions::new(), &graph)?;
    // Run the graph.
    let mut args = SessionRunArgs::new();
    args.add_feed(&graph.operation_by_name_required("x")?, 0, &x);
    args.add_feed(&graph.operation_by_name_required("y")?, 0, &y);
    let z = args.request_fetch(&graph.operation_by_name_required("z")?, 0);
    session.run(&mut args)?;

    // Check our results.
    let z_res: i32 = args.fetch(z)?[0];
    println!("{:?}", z_res);

    z_res
}
