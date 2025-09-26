# Template

This is a simple template for a Rust project using the [Zed](https://zed.dev/) editor and [development containers](https://containers.dev/) and the [DevPod](https://devpod.sh/) CLI to manage it all. It's an opinionated setup for developing on an immutable operating system, such as [Bluefin](https://projectbluefin.io/).

Zed will install the `rust-analyzer` language server inside the container, which will also contain any dependencies your application will need.

## Getting Started

- Install the [DevPod CLI](https://devpod.sh/docs/getting-started/install#install-devpod-cli).
- Copy this repository and file off the serial numbers:
  - `rm -rf .git && git init`
  - Modify the `devcontainer/Dockerfile` to add any dependencies required for your project.
  - Rename the project in `Cargo.toml`.
  - Happy coding!

## Development

```bash
cd /path/to/template

# Build the container, bring it up, and open the editor
devpod up .
```

When you're done, you can stop the container with `devpod down`.
