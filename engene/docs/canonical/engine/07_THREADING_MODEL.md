# Threading Model

## Status of This Document

This document is the root threading summary map.

The authoritative normative law is:

- `constitutions/STRATUMX_THREADING_CONSTITUTION.md`

## Global Ownership

Global parallel execution ownership belongs only to:

- `engine_runtime`

No other crate owns a competing global execution constitution.

## Parallel-Ready Lower Layers

The lower substrate and world stack are designed to support:

- parallel-safe reads;
- deterministic access;
- partition-friendly traversal;
- staged mutation;
- controlled apply.

## Parallel-Ready Upper Families

Critical simulation families and service layers are all parallel-ready, but their execution is runtime-governed.

## Canonical Rule

- one world instance;
- one active runtime authority;
- many parallel-ready families and service layers;
- no competing global schedulers.
