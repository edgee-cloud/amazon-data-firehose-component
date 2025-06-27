<div align="center">
<p align="center">
  <a href="https://www.edgee.cloud">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://cdn.edgee.cloud/img/component-dark.svg">
      <img src="https://cdn.edgee.cloud/img/component.svg" height="100" alt="Edgee">
    </picture>
  </a>
</p>
</div>

<h1 align="center">Amazon Data Firehose component for Edgee</h1>


[![Coverage Status](https://coveralls.io/repos/github/edgee-cloud/amazon-data-firehose-component/badge.svg)](https://coveralls.io/github/edgee-cloud/amazon-data-firehose-component)
[![GitHub issues](https://img.shields.io/github/issues/edgee-cloud/amazon-data-firehose-component.svg)](https://github.com/edgee-cloud/amazon-data-firehose-component/issues)
[![Edgee Component Registry](https://img.shields.io/badge/Edgee_Component_Registry-Public-green.svg)](https://www.edgee.cloud/edgee/amazon-data-firehose)


This component enables seamless integration between [Edgee](https://www.edgee.cloud) and [Amazon Data Firehose](https://aws.amazon.com/firehose/), allowing you to collect and forward analytics events to your delivery streams.


## Quick Start

1. Download the latest component version from our [releases page](../../releases)
2. Place the `firehose.wasm` file in your server (e.g., `/var/edgee/components`)
3. Add the following configuration to your `edgee.toml`:

```toml
[[components.data_collection]]
id = "firehose"
file = "/var/edgee/components/firehose.wasm"
settings.aws_access_key = "YOUR_AWS_ACCESS_KEY"
settings.aws_secret_key = "YOUR_AWS_SECRET_KEY"
settings.aws_region = "YOUR_AWS_REGION"
settings.firehose_stream = "YOUR_STREAM_NAME"
```

Please note that your Firehose Delivery Stream needs "Direct PUT" as source.

## Event Handling

### Event Mapping
The component maps Edgee events to Firehose records as follows:

| Edgee Event | Firehose record | Description |
|-------------|----------------|-------------|
| Page        | `full-event.json` | Full JSON dump of the Page event |
| Track       | `full-event.json` | Full JSON dump of the Track event |
| User        | `full-event.json` | Full JSON dump of the User event |


## Configuration Options

### Basic Configuration
```toml
[[components.data_collection]]
id = "firehose"
file = "/var/edgee/components/firehose.wasm"
settings.aws_access_key = "YOUR_AWS_ACCESS_KEY"
settings.aws_secret_key = "YOUR_AWS_SECRET_KEY"
settings.aws_region = "YOUR_AWS_REGION"
settings.firehose_stream = "YOUR_STREAM_NAME"

# Optional configurations
settings.aws_session_token = "YOUR_AWS_SESSION_TOKEN" # Useful for tests, not recommended in prod since it's short-lived
```


### Event Controls
Control which events are forwarded to Amazon Data Firehose:
```toml
settings.edgee_page_event_enabled = true   # Enable/disable page view tracking
settings.edgee_track_event_enabled = true  # Enable/disable custom event tracking
settings.edgee_user_event_enabled = true   # Enable/disable user identification
```


## Development

### Building from Source
Prerequisites:
- [Rust](https://www.rust-lang.org/tools/install)
- wit-deps: `cargo install wit-deps`

Build command:
```bash
make build
```

Test command:
```bash
make test
```

Test coverage command:
```bash
make test.coverage[.html]
```

### Contributing
Interested in contributing? Read our [contribution guidelines](./CONTRIBUTING.md)

### Security
Report security vulnerabilities to [security@edgee.cloud](mailto:security@edgee.cloud)
