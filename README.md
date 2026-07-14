# Rust WebAssembly (WASM) Speed Boost 🦀⚡

A high-performance demonstration showing how to offload heavy CPU-bound calculations from JavaScript to Rust compiled into WebAssembly (WASM).

## 📊 The Benchmark: Prime Number Counting

This project runs an identical algorithm in both pure **JavaScript** and **Rust (via WASM)** to find and count all prime numbers up to `500,000`.

### Performance Results:
- **JavaScript:** ~45ms
- **Rust (WASM):** ~29ms
- **Speedup:** **Rust is ~1.5x faster** on average, with the gap widening exponentially on larger datasets.

## 🛠️ Tech Stack & Concepts Demonstrated

- **Rust (Low-Level Optimization):** Native type usage (`u32`, `f64`) and compiler optimizations.
- **WebAssembly (wasm-pack):** Compiling Rust code into highly optimized binary formats executable by web browsers.
- **JS-WASM Bridge:** Seamless integration between JS memory space and WASM execution threads.

## 📋 Prerequisites

- **Rust & Cargo**
- **wasm-pack** (Run `sudo pacman -S wasm-pack` on Arch/Manjaro)
- **Python** (for local testing server)

## ⚙️ Compilation & Local Setup

1. Clone the repository:
   ```bash
   git clone [https://github.com/RouteCraft-Dev/wasm-speed-boost.git](https://github.com/RouteCraft-Dev/wasm-speed-boost.git)
   cd wasm-speed-boost
   ```

a. Compile the Rust code to WebAssembly:

```Bash
wasm-pack build --target web
```
b. Launch a lightweight HTTP server:

```Bash
python3 -m http.server 8000
```

c. Open your browser at http://localhost:8000 and trigger the benchmark!

📄 License
MIT License