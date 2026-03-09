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
