# STRATUMX_DEGRADATION_POLICY_LAW

## 1. Purpose

This document defines canonical degradation policy for StratumX.

## 2. Degradation Principles

- critical simulation progression may not be degraded into non-progression;
- lower stack and world truth may not be bypassed as a degradation strategy;
- service layers may be deferred, thinned, or disabled by policy when legal;
- degradation decisions must be explicit, observable, and reversible by legal startup/configuration.

## 3. Degradation Classes

### 3.1. Non-degradable core
May not be degraded away:

- lower stack;
- `engine_material`;
- runtime family;
- `engine_kinetics`;
- `engine_field`;
- `engine_agents`;
- `engine_startup`.

### 3.2. Legally degradable service layers
May be degraded under policy:

- `engine_inference`
- `engine_generation`
- `engine_imaging`
- `engine_acoustics`
- `engine_content`

## 4. Canonical Degradation Decisions

Legal degradation decisions include:

- skip optional service-layer work for a legal cadence window;
- reduce service-layer update frequency;
- reduce synthesis budget;
- reduce generation budget;
- defer optional resource work.

Illegal degradation decisions include:

- skipping apply for world progression;
- dropping critical simulation families;
- replacing world truth with generated outputs;
- creating a second runtime authority.

## 5. Observability

All degradation decisions must be emitted through runtime diagnostics and observability channels.
