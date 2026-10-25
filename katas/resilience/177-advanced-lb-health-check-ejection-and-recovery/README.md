# 177 resilience (advanced): a load balancer that ejects, probes and recovers

**Concept:** A round-robin pool ejects a backend after N failures in a row, ejects it again for twice as long each time a probe fails, brings it back through a trial (one request at a time, M successes) before full traffic, and never ejects more than a set share of the pool.
**Source:** Envoy docs, "Outlier detection" (architecture overview) and the cluster `OutlierDetection` proto; Michael Nygard, *Release It!*, ch. 5 "Circuit Breaker"; katas 90 and 97. (No Readwise highlight was available to the builder.)
**Fails with:** `a backend got full traffic back after one lucky probe`
**Vault:** your note on load balancing and outlier detection

The tests pass `now` into every call, so time moves only when a test says so. `Pool` in the tests plays the backends: `up[i]` decides each answer. A `pick` without a `report` plays a request that is still in flight.

## Task
Make `cargo test` pass by changing only `src/lib.rs`. Don't change the tests, `Cargo.toml` or any public signature. No `unwrap`, `expect` or `panic!` in `src/`, and no new crates. The rules:
- Eject after `consecutive_failures` failures in a row.
- The n-th ejection lasts `base_ejection * 2^(n-1)`, capped at `max_ejection`, and the count starts over once the backend is `Healthy` again.
- When an ejection ends, the backend is on `Trial`: at most one request in flight. `trial_successes` successes in a row make it `Healthy`; one failure ejects it again.
- A reply from a backend that is still ejected changes nothing.
- At most `len * max_ejected_percent / 100` backends are ejected at once.

## Before you fix it
1. Run `cargo test`. Which seven tests fail, and which rule does each one break?
2. In `never_ejects_more_than_the_max_percent`, where does traffic go with the old code once all three backends are ejected?

## Answer in the PR
1. Why is a single success not enough to give a backend full traffic back?
2. Why does the ejection grow on each failed probe, and why is there a cap?
3. All backends fail at once. Why is ejecting all of them worse than keeping some in the rotation?
4. How is this different from kata 97's circuit breaker? Which one protects the caller, and which one protects the pool?
5. What does a lower or higher `consecutive_failures` trade off?

## Think further
1. Envoy outlier detection: `consecutive_5xx`, `interval`, `base_ejection_time`, `max_ejection_time`, `max_ejection_percent`. Map each to a `Config` field. Envoy ejects for `base_ejection_time` times the number of ejections so far (linear, capped at `max_ejection_time`), where this kata doubles. Envoy also lowers that multiplier each `interval` a host stays healthy.
2. Envoy's panic threshold (`healthy_panic_threshold`, default 50%): when the share of healthy hosts falls below it, Envoy stops filtering by health and sends traffic to all hosts. Compare it with `max_ejected_percent`, which stops ejecting instead.
3. Active health checks (a probe on a timer) vs passive ones (watching real traffic, as here).
4. Kubernetes readiness probes, and how a Service drops a pod that isn't ready (`readinessProbe`, `failureThreshold`, `successThreshold`).
5. Many threads sharing one balancer: where the lock goes.
6. See the sd-katas card on load balancing.

## In the wild
- Envoy outlier detection
- NGINX `max_fails`/`fail_timeout` on `upstream` servers (defaults 1 and 10s)
- HAProxy `observe layer7` with `error-limit` and `on-error` (needs health checks on)
- gRPC xDS outlier detection (gRFC A50): an `outlier_detection` policy wrapping a child such as `round_robin`

In a review, check:
- an ejection with no end
- a fixed ejection length that never grows
- one success marking a backend healthy
- no cap on how many backends are ejected

## Related concepts
- **Least connections:** pick the backend with the fewest requests in flight. Use it when request costs vary a lot.
  ```rust
  let id = (0..in_flight.len()).min_by_key(|&i| in_flight[i]);
  ```
- **Power of two choices:** sample two backends at random and take the less loaded one. Use it at scale, where it gives near least-connections results without a global scan.
  ```rust
  let (a, b) = (rng(n), rng(n));
  let id = if load[a] <= load[b] { a } else { b };
  ```
- **L4 vs L7 load balancing:** L4 balances TCP connections and L7 balances requests (HTTP/gRPC). Use L7 when one long connection carries many requests.
  ```rust
  // L4: one pick per accepted connection
  // L7: one pick per request on that connection
  ```
