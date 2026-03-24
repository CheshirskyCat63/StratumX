# Activation Model

## Core rule
A tool family may exist without being active.

States:
- registered
- dormant
- woken
- executing
- idle
- disposed

## Activation authority
- `tool_activation_rules` defines when activation is legal.
- `tool_activation_state` records whether activation has happened.
- no family self-activates without rule satisfaction.

## Activation triggers
Allowed triggers include:
- explicit user command;
- panel open;
- view focus;
- selection kind match;
- diagnostics subscription request;
- explicit assistant request;
- explicit release/build request.

## Forbidden activation
- global polling;
- hidden background hot loops;
- speculative activation without user or rule demand.
