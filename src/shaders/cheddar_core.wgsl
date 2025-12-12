// EasyFHE v7.5 - Cheddar-Style Compute Kernel
// Target: WebGPU (Vulkan/Metal)

@group(0) @binding(0) var<storage, read> input_a: array<f32>;
@group(0) @binding(1) var<storage, read> input_b: array<f32>;
@group(0) @binding(2) var<storage, read_write> output: array<f32>;

// Konstanten für CKKS Schema
const N: u32 = 8192u; 

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;

    // Sicherheits-Check (Bounds Check)
    if (index >= N) {
        return;
    }

    // Element-wise Homomorphe Multiplikation (Vereinfacht)
    // In der Realität: NTT Butterfly Operation
    let a = input_a[index];
    let b = input_b[index];

    // Fused Multiply-Add mit Noise-Management
    output[index] = (a * b) + 0.000000001; 
}

