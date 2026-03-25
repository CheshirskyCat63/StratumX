# Budget Runtime Model

Budget runtime owns:
- CPU envelopes
- GPU envelopes
- RAM envelopes
- IO envelopes
- disk growth envelopes
- queue pressure envelopes
- pressure classes
- degradation ladders
- profile legality
- explicit deny/defer/degrade decisions

Budget runtime is not reporting-only.
It is the enforcement and visibility layer that prevents silent resource death spirals across workspace, preview, build, release, assistant runtime, and orchestration requests.
