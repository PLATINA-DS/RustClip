mod daemon;
mod models;
mod ui;
mod utils;

use std::env;

use daemon::run_daemon;
use ui::run_show;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "daemon" => run_daemon(),
            "show" => run_show(),
            _ => {}
        }
    }
}