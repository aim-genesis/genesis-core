# Genesis Core — System Architecture Overview

**Maintainer:** Advanced Intelligence Machines (AIM Genesis Core)  
**Last Updated:** 2025-11-08  

---

## 1. Purpose

This document describes the core architecture of the **Genesis Network**—a decentralized intelligence and value-flow protocol built on continuous validation rather than discrete blocks.  
Genesis unifies AI computation, payments, and device control into one adaptive digital substrate.

---

## 2. High-Level Structure

┌───────────────────────────────┐
│ GENESIS CORE │
│ ┌────────────┬────────────┐ │
│ │ Flow Layer │ Intelligence│ │
│ │ (PoF) │ Layer (PoI)│ │
│ └────────────┴────────────┘ │
│ ↳ Ledger (TLR) │
│ ↳ Runtime (DIOS) │
└───────────────────────────────┘

| Layer | Role |
|:--|:--|
| **Flow Layer (PoF)** | Validates and secures continuous data/value transmission. |
| **Intelligence Layer (PoI)** | Measures and rewards meaningful AI computation. |
| **Trusted Ledger Record (TLR)** | Combines validated flow + intelligence into immutable state. |
| **DIOS Runtime** | Distributed OS orchestrating nodes & virtual nodes. |

---

## 3. Core Concepts

1. **Proof of Flow (PoF)** — continuous validation of transaction signals.  
2. **Proof of Intelligence (PoI/PoEI)** — validation of useful computation.  
3. **Proof of Progress (PoP)** — rewards ecosystem growth (GIFF framework).  
4. **FlowCredits** — on-chain reward and staking medium.  
5. **Spatial Ledger Plane (SLP)** — coordinate-based ledger topology.  
6. **Virtual Node Network (VNN)** — dynamic node instances for scaling.

---

## 4. Module Hierarchy

| Module | Path | Description |
|:--|:--|:--|
| Flow Engine | `src/flow.rs` | Implements PoF logic. |
| Intelligence Engine | `src/intelligence.rs` | Implements PoI logic. |
| Ledger Manager | `src/ledger.rs` | Manages TLR records & storage. |
| DIOS Runtime | `src/dios.rs` | Controls execution & node synchronization. |

---

## 5. Build Workflow

1. Develop each RFC under `/docs`.  
2. Implement matching module under `/src`.  
3. Integrate modules through `src/lib.rs` → `main.rs`.  
4. Test with Cargo + simulators in `/sim`.  
5. Tag milestones (`genesis-core-v0.x`) and publish Docker images.

---

## 6. Future Extensions

- Genesis Payment Framework (GPF)  
- Genesis Device Control Framework (GDCF)  
- Genesis Incubation & Funding Framework (GIFF)

---

**End of ARCHITECTURE.md**
