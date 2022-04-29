// mod config;
// mod help;
// mod input;
// extern crate pbr;

// use colored::Colorize;
// use dircpy::CopyBuilder;
// use help::help;
// use input::input;
// use notify_rust::Notification;
// use pbr::ProgressBar;
// use std::{env, process};
// use whoami;
// use youchoose;

// #[derive(Clone)]
// struct StructDotfile {
//     source: String,
//     destination: String,
// }

// use std::fmt;

// impl fmt::Display for StructDotfile {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
//         write!(f, "{0: <35} {1}", self.source, self.destination)
//     }
// }
mod cli;
use cli::Cli;

fn main() {
    let cli = Cli::new();
}
