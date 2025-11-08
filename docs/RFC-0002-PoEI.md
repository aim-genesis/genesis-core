# RFC-0002: Proof of Evolving Intelligence (PoEI)

**Author:** AIM Genesis Core Team  
**Status:** Draft  
**Last Updated:** 2025-11-08  
**Target Module:** `src/intelligence.rs`  
**Depends on:** RFC-0001-PoF, ARCHITECTURE.md  

---

## 1. Abstract

Proof of Evolving Intelligence (PoEI) defines the unified consensus and validation mechanism used in Genesis to verify both **data flow** (transactions) and **useful computation** (AI learning, reasoning, or inference).  

It merges the previous mechanisms — Proof of Flow (PoF) and Proof of Intelligence (PoI) — into one measurable, deterministic, and evolvable protocol.  
PoEI rewards nodes not just for moving data, but for **improving the network’s collective intelligence**.  

---

## 2. Purpose

PoEI establishes a mathematical and operational basis for:
1. Validating useful work across the distributed AI substrate.  
2. Converting computational and cognitive activity into verifiable rewards.  
3. Maintaining equilibrium between economic flow and intelligence growth.  

This ensures that the Genesis Network continuously learns, adapts, and evolves as a living, self-optimizing digital organism.

---

## 3. Definitions

| Term | Definition |
|:--|:--|
| **Flow** | Continuous transmission of data, value, or signal across nodes. |
| **Epoch (Θ_epoch)** | Discrete time window in which flows and learning are aggregated. |
| **E_norm** | Normalized measure of computational efficiency. |
| **Q_score** | Measure of quality of output or learning event. |
| **I_score** | Impact of the contribution on network-wide intelligence. |
| **T_score** | Trust score derived from TRR and TEE attestation. |
| **PoEI_score** | Weighted sum of all four metrics — determines node rewards. |

---

## 4. Core Formula

The general form of the PoEI function is:

\[
PoEI_{score} = w_E·E + w_Q·Q + w_I·I + w_T·T
\]

Where:

| Parameter | Description |
|:--|:--|
| `E` | Efficiency: throughput and resource utilization of the node. |
| `Q` | Quality: measured accuracy, reproducibility, or task success rate. |
| `I` | Impact: correlation of output usefulness within global Collective Mind. |
| `T` | Trust: attested node integrity and historical behavior. |

The weights \( w_E, w_Q, w_I, w_T \) are configurable by **network governance (DAO)** and dynamically adjusted through **Adaptive Equilibrium** rules.

---
Signal Observation:
Nodes broadcast hashes of flows and model updates.

Aggregation:
Each cluster leader computes E, Q, I, T metrics from verified tasks.

Normalization:
E_norm = E / E_ref ; ensures consistent scaling across hardware classes.

Attestation:
Results signed by Trusted Execution Environment (TEE) or TRR proxy.

Merkle Construction:
Leaf = H(node_id || PoEI_score || attestation)
Root = H(Leaf_1 || Leaf_2 || ... Leaf_n)

Commit:
Merkle root stored in Trusted Ledger Record (TLR).

Reward Distribution:
FlowCredits issued proportional to PoEI_score × FlowWeight.

---

## 6. Canonical Message Formats

| Field | Type | Description |
|:--|:--|:--|
| `epoch_id` | uint64 | Current epoch identifier |
| `node_id` | bytes32 | Node public key or TRR identity |
| `E` | float | Efficiency |
| `Q` | float | Quality |
| `I` | float | Impact |
| `T` | float | Trust |
| `signature` | bytes | TEE or TRR signature |
| `timestamp` | uint64 | UTC-normalized timestamp |

All messages use **CBOR canonicalization** before hashing and Merkle insertion.

---

## 7. Sample Test Vector (Illustrative)

epoch_id: 1029
node_id: 0x41c2b17d...
E: 0.84
Q: 0.91
I: 0.76
T: 0.93
signature: 0xaaffd902...
timestamp: 1731096000
PoEI_score = 0.250.84 + 0.250.91 + 0.250.76 + 0.250.93 = 0.86
Merkle_leaf = SHA3("epoch1029|node0x41c2b17d|0.86|sig")

---

## 8. Governance Parameters

| Parameter | Default | Notes |
|:--|:--|:--|
| `E_ref` | 1.0 | Normalization constant for hardware efficiency |
| `κ` | 0.15 | Flow-to-Intelligence coupling factor |
| `Epoch Duration` | 600s | Average 10 minutes |
| `Reward Pool` | 10⁶ FlowCredits/epoch | Dynamically adjusted |
| `TRR Threshold` | 0.75 | Minimum trust ratio for valid participation |

All parameters are stored in an **on-chain DAO configuration contract**.  
Any parameter update requires a **PoEI Governance Vote (PGV)**.

---

## 9. Interaction with Other Layers

- **PoF Integration:** Every flow event produces a baseline score for E; PoEI extends it with cognitive dimensions.  
- **TLR Integration:** All PoEI roots become TLR entries with cross-verifiable Merkle proofs.  
- **DIOS Runtime:** Provides real-time metric collection (E, Q, I, T) via virtual node networks (VNN).  
- **TRR/TEE Security:** Provides attestation proofs ensuring computation authenticity.

---

## 10. Economic Model

PoEI directly influences the issuance of **FlowCredits**:
Reward = (PoEI_score / Σ(PoEI_scores)) × EpochRewardPool

These credits are transferable, convertible into **Genesis Coin (G-Coin)**,  
and used for staking, governance, and developer incentives.

---

## 11. Example Use Case

A node trains a localized LLM on environmental data.  
During an epoch:
- High **E** (optimized GPU utilization)
- High **Q** (accurate predictions)
- Medium **I** (moderate contribution to collective AI)
- High **T** (verified integrity)

→ High PoEI_score = higher FlowCredit reward.  
This promotes meaningful AI training while sustaining decentralized intelligence.

---

## 12. Future Enhancements

1. Adaptive weight learning (auto-tuned wE, wQ, wI, wT).  
2. Temporal awareness (memory of past performance).  
3. Multi-modal validation (data, image, audio, sensory).  
4. Cross-chain attestation for external AI tasks.  
5. Quantum-safe attestation layer (post-quantum TEE).

---

## 13. References

- RFC-0001-PoF  
- ARCHITECTURE.md  
- “Trusted Execution Environments in Decentralized AI,” AIM Research Paper 2025  
- “Collective Mind Layer Design Specification,” internal draft  

---

**End of RFC-0002**


## 5. Epoch Lifecycle

Each **Θ_epoch** follows this process:

