# SigmaOS-driver-manager

Driver management: installation, updates, detection, hardware compatibility

- **Category:** System Services
- **Engine:** driver-manager
- **Tier:** 2
- **Language:** Rust
- **Provides:** sigmadrivermanager

Part of the [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)
multi-repository component ecosystem.

## Build & Test

```sh
cargo build
cargo test
```

## Contract

This component declares its interface in `COMPONENT.toml`:

- `provides` — capabilities other components may depend on
- `depends_on` — component names this component requires

## License

Apache-2.0 — see [LICENSE](LICENSE).
