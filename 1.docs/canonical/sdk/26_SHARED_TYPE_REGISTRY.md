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
