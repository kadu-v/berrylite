use berrylite::kernel::micro_operator::f32::{
    conv2d::OpConv2D, fully_connected::OpFullyConnected, max_pool2d::OpMaxPool2D,
    reshape::OpReshape,
};
use berrylite::micro_allocator::BumpArenaAllocator;
use berrylite::micro_errors::Result;
use berrylite::micro_interpreter::BLiteInterpreter;
use berrylite::micro_op_resolver::BLiteOpResolver;
use berrylite::tflite_schema_generated::tflite;
use core::f32::consts::PI;

const BUFFER: &[u8; 3156] = include_bytes!("../resources/models/simple_conv_sin.tflite");

const ARENA_SIZE: usize = 1024 * 1024;
static mut ARENA: [u8; ARENA_SIZE] = [0; ARENA_SIZE];

fn set_input(
    interpreter: &mut BLiteInterpreter<'_, f32>,
    input_h: usize,
    input_w: usize,
    input: f32,
) {
    for h in 0..input_h {
        for w in 0..input_w {
            interpreter.input.data[h * input_w + w] = input;
        }
    }
}

fn predict(input: f32) -> Result<f32> {
    let model = tflite::root_as_model(BUFFER).unwrap();

    let mut allocator = unsafe { BumpArenaAllocator::new(&mut ARENA) };

    let mut op_resolver = BLiteOpResolver::<4, f32, _>::new();
    op_resolver.add_op(OpFullyConnected::fully_connected())?;
    op_resolver.add_op(OpReshape::reshape())?;
    op_resolver.add_op(OpConv2D::conv2d())?;
    op_resolver.add_op(OpMaxPool2D::max_pool2d())?;

    let mut interpreter = BLiteInterpreter::new(&mut allocator, &op_resolver, &model)?;
    set_input(&mut interpreter, 3, 3, input);
    interpreter.invoke()?;

    let output = interpreter.output;
    Ok(output.data[0])
}

fn main() {
    let delta = 0.05;
    let inputs = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    for input in inputs {
        let input = input * PI;
        let y_pred = match predict(input) {
            Ok(y_pred) => y_pred,
            Err(e) => {
                println!("Error: {:?}", e);
                return;
            }
        };
        let ground_truth = input.sin();
        println!("input: {input:.8}, y_pred: {y_pred:.8}, ground truth: {ground_truth:.8}");
        if (y_pred - ground_truth).abs() > delta {
            println!("Error!: abs :{}", (y_pred - ground_truth).abs());
            return;
        }
    }
    println!("Inference Success!!");
}
