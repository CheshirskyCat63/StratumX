# STRATUMX_PERFORMANCE_CONSTITUTION

## 1. Purpose

This document defines the canonical performance law of StratumX.

## 2. Performance Principles

- hot-path work is sacred;
- execution law must remain branch-light and allocation-light;
- lower stack must stay cache-conscious and descriptor-driven;
- critical simulation families must scale under runtime-owned parallelism;
- service layers may be expensive, but may not poison critical progression law;
- performance regressions are blocker-grade events, not advisory notes.

## 3. Canonical Metric Classes

### 3.1. Correctness-linked metrics
Determinism stability, apply legality success, snapshot consistency integrity.

### 3.2. Latency metrics
Tick time, phase time, family step time, service request latency.

### 3.3. Throughput metrics
Entities per second, regions per second, outputs per second, requests per second.

### 3.4. Allocation metrics
Allocation count per hot operation, burst allocations, retained memory after cycle.

### 3.5. Memory metrics
Snapshot size, delta size, world growth, family scratch size.

### 3.6. Contention metrics
Lock frequency, lock duration, blocked-worker ratio, queue pressure.

### 3.7. Scaling metrics
1→N thread efficiency, region scaling, batch scaling, synthesis scaling.

### 3.8. Stability metrics
Output variance, phase jitter, runtime overhead drift, degradation trigger rate.

## 4. Benchmark Classes

- lower-stack benchmarks;
- runtime benchmarks;
- L2 family benchmarks;
- L3 service-layer benchmarks;
- whole-engine profile benchmarks.

## 5. Hard Performance Laws

- no avoidable heap churn in steady-state hot loops;
- no broad authoritative-state locks across compute phases;
- no service-layer work may silently dominate mandatory progression law;
- no family may redefine global scheduling for local convenience;
- no performance waiver is legal without an explicit canon update.

## 6. Canonical Target Direction

- target zero avoidable heap allocations in steady-state hot loops;
- target near-zero lock contention in legal runtime-owned parallel paths;
- target deterministic fixed-step progression overhead so small that whole-engine cost is dominated by real family work, not orchestration;
- target aggressive scaling under legal parallel workloads;
- target brutal regression sensitivity so performance drift is caught early.
