use auth_tarball_from_git::args::Args;
use auth_tarball_from_git::errors::*;
use auth_tarball_from_git::git;
use clap::Parser;
use env_logger::Env;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let log_level = "info";
    env_logger::init_from_env(Env::default().default_filter_or(log_level));

    let tmp_dir = tempfile::Builder::new()
        .prefix("auth-from-git-")
        .tempdir()?;

    if let Some(tag) = args.tag {
        info!("Cloning repository from {:?}", args.repo);
        let path = tmp_dir.path();
        git::clone(path, &args.repo, &tag).await?;

        let keyrings = args.keyrings.iter().map(|p| p.as_ref()).collect::<Vec<_>>();
        git::verify_tag(path, &tag, &keyrings).await?;
        info!("Tag successfully verified");

        info!("Reproducing archive...");
        let prefix = if let Some(mut prefix) = args.prefix {
            if !prefix.ends_with('/') {
                prefix.push('/');
            }
            prefix
        } else {
            let name = if let Some(name) = args.name {
                name
            } else {
                // derive the name from the input path
                let name = args
                    .repo
                    .trim_end_matches('/')
                    .rsplit_once('/')
                    .map(|(_, x)| x)
                    .unwrap_or(&args.repo);
                let name = name.strip_suffix(".git").unwrap_or(name);
                name.to_string()
            };
            let normalized = tag.replace('/', "-");
            format!("{}-{}/", name, normalized)
        };
        let reproduced_archive = git::archive(path, &prefix, &tag, &args.format).await?;
        debug!(
            "Generated tarball is {} bytes big",
            reproduced_archive.len()
        );

        info!("Reading input that should be verified...");
        let input_archive = fs::read(&args.tarball).await?;

        info!("Comparing...");
        if input_archive != reproduced_archive {
            bail!("Reproduced archive did not match");
        } else {
            println!("OK");
        }
    } else if let Some(_commit) = args.commit {
        todo!("Support for signed commits is not implemented yet")
    } else {
        bail!("Either --tag or --commit are needed");
    };

    Ok(())
}
