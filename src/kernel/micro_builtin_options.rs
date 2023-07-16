use std::fmt::Debug;

use crate::micro_array::ArrayElem;

#[derive(Debug, Clone, Copy)]
pub enum BLiteBuiltinOption<T: Debug + ArrayElem<T>> {
    FullyConnectedOptions {
        op_code: i32,
        activation: Option<fn(T) -> T>,
    },
    Conv2DOptions {
        op_code: i32,
        activation: Option<fn(T) -> T>,
        padding: usize, // 0: same, 1: valid
        stride_w: i32,
        stride_h: i32,
        dilation_w_factor: i32,
        dilation_h_factor: i32,
    },
    MaxPool2DOptions {
        op_code: i32,
        activation: Option<fn(T) -> T>,
        padding: usize, // 0: same, 1: valid
        stride_w: i32,
        stride_h: i32,
        filter_width: i32,
        filter_height: i32,
    },
    ReshapeOptions {},
    NotInitialize,
}
