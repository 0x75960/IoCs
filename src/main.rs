extern crate structopt;

use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::io::prelude::*;
use structopt::clap::AppSettings;
use structopt::StructOpt;

pub struct IoCMatcher {
    hashes: Regex,
    ips: Regex,
    urls: Regex,
}

impl IoCMatcher {
    pub fn new() -> IoCMatcher {
        IoCMatcher {
            hashes: Regex::new(r"[[:^xdigit:]](?P<target>[[:xdigit:]]{32}|[[:xdigit:]]{40}|[[:xdigit:]]{64})[[:^xdigit:]]").unwrap(),
            ips: Regex::new(r"(?P<target>(([1-9]?[0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\[?\.\]?){3}([1-9]?[0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5]))").unwrap(),
            urls: Regex::new(r"(?P<target>hxxps?://[\w/:%#\$&\?\(\)~\.=\+\-\[\]]+)").unwrap(),
        }
    }

    pub fn hashes_in(&self, target: &impl AsRef<str>) -> HashSet<String> {
        self.matches_in(&self.hashes, target)
    }

    pub fn ips_in(&self, target: &impl AsRef<str>) -> HashSet<String> {
        self.matches_in(&self.ips, target)
    }

    pub fn urls_in(&self, target: &impl AsRef<str>) -> HashSet<String> {
        self.matches_in(&self.urls, target)
    }

    pub fn matches_in(&self, r: &Regex, target: &impl AsRef<str>) -> HashSet<String> {
        let t = target.as_ref();
        r.captures_iter(t)
            .map(|x| String::from(&x["target"]))
            .collect()
    }
}

/// IoC scraper
#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "AppSettings::InferSubcommands"))]
enum Commands {
    /// scrape hashes(sha256, sha1, md5) from stdin pipe
    #[structopt(name = "hashes")]
    Hashes,

    /// scrape URLs(only start with hxxp, allow to contain "[]") from stdin pipe
    #[structopt(name = "urls")]
    URLs,

    /// scrape ip addresses(allow to contain "[]") from stdin pipe
    #[structopt(name = "ips")]
    IPs,
}

fn main() -> Result<(), Box<dyn Error>> {
    let command = Commands::from_args();

    let mut buffer = String::new();

    std::io::stdin().read_to_string(&mut buffer)?;

    let matcher = IoCMatcher::new();

    match command {
        Commands::Hashes => {
            for item in matcher.hashes_in(&buffer) {
                println!("{}", item);
            }
        }

        Commands::URLs => {
            for item in matcher.urls_in(&buffer) {
                println!("{}", item);
            }
        }

        Commands::IPs => {
            for item in matcher.ips_in(&buffer) {
                println!("{}", item);
            }
        }
    }

    Ok(())
}
