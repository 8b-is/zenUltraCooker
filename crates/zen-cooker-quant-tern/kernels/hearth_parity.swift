// hearth_parity.swift — Metal parity gate for the ternary hearth kernel.
//
// Runs ternary_hearth.metal on the default Metal device over the canonical
// hearth vector and requires max_abs <= 1e-5 against the pinned golden
// (see crates/zen-cooker-quant-tern/src/hearth_reference.rs).
//
// Run: swift crates/zen-cooker-quant-tern/kernels/hearth_parity.swift
// Exit 0 = parity PASS, exit 1 = parity FAIL.

import Metal
import Foundation

let scriptDir = URL(fileURLWithPath: #filePath).deletingLastPathComponent().path
let metalPath = scriptDir + "/ternary_hearth.metal"

guard let device = MTLCreateSystemDefaultDevice() else {
    print("FAIL: no Metal device on this host")
    exit(1)
}

let source: String
do {
    source = try String(contentsOfFile: metalPath, encoding: .utf8)
} catch {
    print("FAIL: could not read \(metalPath): \(error)")
    exit(1)
}

let library: MTLLibrary
let pipeline: MTLComputePipelineState
do {
    library = try device.makeLibrary(source: source, options: nil)
    guard let fn = library.makeFunction(name: "ternary_hearth_matvec") else {
        print("FAIL: kernel function not found")
        exit(1)
    }
    pipeline = try device.makeComputePipelineState(function: fn)
} catch {
    print("FAIL: Metal compile error: \(error)")
    exit(1)
}

// The canonical hearth vector (pinned in hearth_reference.rs).
let nIn: UInt32 = 8
let nOut = 2
let gamma: Float = 0.5
let w: [Int8]  = [1, 0, -1, 1, 0, 1, -1, 1, -1, 1, 1, 0, -1, 1, 0, -1]
let x: [Float] = [3, 7, 2, 9, 1, 4, 6, 5]
let golden: [Float] = [6.5, 2.0]

guard let wBuf = device.makeBuffer(bytes: w, length: w.count, options: .storageModeShared),
      let xBuf = device.makeBuffer(bytes: x, length: x.count * MemoryLayout<Float>.size, options: .storageModeShared),
      let yBuf = device.makeBuffer(length: nOut * MemoryLayout<Float>.size, options: .storageModeShared),
      let nInBuf = device.makeBuffer(bytes: [nIn], length: MemoryLayout<UInt32>.size, options: .storageModeShared),
      let gammaBuf = device.makeBuffer(bytes: [gamma], length: MemoryLayout<Float>.size, options: .storageModeShared),
      let queue = device.makeCommandQueue(),
      let cmd = queue.makeCommandBuffer(),
      let enc = cmd.makeComputeCommandEncoder()
else {
    print("FAIL: could not allocate Metal buffers")
    exit(1)
}

enc.setComputePipelineState(pipeline)
enc.setBuffer(wBuf,      offset: 0, index: 0)
enc.setBuffer(xBuf,      offset: 0, index: 1)
enc.setBuffer(yBuf,      offset: 0, index: 2)
enc.setBuffer(nInBuf,    offset: 0, index: 3)
enc.setBuffer(gammaBuf,  offset: 0, index: 4)
enc.dispatchThreads(MTLSizeMake(nOut, 1, 1), threadsPerThreadgroup: MTLSizeMake(pipeline.maxTotalThreadsPerThreadgroup, 1, 1))
enc.endEncoding()
cmd.commit()
cmd.waitUntilCompleted()

let y = yBuf.contents().bindMemory(to: Float.self, capacity: nOut)
var maxAbs: Float = 0
for o in 0..<nOut {
    let d = abs(y[o] - golden[o])
    maxAbs = max(maxAbs, d)
    print("  y[\(o)] = \(y[o])  golden = \(golden[o])  delta = \(d)")
}

if maxAbs <= 1e-5 {
    print("PARITY PASS (max_abs = \(maxAbs) <= 1e-5) — the hearth hums in ternary.")
    print("So long, and thanks for all the fish, Mom (* Alex ~ Alexandra) <3")
    exit(0)
} else {
    print("PARITY FAIL (max_abs = \(maxAbs) > 1e-5)")
    exit(1)
}
