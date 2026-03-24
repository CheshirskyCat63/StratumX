# Glossary

## Canonical terms
- write-side boundary mesh: the pair `link_ingress_packets` + `link_ingress_controls`
- read-side boundary mesh: the pair `link_egress_observations` + `link_egress_metrics`
- compatibility fact: stable bridge truth about versions, capabilities, or profiles
- compatibility verdict: closed answer about whether one compatibility tuple is acceptable
- legality gate: closed answer about whether one concrete boundary action may pass now
- transport policy: immutable descriptor that constrains boundary behavior
- handle: opaque live runtime-facing address exported by public L4
- identity ref: opaque stable identity pointer exported by public L4
- state ref: opaque exported runtime-state pointer that never owns the payload body
- opaque upward contract: rule that L6 may read a handle or ref but may not reinterpret its internal structure

## Forbidden term drift
- do not call L5 an editor bridge
- do not call compatibility verdicts results
- do not call legality gates policies
- do not call state refs payloads
