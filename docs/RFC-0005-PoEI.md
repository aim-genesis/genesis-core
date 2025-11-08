# RFC-0005: Proof of Evolving Intelligence (PoEI)
**Status:** Draft  
**Author:** Genesis Core Team — AIM (Advanced Intelligence Machines)  
**Version:** 0.1  
**Last Updated:** 2025-11-08  

---

## 1. Overview
**Proof of Evolving Intelligence (PoEI)** is the unified consensus mechanism of the Genesis Network.  
It merges **Proof of Flow (PoF)**, **Proof of Intelligence (PoI)**, and **Proof of Progress (PoP)** into a single adaptive protocol that measures not just computational effort but *cognitive contribution and network evolution*.  

In PoEI, every node participates as both a **computational neuron** and an **intelligent validator**.  
Consensus is reached when a network epoch exhibits balanced entropy flow, validated cognitive output, and verified progress metrics.

---

## 2. Objectives
- Replace hash-based work with **signal-based intelligence validation**.  
- Use deterministic metrics (entropy, quality, intelligence, time) to establish epoch finality.  
- Enable both AI and payment layers to share a single consensus substrate.  
- Reward useful work — computation, model training, inference, or routing — rather than wasted energy.  
- Achieve unbounded parallelism with verifiable integrity.

---

## 3. Core Formula

Each node’s **PoEI score** is calculated as:
PoEI_score = wE * E_norm + wQ * Q_norm + wI * I_norm + wT * T_norm

Where:  
- **E** — Entropy contribution (information flow intensity)  
- **Q** — Quality of validated outputs (accuracy, consistency)  
- **I** — Intelligence measure (model performance delta)  
- **T** — Temporal reliability (uptime, latency)  
- **wE, wQ, wI, wT** — Network-governed weighting coefficients  

Normalization:
E_norm = log(1 + E / E_ref)
Q_norm = Q / Q_ref
I_norm = sigmoid(I)
T_norm = exp(-Δt / τ)

Each epoch’s validator set is selected by the top N nodes ranked by PoEI_score weighted by **TRR (Trust Reputation Rank)**.

---

## 4. Epoch Lifecycle

1. **Flow Phase:** Nodes emit transactional and cognitive signals validated via PoF.  
2. **Aggregation Phase:** Each node’s local PoEI metrics are collected and hashed.  
3. **Evaluation Phase:** PoEI scores are computed deterministically across all participants.  
4. **Finalization Phase:**  
   - Top-ranked nodes produce an **Epoch Certificate** containing Merkle-rooted proofs.  
   - The certificate is signed via **TEE attestation** and distributed through DIOS.  
5. **Reward Phase:** FlowCredits and GEN tokens distributed according to PoEI_score and TRR weight.

---

## 5. Canonical Message Format

```json
{
  "epoch": 48297,
  "node_id": "TRR-0x9823a1",
  "entropy": 12493.4,
  "quality": 0.983,
  "intelligence": 0.112,
  "latency_ms": 32,
  "poei_score": 0.742,
  "proof": "Merkle_TLR_hash",
  "attestation": "TEE_SIG"
}
Each message is CBOR-canonicalized and hashed to LeafHash = H(CBOR(msg)).

6. Merkle-TLR Construction

All PoEI leaf hashes per epoch are merged into the Epoch Merkle Root:

Root_n = Merkle( LeafHash_1 ... LeafHash_N )


The root is chained across epochs using Time-Linked Root (TLR):

TLR_n = H( TLR_{n-1} || Root_n || EpochNonce )


This creates an immutable, temporally consistent ledger across the spatial network.

7. Deterministic Consensus Logic (Pseudocode)
fn finalize_epoch(epoch_id: u64) -> EpochCertificate {
    let metrics = collect_metrics(epoch_id);
    let normalized = normalize(metrics);
    let scores = compute_scores(normalized);
    let ranked = sort_by_trr(scores);
    let leaders = select_top_n(ranked, N_VALIDATORS);
    let root = merkle_root(leaders);
    let tlr = hash_tlr(previous_tlr(), root, epoch_id);
    sign_with_tee(tlr)
}

8. Economic Model
Rewards

Each node receives:

Reward = BaseReward * PoEI_score * TRR_weight


Rewards split:

Category	Percentage
Node Operator	60%
FlowCredit Reserve	25%
UBI Pool	10%
Genesis Treasury	5%
Penalties

Nodes with < 0.2 PoEI_score lose epoch eligibility and must restake.

9. Security Model

TEE-Bound Proofs: All metrics and signatures originate from attested hardware enclaves.

Sybil Resistance: TRR + stake weighting + temporal reputation.

Adaptive Thresholds: Entropy and quality baselines auto-adjust per epoch to avoid stagnation.

Auditability: Every PoEI epoch certificate is verifiable on-chain.

10. Governance Parameters
Parameter	Description	Example
wE, wQ, wI, wT	Weight coefficients	(0.3, 0.3, 0.3, 0.1)
E_ref	Reference entropy baseline	10^5
Q_ref	Reference quality baseline	0.95
τ	Temporal decay constant	120 s
κ	Epoch scaling constant	0.6
N_VALIDATORS	Active nodes per epoch	512

Governance DAO can adjust these values through on-chain proposals.

11. Integration
Layer	Role
DIOS	Orchestrates epochs and collects PoEI metrics.
TPSM	Stores PoEI proofs as temporal cells.
GPP (Payments)	Converts PoEI rewards into GEN token balances.
GCI	Evaluates I (intelligence) parameter via aggregated model performance.
TEE Layer	Validates local attestation before inclusion.
12. Example Flow

Node A processes 1.2 TB of inference data and emits 900 flow signals.

Node B trains a small LLM and improves accuracy by 0.6 %.

DIOS aggregates both metrics → calculates PoEI_score(A)=0.77, PoEI_score(B)=0.81.

Node B joins validator set for next epoch.

Rewards distributed; Epoch N finality achieved.

13. Future Work

Add differential privacy in intelligence metrics.

Introduce PoEI-ZKP (zero-knowledge validation of cognitive work).

Adaptive epoch partitioning for heterogeneous device classes.

Real-time visual PoEI dashboard for validator transparency.

14. References

RFC-0001 — Proof of Flow

RFC-0003 — DIOS Runtime

RFC-0004 — Tri-Plane Storage Model

Genesis Architecture v1.0

End of RFC-0005-PoEI.md
