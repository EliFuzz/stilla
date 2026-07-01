use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;

use serde::Deserialize;
use tokio::io::AsyncReadExt;
use tokio::process::Command;
use tokio::time::timeout;
use xxhash_rust::xxh3::xxh3_64;

const MAX_SCRIPT_OUTPUT: usize = 256 * 1024;

#[derive(Deserialize)]
pub struct ScriptResult {
    pub title: String,
    pub items: Vec<ScriptItem>,
    #[serde(skip)]
    pub raw_output_hash: u64,
}

#[derive(Deserialize)]
pub struct ScriptItem {
    pub item_type: ScriptItemType,
    pub title: Option<String>,
    pub path: Option<String>,
    pub items: Option<Vec<ScriptItem>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScriptItemType {
    Link,
    Submenu,
    Divider,
    Text,
}

pub async fn run_script(path: &Path) -> Result<Option<ScriptResult>, Error> {
    let Ok(mut child) = Command::new("sh")
        .arg(path)
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()
    else {
        return Err(Error::other("failed to spawn script"));
    };
    let Some(stdout) = child.stdout.take() else {
        let _ = child.kill().await;
        return Err(Error::other("failed to capture stdout"));
    };
    let Ok(Ok(buf)) = timeout(Duration::from_secs(30), async {
        let mut buf = Vec::with_capacity(4096);
        stdout
            .take((MAX_SCRIPT_OUTPUT + 1) as u64)
            .read_to_end(&mut buf)
            .await?;
        if buf.len() > MAX_SCRIPT_OUTPUT {
            let _ = child.kill().await;
            return Err(Error::new(
                ErrorKind::InvalidData,
                "script output too large",
            ));
        }
        child.wait().await?;
        Ok::<Vec<u8>, Error>(buf)
    })
    .await
    else {
        let _ = child.kill().await;
        return Err(Error::new(
            ErrorKind::TimedOut,
            "script timed out or read failed",
        ));
    };
    if buf.is_empty() {
        return Ok(None);
    }
    let mut result: ScriptResult =
        serde_json::from_slice(&buf).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    result.raw_output_hash = xxh3_64(&buf);
    result.items.iter().try_for_each(validate_item)?;
    Ok(Some(result))
}

fn validate_item(item: &ScriptItem) -> Result<(), Error> {
    match (&item.item_type, &item.title, &item.path, &item.items) {
        (ScriptItemType::Link, Some(_), Some(_), None) => Ok(()),
        (ScriptItemType::Submenu, Some(_), None, Some(i)) => i.iter().try_for_each(validate_item),
        (ScriptItemType::Divider, None, None, None) => Ok(()),
        (ScriptItemType::Text, Some(_), None, None) => Ok(()),
        _ => Err(Error::new(ErrorKind::InvalidData, "invalid script item")),
    }
}
