# Shared Type Registry

## Id classes
- `IngressPacketEnvelopeId`
- `IngressControlEnvelopeId`
- `EgressObservationEnvelopeId`
- `EgressMetricEnvelopeId`
- `CompatVersionSetId`
- `CompatCapabilitySetId`
- `CompatProfileId`
- `CompatVerdictId`
- `TransportPolicyId`
- `LegalityGateEvalId`

## Handle and ref classes
- `EngineSessionHandle`
- `RuntimeInstanceHandle`
- `EngineObjectHandle`
- `EngineRuntimeHandle`
- `EngineIdentityRef`
- `EngineStateRef`
- `StackVersionRef`
- `TransportPolicyRef`

## Boundary class enums
- `PacketClass`
- `PacketPayloadType`
- `IngressControlKind`
- `ObservationClass`
- `ObservationPayloadType`
- `MetricClass`
- `MetricPayloadType`
- `BoundaryActionKind`
- `RuntimeSurfaceKind`

## Fact and policy enums
- `EngineCapabilityFlags`
- `BridgeCapabilityFlags`
- `UnsupportedFeatureFlags`
- `CompatProfileKind`
- `CompatibilityMode`
- `ConcurrencyProfile`
- `IngressOrderMode`
- `EgressFanoutMode`
- `EnvelopeSizeClass`
- `BackpressurePolicy`
- `CancellationPolicy`

## Handle/ref enums
- `OwnershipDomain`
- `HandleLivenessState`
- `EngineObjectKind`
- `EngineIdentityKind`
- `EngineStateKind`
- `ResolverScope`
- `StateFreshnessClass`

## Verdict and status enums
- `CompatVerdictStatus`
- `GateVerdict`
- `PublicationEpoch`
- `SubmissionOrderKey`
- `EmissionEpoch`
- `MeasurementWindow`

## Rule
No shared type may be invented inside a local layer document.

## Registry

| Type Name | Category | Domain | Status | Description |
|-----------|----------|--------|--------|-------------|
| PacketBase | Envelope | link_ingress | frozen | Base packet type for ingress boundary |
| ObservationBase | Envelope | link_egress | frozen | Base observation type for egress boundary |
| CompatibilityVerdict | Verdict | compat_verdicts | frozen | Compatibility decision result |
| LegalityGate | Gate | legality_gates | frozen | Boundary legality check result |
| HandleBase | Handle | engine_handles | frozen | Opaque runtime handle base |
| RefBase | Reference | engine_refs | frozen | Opaque reference base |

## Rule
All shared types must be registered in this table before use across L5 boundaries.
