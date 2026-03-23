# STRATUMX_LAYER_TEST_MATRIX

## 1. Purpose

This document defines mandatory test classes and invariant obligations for each canonical layer class.

## 2. Matrix

| Layer Class | Mandatory Core Test Classes | Mandatory Operational Subclasses | Hard Invariants |
|---|---|---|---|
| L-1..L0 lower stack | structural, contract, invariants, performance smoke | determinism where crate role requires, property where crate role requires, benchmark gate where crate role is benchmarked | identity stability, handle legality, access legality, world apply boundary, snapshot consistency |
| L0.5 material substrate | structural, contract, invariants, concurrency legality, performance smoke | benchmark gate where hot-path relevant | immutable lookup legality, descriptor consistency, cross-family property consistency |
| L1 runtime family | structural, contract, invariants, determinism, concurrency legality, performance smoke, fault and degradation | degradation, benchmark gate | one active runtime authority, phase legality, apply ordering legality, profile law parity |
| L2 critical simulation families | structural, contract, invariants, determinism, concurrency legality, property, performance smoke | benchmark gate | staged-output legality, no direct authoritative mutation, deterministic progression under fixed inputs |
| L3 service layers | structural, contract, invariants, fault and degradation, performance smoke | enablement legality, degradation, compatibility rejection, benchmark gate | optional-layer legality, isolation from critical simulation law, explicit degradation visibility |
| L4 startup | structural, contract, invariants, performance smoke | restoration legality, compatibility rejection, enablement legality | legal assembly decision, legal enablement set, legal restoration decision |

## 3. Canonical Rule

No canonical layer may ship without the mandatory core classes and required operational subclasses listed here.
