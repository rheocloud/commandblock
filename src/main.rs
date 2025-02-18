use kube::{Client, CustomResourceExt};
use kube_runtime::controller::Controller;
use serde::{Deserialize, Serialize};
use kube::core::crd::CustomResourceExt;

#[derive(Clone, Debug, CustomResource, Serialize, Deserialize, Default)]
#[kube(
    group = "comparator.rheocloud.com",
    version = "v1",
    kind = "Comparator",
    namespaced
)]

struct ComparatorSpec {
    version: String,
    player_slots: i32,
    auto_start: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    let crds = vec![Comparator::crd()];
    
    let controller = Controller::new(
        kube::api::Api::all(client.clone()),
        kube_runtime::controller::Config::default(),
    )
    .run(reconcile, on_error)
    .for_each(|_| futures::future::ready(()));

    tokio::select! {
        _ = controller => {},
    }

    Ok(())
}

async fn reconcile(server: Comparator, _: kube::Context) -> Result<(), kube::Error> {
    // Your reconciliation logic here
    Ok(())
}

fn on_error(_: &Comparator, err: &kube::Error, _: kube::Context) {
    eprintln!("Error: {:?}", err);
}