# Rust Application Template

  This is a template for developing Rust Applications, inspired by the [cosmic-app-template](https://github.com/pop-os/cosmic-app-template).

  It also includes support for the [Zed](https://zed.dev/) editor, [development containers](https://containers.dev/) and the [DevPod](https://devpod.sh/) CLI to manage it all. It's an opinionated setup for developing on an immutable operating system, such as [Bluefin](https://projectbluefin.io/).

Zed will install the `rust-analyzer` language server inside the container, which will also contain any dependencies your application will need.

## Getting Started

To create an application with this template, install [cargo-generate] and run:

```bash
cargo generate gh:AdamIsrael/template-rust
```

Also, you'll need to install the [DevPod CLI](https://devpod.sh/docs/getting-started/install#install-devpod-cli).

A [justfile](./justfile) is included by default with common recipes. Install from [casey/just][just]

- `just` builds the application with the default `just build-release` recipe
- `just run` builds and runs the application
- `just install` installs the project into the system
- `just vendor` creates a vendored tarball
- `just build-vendored` compiles with vendored dependencies from that tarball
- `just check` runs clippy on the project to check for linter warnings
- `just check-json` can be used by IDEs that support LSP
- `just up` brings up the devcontainer and launches your editor
- `just down` brings down the devcontainer
## Customization

Modify the `devcontainer/Dockerfile` to add any dependencies required for your project.


[cargo-generate]: https://cargo-generate.github.io/cargo-generate/installation.html
[just]: https://github.com/casey/just
