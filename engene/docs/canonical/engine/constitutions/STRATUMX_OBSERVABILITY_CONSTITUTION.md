# STRATUMX_OBSERVABILITY_CONSTITUTION

## 1. Purpose

This document defines the canonical observability law of StratumX.

## 2. Observability Principles

### 2.1. Observability is first-class
Execution and simulation must be visible enough to diagnose correctness and performance.

### 2.2. Runtime is the primary execution observability owner
Runtime owns execution-facing diagnostics and timing visibility.

### 2.3. Families expose domain-relevant observability through legal boundaries
Critical simulation families and service layers expose structured diagnostics without redefining observability law.

## 3. Canonical Observability Classes

### 3.1. Tracing
Phase-level, family-level, and profile-level execution traces.

### 3.2. Timing
Tick timing, phase timing, family timing, apply timing, and service-layer timing where enabled.

### 3.3. Diagnostics
Execution status, lag, degradation state, budget pressure, and synchronization visibility.

### 3.4. Metrics
Counters and gauges for execution, simulation, synthesis, and resource processing.

## 4. Ownership

- `engine_runtime` owns execution observability;
- runtime profiles extend profile-facing visibility;
- critical simulation families emit family-local diagnostics;
- service layers emit service-local diagnostics;
- `engine_startup` owns startup diagnostics.

## 5. Degradation Visibility

Any degradation decision that affects legal service-layer execution must be surfaced as an observable runtime decision.

See:

- `STRATUMX_DEGRADATION_POLICY_LAW.md`

## 6. Canonical Laws

- observability never changes authority;
- diagnostics may expose state, but do not become state authority;
- degraded service-layer execution must remain visible;
- execution timing must remain attributable to phase and family classes.
