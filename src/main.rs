use clap::Parser;
use wc_rs::Options;

fn main() {
    let opts = Options::parse();
    println!("{:?}", opts);
}

fn handle_options(opts: Options) {
    println!("{}", opts.longest);
    println!("{}", opts.lines);
    println!("{}", opts.count);
    println!("{}", opts.chars);
    println!("{}", opts.words);
}
