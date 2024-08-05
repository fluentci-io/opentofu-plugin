# OpenTofu Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/opentofu)](https://pkg.fluentci.io/opentofu)
[![ci](https://github.com/fluentci-io/opentofu-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/opentofu-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [OpenTofu](https://opentofu.org).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm opentofu setup
```

## Functions

| Name     | Description                                        |
| -------- | -------------------------------------------------- |
| setup    | Install a specific version of OpenTofu.            |
| init     | Prepare your working directory for other commands  |
| validate | Check whether the configuration is valid           |
| plan     | Show changes required by the current configuration |
| apply    | Create or update infrastructure                    |
| destroy  | Destroy previously-created infrastructure          |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.2.1"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/opentofu@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: opentofu
    args: |
      setup
- name: Show opentofu version
  run: |
    type tofu
    tofu version
```
