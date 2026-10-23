# 54 advanced: a Merkle sum tree

**Concept:** A Merkle sum tree commits to every client balance and to the total. Sums are checked, never wrapped. The hash covers the sums. Each client recomputes their own leaf and folds the path up to the published root.
**Source:** RustCrypto `sha2` docs; std `u64::checked_add` docs; kata 42; own design (exchange proof of liabilities)
**Fails with:** `a total that overflows is rejected` (three other tests also fail)
**Vault:** your note on proof of reserves

## Task
Make `cargo test` pass without changing `tests/` or `Cargo.toml`. Keep the public API. None of the following in `src/`: `unwrap`, `expect`, `panic!`, `as`, `wrapping_*`, `saturating_*`, `#[allow(...)]`.

## Before you fix it
1. Run `cargo test`. Which tests pass, and why?
2. What is `100u64.wrapping_add(u64::MAX - 49)`?

## Answer in the PR
1. Which client's debt did the wrapped total hide, and who on the exchange's side could exploit it?
2. Why must the node hash cover the sums? Describe how an exchange shows different clients different sibling sums.
3. Why must the client rebuild the leaf from its own id and balance instead of trusting `proof.leaf`?
4. Why is `checked_add` needed in `verify` when `build` already checks? Whose code runs `verify`?
5. Why `Node::EMPTY` padding and not duplicating the last leaf?

## Think further
1. Leaf and node hashes share one input format. How would domain separation stop a second-preimage trick, where a node is passed off as a leaf?
2. A client id in the tree is a privacy leak. What would a salted or hashed id per client hide, and how could the tree size be hidden?
3. A proof of liabilities alone is not reserves. What else must an auditor check on chain?
4. How would a leaf hold many assets?
5. Why does a zero-balance account still need a leaf?

## In the wild
- An exchange publishes a root and lets each client download an inclusion proof, sometimes with a zero-knowledge proof that every leaf is non-negative.
- An auditor attests to the total liabilities at a snapshot time, and clients check their own leaf against the same root.
- A broker uses the same shape to prove that no client balance was left out of a published total.

In a review, check:
- Wrapping or unchecked sums.
- A hash that skips a field it vouches for.
- A verifier that trusts the prover's own data.
- `as` casts on amounts.
