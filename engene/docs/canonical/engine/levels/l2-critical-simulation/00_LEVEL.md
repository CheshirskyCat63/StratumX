# L2. Critical Simulation Families

## Level Role

Critical world-progression compute families.

## Crates

| Crate | Primary Role |
|---|---|
| `engine_kinetics` | Local dynamics and contact simulation family. |
| `engine_field` | Distributed physical field simulation family. |
| `engine_agents` | Agent and society simulation family. |

## Upward Role

This level provides its canonical outputs to the next higher level without transferring away its ownership class.

## Downward Dependence

This level stays constrained by lower-layer contracts and substrates and does not bypass them.

## Threading Posture

See crate-level `31_THREADING.md` documents inside this level.

## Crate Folders

- `kinetics/`
- `field/`
- `agents/`
