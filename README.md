# kubectl-decodesecret

A kubectl plugin that decodes and displays Kubernetes secret values without needing to manually base64-decode them.

## Installation

Build and place the binary somewhere on your `PATH` with the `kubectl-` prefix so kubectl can discover it as a plugin:

```sh
mise run build
cp target/release/kubectl-decodesecret ~/.local/bin/kubectl-decodesecret
```

Or install directly to your Cargo bin directory:

```sh
mise run install
```

Verify kubectl picks it up:

```sh
kubectl plugin list
```

## Shell Completions

This plugin supports shell completions via `clap_complete`. Completions are driven by the `COMPLETE` environment variable.

**Note:** Completions register under the binary name `kubectl-decodesecret`, so you should invoke the binary directly (or use a shell alias/abbreviation) rather than `kubectl decodesecret` for completions to work.

### Fish

Add to `~/.config/fish/config.fish`:

```fish
COMPLETE=fish kubectl-decodesecret | source
```

Optionally add an abbreviation for convenience:

```fish
abbr -a kds kubectl-decodesecret
```

### Bash

Add to `~/.bashrc`:

```bash
source <(COMPLETE=bash kubectl-decodesecret)
```

### Zsh

Add to `~/.zshrc`:

```zsh
source <(COMPLETE=zsh kubectl-decodesecret)
```

## Usage

```
kubectl decodesecret <SECRET_NAME> [OPTIONS]

Arguments:
  <SECRET_NAME>  Name of the secret to decode

Options:
  -n, --namespace <NAMESPACE>  Kubernetes namespace (defaults to current context namespace)
  -k, --key <KEY>              Print only the value for a specific key
  -h, --help                   Print help
```

## Examples

Print all decoded keys and values from a secret:

```sh
kubectl decodesecret my-app-secrets -n my-namespace
```

```
API_KEY: abc123
DB_PASSWORD: supersecret
```

Print only the value for a specific key:

```sh
kubectl decodesecret my-app-secrets -n my-namespace -k API_KEY
```

```
abc123
```

Pipe a single value directly into another command:

```sh
kubectl decodesecret my-app-secrets -n my-namespace -k API_KEY | pbcopy
```

Use the current context's default namespace:

```sh
kubectl decodesecret my-app-secrets -k API_KEY
```
