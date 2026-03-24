# Scope

This package covers only L6.

L6 includes:
- tooling conveyor core;
- activation law;
- observation and projection lanes;
- intent and task lanes;
- panel and view attachment refs;
- tool families composed over lanes.

L6 excludes:
- engine truth, runtime law, world authority, simulation authority;
- SDK compatibility, startup legality, packet submission, boundary law;
- product-specific game logic or product-owned content law.

L6 may consume:
- typed observations from `L5.1 link_egress`;
- typed compatibility-safe refs and intents from L5;
- engine session handles surfaced through L5.

L6 may publish:
- typed workflow intents;
- typed task requests;
- typed projections and read models;
- typed panel/view attachment refs.

L6 is not an editor monolith. L6 is a composition of thin lanes and tool families.
