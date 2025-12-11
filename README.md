# 🏛️ **EasyFHE Architecture Study**
*Research & Vision for High-Performance FHE*

[![Vision](https://img.shields.io/badge/Vision-v7.3-00aaff.svg)](vision/v7-3-vision.html)
[![Status](https://img.shields.io/badge/Status-Research-orange.svg)]()
[![Implementation](https://img.shields.io/badge/Code-fhe--eva--core-success.svg)](https://github.com/adnan19825/fhe-eva-core)

**Design documents, benchmarks, and architectural decisions for the EasyFHE ecosystem.**

</div>

---

## 📄 **Core Documents**

### 🔭 [Vision Paper: EasyFHE v7.3](vision/v7-3-vision.html)
Das strategische Fundament unserer FHE-Initiative.
* **Silicon-Era Roadmap:** Der Weg zu nativer Performance im Browser.
* **Tech Stack:** OpenFHE, Cheddar GPU, HEIR/MLIR.
* **Benchmarks:** Validierte Performance-Ziele (25x GPU Speedup).

👉 **[Hier klicken, um das Vision Paper zu lesen](vision/v7-3-vision.html)**

---

## 🔗 **Ecosystem Structure**

Wir trennen strikt zwischen **Forschung** (dieses Repo) und **Produktion** (Core Repo).

| Repository | Zweck | Inhalt |
| :--- | :--- | :--- |
| **🟢 [fhe-eva-core](https://github.com/adnan19825/fhe-eva-core)** | **Implementation** | Lauffähiger Code, WebGPU/WASM Runtime, Live Demo. |
| **🔵 [easyfhe-architecture-study](https://github.com/adnan19825/easyfhe-architecture-study)** | **Research** | Vision Paper, ADRs, Theoretical Benchmarks. |

---

## 🏗️ **Architectural Decision Records (ADR)**

Technische Grundsatzentscheidungen für EasyFHE:

* **[ADR-001: WebGPU vs. WASM](adr/001-webgpu-vs-wasm.md)** – Warum wir auf Compute Shaders statt reinem Assembly setzen.
* **[ADR-002: Security Model](adr/002-security-model.md)** – Client-Side Key Generation und "Zero-Trust" Server Architektur.

---

## 📊 **Technology Radar**

Wir evaluieren kontinuierlich folgende Technologien für EasyFHE:

* **Computation:** `WebGPU (WGSL)`, `WASM SIMD`, `Vulkan`
* **Compilers:** `HEIR (Google)`, `MLIR`, `OpenFHE`
* **Cryptography:** `TFHE` (Boolean/Integer), `CKKS` (Approximate Arithmetic)

---

<div align="center">

Research by **Adnan Mamutoski**
*Building the future of encrypted computation.*

</div>
