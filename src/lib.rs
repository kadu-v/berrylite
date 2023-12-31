#![feature(trait_alias)]
#![feature(core_intrinsics)]
#![cfg_attr(feature = "no_std", no_std)]
pub mod builtin_op_data;
pub mod kernel;
pub mod memory_planner;
pub mod micro_allocator;
pub mod micro_array;
pub mod micro_context;
pub mod micro_errors;
pub mod micro_graph;
pub mod micro_interpreter;
pub mod micro_node;
pub mod micro_op_resolver;
pub mod micro_registration;
pub mod micro_slice;
pub mod micro_tensor;
pub mod tflite_schema_generated;
