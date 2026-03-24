# L6.F13 Level

Canonical layer: `release_family`

Exists to own exactly one tooling role: release family composition.
Core data classes: build lane, package lane, deploy lane, test lane, artifact lane.
It explicitly does not own: build execution law, package authority, engine boundary law.
It exists to keep its adjacent layers from collapsing into one mixed layer.
