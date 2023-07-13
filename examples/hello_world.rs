use berrylite::kernel::micro_operator::fully_connected::OpFullyConnected;
use berrylite::kernel::micro_operator::BLiteOperator;
use berrylite::micro_allocator::BumpArenaAllocator;
use berrylite::micro_erros::Result;
use berrylite::micro_graph::*;
use berrylite::micro_op_resolver::BLiteOpResolver;
use berrylite::tflite_schema_generated::tflite;

const BUFFER: &[u8; 3164] =
    include_bytes!("../models/hello_world_float.tflite");
// const BUFFER: &[u8; 300568] =
//     include_bytes!("../models/person_detect.tflite");

// const BUFFER: &[u8; 41240] =
//     include_bytes!("../models/trained_lstm.tflite");

const ARENA_SIZE: usize = 1024 * 1024;
static mut ARENA: [u8; ARENA_SIZE] = [0; ARENA_SIZE];

fn predict() -> Result<()> {
    let model = tflite::root_as_model(BUFFER).unwrap();
    println!("model version: {}", model.version());

    let mut allocator =
        unsafe { BumpArenaAllocator::new(&mut ARENA) };

    let mut op_resolver = BLiteOpResolver::<1, f32>::new();
    op_resolver
        .add_op(OpFullyConnected::fully_connected())?;

    let graph = BLiteGraph::allocate_graph(
        &mut allocator,
        &op_resolver,
        &model,
    )?;

    graph.invoke()?;

    for (i, tensor) in graph.subgraphs[0]
        .borrow()
        .tensors
        .iter()
        .enumerate()
    {
        println!("{} -> {:?}", i, tensor);
    }
    Ok(())
}

fn main() {
    if let Err(e) = predict() {
        println!("Error: {:?}", e);
    }
    println!("Inference Success!!");
}
