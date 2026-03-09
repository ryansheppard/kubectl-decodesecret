use anyhow::Result;
use clap::Parser;
use k8s_openapi::api::core::v1::Secret;
use kube::config::KubeConfigOptions;
use kube::Api;
use kube::Client;
use kube::Config;

#[derive(Debug, Parser)]
#[command(
    name = "kubectl-decodesecret",
    about = "View decoded Kubernetes secret values"
)]
pub struct Args {
    #[arg(help = "Name of the secret to decode")]
    pub secret_name: String,
    #[arg(
        short,
        long,
        help = "Kubernetes namespace (defaults to current context namespace)"
    )]
    pub namespace: Option<String>,
    #[arg(short, long, help = "Print only the value for a specific key")]
    pub key: Option<String>,
    #[arg(long, help = "Kubernetes context to use")]
    pub context: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config = Config::from_kubeconfig(&KubeConfigOptions {
        context: args.context,
        ..Default::default()
    })
    .await?;
    let namespace = args
        .namespace
        .unwrap_or_else(|| config.default_namespace.clone());
    let client = Client::try_from(config)?;
    let secrets_api: Api<Secret> = Api::namespaced(client, &namespace);

    let secret = secrets_api.get(&args.secret_name).await?;
    let data = secret
        .data
        .as_ref()
        .filter(|d| !d.is_empty())
        .ok_or_else(|| anyhow::anyhow!("secret '{}' has no data", args.secret_name))?;

    if let Some(key) = &args.key {
        if let Some(value) = data.get(key.as_str()) {
            println!("{}", String::from_utf8_lossy(&value.0));
        } else {
            anyhow::bail!("key '{}' not found in secret '{}'", key, args.secret_name);
        }
    } else {
        data.iter()
            .for_each(|(k, v)| println!("{}: {}", k, String::from_utf8_lossy(&v.0)));
    }

    Ok(())
}
