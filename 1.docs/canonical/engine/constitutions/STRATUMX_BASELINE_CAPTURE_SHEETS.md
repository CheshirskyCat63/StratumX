# STRATUMX_BASELINE_CAPTURE_SHEETS

## 1. Purpose

This document freezes the canonical capture-sheet artifacts that back locked baselines.
A locked baseline is illegal unless it resolves to a registered capture sheet here.

## 2. Rule

Each capture sheet binds:
- stack marker;
- hardware floor id;
- backend id;
- driver family id;
- driver posture id;
- OS power-mode id;
- turbo or boost posture id;
- benchmark protocol id;
- compiler version and target triple;
- fixture family and capture profile.

The locked baseline table may reference only capture-sheet ids registered here.

## 3. Capture Sheets

| Capture Sheet ID | Stack Marker | Hardware Floor ID | Capture Profile | Backend ID | Driver Family ID | Driver Posture ID | Power Mode ID | Turbo/Boost ID | Protocol ID | Compiler Version | Target Triple | Fixture Family | Canonical notes |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `CS-CMN-CPU-01` | `SX-CANON/1.0.6/STACK-v12` | `HF-CMN-CPU-01` | `benchmark-common-cpu` | `CPU-ONLY` | `DRV-N/A` | `DRV-N/A` | `PWR-PERF-LOCK-01` | `BOOST-OFF-01` | `BENCH-PROTO-v2` | `rustc 1.94.0` | `x86_64-pc-windows-msvc` | lower-stack, runtime contention, common memory-trim, storage-mutation, replay, service-normalization, and content-transform CPU rows | CPU-only common floor |
| `CS-INT-60-CPU-01` | `SX-CANON/1.0.6/STACK-v12` | `HF-INT-60-01` | `interactive-60` | `CPU-ONLY` | `DRV-N/A` | `DRV-N/A` | `PWR-PERF-LOCK-01` | `BOOST-LOCKED-ALLCORE-01` | `BENCH-PROTO-v2` | `rustc 1.94.0` | `x86_64-pc-windows-msvc` | interactive CPU-only rows on the canonical interactive floor | canonical interactive CPU-only capture |
| `CS-INT-60-VK-01` | `SX-CANON/1.0.6/STACK-v12` | `HF-INT-60-01` | `interactive-60` | `VK-RT-01` | `DRV-VK-STABLE-01` | `DRV-POSTURE-STABLE-01` | `PWR-PERF-LOCK-01` | `BOOST-LOCKED-ALLCORE-01` | `BENCH-PROTO-v2` | `rustc 1.94.0` | `x86_64-pc-windows-msvc` | realtime and GPU-facing interactive rows | canonical interactive realtime capture |
| `CS-LH-60-CPU-01` | `SX-CANON/1.0.6/STACK-v12` | `HF-LH-60-01` | `listen-host-60` | `CPU-ONLY` | `DRV-N/A` | `DRV-N/A` | `PWR-PERF-LOCK-01` | `BOOST-LOCKED-ALLCORE-01` | `BENCH-PROTO-v2` | `rustc 1.94.0` | `x86_64-pc-windows-msvc` | listen-host CPU-only rows on the canonical host floor, including `L1.5`, network, acoustics, world-apply CPU rows, and `agent-cycle-a` | canonical listen-host CPU-only capture |
| `CS-LH-60-VK-01` | `SX-CANON/1.0.6/STACK-v12` | `HF-LH-60-01` | `listen-host-60` | `VK-RT-01` | `DRV-VK-STABLE-01` | `DRV-POSTURE-STABLE-01` | `PWR-PERF-LOCK-01` | `BOOST-LOCKED-ALLCORE-01` | `BENCH-PROTO-v2` | `rustc 1.94.0` | `x86_64-pc-windows-msvc` | realtime, imaging, mixed-pressure, and transfer rows with GPU-facing work | canonical listen-host realtime capture |
| `CS-HD-20-CPU-01` | `SX-CANON/1.0.6/STACK-v12` | `HF-HD-20-01` | `headless-20` | `CPU-ONLY` | `DRV-N/A` | `DRV-N/A` | `PWR-PERF-LOCK-01` | `BOOST-OFF-01` | `BENCH-PROTO-v2` | `rustc 1.94.0` | `x86_64-pc-windows-msvc` | headless mixed-pressure, headless runtime, headless network, and headless `L1.5` rows | canonical headless capture |

## 4. Canonical Baseline-Sheet Fields

Every baseline capture sheet must record at least:
- `Capture Sheet ID`
- `Stack Marker`
- `Hardware Floor ID`
- `Capture Profile`
- `Backend ID`
- `Driver Family ID`
- `Driver Posture ID`
- `OS Power Mode ID`
- `Turbo/Boost ID`
- `Protocol ID`
- `Compiler Version`
- `Target Triple`
- `Fixture ID`
- `Baseline ID`
- metric values

## 5. Legality

A performance proof is illegal when:
- a baseline row lacks a capture-sheet id;
- a capture-sheet id is missing here;
- the capture sheet binds a different stack marker, hardware floor, backend, driver family, or posture than the baseline row claims.
