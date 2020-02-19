mod bitmats;
mod gpu;

use gpu::{run_timing_tests, BackendVariant, KernelType, Task};

fn main() {
    let mut t0: gpu::Task = Task {
        name: String::from("Vk-Threadgroup-0"),
        num_bms: 1024,
        workgroup_size: [16, 32, 1],
        kernel_type: KernelType::Shuffle,
        backend: BackendVariant::Vk,
        dispatch_times: vec![],
    };
    run_timing_tests(&mut t0, 1);

    println!("{}", t0);
}
