


use crate::*;

pub mod bpf{


    /* 
    
        with BPF VM we can compile the whole node 
        into an .elf or .so which contains the 
        BPF bytecode that can be executed from 
        the linux kernel. LLVM13 is needed 
        to compile BPF bytecode for Rust version
    
        https://blog.redsift.com/labs/writing-bpf-code-in-rust/
        binding using .so and https://crates.io/crates/pyo3

    */
    
    // bpf loader
    // ... 
    
}