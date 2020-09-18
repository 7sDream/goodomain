use std::{
    fs::OpenOptions,
    path::PathBuf,
    io::Write,
    borrow::Cow,
};

const TLD_DATA_URL: &str = "https://data.iana.org/TLD/tlds-alpha-by-domain.txt";

fn main() {
    let out_dir: PathBuf = std::env::var("OUT_DIR").unwrap().into();

    let resp = ureq::get(TLD_DATA_URL).call();
    let data = resp.into_string().unwrap();
    let mut lines = data.lines();

    let first_line = lines.next().unwrap();
    if !first_line.starts_with("# Version ") {
        panic!("Invalid TLD data file");
    }
    let version = first_line[10..20].parse::<u64>().unwrap();
    let tld_list = lines.map(|line| {
        if line.starts_with("XN--") {
            Cow::from(punycode::decode(&line[4..]).unwrap())
        } else {
            Cow::from(line)
        }
    });

    let version_file_path = out_dir.join("tld_version.in");
    let count_file_path = out_dir.join("tld_count.in");
    let data_file_path = out_dir.join("tld_list.in");

    let mut op = OpenOptions::new();
    op.create(true).write(true).truncate(true);

    let mut version_file = op.open(version_file_path).unwrap();
    version_file.write(format!("{}", version).as_bytes()).unwrap();

    let mut data_file = op.open(data_file_path).unwrap();
    data_file.write(b"{\n    let mut result= std::collections::HashSet::<&'static [char]>::with_capacity(TLD_COUNT);\n").unwrap();
    let mut count: usize = 0;
    for tld in tld_list {
        data_file.write(b"    result.insert(&[").unwrap();
        for char in tld.chars() {
            data_file.write(format!("'{}', ", char.to_ascii_lowercase()).as_bytes()).unwrap();
        }
        data_file.write(b"]);\n").unwrap();
        count += 1;
    }
    data_file.write(b"    result\n}").unwrap();

    let mut count_file = op.open(count_file_path).unwrap();
    count_file.write(format!("{}", count).as_bytes()).unwrap();
}
