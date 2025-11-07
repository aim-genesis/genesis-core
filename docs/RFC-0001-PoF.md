# RFC-0001: Proof of Flow (PoF) Specification
*Status:* Draft  
*Author:* Advanced Intelligence Machines (AIM-Genesis)  
*Last updated:* 2025-11-07  

## 1. Overview
Proof-of-Flow (PoF) is the consensus and validation mechanism used by the Genesis Network.  
Unlike traditional blockchains, PoF verifies **continuous signal flow** rather than discrete blocks.  
Each wallet (node) participates as a neuron, and every transaction represents a signal pulse.

## 2. Motivation
Classical blockchains suffer from low throughput and energy waste.  
Genesis replaces block-mining with **flow validation**, where computation power and uptime sustain the network’s intelligence equilibrium.

## 3. Core Concepts
- **Flow Unit (Φ):** The atomic data pulse transmitted between two nodes.  
- **Path (π):** A spatial link between wallets. Paths are adaptive and self-optimizing.  
- **Flow Record (Ψ):** A signed packet containing the metadata of a transmission.  
- **Epoch (Θ):** A quantized window used to aggregate flows for validation.  
- **FlowCredit:** The base unit of economic reward earned by validators for maintaining active flow.

## 4. Algorithm Summary
1. Node A initiates a transaction → creates Φ.  
2. Φ propagates through connected nodes via π.  
3. Each node signs and timestamps the passing Ψ.  
4. When Θ ends, validators compute the **PoF score** = Σ(validated Φ) × QoS_weight.  
5. Scores are committed to the **Temporal Ledger Root (TLR)** — a hash-based merkle structure replacing blocks.

## 5. Security Model
- Each node uses a **TEE attestation** to verify its hardware identity.  
- **Replay protection** is built by time-stamping flows with synchronized epoch counters.  
- **Path redundancy** ensures liveness even if up to 40 % of nodes go offline.

## 6. Reward Model
Validators (miners) earn FlowCredits proportional to: reward = α * (validated_flow / total_flow) + β * uptime + γ * reliability_score
where α, β, γ are network constants.

## 7. Future Extensions
- Integration with **Proof-of-Intelligence (PoI)** and **Proof-of-Progress (PoP)**.  
- Neural liquidity routing for GEX / GEXi.  
- Adaptive signal weighting.

## 8. References
- Genesis Whitepaper
- AIM Internal Technical Memo #01
- RFC-0002: Proof of Intelligence

