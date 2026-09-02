# Consumed `poc-v1` artifact

This is the exact offline release-like artifact copied from `sts2-protocol` PR #2 head
`cad3c85d3cba3363ad387f9c26a3c3cac2782267`. The core maps its fields locally and never imports
protocol implementation modules or a sibling path dependency. `SHA256SUMS` explicitly covers the
eight copied schema, manifest, golden, and fixture files consumed by the core verifier; protocol
source-schema and conformance evidence remain owner-local.
