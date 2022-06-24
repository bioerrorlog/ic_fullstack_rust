use anyhow::{Result, Context};
use candid::{
    Deserialize,
};
use ic_agent::{
    agent::{
        Agent,
        http_transport::ReqwestHttpReplicaV2Transport,
    },
    identity::BasicIdentity,
};
use std::{
    env,
    fs::File,
    path::{Path, PathBuf},
};

pub enum Network {
    /// The mainnet at <https://ic0.app/>.
    Ic,
    /// The local replica at <http://localhost:8000/>.
    Local,
}

#[derive(Deserialize)]
pub struct DefaultIdentity {
    default: String,
}

pub async fn get_agent(network: Network) -> Result<Agent> {
    let url = match network {
        Network::Local => "http://localhost:8000",
        Network::Ic => "https://ic0.app",
    };
    let user_home = env::var_os("HOME").unwrap();
    let file = File::open(Path::new(&user_home).join(".config/dfx/identity.json"))
        .context("Configure an identity in `dfx` or provide an --identity flag")?;
    let default: DefaultIdentity = serde_json::from_reader(file)?;
    let pemfile = PathBuf::from_iter([
        &*user_home,
        ".config/dfx/identity/".as_ref(),
        default.default.as_ref(),
        "identity.pem".as_ref(),
    ]);
    let identity = BasicIdentity::from_pem_file(pemfile)?;
    let agent = Agent::builder()
        .with_transport(ReqwestHttpReplicaV2Transport::create(url)?)
        .with_identity(identity)
        .build()?;
    if let Network::Local = network {
        agent.fetch_root_key().await?;
    }
    Ok(agent)
}
