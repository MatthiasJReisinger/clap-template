extern crate clap;

use clap::{crate_authors, crate_name, crate_version, App, Arg};

fn main() {
    let _matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .get_matches();
}
