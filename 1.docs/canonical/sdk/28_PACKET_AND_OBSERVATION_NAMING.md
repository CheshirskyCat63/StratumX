# Packet and Observation Naming

## Canonical rule
- `PacketClass` is a class enum only.
- `PacketPayloadType` is a payload-type enum only.
- `ObservationClass` is a class enum only.
- `ObservationPayloadType` is a payload-type enum only.
- `MetricClass` is a class enum only.
- `MetricPayloadType` is a payload-type enum only.

## Forbidden drift
- do not list payload-type names as class enums
- do not list class enums as payload types
- do not collapse observation naming with metric naming
