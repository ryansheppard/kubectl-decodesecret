use std::collections::BTreeMap;

use anyhow::Result;
use clap::CommandFactory;
use clap::Parser;
use clap_complete::engine::{ArgValueCompleter, CompletionCandidate};
use k8s_openapi::api::core::v1::Secret;
use kube::Api;
use kube::Client;
use kube::Config;
use kube::config::KubeConfigOptions;

#[derive(Debug, Parser)]
#[command(
    name = "kubectl-decodesecret",
    about = "View decoded Kubernetes secret values"
)]
pub struct Args {
    #[arg(help = "Name of the secret to decode", add = ArgValueCompleter::new(secret_completer))]
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
    #[arg(short, long, help = "Outputs the key/values as JSON")]
    pub json: bool,
}

async fn get_secrets_api(
    context: Option<String>,
    namespace: Option<String>,
) -> Result<Api<Secret>> {
    let config = Config::from_kubeconfig(&KubeConfigOptions {
        context,
        ..Default::default()
    })
    .await?;
    let ns = namespace.unwrap_or_else(|| config.default_namespace.clone());
    let client = Client::try_from(config)?;
    Ok(Api::namespaced(client, &ns))
}

fn secret_completer(current: &std::ffi::OsStr) -> Vec<CompletionCandidate> {
    let Some(current) = current.to_str() else {
        return vec![];
    };

    let Some(rt) = tokio::runtime::Runtime::new().ok() else {
        return vec![];
    };

    let args: Vec<String> = std::env::args().filter(|a| a != "--").collect();
    let matches = Args::command().ignore_errors(true).get_matches_from(args);
    let namespace = matches.get_one::<String>("namespace").cloned();
    let context = matches.get_one::<String>("context").cloned();

    rt.block_on(async {
        let secrets_api = get_secrets_api(context, namespace).await.ok()?;
        let list = secrets_api.list(&Default::default()).await.ok()?;

        Some(
            list.items
                .iter()
                .filter_map(|s| s.metadata.name.as_deref())
                .filter(|s| s.starts_with(current))
                .map(CompletionCandidate::new)
                .collect(),
        )
    })
    .unwrap_or_default()
}

fn main() -> Result<()> {
    clap_complete::CompleteEnv::with_factory(Args::command).complete();

    let args = Args::parse();

    let rt = tokio::runtime::Runtime::new()?;
    let secret = rt.block_on(async {
        let secrets_api = get_secrets_api(args.context, args.namespace).await?;

        let secret = secrets_api.get(&args.secret_name).await?;
        Ok::<_, anyhow::Error>(secret)
    })?;

    let data = secret
        .data
        .as_ref()
        .filter(|d| !d.is_empty())
        .ok_or_else(|| anyhow::anyhow!("secret '{}' has no data", args.secret_name))?;

    let decoded: BTreeMap<&str, String> = match &args.key {
        Some(key) => {
            let value = data.get(key.as_str()).ok_or_else(|| {
                anyhow::anyhow!("key '{}' not found in secret '{}'", key, args.secret_name)
            })?;
            BTreeMap::from([(key.as_str(), String::from_utf8_lossy(&value.0).into_owned())])
        }
        None => data
            .iter()
            .map(|(k, v)| (k.as_str(), String::from_utf8_lossy(&v.0).into_owned()))
            .collect(),
    };

    if args.json {
        println!("{}", serde_json::to_string(&decoded)?);
    } else {
        decoded.iter().for_each(|(k, v)| println!("{}: {}", k, v));
    }

    Ok(())
}
