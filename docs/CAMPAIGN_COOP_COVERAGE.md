# Campaign and Co-op Core Coverage

## Scope and evidence

This is a source-derived inventory of `sts2-game-core` at the reviewed baseline plus deterministic
fixture coverage. It records a pure semantic package, not a host or protocol capability. “Confirmed”
below means the named target-local test exercises the listed pure behavior; native game, multiplayer,
transport, and runtime behavior remain **unverified**.

“Unsupported in core” is an owner-seam status only. It does not narrow the intended whole-project
campaign or native co-op support: those capabilities remain required at their owning host,
control-plane, and integration boundaries.

| Intended concern | Core source seam | Fixture/test oracle | Status and boundary |
| --- | --- | --- | --- |
| Actor, session, and generation freshness | `identity.rs`, `combat.rs`, `end_turn.rs`, `play_card.rs` | `end_turn.rs`, `play_card.rs`, `runtime_v2_projection.rs` | Confirmed component invariant for one actor/session snapshot; no peer roster or gateway fence. |
| Combat card selection and energy | `play_card.rs`, `calculators.rs` | `play_card.rs`, `combat_calculators.rs` | Confirmed component arithmetic only; host legality and settlement unverified. |
| Defeated combat targets | `calculators.rs` | `calculation_bounds.rs::defeated_enemies_remain_observable_but_cannot_receive_actions` | Confirmed component safety correction: defeated enemies are observable but invalid targets. |
| End-turn action | `end_turn.rs`, `combat.rs` | `end_turn.rs` | Confirmed frozen pure transition; no host turn execution or receipt/reconciliation. |
| Player defeat calculation | `calculators.rs` | `calculation_bounds.rs::survival_matches_integer_balance_for_all_small_inputs` | Confirmed arithmetic result only; no campaign Defeat terminal representation. |
| Encounter victory / campaign terminal | None | None | Unsupported in core: no Victory/Defeat/terminal-state type or terminal action policy. Requires named game-mod producer and consumer contract. |
| Characters, seeds, map branches, rewards, shops, events, rest sites | None | None | Unsupported in core; host-observation/campaign ownership must be agreed before a domain contract is added. |
| Enemy hit-point scaling for a declared participant count | `rules_reference_coverage.rs` (declared pure record) | `rules_reference_coverage.rs` | Source-derived declaration only: integer scaling with half-up rounding is confirmed for the declared inputs. Peer roster, shared decisions, and per-player rewards remain unsupported in core. |
| Multiplayer peer identity/admission | None | None | Unsupported in core; `Identity` is an action actor, not a peer-admission model. Gateway and game-mod own admission/fencing. |
| Shared decisions, votes, effects, disagreement | None | None | Unsupported in core; no shared-decision state or deterministic resolution policy exists. |
| Disconnect, rejoin, restart, recovery | None | None | Unsupported in core; lifecycle, receipts, and reconciliation are explicitly boundary-owned. |

## Contract gate

Adding an unsupported row requires root coordination before implementation: a named host producer,
named consumer(s), ownership split, validation/error precedence, compatibility classification,
serialization/version decision where crossing a repository boundary, and deterministic fixtures.
This target must not infer native co-op or campaign completion from unit tests.
