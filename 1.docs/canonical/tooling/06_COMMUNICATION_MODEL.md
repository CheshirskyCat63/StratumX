# Communication Model

## Core laws
- facts publish only compact facts;
- requests publish only typed request envelopes;
- events publish only typed observation envelopes;
- projections publish only read-side structures;
- result refs publish only references to completed work.

## Direction rules
- `tool_diagnostics_events` is read-only from `link_egress`.
- intent layers are write-only toward SDK work surface or task host.
- projection layers are read-only from facts, events, and result refs.
- family layers compose and route but do not mutate adjacent lane truth.

## Fan-in and fan-out
- fan-in is allowed at projection layers and task requests;
- fan-out is allowed at diagnostics events and family composition;
- merge is allowed only into typed projections or typed task plans;
- no layer may emit opaque blob traffic.
