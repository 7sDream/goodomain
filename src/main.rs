mod data;
mod args;

fn main() {
    assert_eq!(data::TLD_COUNT, data::TLD_LIST.len());

    let mut args = args::get();

    if args.version {
        println!("Binary version: {}", env!("CARGO_PKG_VERSION"));
        println!("Data file version: {}, TLD count {}", data::VERSION, data::TLD_COUNT);
        return;
    }

    if args.list {
        data::TLD_LIST.iter().for_each(|tld| {
            tld.iter().for_each(|c| print!("{}", c));
            println!()
        });
        return;
    }

    let word = args.word.take().unwrap();

    let char_with_index : Vec<(usize, char)> = word.char_indices().collect();
    let chars: Vec<char> = char_with_index.iter().map(|x| x.1.to_ascii_lowercase()).collect();
    let mut indexes: Vec<usize> = char_with_index.iter().map(|x| x.0).collect();
    indexes.push(word.len());

    for start in 0 .. chars.len() {
        for end in start + 1 ..= chars.len() {
            let target = &chars[start .. end];
            if data::TLD_LIST.contains(target) {
                if start == 0 {
                    print!("<something>.");
                } else {
                    print!("{}.", &word[..indexes[start]]);
                }

                print!("{}", &word[indexes[start] .. indexes[end]]);

                if end != chars.len() {
                    print!("/{}", &word[indexes[end]..])
                }

                print!("\n");
            }
        }
    }
}
