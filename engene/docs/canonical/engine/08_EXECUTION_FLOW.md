# Execution Flow

## Status of This Document

This document is the root execution summary map.

The authoritative normative laws are:

- `constitutions/STRATUMX_EXECUTION_CONSTITUTION.md`
- `constitutions/STRATUMX_WORLD_PROGRESSION_FLOW.md`

## Canonical Startup Flow

`engine_startup`:

1. loads configuration;
2. selects runtime profile;
3. wires lower substrate access;
4. wires critical simulation families;
5. wires service layers where required by profile and cadence;
6. launches runtime.

## Canonical Runtime Flow

A runtime profile:

1. opens a legal read window over world truth;
2. schedules critical simulation families;
3. collects staged outputs;
4. schedules service-layer work where required by profile and cadence;
5. orders apply;
6. integrates staged changes into `engine_world`;
7. emits events, outputs, and diagnostics.

## Canonical Family Flow

- `engine_kinetics` emits local motion/contact/impact products;
- `engine_field` emits distributed field and region deltas;
- `engine_agents` emits navigation/perception/decision/society/schedule products;
- `engine_inference` emits normalized model outputs;
- `engine_generation` emits generated products;
- `engine_imaging` emits image synthesis outputs;
- `engine_acoustics` emits acoustic synthesis outputs;
- `engine_content` emits prepared resource products.
