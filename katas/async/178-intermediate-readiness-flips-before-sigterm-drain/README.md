# 178 async (intermediate): readiness flips before the SIGTERM drain

**Concept:** Readiness, liveness and accepting are three different states. On SIGTERM, readiness goes false first while the service keeps serving for a grace delay long enough for the load balancer to notice. Then it stops accepting, lets in-flight requests finish, and exits before the kill deadline, which is counted from SIGTERM. Liveness stays true the whole time.
**Source:** Kubernetes docs: Pod lifecycle, "Termination of Pods" (https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#pod-termination); "Configure Liveness, Readiness and Startup Probes" (https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/); "Container Lifecycle Hooks" (https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/); tokio `watch` and `time` docs; kata 58.
**Fails with:** `the load balancer got connection errors during shutdown`
**Vault:** your note on graceful shutdown behind a load balancer

`Service` is one pod. `ready()` is its readiness probe, `live()` its liveness probe and `handle` its accept loop. The tests use a fake load balancer: it probes `ready()` every 100 ms, ejects the pod after 3 failures in a row, and sends a request every 10 ms until then. Time is paused, so every timing is exact.

## Task
Make `cargo test` pass by changing only `src/lib.rs`. Don't change the tests, `Cargo.toml` or any public signature. No `unwrap`, `expect` or `panic!` in `src/`, no `#[allow]`, and no new crates.

The shutdown must follow these rules:
- On SIGTERM, readiness goes false at once.
- New requests are accepted for `grace_delay` after SIGTERM, then refused.
- `run` returns `Exit::Clean` once no request is in flight.
- `run` returns no later than `deadline` after SIGTERM, as `Exit::Killed { in_flight }`.
- `live()` stays true until `run` returns.

## Before you fix it
1. Run `cargo test`. Which three tests fail?
2. In the load-balancer test, draw the timeline: SIGTERM at 1005 ms, probes at 1100, 1200 and 1300 ms. Which requests are refused?

## Answer in the PR
1. Why can't the service make the load balancer stop sooner, and where did the 29 refused requests go?
2. What does the kubelet do when a liveness probe fails, and why would that be wrong during shutdown?
3. When does your drain's deadline clock start? What happens when `grace_delay` is longer than `deadline`?
4. Between readiness going false and accepting stopping, who still sends requests, and why?

## Think further
1. On pod deletion, the kubelet runs the `preStop` hook, then sends SIGTERM. At the same time the control plane marks the pod's endpoint as terminating, and kube-proxy, the ingress and the cloud load balancer each take their own time to see it. Which of these is the "fake load balancer" here?
2. A manifest with the pieces of this kata:
   ```yaml
   spec:
     terminationGracePeriodSeconds: 30
     containers:
       - name: app
         readinessProbe: { httpGet: { path: /readyz, port: 8080 }, periodSeconds: 5, failureThreshold: 2 }
         livenessProbe:  { httpGet: { path: /livez,  port: 8080 }, periodSeconds: 10, failureThreshold: 3 }
         lifecycle:
           preStop:
             sleep: { seconds: 10 }
   ```
   Map `grace_delay` and `deadline` onto it. Does the `preStop` time count against `terminationGracePeriodSeconds`? What does an `exec: sleep` preStop need that the `sleep` action doesn't?
3. A rolling update (`maxSurge`, `maxUnavailable`) and an HPA scale-down both delete pods. Why do both need this order?
4. Real SIGTERM in `main`: `tokio::signal::unix::signal(SignalKind::terminate())` (tokio `signal` feature). Why is it a future passed into `run` here?
5. See the sd-katas card on load balancing and health checks.

## In the wild
- Kubernetes "Termination of Pods": preStop, SIGTERM, grace period, SIGKILL.
- Kubernetes probes: readiness removes a pod from Service endpoints, and a failed liveness probe restarts the container.
- Cloud load balancers' deregistration delay (connection draining), for example on AWS target groups.
- axum `serve(...).with_graceful_shutdown` and kata 58's drain.

In a review, check:
- One `healthy` flag behind both probes
- The listener closed as soon as SIGTERM arrives
- A liveness handler that checks shutdown state
- A drain timeout that starts after a preStop or grace sleep
- A grace delay shorter than probe period × failure threshold

## Related concepts

### `tokio::signal::unix::signal`
A stream of one Unix signal. Use it in `main` to turn SIGTERM into a future.
```rust
let mut term = signal(SignalKind::terminate())?;
term.recv().await;
```

### Startup probe
A third probe that holds off liveness and readiness checks until the app has started. Use it so a slow-booting app isn't restarted.
```yaml
startupProbe: { httpGet: { path: /livez, port: 8080 }, failureThreshold: 30, periodSeconds: 10 }
```

### `tokio_util::sync::CancellationToken`
A cloneable stop signal tasks can await. Use it to tell many tasks to stop at once (kata 58).
```rust
let token = CancellationToken::new();
let child = token.child_token();
tokio::spawn(async move { child.cancelled().await });
token.cancel();
```

### axum `with_graceful_shutdown`
Stops accepting when its future resolves, then waits for open connections. Use it to drain an HTTP server.
```rust
axum::serve(listener, app).with_graceful_shutdown(shutdown).await?;
```
