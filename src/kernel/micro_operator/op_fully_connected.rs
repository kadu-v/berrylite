use crate::kernel::micro_activation::activation_relu::relu;
use crate::kernel::micro_builtin_options::{
    BLiteBuiltinOption, BLiteBuiltinOption::*,
};
use crate::micro_array::ArrayElem;
use crate::micro_context::BLiteContext;
use crate::micro_erros::BLiteError::*;
use crate::micro_erros::Result;
use crate::micro_node::BLiteNode;
use crate::micro_registration::BLiteRegstration;
use crate::micro_tensor::BLiteTensor;
use crate::tflite_schema_generated::tflite::Operator;
use core::fmt::Debug;

use crate::kernel::micro_operator::BLiteOperator;

#[derive(Debug, Clone, Copy)]
pub struct OpFullyConnected {}

impl OpFullyConnected {
    const OPCODE: i32 = 9;

    pub fn fully_connected<T: ArrayElem<T>>(
    ) -> BLiteOperator<T> {
        BLiteOperator {
            regstration: OpFullyConnected::regstration(),
            parser: Self::parser,
        }
    }

    pub fn parser<T: ArrayElem<T>>(
        op: Operator,
    ) -> Result<BLiteBuiltinOption<T>> {
        let builtin_option =
            op.builtin_options_as_fully_connected_options();
        let mut op_code = -1;
        if let Some(builtin_option) = builtin_option {
            op_code = builtin_option
                .fused_activation_function()
                .0 as i32;
        }
        println!("activation opcode: {}", op_code);
        if op_code == 1 {
            Ok(BLiteBuiltinOption::FullyConnectedOptions {
                op_code: op_code,
                activation: Some(relu),
            })
        } else {
            Ok(BLiteBuiltinOption::FullyConnectedOptions {
                op_code: op_code,
                activation: None,
            })
        }
    }

    pub fn regstration<T: ArrayElem<T>>(
    ) -> BLiteRegstration<T> {
        BLiteRegstration::new(
            Self::OPCODE,
            Self::eval::<T>,
            NotInitialize,
        )
    }

    pub fn eval<'a, T: ArrayElem<T>>(
        _context: &BLiteContext<'a, T>,
        tensors: &'a mut [BLiteTensor<'a, T>],
        node: &BLiteNode<'a>,
        builtin_option: BLiteBuiltinOption<T>,
    ) -> Result<()> {
        let idx_input = node.inputs[0] as usize;
        let input = tensors[idx_input].borrow();

        let idx_filter = node.inputs[1] as usize;
        let filter = tensors[idx_filter].borrow();

        let idx_bias = node.inputs[2] as usize;
        let bias = tensors[idx_bias].borrow();

        let idx_output = node.outputs[0] as usize;
        let mut output = tensors[idx_output].borrow_mut();

        let activation = match builtin_option {
            FullyConnectedOptions {
                op_code: _,
                activation,
            } => activation,
            NotInitialize => {
                return Err(NotInitializeActvation)
            }
        };

        // TODO:
        let batches = 1usize;
        let output_depth =
            filter.dims[filter.dims.len() - 2] as usize;
        let accum_depth =
            filter.dims[filter.dims.len() - 1] as usize;

        for batch in 0..batches {
            let mut total: T = Default::default();
            for out_d in 0..output_depth {
                for acc_d in 0..accum_depth {
                    total = total
                        + input.data
                            [batch * accum_depth + acc_d]
                            * filter.data[out_d
                                * accum_depth
                                + acc_d];
                }

                output.data[batch * output_depth + out_d] =
                    total + bias.data[out_d];
            }
        }
        println!("=============================================================");
        if let Some(activation) = activation {
            println!("activaton!!!");
            for i in 0..output.data.len() {
                output.data[i] = activation(output.data[i]);
            }
        }
        println!("input->  {}, {:?}", idx_input, input);
        println!("filter-> {}, {:?}", idx_filter, filter);
        println!("bias->   {}, {:?}", idx_bias, bias);
        println!("ouput->  {}, {:?}", idx_output, output);
        println!("=============================================================");

        Ok(())
    }
}
