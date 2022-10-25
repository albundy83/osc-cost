use std::process::exit;
use clap::Parser;
use std::sync::atomic::{AtomicBool, Ordering};

mod oapi;
mod core;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = Args::parse();
    if args.debug {
        set_debug_on();
    }

    let mut oapi_input = match oapi::Input::new(args.profile) {
        Ok(input) => input,
        Err(e) => {
            eprintln!("error: cannot load Outscale API as default: {:?}", e);
            exit(1);
        }
    };
    if let Err(error) = oapi_input.fetch() {
        eprintln!("error while fetching ressources: {:?}", error);
    }
    let ressources = core::Resources::from(oapi_input);
    eprintln!("debug: resources has {} vms", ressources.vms.len());
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    // Profile name to use in ~/.osc/config.json
   #[arg(long, short = 'p', default_value_t = String::from("default"))]
   profile: String,
   #[arg(long, default_value_t = false)]
   debug: bool,
}

static DEBUG: AtomicBool = AtomicBool::new(false);

fn set_debug_on() {
    eprintln!("info: debug mode on");
    DEBUG.store(true, Ordering::SeqCst);
}

fn debug() -> bool {
    DEBUG.load(Ordering::SeqCst)
}