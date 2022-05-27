use std::path::PathBuf;

/// Verify a tarball using signed git tags and reproducible builds
#[derive(Debug, clap::Parser)]
pub struct Args {
    /// A file with trusted public keys to verify with
    #[clap(long)]
    pub keyring: PathBuf,
    /// The tag to verify and use to reproduce the tarball
    #[clap(long)]
    pub tag: Option<String>,
    /// Specify a signed commit that should be used instead of a signed tag
    #[clap(long)]
    pub commit: Option<String>,
    /// Resolve a tag to a commit and verify
    #[clap(long)]
    pub resolve_unsigned_tag: bool,
    /// The prefix to use when generating the tarball, this is automatically detected otherwise
    #[clap(long)]
    pub prefix: Option<String>,
    /// Use this name for the archive prefix instead of deriving one automatically
    #[clap(long)]
    pub name: Option<String>,
    /// Use a specific format for the archive
    #[clap(long, default_value = "tar.gz")]
    pub format: String,
    /// The remote repository to clone
    pub repo: String,
    /// Path to the tarball that should be verified
    pub tarball: String,
}
