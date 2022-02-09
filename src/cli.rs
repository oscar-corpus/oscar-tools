//! Commands enum
use crate::compress::CompressCorpus;
use crate::error::Error;
use crate::extract_clean::ExtractCleanCorpus;
use crate::extract_text::ExtractText;
use crate::lang_codes::UpdateLangCodes;
use crate::split_latest::SplitLatest;
use structopt::StructOpt;

/// Runnable traits have to be implemented by commands
/// in order to be executed from CLI.
// TODO: Currently, run returns (), so if the command
// actually returns something usable, it cannot pass it on.
// shall we provide flexibility to the Runnable trait by using generics
// or provide another trait like Queryable to fetch results?
pub trait Runnable {
    fn run(&self) -> Result<(), Error>;
}

#[derive(Debug, StructOpt)]
#[structopt(name = "oscar-tools", about = "A collection of tools for OSCAR corpus")]
/// Holds every command that is callable by the `oscar-tools` command.
pub enum OscarTools {
    #[structopt(about = "update language codes to BCP-47 and fix mistakes from OSCAR v1.")]
    UpdateLangCodes(UpdateLangCodes),
    #[structopt(about = "Extract a clean corpus from provide OSCAR Schema v2 corpus")]
    ExtractCleanCorpus(ExtractCleanCorpus),
    #[structopt(about = "Split a corpus into a set of smaller files")]
    SplitLatest(SplitLatest),
    #[structopt(about = "Compress corpus. Useable on files and folders (compresses on a depth=1)")]
    Compress(CompressCorpus),
    #[structopt(
        about = "Extracts textual information, discarding metadata. Produces a corpus following OSCAR Scheme v1"
    )]
    Extract(ExtractText),
}

impl Runnable for OscarTools {
    fn run(&self) -> Result<(), Error> {
        match self {
            OscarTools::UpdateLangCodes(u) => u.run(),
            OscarTools::ExtractCleanCorpus(u) => u.run(),
            OscarTools::SplitLatest(u) => u.run(),
            OscarTools::Compress(u) => u.run(),
            OscarTools::Extract(u) => u.run(),
        }
    }
}
