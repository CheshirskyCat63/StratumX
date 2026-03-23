# STRATUMX_ENABLEMENT_LEGALITY_MATRIX

## 1. Purpose

This document defines which legal units are always-on, legally optional, or illegal to disable.

## 2. Scope

This matrix covers:
- lower stack;
- shared world property substrate;
- runtime family;
- critical simulation families;
- service layers;
- startup.

## 3. Matrix

| Unit | Unit Class | Always-On | Legal to Disable | Canonical Note |
|---|---|---:|---:|---|
| lower stack | mandatory unit | yes | no | Foundation through world truth are mandatory in every legal engine assembly. |
| engine_material | mandatory unit | yes | no | Shared world property substrate is mandatory for canonical upper execution. |
| runtime family | mandatory unit | yes | no | One runtime profile must exist for one legal world instance. |
| engine_runtime_headless | profile unit | no | yes | Optional as a profile implementation when not selected. |
| engine_runtime_realtime | profile unit | no | yes | Optional as a profile implementation when not selected. |
| engine_kinetics | mandatory family | yes | no | Critical simulation family. |
| engine_field | mandatory family | yes | no | Critical simulation family. |
| engine_agents | mandatory family | yes | no | Critical simulation family. |
| engine_inference | optional service layer | no | yes | Legal to disable if no inference-driven workload is assembled. |
| engine_generation | optional service layer | no | yes | Legal to disable if no generation workload is assembled. |
| engine_imaging | optional service layer | no | yes | Legal to disable in headless or non-image-producing assemblies. |
| engine_acoustics | optional service layer | no | yes | Legal to disable in silent or non-acoustic assemblies. |
| engine_content | optional service layer | no | yes | Legal to disable only when startup consumes pre-prepared resource inputs. |
| engine_startup | mandatory unit | yes | no | Assembly-time mandatory owner. Not a runtime-resident unit. |

## 4. Canonical Rule

Only units marked legal to disable may be absent from a legal enablement set.
