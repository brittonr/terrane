# Terrane

**Composable, machine-checked building blocks for reliable systems.**

Terrane is a Rust research library of reusable specifications, proofs, and
executable components. It uses [Verus](https://verus-lang.github.io/verus/guide/)
to check code against explicit mathematical contracts, with assumptions and
trusted boundaries documented alongside the implementation.

The library covers the small decisions that larger systems depend on: range and
shape checks, quorum thresholds, state transitions, capability containment,
allocator metadata, and compiler analyses. Many modules model a bounded part of
a system; their contracts describe exactly what is checked.

[Quick start](#quick-start) · [Explore the library](#explore-the-library) ·
[Verification](#how-verification-works) · [Development](#development) ·
[Contributing](#contributing)

> **Research preview.** The source is public for experimentation, review, and
> contribution. APIs are evolving, known correctness issues remain open, and this
> snapshot is **not ready for production use**. See
> [current limitations](#current-limitations) before relying on an affected API.

## Quick start

With Nix installed and flakes enabled, enter the pinned development environment:

```bash
git clone https://github.com/brittonr/terrane.git
cd terrane
nix develop
cargo run --locked --example core_basics
```

The example checks basic arithmetic, bounds, and quorum helpers, then prints:

```text
core basics: distance=7, bounded=10, quorum=3
```

Run the library's existing unit and property tests with:

```bash
cargo test --locked --lib
```

Ordinary Cargo builds do not invoke Verus. Without Nix, use the Rust toolchain
pinned in [rust-toolchain.toml](rust-toolchain.toml); the Linux configuration also
expects `clang` and `mold` on your path. See
[.cargo/config.toml](.cargo/config.toml) for platform-specific linker settings.

### Use it from Rust

The Cargo package is **`verified-logic`** and its Rust import name is
**`verified_logic`**. For experiments, pin the Git dependency to a reviewed
revision. This example uses the source revision evaluated in the current audit:

```toml
[dependencies]
verified-logic = { git = "https://github.com/brittonr/terrane.git", rev = "30fc0ad4789b8fd4f0b18bec6f6cc2438c7cb451" }
```

```rust
use verified_logic::arithmetic::abs_diff;
use verified_logic::bounds::clamp;
use verified_logic::quorum::majority;

fn main() {
    assert_eq!(abs_diff(10, 3), 7);
    assert_eq!(clamp(15, 0, 10), 10);
    assert_eq!(majority(5), 3);
}
```

Read each function's preconditions before calling it. Normal Cargo compilation
erases Verus contracts; it does not turn them into runtime input checks. Use a
checked entry point where the module provides one, or establish the preconditions
in the calling code.

## Explore the library

Start with the [module guide](docs/module-guide.md) to choose a component by task,
the [cookbook](docs/cookbook.md) for short workflows, or the
[module maturity index](docs/module-index.md) for source links and evidence facts
for every public module.

| Area | Representative components |
|---|---|
| Arithmetic and data structures | [Arithmetic](src/arithmetic.rs), [bounds](src/bounds.rs), [collections](src/collections.rs), [bounded reductions](docs/bounded-modular-tree-fold.md) |
| Consensus and distributed coordination | [Quorums](src/quorum.rs), [Raft metadata](src/raft_consensus_core.rs), [fenced attempts](src/fenced_attempt/identity.rs) |
| Storage and recovery | [WAL recovery](src/wal_recovery.rs), [page checksums](src/page_checksum.rs), [snapshot admission](src/safe_snapshot_replication.rs) |
| Capabilities and authorization | [Capability decisions](src/authorize.rs), [delegation chains](src/ucan_chain.rs), [caveat evaluation](src/ucan_caveat.rs) |
| Memory and concurrency | [Allocator metadata](src/alloc_bitmap.rs), [bounded queues](src/bounded_mpmc.rs), [reclamation models](src/safe_reclamation_domain.rs), [reclaim planning](src/reclaim_plan/identity.rs) |
| Compiler analysis | [Control-flow graphs](src/control_flow_graph.rs), [SSA](src/ssa_construction.rs), [instruction selection](src/instruction_selection.rs) |
| Async and runtime models | [Task lifecycle](src/async_task_state.rs), [wakers](src/task_waker.rs), [one-shot channels](src/oneshot_channel.rs) |
| Constraint systems | [R1CS rows](src/r1cs_constraint.rs), [gates](src/r1cs_gates.rs), [bounded systems](src/r1cs_system.rs) |

These are reusable models, helpers, and admission checks. Integration with a
network, storage engine, allocator, scheduler, or compiler needs its own
validation. The [full module catalog](#modules) is available below.

### Runnable examples

Run these commands inside the development environment. Each example executes
through ordinary Cargo; it is an introduction to the modeled API, not evidence
of a complete downstream system.

| Example | Command |
|---|---|
| [Core arithmetic, bounds, and quorums](examples/core_basics.rs) | `cargo run --example core_basics` |
| [Bounded modular reduction](examples/bounded_modular_tree_fold.rs) | `cargo run --example bounded_modular_tree_fold` |
| [Consensus admission](examples/consensus_admission.rs) | `cargo run --example consensus_admission` |
| [UCAN and trust-policy checks](examples/ucan_web_of_trust.rs) | `cargo run --example ucan_web_of_trust` |
| [Single-row R1CS](examples/r1cs_single_row.rs) | `cargo run --example r1cs_single_row` |
| [Storage and allocator checks](examples/storage_allocator.rs) | `cargo run --example storage_allocator` |
| [Integration adapters](examples/integration_adapters.rs) | `cargo run --example integration_adapters` |

For a small proof you can run and an intentionally failing counterpart, see
[runnable proof examples](docs/proof-examples.md).

## How verification works

A typical module contains a mathematical specification, an executable function,
and proofs of useful properties. For example:

```rust
use vstd::prelude::*;

verus! {
    pub open spec fn abs_diff_spec(a: u64, b: u64) -> u64 {
        if a >= b { (a - b) as u64 } else { (b - a) as u64 }
    }

    pub fn abs_diff(a: u64, b: u64) -> (result: u64)
        ensures result == abs_diff_spec(a, b)
    {
        if a >= b { a - b } else { b - a }
    }

    proof fn abs_diff_symmetric(a: u64, b: u64)
        ensures abs_diff_spec(a, b) == abs_diff_spec(b, a)
    {}
}
```

Verus checks the implementation against the stated postcondition, subject to its
preconditions and trusted assumptions. [Tracey](https://tracey.bearcove.eu/)
connects the [written requirements](docs/requirements.md) to implementation,
proof, and test sites. Runtime tests exercise the executable code.

A proved implementation can still implement an incomplete specification. A
composition also needs to establish every required condition at its boundary.
Review the contract, the [module's evidence facts](docs/module-index.md), and
[open issues](https://github.com/brittonr/terrane/issues) together.

Normal builds use `vstd` to erase specifications and proof code. Verification is
a separate operation. See [runtime-cost evidence](docs/runtime-cost.md) for
recorded normal-build observations, and the
[trusted-boundary inventory](docs/proof-gap-sites.tsv) for the exact sites that
rely on external implementations or assumptions.

## Current limitations

An audit of revision
[`30fc0ad`](https://github.com/brittonr/terrane/commit/30fc0ad4789b8fd4f0b18bec6f6cc2438c7cb451)
ran **5,223 existing library tests successfully** and reproduced correctness
failures outside their coverage. The following issues track the remaining work:

| Area | Open findings |
|---|---|
| Authentication and recovery | [Publicly constructible web-of-trust signatures](https://github.com/brittonr/terrane/issues/8); [gapped WAL recovery admission](https://github.com/brittonr/terrane/issues/9) |
| Lifetimes and read admission | [RCU reader lifetime](https://github.com/brittonr/terrane/issues/10); [lease start bounds](https://github.com/brittonr/terrane/issues/12); [allocation generation exhaustion](https://github.com/brittonr/terrane/issues/13) |
| Compiler and consensus models | [SSA entry parameters](https://github.com/brittonr/terrane/issues/11); [instruction-selection cost overflow](https://github.com/brittonr/terrane/issues/14); [Raft vote-ID bounds](https://github.com/brittonr/terrane/issues/15); [Wasm validation](https://github.com/brittonr/terrane/issues/6) |
| Verification and CI | [Tracey result parsing](https://github.com/brittonr/terrane/issues/16); [postcondition parsing](https://github.com/brittonr/terrane/issues/17); [nested-module coverage](https://github.com/brittonr/terrane/issues/18); [default-branch push CI](https://github.com/brittonr/terrane/issues/19) |

In particular, the current web-of-trust byte checks must not be used as
cryptographic authentication, and recovery admission must not be assumed to
reject every invalid log prefix. Issue threads contain reproductions and the
acceptance criteria for repairs.

The [recorded public-source validation](.cairn/archive/2026-10-14-publish-independent-public-source/design.md)
also reports an Octet lint-baseline rejection. Publishing the source or updating
this README does not clear that release gate.

## Development

The development environment and verification tools are pinned separately:

| Purpose | Pin |
|---|---|
| Ordinary Cargo builds | `nightly-2026-10-14` |
| Verus verifier | `0.2026.10.18.7d82994` |
| Verus Rust toolchain | `1.99.1` |
| Verus standard library | `vstd = "=0.0.0-2026-10-18-0147"` |

See [rust-toolchain.toml](rust-toolchain.toml), [Cargo.toml](Cargo.toml),
[flake.nix](flake.nix), and [nix/verus.nix](nix/verus.nix) for the authoritative
configuration. The pinned Verus package includes archives for x86-64 Linux,
Apple Silicon macOS, and Intel macOS.

```bash
cargo check --locked
cargo test --locked
cargo clippy --all-targets -- -D warnings
./scripts/verus.sh          # pinned Verus proof check
./scripts/verify.rs         # maintained verification pipeline
```

The full verification pipeline also requires `tracey` on your path; it is not
included in the default development shell. The commands above include gates
with known failures or enforcement gaps listed above. For module-scoped work,
start with the focused checks in [CONTRIBUTING.md](CONTRIBUTING.md).

<details>
<summary>Verification inventory, evidence, and specialized checks</summary>

The repository maintains the following inventory. Counts describe tracked
artifacts and source structure; they are not a guarantee of complete API or
system correctness, and this README update does not refresh every proof receipt.

**11139 Verus proofs · 5223 tests · 2270 requirements · 624 modules · 16 trusted/proof-gap boundaries**

[README metrics](scripts/check-readme-metrics.rs) checks the headline format and
source-derived module/boundary counts. Its `--full` mode checks command-backed
proof, test, and requirement totals. The
[proof-gap checker](scripts/check-proof-gap-inventory.rs) validates the exact
trusted sites and their rationale against [the inventory](docs/proof-gap-sites.tsv)
and [proof-gap policy](openspec/specs/verification-gaps/spec.md).

| Evidence or compatibility topic | Guide | Maintained check |
|---|---|---|
| Kani evidence boundaries | [Kani boundary](docs/kani-boundary.md) | [check-kani-boundary.rs](scripts/check-kani-boundary.rs) |
| Exported primitive evidence | [Artifact manifest](docs/proof-artifact-manifest.md) | [check-verified-primitive-manifest.rs](scripts/check-verified-primitive-manifest.rs) |
| UCAN bridge evidence | [Bridge coverage](docs/ucan-bridge-coverage.md) | [check-ucan-bridge-coverage.rs](scripts/check-ucan-bridge-coverage.rs) |
| Dependency and vendoring metadata | [Consumer profile](docs/verified-logic-consumer-profile.md) | [check-verified-logic-consumer-profile.rs](scripts/check-verified-logic-consumer-profile.rs) |
| Module and example discovery | [Maturity index](docs/module-index.md) | [Catalog](scripts/check-readme-module-catalog.rs), [index](scripts/check-module-index.rs), [examples](scripts/check-examples.rs) |

Further reading: [proof-evidence bundles](docs/proof-evidence-bundles.md),
[artifact provenance](docs/proof-artifact-provenance.md),
[Aeneas/Lean pilot](docs/aeneas-cross-verifier-pilot.md),
[Nickel toolchain](docs/nickel-toolchain.md),
[exporter integration](docs/nickel-export-cutover.md), and
[upstream proof-corpus catalog](upstream-verus-corpus/catalog.ncl).

Evidence remains tied to its recorded source, toolchain, assumptions, and
provider. Historical `trellis-*` names and receipt identities are retained;
renaming the project does not regenerate their proofs. External corpora and
adapted upstream results do not certify Terrane or downstream systems.

</details>

## Contributing

Bug reports, small executable counterexamples, stronger specifications, and
focused proof or test improvements are welcome. Start with
[CONTRIBUTING.md](CONTRIBUTING.md) for the module template, requirement annotations,
proof-gap policy, and validation workflow. Include the affected revision and a
minimal reproduction when [opening an issue](https://github.com/brittonr/terrane/issues/new).
For sensitive vulnerability reports, consult [SECURITY.md](SECURITY.md).

## Why Terrane?

**Terrane** (pronounced *tuh-RAYN*) takes its name from geology: a
[distinct block of Earth's crust with its own geological history](https://pubs.usgs.gov/of/1998/of98-136/doc/html/gnotes.htm).
Terranes can join through accretion to form larger structures. The name evokes
components with explicit boundaries and their own supporting evidence,
composed into larger systems.

## Modules

Use the [module guide](docs/module-guide.md) to find a starting point and the
[maturity index](docs/module-index.md) to inspect the evidence associated with a
module. Catalog descriptions summarize the modeled purpose; current contracts,
trusted boundaries, and open issues determine what can be relied on.

<details>
<summary>Browse the complete public module catalog</summary>

### Composite primitives

| Module | Modeled purpose |
|---|---|
| `durable_job_executor` | durable, idempotent, retry-aware job execution |
| `distributed_lock` | stale-holder-safe lease writes with fencing tokens |
| `linearizable_read_gate` | Raft read-index / lease-read admission |
| `safe_snapshot_replication` | complete, integrity-checked snapshot installation |
| `crash_safe_page_cache` | WAL-backed dirty-page recovery |
| `verified_lsm_ingest` | flush and compaction-safe LSM ingest |
| `serializable_txn_guard` | MVCC / dependency / 2PL commit admission |
| `web_of_trust_ucan_authorizer` | UCAN authorization plus rooted web-of-trust policy |
| `policy_decision_engine` | delegation, containment, scope, pattern, and trust allow/deny |
| `blocking_bounded_channel` | bounded queue capacity plus futex/backoff waiting |
| `safe_reclamation_domain` | RCU, epoch, hazard-pointer, and ref-count reclamation |
| `cache_coherent_memory_region` | coherence, prefetch, store-buffer, and write-policy invariants |
| `verified_compiler_pipeline` | raw-IR analysis and optimization preservation |
| `backend_frame_allocator` | register/spill stack allocation conflict freedom |
| `dependency_scheduler` | DAG-safe, fair, capacity-bounded job scheduling |
| `raft_membership_manager` | safe config changes, learner promotion, leadership transfer |
| `exactly_once_ledger_transfer` | replay-safe balanced transfer posting |
| `reclaim_plan` | reachability-based reclaim plans: no live or protected candidate, no dangling reference, no leak |
| `fenced_attempt_admission` | fenced durable-attempt phases, epoch safety, event idempotence, completion linkage, and exact rejection preservation |
| `sel4_invocation_step` | bounded seL4 invocation transitions preserving invariants |
| `nix_eval_closure_policy` | Nix/Nickel merge plus reachable closure admission |
| `chaoscontrol_exploration_core` | deterministic, oracle-checked exploration steps |
| `integration_adapters` | deterministic adapters over consensus, UCAN, Nix/Nickel, and WAL primitives |

The additional `fenced_attempt_admission::verified` decision kernel proves
specification/executable equality over seven phase and report kinds, caller-
classified event relations, fence/attempt identity, worker/output authority,
and result linkage. Its caller must establish event classification against
complete retained history. This kernel does not prove event digest computation,
history lookup, consumer projection, Mantle runtime equivalence, transport,
storage, verifier soundness, a Valence property receipt, or release eligibility.

The `reclaim_plan::verified` decision kernel proves specification/executable
equality for checking a reclaim plan over indexed nodes, references, roots, and
reclaimable flags: retained is closed under references and contains every root,
and candidates are exactly the unretained reclaimable nodes. Named proofs show
that a checked plan never makes a node reachable from a root, or a protected
node, a candidate, and never leaves a retained node referencing a candidate.
The bounded fixpoint `plan` is proved to return a checked plan that reclaims
every unreachable reclaimable node on every admissible input and to reject every
other input. A planned result is the least valid plan and therefore unique, more
roots, references, or protection never add a candidate, and reordering or
duplicating roots and references does not change it. `project_keys` and
`key_mask` validate a caller's mapping from byte-string keys to indices, so the
same safety holds over keys: no listed candidate key is reachable from a root
key, and every reachable key is listed as retained. Its caller must supply
complete roots and references and pass each entry's key, references, and flag
together. This kernel does not prove root or reference completeness, plan
identity, or deletion.

### Core arithmetic and data structures

| Module | Modeled behavior |
|---|---|
| `arithmetic` | abs diff, saturating add/sub, checked mul, gcd, div ceil |
| `bounded_modular_tree_fold` | bounded power-of-two `u24` admission, sequential and balanced refinement, pair-round equivalence, and exact abstract combine counts; see [the profile guide](docs/bounded-modular-tree-fold.md) |
| `bitwise` | power-of-two, bit get/set/clear/toggle |
| `bitfield` | multi-bit field extraction and insertion in u64 words |
| `bitmap` | fixed-size bit set over u64 words |
| `bounds` | range predicates, clamping |
| `collections` | sorted insert, binary search, merge sorted, partitioning, find, count |
| `conversion` | checked narrowing (u32/u16/u8), signed↔unsigned |
| `integer` | parity, alignment, log2, next-power-of-two |
| `interval` | half-open interval contains, overlap, split, adjacent, merge |
| `option` | verified Option combinators (map, and_then, or_else, flatten, zip, filter) |
| `ordering` | min, max, median, sign, compare-and-swap |
| `result` | verified Result combinators (map_ok, map_err, and_then, or_else) |
| `ring_buffer` | FIFO queue with power-of-two capacity |
| `sequence` | monotonic sequence generator with batch reservation |
| `slice` | chunk count, prefix/suffix equality, all-equal, mismatch |
| `u24` | explicit 24-bit normalization and overflow-safe wrapping addition with closure, identity, commutativity, and associativity proofs |

### Rust async primitives

| Module | Modeled behavior |
|---|---|
| `async_task_state` | finite pending/ready/completed/cancelled task lifecycle admission with terminal-transition rejection proofs and focused positive/negative tests |
| `task_waker` | finite empty/registered/notified/taken generation-tagged waker-slot admission plus no-lost-wake bridge predicates that avoid callback/fairness claims |
| `oneshot_channel` | finite open/sent/received/closed/cancelled single-slot channel admission plus receiver readiness/waker-notification composition |
| `drop_guard` | finite armed/disarmed/cleaned exactly-once cleanup guard admission with repeated-cleanup rejection evidence |

### Circuit primitives

| Module | Modeled behavior |
|---|---|
| `r1cs_constraint` | bounded single-row R1CS shape, modular dot product, and satisfaction admission without multi-row/proving-system claims |
| `r1cs_bit_decomposition` | bounded two-bit decomposition over boolean R1CS witness slots without arbitrary-width range-check/proving-system claims |
| `r1cs_lookup` | bounded lookup-table membership over active R1CS witness/table prefixes without PLONK lookup/proving-system claims |
| `r1cs_boolean` | bounded booleanity row encoding and satisfaction for witness-slot bit constraints without range-check/proving-system claims |
| `r1cs_gates` | bounded multiplication, addition, and constant gate adapters into R1CS rows without circuit-builder/proving-system claims |
| `r1cs_system` | bounded multi-row R1CS system shape and shared-witness satisfaction without proving-system claims |
| `r1cs_public_inputs` | bounded public-input binding and R1CS statement satisfaction without proving-system claims |

### Raft consensus

| Module | Modeled behavior |
|---|---|
| `election` | leader election state machine |
| `election_init` | candidate election initiation |
| `election_jitter` | election jitter via Knuth multiplicative hash |
| `election_timer` | election timer reset conditions |
| `election_safety_proof` | at most one leader per term (system-level) |
| `pre_vote` | pre-vote protocol |
| `pre_vote_tally` | pre-vote response counting |
| `pre_vote_lease` | PreVote + CheckQuorum interaction |
| `priority_election` | priority-based leader election with delay and yield |
| `vote_tally` | vote response counting |
| `ballot` | vote tracking |
| `epoch` | lexicographic (term, index) comparison |
| `quorum` | majority threshold, intersection theorem, fault tolerance |
| `heartbeat` | heartbeat send/receive primitives |
| `check_quorum` | leader liveness via active peer counting |
| `term_authority` | step-down on higher term |
| `log` | append-only log with Log Matching Property |
| `log_conflict` | log conflict detection and resolution |
| `log_compaction` | prefix truncation for snapshot-based compaction |
| `log_invariant_inductive` | inductive proof of LMP preservation across transitions |
| `append_entries_build` | AppendEntries RPC construction |
| `append_entries_check` | AppendEntries consistency check |
| `append_response` | AppendEntries response dispatch |
| `match_index` | match/next index tracking for replication |
| `commit_advance` | commit index advancement |
| `commit_notify` | commit notification dispatch |
| `fast_backtrack` | accelerated log backtracking |
| `replication_mode` | per-peer FSM (Probe/Replicate/Snapshot) |
| `replication_pipeline` | pipelined replication flow control |
| `follower_commit` | follower commit index update |
| `follower_read` | follower linearizable reads via leader confirmation |
| `follower_redirect` | follower redirect primitives |
| `request_forward` | follower→leader write forwarding |
| `read_index` | read index protocol for linearizable reads |
| `lease_read` | lease-based reads |
| `noop_commit` | no-op entry for leader commitment |
| `install_snapshot` | snapshot installation |
| `snapshot_compaction` | snapshot creation and compaction trigger |
| `snapshot_transfer` | chunked snapshot transfer |
| `snapshot_integrity` | dual-hash integrity scheme |
| `config_change` | configuration change state machine |
| `joint_consensus` | joint consensus for membership changes |
| `voter_set` | voter set promotion |
| `learner_change` | learner add/remove primitives |
| `learner_readiness` | learner catch-up readiness |
| `leadership_transfer` | leadership transfer protocol |
| `witness` | witness replica primitives (vote without full log) |
| `bounded_peers` | bounded peer collection for bootstrap |
| `node_classify` | node classification for cluster reporting |
| `persistent_state` | crash recovery state management |
| `client_routing` | client request routing |
| `client_session` | exactly-once session tracking |
| `entry_type` | log entry type routing |
| `last_applied` | state machine application tracking |
| `linearizability` | linearizability specification |
| `state_machine_safety` | state machine safety theorem |
| `leader_append_only` | leader append-only invariant |
| `leader_completeness` | leader completeness theorem |
| `leader_completeness_ghost` | leader completeness with ghost state |
| `ghost_history` | ghost vote/provenance records for inductive proofs |
| `raft_step` | step decomposition for modular invariant proofs |
| `raft_timing` | timeout configuration validators |
| `trace_relation` | Raft internal state → client-visible trace |

### Other consensus protocols

| Module | Modeled behavior |
|---|---|
| `paxos` | single-decree Paxos acceptor: ballot ordering, prepare/accept, promise safety |
| `quepaxa` | finite deterministic QuePaxa proposal ordering, ISR transitions, abstract propagation, phase guards, fast path, and hedging metadata |
| `quorum_cert` | quorum certificate aggregation, vote counting, threshold checks |
| `view_change` | view-based reconfiguration: consecutive views, leader rotation |

### Capability authorization

| Module | Modeled behavior |
|---|---|
| `access_mode` | access mode partial order (Read ⊂ Full) |
| `authorize` | authorization composition |
| `containment` | delegation containment |
| `delegation` | delegation chain validation |
| `glob` | glob pattern matching with wildcards |
| `pattern` | set-membership pattern matching |
| `scope` | scope coverage, compound scoping |
| `subsumption` | variant subsumption |
| `token_timestamp` | UCAN token timestamp validity |

### Language and computation admission

| Module | Modeled behavior |
|---|---|
| `ability_effect` | finite ability requirements, handler discharge, and sandbox attenuation |
| `algebraic_effect` | finite algebraic-effect signatures, rows, summaries, handlers, and single-use continuations |
| `content_addressed_code` | content-addressed definition identity, dependency closure, and recursive group admission |
| `remote_computation` | remote computation dependency, ability, and resource admission |
| `semantic_refactoring` | immutable name movement, compatible propagation, and unresolved-dependent admission |
| `total_config` | finite typed node graphs, required-field admission, canonical normal forms, digest-input selection, frozen import policy, and digest-aware refactors |
| `s_expression` | bounded dialect-neutral atom/list trees, canonical UTF-8 rendering, canonical-only parsing, deterministic failures, and roundtrip checks |
| `scheme_logic` | finite reader spans, explicit interning, module linkage, contract/blame metadata, bytecode safety, cycle-safe values, and collection/bytevector shapes |
| `typed_durable_reference` | typed durable references, unpersist gates, and migration admission |

### Memory management

| Module | Modeled behavior |
|---|---|
| `buddy` | block sizing, buddy addressing, alignment, coalescing, order-for-size |
| `page` | virtual address decomposition, frame numbers, page alignment |
| `region` | address region contains, disjoint, adjacent, merge, split |
| `slab` | slot index/offset/address, generation handles, slab capacity |
| `watermark` | bump allocation with remaining/fits/alloc/reset |
| `heap_index` | binary heap parent/left/right child index arithmetic |

### Memory allocator

| Module | Modeled behavior |
|---|---|
| `alloc_token` | allocation lifecycle FSM: free → allocated → freed |
| `size_class` | size class bucketing, waste bounds, delegates to buddy order |
| `free_list` | intrusive free list: LIFO push/pop, no-cycle, containment |
| `split_coalesce` | buddy split/coalesce sequences with block conservation |
| `alloc_bitmap` | generational bitmap allocation, double-free detection |
| `arena` | region-based bump allocation with collective free |
| `pool` | fixed-size slab pool: alloc/free with bounded capacity |
| `segregated_free` | per-size-class segregated free lists |
| `coalesce_policy` | immediate vs deferred coalescing policies |
| `fragmentation` | external/internal fragmentation metrics with formal bounds |

### Concurrency primitives

| Module | Modeled behavior |
|---|---|
| `lock_ordering` | lock acquisition order, deadlock detection, cycle freedom via strict total order |
| `epoch_reclaim` | epoch-based reclamation: three-epoch cycling, pin/unpin, safe-to-reclaim gap proof |
| `hazard_pointer` | hazard pointer protocol: protect/unprotect, no-use-after-free, reclaimable scan |
| `seqlock` | sequence lock: even/odd counter, valid read detection, writer versioning |
| `ref_count` | reference counting: strong/weak, dealloc trigger, upgrade conditions |
| `work_stealing` | work-stealing deque: push/pop/steal bounds, circular buffer index |
| `barrier` | thread barrier: arrive/release, generation counter, leader detection |
| `spin_backoff` | spin-wait backoff: two-phase (spin then yield), phase state machine |
| `rcu` | read-copy-update: grace periods, quiescent state, safe reclamation |
| `mpsc_bounds` | MPSC channel bounds: occupancy, send/receive, capacity, close/drain |
| `futex_state` | futex wait/wake FSM: idle/waiting/notified, notify idempotence |
| `parking_lot_core_model` | finite typed ParkKey/ThreadId queues with uniqueness, requeue, and metadata validity |
| `parking_lot_raw_mutex` | typed thread ownership and compact raw-mutex locked/parked-bit state transitions |
| `parking_lot_condvar_model` | condition-variable association, notify, timeout, and no-spurious requeue semantics |
| `parking_lot_rwlock_model` | packed reader/writer/upgradable RwLock state transitions and fairness gates |
| `parking_lot_contracts` | parking-lot fairness, timeout, reentrant mutex, Once, raw-lock, and deadlock contracts |
| `ticket_lock` | FIFO-fair spinlock: ticket/serving counters, fairness, bounded waiters |
| `cas_loop` | CAS retry loop modeling: ABA detection via pack/unpack, retry bounds |
| `bounded_mpmc` | bounded MPMC ring buffer: head/tail, lap-based empty/full, occupancy |
| `memory_order` | memory ordering lattice: Relaxed ⊏ Acquire/Release ⊏ AcqRel ⊏ SeqCst |
| `cache_hierarchy` | inclusive/exclusive/non-inclusive L1/L2 invariants, back-invalidation, capacity, and snoop scope |
| `directory_coherence` | single-line directory protocol with sharer bit-vectors, targeted invalidations, exclusive-owner grants, eviction notifications, and MESI-compatible embedding |
| `cache_prefetch` | architecturally invisible prefetch hints with MESI-compatible cache effects and dirty-victim writeback |
| `store_buffer` | fixed-capacity store queue with youngest-match forwarding, FIFO drain, and fence-empty semantics |
| `tlb_shootdown` | page-local TLB shootdown: initiator self-invalidation, monotone acknowledgments, completion freshness, disjoint-page isolation, and range coverage |
| `semaphore` | counting semaphore: acquire/release permits, capacity bounds |
| `two_phase_lock` | two-phase locking: growing/shrinking phase discipline |
| `lease_mutex` | lease-based distributed mutual exclusion with fencing tokens |

### Financial accounting

| Module | Modeled behavior |
|---|---|
| `balance` | debit/credit with overflow guards |
| `double_entry` | double-entry bookkeeping invariants |
| `escrow` | fund/release/refund/dispute state machine with conservation |
| `fee` | basis-point fee computation, monotonicity, conservation |
| `journal` | append-only transaction journal with running balances |
| `ledger` | multi-account ledger with total conservation theorem |
| `settlement` | bilateral netting with positions-sum-zero theorem |
| `transfer` | value transfer between accounts, duplicate detection |

### Storage engine primitives

| Module | Modeled behavior |
|---|---|
| `btree_balance` | B-tree node fill percentage, split/merge sizing, underfull detection |
| `btree_insert` | B-tree node insert: sorted position, split on overflow |
| `btree_delete` | B-tree node delete: key removal, merge/rebalance on underflow |
| `lsm_level` | LSM level target sizing, level count, space/write amplification |
| `wal_segment` | WAL entry sequencing, segment space, truncation, contiguity |
| `wal_recovery` | WAL crash recovery: replay range, idempotent replay, checkpoint |
| `compaction_fence` | log compaction coordination with replication, safe compact index |
| `page_checksum` | Fletcher-style two-sum checksum, torn-write detection |
| `free_space_map` | free-space bitmap tracking: allocate/free, count, find-first-free |
| `write_buffer` | write buffer (memtable): size tracking, flush threshold, swap |
| `write_policy` | write-back/write-through dirty invariants, eviction, flush, read equivalence |
| `block_cache` | block cache: occupancy, eviction, dirty tracking, write-back |
| `set_associative_lookup` | cache address decomposition, per-set tag lookup, duplicate-tag checks |

### Distributed systems

| Module | Modeled behavior |
|---|---|
| `causal_order` | vector-clock-based causal ordering |
| `circuit_breaker` | circuit breaker state machine |
| `consistent_hash` | consistent hash ring lookup |
| `consistent_snapshot` | consistent cut: vector-clock-based cut validity, monotonicity |
| `crdt` | G-Counter, PN-Counter, LWW-Register |
| `distributed_lock` | distributed lock protocol |
| `fencing` | fencing tokens for stale-write rejection |
| `gossip` | gossip infection bounds |
| `markov` | Markov chain prediction |
| `merkle` | Merkle proof verification |
| `rwlock` | read-write lock mode transitions, mutual exclusion |
| `per_core_rwlock` | per-core reader aggregation, writer drain, and rwlock mode refinement |
| `vector_clock` | vector clock merge, happens-before, concurrency |

### Nix package manager

| Module | Modeled behavior |
|---|---|
| `nix_attrset` | attribute set operations (get, has, merge, intersect, remove) |
| `nix_closure` | generic closure computation |
| `nix_list` | list operations (head, tail, elem, concat, gen_list) |
| `nix_string` | string operations (substring, length, replace) |
| `nix_version` | version comparison (same algorithm as Nix's compareVersions) |

### Nickel language semantics

| Module | Modeled behavior |
|---|---|
| `nickel_priority` | Nickel merge priority total order: Bottom < Neutral < Numeral(n) < Top |
| `nickel_merge_split` | Nickel three-way record split: left-only, center, right-only |
| `nickel_field_merge` | Nickel field-level merge resolution by priority |
| `nickel_record_merge` | Nickel full record merge with identity, disjoint commutativity, and neutral-record associativity proofs |
| `nickel_subtype` | Nickel gradual type subtyping: reflexivity, covariance, record-dict |
| `nickel_polarity` | Nickel contract blame polarity: flip on domain, double-flip identity |
| `nickel_pattern_coverage` | Nickel pattern completeness and overlap detection |

### Networking

| Module | Modeled behavior |
|---|---|
| `tcp_state` | TCP connection state machine (RFC 793): 11 states, 18 transitions, reachability |

### Database internals

| Module | Modeled behavior |
|---|---|
| `txn_isolation` | SQL isolation level hierarchy: Read Uncommitted through Serializable |
| `mvcc` | multi-version concurrency control: version visibility, snapshot reads |
| `snapshot_isolation` | write-write conflict detection for snapshot isolation |
| `query_cost` | query plan cost model: sequential scan, index scan, selectivity |
| `index_select` | covering index checks, index selection by estimated cost |
| `txn_dependency` | transaction dependency graph: read-write and write-write conflicts |

### Graph algorithms

| Module | Modeled behavior |
|---|---|
| `topo_sort` | topological sort via Kahn's algorithm: validity, completeness |
| `cycle_detect` | cycle detection: remaining node count proves acyclicity |
| `reachability` | BFS reachability: soundness, visited-set monotonicity |
| `dominator_tree` | iterative dominator sets, immediate dominators, and dominance frontiers |
| `loop_detection` | back-edge classification, loop bodies over reachability/dominator views, exit edges, and nesting |
| `liveness_analysis` | backward live-in/live-out analysis, fixed-point checks, and live-range projection |
| `ssa_construction` | iterated-dominance-frontier phi placement, SSA renaming, and phi elimination |
| `dead_code_elimination` | dead-instruction classification, block elimination, observable-trace preservation, idempotence |
| `graph_color` | graph 2-coloring (bipartiteness): no adjacent same-color |
| `shortest_path` | unweighted shortest path via BFS: distance bounds |

### Compiler frontend

| Module | Modeled behavior |
|---|---|
| `hir_attribute_parsing` | finite inert parser descriptors, parser classes, target/duplicate/safety/stability policy, deterministic diagnostics, and early/late metadata agreement |
| `hir_id_lowering` | finite owner-scoped source mappings, generated-ID freshness, created-ID use, uniqueness, and definition correspondence admission |
| `mir_core` | finite Rust-shaped MIR bodies, locals, places, shallow rvalues, block terminators, deterministic structural admission, and CFG facts |
| `mir_dataflow_framework` | finite join-table laws, forward/backward ordered effects, monotone edge transfers, bounded fixed points, and pure result cursors |
| `mir_destruct` | finite structural obligations, initializedness updates, static/dead/conditional/open decisions, guarded expansion, and replacement ordering |
| `mir_visitor_traversal` | explicit MIR visit locations, exactly-once coverage, deterministic preorder/reverse-postorder, reachability, and shape-preserving mutation admission |
| `mir_pass_pipeline` | finite MIR phase, pass-registration, validation, optimization-gate, and artifact read/borrow/steal trace admission |
| `rustc_metadata_identity` | finite rlib/dylib/rmeta capability, rustc identity-field, encoded-MIR, pipelining, and bounded dependency-selection admission |

### Compiler backend

| Module | Modeled behavior |
|---|---|
| `backend_codegen_trace` | finite typed builder handles, MIR block mappings, source-tied operations, defined-before-use values, control targets, terminators, and local storage classes |
| `rustc_monomorphization` | finite concrete function/static/shim collection, reachability deduplication, stable/volatile classification, and disjoint complete unit admission |
| `register_allocation` | live-range overlap graphs, conflict-free register assignment, and deterministic spill fallback |
| `stack_frame_layout` | aligned stack-slot placement, exact rounded frame sizing, and explicit safe-reuse checks over live ranges |
| `track_caller_abi` | finite attribute, direct-call, tracked-chain, trait-inheritance, reify-shim, and const/codegen location-metadata admission |
| `wasm_primitives` | exact Wasm value tags, `i32.add` stack discipline, typed local access, linear-memory bounds/grow semantics, control-label branch/return arity, unreachable-stack validation, control-frame stack-height restoration, finite indirect-call table/signature admission, and branch-table target validation |
| `wasm_runtime_bounds` | finite transition admission, exact fuel, deterministic segmentation, identity-bound continuation, initialization-only allocation, growth policy, and exact chunk coverage |
| `gpu_primitives` | finite GPU program shapes, launch/grid/block/thread bounds, pitched/tensor memory-layout, coalesced access, and shared-tile admission |
| `hdl_ir` / `hdl_emit` / `hdl_adapters` / `hdl_validation` | finite HDL IR admission, deterministic Verilog/SystemVerilog emission, adapter refinement, generated-vector collateral, and tool-evidence classification |

### Scheduling primitives

| Module | Modeled behavior |
|---|---|
| `deadline_queue` | earliest-deadline-first ordering, deadline comparison |
| `fair_share` | weighted fair allocation: monotonicity, conservation |
| `priority_inversion` | priority inheritance protocol: effective priority, inversion detection |
| `weighted_round_robin` | weighted round-robin: fairness, wrap-around, next consumer |
| `job_dag` | job dependency DAG: all-deps-satisfied, ready-to-run |

### Job runner

| Module | Modeled behavior |
|---|---|
| `job_state` | job lifecycle FSM: Pending → Ready → Running → Completed/Failed/Cancelled |
| `job_priority` | priority ordering: (class, sequence) total order, higher-first selection |
| `job_slot` | bounded concurrency slot pool: acquire/release, capacity |
| `job_cancel` | cancellation propagation: dependent cancellation, surviving paths |
| `job_idempotency` | idempotency tokens: consume-once, duplicate rejection |
| `durable_history` | append-only durable event history, contiguous prefix, append admission |
| `durable_replay` | replay gate for durable prefix reads and effect deduplication |
| `durable_resume` | checkpoint advancement and crash-recovery resume-point derivation |
| `durable_terminal` | terminal outcome detection and deterministic recovery status |
| `job_retry` | retry decisions: attempts, budget, retryable classification |
| `job_timeout` | deadline computation, timeout checking, not-timed-out-at-start |
| `job_result` | exit code classification: success/failure/error, completion status |

### Serialization

| Module | Modeled behavior |
|---|---|
| `serialize_inj` | canonical byte encoding injectivity: tag-length-value roundtrips, u64 encode/decode |

### Systems decision logic

| Module | Modeled behavior |
|---|---|
| `approval` | merge gating: approval counting, stale detection |
| `backoff` | exponential backoff with cap and jitter bounds |
| `batch` | write batch limit checking and flush decisions |
| `bloom` | Bloom filter insert/query monotonicity, no false negatives |
| `bounded_range` | ranges with optional upper bound |
| `cas` | compare-and-swap condition evaluation |
| `chunk` | ceiling-division chunk arithmetic |
| `constant_time_eq` | constant-time 32-byte equality |
| `cost_tracker` | cost accumulation, budget enforcement, model selection |
| `cow` | copy-on-write ref counting, snapshot isolation |
| `drain` | graceful shutdown admission protocol |
| `drain_protocol` | drain with double-check admission |
| `env_scrub` | environment variable scrubbing for sandbox isolation |
| `eviction` | LRU cache eviction with protected entries |
| `eviction_score` | multi-factor eviction scoring |
| `expiration` | TTL calculation, lease/key expiration |
| `hash_bucket` | hash-to-bucket assignment (modular and power-of-two) |
| `kv_version` | KV entry version computation |
| `layout_constraint` | layout constraint resolution |
| `lease` | lease management with disjointness and renewal |
| `monotonic` | advancement predicates for terms, indices |
| `node_lifecycle` | node lifecycle state machine |
| `partition` | non-overlapping gap-free interval partitions |
| `peer_spec` | peer specification parsing |
| `permission` | Unix file permission bit manipulation |
| `pressure` | multi-threshold severity classification |
| `process_state` | process lifecycle state machine |
| `rate_limit` | integer token bucket rate limiting |
| `retry` | retry decision logic with budget tracking |
| `sequential` | sequential access detection, prefetch triggering |
| `signal` | signal set operations |
| `tier` | tier classification for tiered caching |
| `tiered_rate_limit` | two-tier rate limit (global + per-peer) |
| `tiling` | tiling layout: disjoint rects, containment, scroll bounds |
| `trust_level` | trust level resolution and access control |
| `two_phase` | two-phase commit decision logic |
| `type_lattice` | type conversion lattice |
| `typed_state` | compile-time verification pipeline enforcement via phantom types |
| `quota` | resource quota enforcement: remaining capacity, overcommit detection |

### Generic proof infrastructure

| Module | Modeled behavior |
|---|---|
| `algebraic` | commutative, associative, idempotent, semilattice, partial order predicates |
| `pointwise` | scalar operation lifting to sequences |
| `seq_fold` | recursive counting and folding over sequences |
| `seq_set` | set-theoretic operations over integer sequences |
| `forest` | parent-pointer forest: ancestors, LCA, subtree walks |
| `session_tree` | session tree as forest + seq_set composition |
| `chain` | chain hash integrity with append-preserves proof |
| `choreography_global` | finite global choreography syntax, participants, and well-formedness |
| `choreography_ownership` | finite value-owner identities and atomic owner-transfer admission |
| `choreography_local` | finite local choreography syntax, branch lookup, and merge |
| `choreography_projection` | projectability checks and endpoint projection |
| `choreography_step` | one-step global/local reduction and shared-prefix alignment |
| `debruijn` | de Bruijn shifting, substitution, beta reduction, and scope preservation |
| `acid` | atomicity, consistency, isolation, durability over state vectors |
| `verification_frontier` | finite contract implication, type-bound facts, slice-access certificates, arena provenance, progress budgets, and external-reference admission |

### Cross-module properties

| Module | Modeled behavior |
|---|---|
| `properties` | 200+ cross-module proofs: signal↔bitwise, permission independence, escrow↔balance, ledger↔transfer, fee conservation, election↔quorum, fencing↔CAS, vector clock↔CRDT, mpmc↔mpsc occupancy, spin↔CAS retry, seqlock↔memory order, RCU↔epoch reclaim, ref count↔hazard pointer, lock ordering↔ticket lock, barrier↔quorum, futex↔notify roundtrip, job state↔timeout↔result, CAS ABA↔pack roundtrip, concurrency protocol↔data structure composition, choreography projection/choice/step correspondence |
| `concurrent_composition` | concurrency protocol ↔ data structure invariant chains across 7 groups |
| `consensus_path` | end-to-end Raft read/write path composition proofs |
| `consensus_fast_path` | crash-fault fast-quorum arithmetic, same-view certificates, ordering prerequisites, commutativity contract, recovery barrier, and dual-path convergence proofs |

### Additional public modules

Additional modules are grouped below; detailed descriptions are in the
[crate-level source documentation](src/lib.rs).

- **beam**: `beam_link`, `beam_mailbox`, `beam_monitor`, `beam_process`, `beam_registry`, `beam_scheduler`, `beam_send`
- **bounded**: `bounded_interval`
- **cache**: `cache_partition`
- **chaoscontrol**: `chaoscontrol_coverage`, `chaoscontrol_fault_schedule`, `chaoscontrol_fault_state`, `chaoscontrol_frontier`, `chaoscontrol_input_tree`, `chaoscontrol_oracle`, `chaoscontrol_scheduler`
- **composite**: `composite_outcome`, `composite_receipt_envelope`
- **config**: `config_admission`, `config_default`, `config_key`, `config_override_precedence`, `config_typed_value`, `config_validation_decision`
- **consistency**: `consistency_history`, `consistency_object`, `consistency_refinement`, `consistency_relations`, `consistency_session`, `consistency_transactional`
- **constant**: `constant_folding`
- **constraint**: `constraint_admission`, `constraint_assignment`, `constraint_consistency`, `constraint_domain`, `constraint_propagation`, `constraint_relation`, `constraint_variable`, `constraint_witness`
- **control**: `control_actuator_saturation`, `control_deadband`, `control_error_signal`, `control_flow_graph`, `control_hysteresis`, `control_rate_limit`, `control_safety_envelope`, `control_setpoint`
- **cron**: `cron_field`, `cron_match`, `cron_next`
- **ct**: `ct_select`
- **data**: `data_validation_admission`, `data_validation_cross_field`, `data_validation_enum_constraint`, `data_validation_error_accumulation`, `data_validation_field_presence`, `data_validation_normalization_decision`, `data_validation_range_constraint`, `data_validation_schema_predicate`
- **ds**: `ds_bounded_array`, `ds_fifo_queue`, `ds_key_value_map`, `ds_priority_queue`, `ds_ring_buffer`, `ds_union_find`
- **false**: `false_sharing`
- **feature**: `feature_flag_gate`, `feature_flag_rollout_bucket`
- **fetch**: `fetch_add`
- **fs**: `fs_path_admission`, `fs_path_containment`, `fs_path_extension_type`, `fs_path_form`, `fs_path_join_split`, `fs_path_normalization`, `fs_path_segment`, `fs_path_traversal`
- **geom**: `geom_admission`, `geom_bounds`, `geom_containment`, `geom_grid_cell`, `geom_intersection`, `geom_orientation`, `geom_point_vector`, `geom_transform`
- **gis**: `gis_bounding_box`, `gis_coordinate_envelope`, `gis_linestring_topology`, `gis_tile_index`
- **graph**: `graph_adjacency`, `graph_component_partition`, `graph_frontier_cut`, `graph_query_composition`, `graph_reachability`, `graph_shortest_path_envelope`, `graph_topological_order`
- **hypervisor**: `hypervisor_dirty_log`, `hypervisor_exit`, `hypervisor_gpa`, `hypervisor_interrupt`, `hypervisor_memslot`, `hypervisor_vcpu`
- **id**: `id_alias_map`, `id_canonicalization`, `id_collision_metadata`, `id_identifier_envelope`, `id_lookup_result`, `id_namespace_scope`, `id_registration_admission`, `id_scoped_uniqueness`
- **index**: `index_query_pagination`
- **instruction**: `instruction_selection`
- **knowledge**: `knowledge_graph`
- **left**: `left_right`
- **metric**: `metric_admission`, `metric_aggregate`, `metric_alert`, `metric_counter`, `metric_gauge`, `metric_histogram`, `metric_id`, `metric_labels`, `metric_window`
- **ml**: `ml_batch_iterator`, `ml_confusion_matrix`, `ml_dataset_split`, `ml_feature_normalization`, `ml_inference_envelope`, `ml_label_encoding`, `ml_tensor_shape`, `ml_top_k`
- **net**: `net_checksum_shape`, `net_endpoint_address`, `net_header_field`, `net_message_sequence`, `net_packet_frame`, `net_parser_admission`, `net_payload_bound`, `net_protocol_admission`
- **nix**: `nix_base32`, `nix_drv_serialize`, `nix_fingerprint`, `nix_hash_truncate`, `nix_store_path`
- **numeric**: `numeric_affine_transform_discrete`, `numeric_checked_arithmetic_envelope`, `numeric_denominator_safe_ratio`, `numeric_fixed_point_scale`, `numeric_histogram_counter`, `numeric_monotone_sequence`, `numeric_prefix_sum`, `numeric_quantized_threshold`
- **opt**: `opt_admission`, `opt_bound`, `opt_candidate_ranking`, `opt_dominance`, `opt_feasible_set`, `opt_frontier`, `opt_objective`, `opt_optimum_witness`
- **parsing**: `parsing_format`
- **physics**: `physics_collision_envelope`, `physics_contact_manifold`, `physics_energy_budget`, `physics_integrator_step`, `physics_momentum_budget`, `physics_time_horizon`, `physics_units`, `physics_vector_bounds`
- **policy**: `policy_action`, `policy_authorization`, `policy_conflict`, `policy_delegation`, `policy_grant_denial`, `policy_principal`, `policy_resource`, `policy_rule`
- **probability**: `probability_statistics`
- **proof**: `proof`, `proof_gap_boundary_audit`
- **raft**: `raft_consensus_core`
- **rect**: `rect_grid_index`
- **resource**: `resource_accounting`
- **scheduling**: `scheduling_calendar`
- **sel4**: `sel4_asid`, `sel4_cap_rights`, `sel4_cdt`, `sel4_cspace`, `sel4_endpoint`, `sel4_fault_ipc`, `sel4_invariants` (kernel invariant plus preservation matrix), `sel4_invocation`, `sel4_irq`, `sel4_notification`, `sel4_ready_queue`, `sel4_reply`, `sel4_tcb`, `sel4_untyped`, `sel4_vspace`
- **serialization**: `serialization_codec`
- **shamir**: `shamir_field`, `shamir_lagrange`, `shamir_poly`, `shamir_reconstruct`, `shamir_split`, `shamir_threshold`
- **signal**: `signal_processing`
- **spreadsheet**: `spreadsheet_address`, `spreadsheet_dependency`, `spreadsheet_formula`, `spreadsheet_recalc`
- **state**: `state_machine_workflow`
- **storage**: `storage_index`
- **surelock**: `surelock_deadlock`, `surelock_id`, `surelock_key`, `surelock_level`, `surelock_lockset`
- **table**: `table_relational`
- **text**: `text_string`
- **type**: `type_unification`
- **ucan**: `ucan_ability`, `ucan_backend_transcript`, `ucan_base64url`, `ucan_capability_merge`, `ucan_caveat`, `ucan_caveat_evaluation`, `ucan_caveat_preservation`, `ucan_chain`, `ucan_cid`, `ucan_claims_presence`, `ucan_compact`, `ucan_delegation_depth`, `ucan_did`, `ucan_header`, `ucan_invocation`, `ucan_json`, `ucan_parsed_capability_shape`, `ucan_parsed_caveat_payload`, `ucan_parsed_caveat_shape`, `ucan_parsed_did_claim`, `ucan_principal_link`, `ucan_proof_reference`, `ucan_proof_reference_collection`, `ucan_proof_reference_list_shape`, `ucan_proof_store_result`, `ucan_proof_traversal`, `ucan_replay_admission`, `ucan_revocation`, `ucan_revocation_check`, `ucan_signature`, `ucan_temporal`, `ucan_temporal_order`, `ucan_token`, `ucan_verification_stage`
- **ungrouped**: `calendar`, `cryptoshedding`, `mesi`, `moesi`, `nonce`, `timeseries`, `zeroize`
- **vision**: `vision_color_layout`, `vision_color_transform_envelope`, `vision_connected_components`, `vision_convolution_window`, `vision_detection_nms`, `vision_image_buffer`, `vision_image_pyramid`, `vision_roi`
- **web**: `web_of_trust_attestation`, `web_of_trust_chain`, `web_of_trust_policy`, `web_of_trust_principal`, `web_of_trust_resolution`, `web_of_trust_revocation`
- **zircon**: `zircon_channel`, `zircon_event`, `zircon_fifo`, `zircon_futex`, `zircon_handle`, `zircon_job`, `zircon_port`, `zircon_rights`, `zircon_socket`, `zircon_timer`, `zircon_wait`

</details>

## License

Terrane project source is licensed under the **Mozilla Public License 2.0**;
see [LICENSE](LICENSE). Contributions are accepted under the same license.

Third-party material retains its own licensing. In particular, the historical
Cairn policy and Nickel authoring copies have an
[exact-blob license grant](cairn-policy/LICENSING.md); Terrane selects MIT for
those copies and includes the [copyright notice and license](LICENSES/Cairn-MIT.txt).
See [upstream provenance](cairn-policy/UPSTREAM.md) and the
[license notices](LICENSES/) for the retained terms. The project license does
not relicense separate upstream tools or private history.
