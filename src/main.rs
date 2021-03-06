#[macro_use]
extern crate clap;
extern crate rustget;

use std::process;

use rustget::download::{ftp_download, http_download};
use rustget::utils;

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        }
    }
}

fn run() -> Result<(), Box<::std::error::Error>> {
    let args = clap_app!(rustget =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg quiet: -q --quiet "quiet (no output)")
        (@arg continue: -c --continue "resume getting a partially-downloaded file")
        (@arg FILE: -O --output +takes_value "write documents to FILE")
        (@arg AGENT: -U --useragent +takes_value "identify as AGENT instead of rustget/VERSION")
        (@arg SECONDS: -T --timeout +takes_value "set all timeout values to SECONDS")
        (@arg URL: +required +takes_value "url to download")
        ).get_matches();

    let url = utils::parse_url(args.value_of("URL").unwrap())?;
    let quiet_mode = args.is_present("quiet");
    let file_name = args.value_of("FILE");

    match url.scheme() {
        "ftp" => ftp_download(&url, quiet_mode, file_name),
        "http" | "https" => http_download(&url, &args, crate_version!()),
        _ => utils::gen_error(format!("unsupported url scheme '{}'", url.scheme())),
    }
}
