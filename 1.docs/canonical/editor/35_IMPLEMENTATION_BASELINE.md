# Implementation Baseline

## Baseline expectations
- Rust-native editor product package
- shell and panels above lower-stack authority
- viewport-centric interaction
- suite and service registration
- activation-bounded execution
- request/view separation

## Physical implementation expectations
- shell and surface state split from heavy service jobs
- view models fed by immutable snapshots, bounded streams, indices, and projections from lower stack
- tool contexts and suite state kept narrow and local
- background jobs isolated from focused interaction routing
