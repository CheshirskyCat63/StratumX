# Role Class Separation Matrix

| Role | Allowed class | Forbidden confusion |
|---|---|---|
| ingress packet | mutation envelope | control, ref |
| ingress control | binding or execution signal | packet, verdict |
| observation egress | observation record | metric frame |
| metric egress | metric frame | observation record |
| compatibility fact | immutable fact | verdict |
| compatibility verdict | derived decision | fact |
| legality gate | derived legality decision | transport policy |
| opaque handle | live token | ref |
| opaque ref | read projection token | handle |
| opaque artifact ref | generated product token | state ref |
