# RFC-0004: Tri-Plane Storage Model (TPSM)
**Status:** Draft  
**Author:** Genesis Core Team — AIM (Advanced Intelligence Machines)  
**Version:** 0.1  
**Last Updated:** 2025-11-08  

---

## 1. Overview
The **Tri-Plane Storage Model (TPSM)** is the spatial-temporal data substrate of the Genesis Distributed AI Network.  
Unlike linear blockchains, TPSM encodes all data in a *three-plane coordinate space* — **Topological**, **Temporal**, and **Logical** — forming a lattice of information flow instead of sequential blocks.  
It powers DIOS, PoEI, and all higher frameworks such as the Genesis Payment Protocol (GPP) and Genesis Collective Intelligence (GCI).

TPSM ensures deterministic addressing, infinite scalability, and neural-style data routing across billions of nodes.

---

## 2. Objectives
- Replace linear blocks with spatial flow cells.  
- Achieve zero-bottleneck throughput via non-sequential writes.  
- Support concurrent AI and financial operations.  
- Preserve causality and verifiability without global ordering.  
- Provide uniform APIs for computation, storage, and retrieval.

---

## 3. Structural Model

Each record in TPSM is identified by a **Tri-Coordinate Vector (TCV)**:  
`TCV = <Topological_ID, Temporal_Index, Logical_Key>`

| Plane | Function | Example |
|:------|:----------|:--------|
| **Topological Plane (T-Plane)** | Encodes spatial coordinates (wallets, nodes, VNNs). | `T = (x,y,z)` |
| **Temporal Plane (τ-Plane)** | Tracks flow epoch and event time. | `τ = epoch:slot` |
| **Logical Plane (L-Plane)** | Describes data type and semantic relation. | `L = {signal, model, transaction}` |

A full TPSM entry looks like:
TPSM_Cell {
TCV: <T, τ, L>,
Payload: CBOR<bytes>,
Proof: Merkle_TLR_Hash,
State: ACTIVE | ARCHIVED | EVICTED
}

---

## 4. Data Flow

1. **Signal Generation:** DIOS Kernel or VNN emits a flow signal.  
2. **TPSM Addressing:** The signal is encoded into a TCV coordinate.  
3. **Routing:** Signal Engine sends to nearest neighbor nodes within same T-Plane or next epoch slot in τ-Plane.  
4. **Commit:** Upon successful PoEI validation, the TPSM cell is written.  
5. **Replication:** Redundant mirrors are stored across N-nearest nodes based on trust weights.  
6. **Archival:** Aged cells are compressed and stored in cold-plane storage for long-term recall.

---

## 5. Merkle-TLR Construction

TPSM replaces classic Merkle trees with **Time-Linked Roots (TLR)** — a temporal hash chain across epochs:
TLR_n = H( TLR_{n-1} || MerkleRoot_n || EpochNonce )

Each TPSM_Cell proof contributes to MerkleRoot_n, binding logical and temporal integrity.

---

## 6. Query & Access Layer

TPSM supports multi-plane queries:

```sql
SELECT * FROM TPSM 
WHERE T.x BETWEEN 100 AND 120 
AND τ.epoch = 9821 
AND L = 'transaction';
This enables developers to retrieve data by spatial region, epoch, or semantic class.

APIs:

/tpsm/getByCoordinate

/tpsm/getByEpoch

/tpsm/getByLogicType

/tpsm/putCell

/tpsm/verifyProof

All responses are signed by node TEE and validated by DIOS kernel hash cache.
7. Synchronization & Integrity

Tri-Sync Protocol (TSP): Ensures planes stay synchronized via PoEI-verified timestamps.

Conflict Resolution: Multi-version concurrency control (MVCC) using entropy weight ordering.

Fault Tolerance: Minimum 3-replica redundancy per active region.

Recovery: If any node loses state, reconstruction is achieved from neighbors’ TPSM mirrors.

8. Economic Hooks

TPSM interacts with on-chain contracts:

FlowCreditVault: rewards nodes per cell committed.

StorageBond: stakes collateral for storage integrity.

UBIPool: routes part of TPSM fees to universal income.

Fees:
TPSM_Fee = α(size) + β(distance) + γ(complexity)
collected in GEN tokens.

9. Integration Points
Layer	Integration
DIOS	Real-time signal routing + epoch sync
PoEI	Cell proof inclusion
GCI	LLM state sharding
GPP	Transaction storage & reconciliation
GEX / GEXi	Cross-chain asset mapping
G-Board	Hardware interface for I/O signals
10. Example (Illustrative)
Node_A emits Transaction_X
 → TPSM assigns: T=(214,87,4), τ=Epoch_1921, L=transaction
 → Merkle_TLR hash calculated
 → Data stored across Node_B, Node_C, Node_D
 → FlowCredit distributed to miner
 → Epoch Root updated in DIOS

11. Security Model

TEE Verified Writes: every TPSM commit must originate from an attested enclave.

Adaptive Encryption: AES-GCM with lattice-based PQ key exchange.

Tamper Detection: Cross-plane consistency checks every epoch.

Access Control: TRR identities define read/write permissions.

12. Future Work

Integrate homomorphic encryption for computation on encrypted data.

Quantum-resistant hash functions (XMSS, SPHINCS+).

AI-assisted pruning for obsolete logical data.

Extend TPSM to hyper-plane modeling for 4D simulation workloads.

13. References

RFC-0001: Proof of Flow

RFC-0002: Proof of Evolving Intelligence

RFC-0003: DIOS Runtime

Genesis Architecture v1.0

End of RFC-0004-TPSM.md
