# Resilience and Discoverability

## Overview

This document will describe the following:
- Platform provided resilience mechanisms.
- Service processes that were designed and programmed to be resilient.

## Platform provided resilience
As part of the platform, *fly.io* provides several resilience mechanisms, namely:
- Automatic load-balancing to all available instances.
- Discoverability: all you need to know is the internal name of the service, and the `fly proxy` will always route the request to one available machine.
- Health checks
- Zero-downtime deployments

## Health check

The service exposes a `GET /healthz` endpoint, therefore we can configure a health check in the platform.

- `fly.toml` HTTP service health check configuration:
   ```toml
   [[http_service.checks]]
   interval = "15s"
   timeout = "2s"
   grace_period = "10s"
   method = "GET"
   path = "/healthz"
   ```

This health check is used to ensure there are zero-downtime deployments, when you have more than one instance of the service running.

### References

- https://fly.io/docs/reference/autoscaling/
- https://fly.io/docs/reference/health-checks/
- https://fly.io/docs/blueprints/seamless-deployments/
- https://fly.io/docs/reference/fly-proxy/

## Resilience in the service code

The service implements a Redis-based event listener and handler mechanism, which we must ensure that continues working even if there is a failure when processing an invidual event.  In other words, a failure in the handling of an individual event must not affect the entire event listener mechanism.

I implemented a robust asynchronous task-based event listener, where a parent task will supervise the event-listener task, and restart it if it fails, for example, if it loses the connection to Redis temporarily.

Also, the handling of individual events are done in their own separate task for each event, isolated from the main event listener.

See the code here: [events.rs](../src/events.rs).

To demostrate this, I will run the service and kill Redis temporarily, and then we will see how our service recovers.

Logs:
```
2025-09-21T03:12:38.335457Z  INFO routine: main server listening on 0.0.0.0:8080
2025-09-21T03:12:38.335457Z  INFO routine::events: Starting event processing loop (restart count: 0)
2025-09-21T03:12:38.335722Z  INFO routine: metrics server listening on 0.0.0.0:8081
2025-09-21T03:12:56.750537Z  INFO handle_goal_set_event: routine::events: Got GoalSetEvent event: {"user_id":"user01", "goal":"Strength"}

2025-09-21T03:13:09.658489Z ERROR routine::events: Event processing loop error: RedisError { location: Location { file: "src/events.rs", line: 50, column: 30 }, source: broken pipe }
2025-09-21T03:13:09.658926Z  INFO routine::events: Restarting event processing in 5s
2025-09-21T03:13:14.661047Z  INFO routine::events: Starting event processing loop (restart count: 1)

2025-09-21T03:13:39.150262Z  INFO handle_goal_set_event: routine::events: Got GoalSetEvent event: {"user_id":"user02", "goal":"Strength"}
```
