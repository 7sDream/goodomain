mod args;

fn main() {
    let args = args::get();

    if args.version {
        println!("Binary version: {}", env!("CARGO_PKG_VERSION"));
        println!(
            "Data file version: {}, TLD count {}",
            goodomain::TLD_VERSION,
            goodomain::TLD_LIST.len()
        );
        return;
    }

    if args.list {
        for domain in goodomain::TLD_LIST {
            println!("{}", domain)
        }
        return;
    }

    let word = args.word.unwrap();
    match goodomain::find(&word) {
        Ok(tlds) => {
            for tld in tlds {
                println!("{}", tld.display(&word));
            }
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    };
}
