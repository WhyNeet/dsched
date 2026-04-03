use tokio::{fs, io::AsyncWriteExt};
use uuid::Uuid;

#[cfg(debug_assertions)]
const IDENTITY_PATH: &str = "./cluster.id";
#[cfg(not(debug_assertions))]
const IDENTITY_PATH: &str = "/var/lib/dsched/cluster.id";

pub async fn get() -> anyhow::Result<Uuid> {
    tracing::debug!("retrieving cluster identity");

    Ok(if fs::try_exists(IDENTITY_PATH).await? {
        Uuid::from_slice(fs::read(IDENTITY_PATH).await?.as_slice())?
    } else {
        let id = Uuid::new_v4();
        fs::File::create(IDENTITY_PATH)
            .await?
            .write_all(id.as_bytes())
            .await?;
        id
    })
}
