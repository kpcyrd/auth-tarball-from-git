use crate::errors::*;
use std::path::Path;
use std::process::Stdio;
use tokio::fs;
use tokio::process::Command;

pub async fn clone(folder: &Path, url: &str, tag: &str) -> Result<()> {
    let cmd = Command::new("git")
        .arg("clone")
        .arg("-q")
        .arg("--bare")
        .arg("--depth=1")
        .arg("--branch")
        .arg(tag)
        .arg("--")
        .arg(url)
        .arg(folder)
        .spawn()
        .context("Failed to run git clone")?;

    let out = cmd.wait_with_output().await?;
    if !out.status.success() {
        bail!("Process (git clone) exited with error: {:?}", out.status);
    }

    Ok(())
}

pub struct Tag {
    data: Vec<u8>,
    sig_pos: Option<usize>,
}

impl Tag {
    pub fn data(&self) -> &[u8] {
        if let Some(pos) = self.sig_pos {
            &self.data[..pos]
        } else {
            &self.data
        }
    }

    pub fn sig(&self) -> Option<&[u8]> {
        if let Some(pos) = self.sig_pos {
            Some(&self.data[pos..])
        } else {
            None
        }
    }

    pub async fn verify(&self, keyring: &Path) -> Result<()> {
        let obj = self.data();
        let sig = self
            .sig()
            .ok_or_else(|| anyhow!("Failed to find signature in tag"))?;

        let tmp_dir = tempfile::Builder::new()
            .prefix("auth-from-git-")
            .tempdir()?;
        let path = tmp_dir.path();
        let obj_path = path.join("obj");
        let sig_path = path.join("sig");

        fs::write(&obj_path, obj).await?;
        fs::write(&sig_path, sig).await?;

        let cmd = Command::new("sqv")
            .arg("--keyring")
            .arg(keyring)
            .arg("--")
            .arg(sig_path)
            .arg(obj_path)
            .stdout(Stdio::null())
            .spawn()
            .context("Failed to run sqv")?;

        let out = cmd.wait_with_output().await?;
        if !out.status.success() {
            bail!("Process (sqv) exited with error: {:?}", out.status);
        }

        Ok(())
    }
}

pub async fn read_tag(folder: &Path, tag: &str) -> Result<Tag> {
    let tag_bytes = cat_tag(folder, tag).await?;
    let needle = b"-----BEGIN PGP SIGNATURE-----\n";
    let pos = tag_bytes
        .windows(needle.len())
        .position(|window| window == needle);

    Ok(Tag {
        data: tag_bytes,
        sig_pos: pos,
    })
}

pub async fn verify_tag(folder: &Path, tag: &str, keyring: &Path) -> Result<()> {
    let tag = read_tag(folder, tag).await?;
    tag.verify(keyring).await
}

pub async fn cat_tag(folder: &Path, tag: &str) -> Result<Vec<u8>> {
    let cmd = Command::new("git")
        .arg("cat-file")
        .arg("--")
        .arg("tag")
        .arg(tag)
        .stdout(Stdio::piped())
        .current_dir(folder)
        .spawn()
        .context("Failed to run git cat-file")?;

    let out = cmd.wait_with_output().await?;
    if !out.status.success() {
        bail!("Process (git cat-file) exited with error: {:?}", out.status);
    }

    Ok(out.stdout)
}

pub async fn archive(path: &Path, prefix: &str, tag: &str, format: &str) -> Result<Vec<u8>> {
    let cmd = Command::new("git")
        .args(&["archive", "--format", format, "--prefix", prefix, "--", tag])
        .stdout(Stdio::piped())
        .current_dir(path)
        .spawn()
        .context("Failed to run git archive")?;

    let out = cmd.wait_with_output().await?;
    if !out.status.success() {
        bail!("Process (git archive) exited with error: {:?}", out.status);
    }

    Ok(out.stdout)
}
