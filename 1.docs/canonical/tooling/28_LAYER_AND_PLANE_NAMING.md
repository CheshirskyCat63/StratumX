# Layer And Plane Naming

Naming freeze:
- levels own one role only
- families compose only
- planes are named by data responsibility, not by feature fantasy
- `L6` is `Editor Core`
- `L6A` is `Assistant Runtime`
- `L7` is `Studio Orchestrator`
- `L7A` is `Assistant Planner`
- assistant split is runtime in `L6A` and planning in `L7A`
- artifact means deterministic generated product, not cache entry
- budget means runtime pressure and degradation control, not reporting only
- campaign means compiled orchestration bundle, not frame-level runtime
- plan means reasoning output, not mutation authority
