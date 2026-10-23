# 48 advanced: a multi-hop route

**Concept:** A route is priced one hop at a time in whole atoms. Each hop rounds its fill down, charges its fee rounded up on that fill, and checks its minimum against what actually reaches it. The cheaper route is the one that gives the client the most, with ties going to direct.
**Source:** std `u128::div_ceil`, https://doc.rust-lang.org/std/primitive.u128.html#method.div_ceil, and `TryFrom` for integers, https://doc.rust-lang.org/std/convert/trait.TryFrom.html; own design (an exchange smart order router); kata 46.
**Fails with:** `fees decide the route`
**Vault:** your note on routing through an intermediate

## Task
Make `cargo test` pass without changing the tests. No `f32` or `f64` and no `as` casts in `src/`. No `unwrap`, `expect`, `panic!`, `saturating_*` or `wrapping_*`. std only. Don't change the public types or `best_route`'s signature.

## Before you fix it
1. Which two tests already pass, and why?
2. Price `fees_decide_the_route` by hand, both routes.

## Answer in the PR
1. Walk the starting code through `fees_decide_the_route`. Which route did it pick, and how many cents did the client lose?
2. Why does each hop round to whole atoms instead of carrying the exact fraction to the end? Who holds the intermediate USDT?
3. Why is the second hop's minimum checked against the first hop's net, not its gross?
4. Why is the fee charged on the whole-atom gross and rounded up? What would one-step `floor(amount × rate × (1 − fee))` give away, and which test tells the two apart?
5. Why is a too-small order skipped (and finally `BelowMinimum`), while overflow and a bad fee stop the search?

## Think further
1. Routes with two or more intermediates: graph search. What replaces "the max of a few"?
2. Depth: a rate that worsens with size, and splitting one order across routes.
3. Lot or step size for each hop, and the leftover dust.
4. A minimum notional on the output.
5. Prices moving between hops (kata 47).
6. A fee charged in a third asset.

## In the wild
- Broker routers compare a direct pair with a synthetic cross through a stablecoin or USD.
- Exchange symbol filters (min qty, step size, min notional) apply per market, so each hop has its own.
- FX desks price crosses via USD.
- DEX aggregators split one order across pools.

In a review, check:
- Floats on a route.
- One fee for the whole route.
- A minimum checked on the input only.
- Rounding once at the end.
- `as u64` on a money path.
