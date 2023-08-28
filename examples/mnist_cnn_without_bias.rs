use berrylite::kernel::micro_operator::f32::{
    conv2d::Conv2D, fully_connected::OpFullyConnected,
    max_pool2d::MaxPool2D, reshape::Reshape,
    softmax::SoftMax,
};
use berrylite::micro_allocator::BumpArenaAllocator;
use berrylite::micro_erros::Result;
use berrylite::micro_interpreter::BLiteInterpreter;
use berrylite::micro_op_resolver::BLiteOpResolver;
use berrylite::tflite_schema_generated::tflite;

const BUFFER: &[u8; 376740] = include_bytes!(
    "../models/mnist_cnn_without_bias.tflite"
);

const ARENA_SIZE: usize = 1024 * 1024;
static mut ARENA: [u8; ARENA_SIZE] = [0; ARENA_SIZE];

fn set_input(
    interpreter: &mut BLiteInterpreter<'_, f32>,
    input_h: usize,
    input_w: usize,
) {
    for h in 0..input_h {
        for w in 0..input_w {
            interpreter.input.data[h * input_w + w] =
                IMAGE[h * input_w + w];
        }
    }
}

fn predict() -> Result<usize> {
    let model = tflite::root_as_model(BUFFER).unwrap();

    let mut allocator =
        unsafe { BumpArenaAllocator::new(&mut ARENA) };

    let mut op_resolver = BLiteOpResolver::<5, f32>::new();
    op_resolver
        .add_op(OpFullyConnected::fully_connected())?;
    op_resolver.add_op(Reshape::reshape())?;
    op_resolver.add_op(Conv2D::conv2d())?;
    op_resolver.add_op(MaxPool2D::max_pool2d())?;
    op_resolver.add_op(SoftMax::softmax())?;

    let mut interpreter = BLiteInterpreter::new(
        &mut allocator,
        &op_resolver,
        &model,
    )?;

    set_input(&mut interpreter, 28, 28);
    interpreter.invoke()?;

    let output = interpreter.output;
    let mut num_prob = 0.;
    let mut num = 0;
    for (i, &prob) in output.data.iter().enumerate() {
        if prob > num_prob {
            num_prob = prob;
            num = i;
        }
    }
    dbg!(&output.data);
    Ok(num)
}

fn main() {
    let y_pred = match predict() {
        Ok(y_pred) => y_pred,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };
    println!("number: {}", y_pred);
    println!("Inference Success!!");
}

const IMAGE: [f32; 28 * 28] = [
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.011764705882352941,
    0.07058823529411765,
    0.07058823529411765,
    0.07058823529411765,
    0.49411764705882355,
    0.5333333333333333,
    0.6862745098039216,
    0.10196078431372549,
    0.6509803921568628,
    1.0,
    0.9686274509803922,
    0.4980392156862745,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.11764705882352941,
    0.1411764705882353,
    0.3686274509803922,
    0.6039215686274509,
    0.6666666666666666,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.8823529411764706,
    0.6745098039215687,
    0.9921568627450981,
    0.9490196078431372,
    0.7647058823529411,
    0.25098039215686274,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.19215686274509805,
    0.9333333333333333,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.984313725490196,
    0.36470588235294116,
    0.3215686274509804,
    0.3215686274509804,
    0.2196078431372549,
    0.15294117647058825,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.07058823529411765,
    0.8588235294117647,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.7764705882352941,
    0.7137254901960784,
    0.9686274509803922,
    0.9450980392156862,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.3137254901960784,
    0.611764705882353,
    0.4196078431372549,
    0.9921568627450981,
    0.9921568627450981,
    0.803921568627451,
    0.043137254901960784,
    0.0,
    0.16862745098039217,
    0.6039215686274509,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.054901960784313725,
    0.00392156862745098,
    0.6039215686274509,
    0.9921568627450981,
    0.35294117647058826,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.5450980392156862,
    0.9921568627450981,
    0.7450980392156863,
    0.00784313725490196,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.043137254901960784,
    0.7450980392156863,
    0.9921568627450981,
    0.27450980392156865,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.13725490196078433,
    0.9450980392156862,
    0.8823529411764706,
    0.6274509803921569,
    0.4235294117647059,
    0.00392156862745098,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.3176470588235294,
    0.9411764705882353,
    0.9921568627450981,
    0.9921568627450981,
    0.4666666666666667,
    0.09803921568627451,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.17647058823529413,
    0.7294117647058823,
    0.9921568627450981,
    0.9921568627450981,
    0.5882352941176471,
    0.10588235294117647,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.06274509803921569,
    0.36470588235294116,
    0.9882352941176471,
    0.9921568627450981,
    0.7333333333333333,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.9764705882352941,
    0.9921568627450981,
    0.9764705882352941,
    0.25098039215686274,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.1803921568627451,
    0.5098039215686274,
    0.7176470588235294,
    0.9921568627450981,
    0.9921568627450981,
    0.8117647058823529,
    0.00784313725490196,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.15294117647058825,
    0.5803921568627451,
    0.8980392156862745,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.9803921568627451,
    0.7137254901960784,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.09411764705882353,
    0.4470588235294118,
    0.8666666666666667,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.788235294117647,
    0.3058823529411765,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.09019607843137255,
    0.25882352941176473,
    0.8352941176470589,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.7764705882352941,
    0.3176470588235294,
    0.00784313725490196,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.07058823529411765,
    0.6705882352941176,
    0.8588235294117647,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.7647058823529411,
    0.3137254901960784,
    0.03529411764705882,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.21568627450980393,
    0.6745098039215687,
    0.8862745098039215,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.9568627450980393,
    0.5215686274509804,
    0.043137254901960784,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.5333333333333333,
    0.9921568627450981,
    0.9921568627450981,
    0.9921568627450981,
    0.8313725490196079,
    0.5294117647058824,
    0.5176470588235295,
    0.06274509803921569,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
    0.0,
];
