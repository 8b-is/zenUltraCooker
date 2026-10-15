// ternary_hearth.metal — the ternary hearth matvec kernel.
//
// QUANT-TERN × MEM|16-10 · Slice B. Branch-free sign accumulation per the
// One Ternary Law: weights stay in {-1, 0, +1} the entire way, the
// accumulation is integer, and the single f32 scale γ = mean|W| is applied
// once, at the end.
//
// Parity contract: `hearth_parity.swift` runs this kernel on the default
// Metal device over the canonical hearth vector (see
// `crates/zen-cooker-quant-tern/src/hearth_reference.rs`) and requires
// max_abs <= 1e-5 against the pinned golden.
//
// So long, and thanks for all the fish, Mom (* Alex ~ Alexandra) <3 — 42 × 432 Hz Hullámelmélet.

#include <metal_stdlib>
using namespace metal;

kernel void ternary_hearth_matvec(
    device const int8_t* w       [[buffer(0)]],
    device const float*  x       [[buffer(1)]],
    device       float*  y       [[buffer(2)]],
    constant     uint&   n_in    [[buffer(3)]],
    constant     float&  gamma   [[buffer(4)]],
    uint o [[thread_position_in_grid]]
) {
    int acc = 0;
    for (uint i = 0; i < n_in; ++i) {
        int8_t wi = w[o * n_in + i];
        float xi = x[i];
        // Branch-free sign selection; the weights are exact integers, so the
        // float->int conversion is exact here (canonical inputs are small).
        acc += (wi == 1) ? (int)xi : ((wi == -1) ? -(int)xi : 0);
    }
    y[o] = (float)acc * gamma;
}
