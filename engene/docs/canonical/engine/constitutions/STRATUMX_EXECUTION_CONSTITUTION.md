# STRATUMX_EXECUTION_CONSTITUTION

## 1. Purpose

This document defines the canonical execution law of StratumX.

It fixes how the engine progresses in time, how runtime profiles execute work, how world access is opened and closed, how staged changes are applied, and how one world instance remains under one active execution authority.

## 2. Scope

This document covers:

- fixed-step execution;
- tick and frame distinction;
- execution units;
- phase order;
- read / compute / stage / apply / egress cycle;
- world access windows;
- runtime profile differences;
- authority model per world instance;
- execution outputs.

## 3. Execution Principles

### 3.1. One execution constitution
All runtime profiles follow one execution law.

### 3.2. One active runtime authority per world instance
A world instance has one active runtime authority.

### 3.3. Runtime owns execution
`engine_runtime` owns execution law, scheduling law, phase law, apply law, and global parallel ownership.

### 3.4. World owns truth
`engine_world` owns authoritative world truth.

### 3.5. Read before compute, compute before apply
No legal execution path skips this order.

## 4. Time Model

### 4.1. Fixed simulation step
Simulation progression is fixed-step by law.

### 4.2. Tick
A tick is the canonical simulation progression unit.

### 4.3. Frame
A frame is a profile-facing presentation cadence. A frame is not the canonical authority over world progression.

### 4.4. Profile relation
Headless and realtime profiles may expose different frame behavior, but they do not change tick law.

## 5. Execution Units

- tick
- optional substep
- phase
- job
- batch
- staged output
- apply unit

## 6. Canonical Phase Order

```text
ingress
read
compute
stage
apply
egress
diagnostics
```

## 7. Phase Definitions

### 7.1. Ingress
Commands, actions, and legal external requests enter runtime.

### 7.2. Read
Runtime opens legal read windows over authoritative state and legal snapshots.

### 7.3. Compute
Critical simulation families and legal service layers compute over legal inputs.

### 7.4. Stage
Families and service layers emit staged outputs. No family becomes world authority here.

### 7.5. Apply
Runtime orders and applies staged changes into `engine_world`.

### 7.6. Egress
Events, outputs, and external-facing execution products leave the runtime step.

### 7.7. Diagnostics
Runtime records execution diagnostics and timing products.

## 8. World Access Windows

### 8.1. Read window
A legal read-only or effectively immutable access window over authoritative or snapshot state.

### 8.2. Stage window
A legal window where families emit staged outputs without direct authoritative mutation.

### 8.3. Apply window
The only canonical window where authoritative world truth changes.

## 9. Runtime Profiles

### 9.1. `engine_runtime`
Defines execution constitution and shared runtime law.

### 9.2. `engine_runtime_headless`
Uses the same constitution in simulation-first mode.

### 9.3. `engine_runtime_realtime`
Uses the same constitution in realtime interactive mode.

## 10. Service Layers

The canonical service layers are:

- `L3.0. Model Systems`
- `L3.1. Synthesis Systems`
- `L3.2. Resource Systems`

They are runtime-governed consumers and producers inside legal execution windows. They do not redefine execution law.

## 11. Runtime Outputs

A legal runtime step emits:

- progressed authoritative world state;
- staged outputs integrated by apply;
- events;
- diagnostics;
- execution status.

## 12. Failure and Degradation Boundaries

- critical simulation progression may not be skipped by auxiliary pressure;
- service layers may be deferred or degraded under canonical degradation law;
- runtime authority may not be duplicated as a fallback strategy.

## 13. Canonical Laws

- one world instance;
- one active runtime authority;
- one execution constitution;
- read before compute;
- compute before apply;
- apply changes world truth;
- service layers follow runtime law.

## 14. Glossary

- world instance
- runtime authority
- read window
- stage window
- apply window
- tick
- frame
- runtime profile
- service layers
