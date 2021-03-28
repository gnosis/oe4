// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

mod send;

use clap::Clap;
use std::error::Error;

#[derive(Clap, Debug)]
struct CommonOptions {
  #[clap(long, short, about="More extensive logging")]
  verbose: bool,
}

#[derive(Clap, Debug)]
struct Opts {
  #[clap(flatten)]
  global: CommonOptions,

  #[clap(subcommand)]
  command: Command,
}

#[derive(Clap, Debug)]
enum Command {
  Send(send::SendOptions),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let opts = Opts::parse();
  match opts.command {
    Command::Send(send_opts) => send::run(&send_opts, &opts.global).await,
  }
}
