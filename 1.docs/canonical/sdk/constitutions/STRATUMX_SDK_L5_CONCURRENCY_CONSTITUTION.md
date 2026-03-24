# StratumX SDK L5 Concurrency Constitution

The default L5 rule is immutable publication.

- write-side boundary layers are ordered and single-writer
- read-side boundary layers may fan out immutable envelopes
- fact and policy layers are low-churn and single-writer
- verdict and gate layers may evaluate in parallel and publish singular immutable answers
- handle/ref layers publish immutably and never own UI-thread coupling
