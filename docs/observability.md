# Observability

## Overview

Observability is achieved using the following systems, which also require proper configuration and practices in the codebase.

### Error Monitoring, Tracing & Log aggregation
- **Sentry.io**
  - Error monitoring and alerts
  - Distributed tracing
  - Centralized Log aggregation

### Metrics & Dashboards
- **Grafana + Prometheus**
  - Metrics-based service health dashboard
  - Real-time performance monitoring

## Error Monitoring

I purposefuly sent an event with an invalid format to trigger an error in the system so we can see how this error monitoring system works.

![Error sample #1](media/error-sample-01.png)

I also got an email for the same error.

![Error sample #1 email](media/error-sample-01-email.png)

We can even see where the error ocurred in the code using the tracing data.

![Error sample #1 location](media/error-sample-01-location.png)

## Distributed tracing

We can see a sample of all traces being captured in the system.

![Trace samples](media/trace-samples.png)

And if we go into a specific trace, we can see more details about it.

![Trace sample #1](media/trace-sample-01.png)

## Log aggregation

Logs are also published and searchable in *Sentry.io*.

For example, we can seach for the logs of the invalid events we sent earlier to test the error monitoring.

![Invalid events logs](media/logs-invalid-events.png)

And each log line includes more contextual information.

![Invalid event log details](media/logs-invalid-event-details-01.png)

## Metrics & Dashboards

The service is publishing prometheus metrics, and *fly.io* supports reading this metrics and provides a Grafana instance so users can create dashboards based on these metrics.

- `fly.toml` metrics configuration:
   ```toml
   [metrics]
   port = 8081
   path = "/metrics"
   ```

With the metrics published by this service, I created this custom dashboard to monitor its health.

![Service health dashboard](media/metrics-service-health-dashboard.png)
