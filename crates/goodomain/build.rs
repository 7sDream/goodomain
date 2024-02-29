//! Build script

use std::{
    borrow::Cow,
    fs::OpenOptions,
    io::{self, Write},
    path::PathBuf,
};

const TLD_DATA_URL: &str = "https://data.iana.org/TLD/tlds-alpha-by-domain.txt";

fn download_tld_data() -> io::Result<String> {
    ureq::get(TLD_DATA_URL)
        .call()
        .map_err(io::Error::other)?
        .into_string()
}

struct TLDData {
    version: u64,
    data: String,
}

impl TLDData {
    pub fn parse(data: String) -> Result<Self, &'static str> {
        let mut lines = data.lines();
        let first_line = lines.next().unwrap();
        let version = first_line
            .strip_prefix("# Version ")
            .and_then(|s| s[..s.find(',').unwrap_or(s.len())].parse::<u64>().ok())
            .ok_or("Invalid version number")?;

        Ok(Self { version, data })
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn domains(&self) -> impl Iterator<Item = Cow<'_, str>> {
        self.data.lines().skip(1).map(|line| {
            if let Some(stripped) = line.strip_prefix("XN--") {
                Cow::from(punycode::decode(stripped).unwrap())
            } else {
                Cow::from(line)
            }
        })
    }
}

fn tld_domains_to_vec_source(tld: &TLDData, mut f: impl Write) -> io::Result<usize> {
    let mut count = 0;
    f.write_all(b"[")?;
    for domain in tld.domains() {
        f.write_all(b"\"")?;
        for char in domain.chars() {
            let char_source = format!("\\u{{{:x}}}", char.to_ascii_lowercase() as u32);
            f.write_all(char_source.as_bytes())?;
        }
        f.write_all(b"\", ")?;
        count += 1;
    }
    f.write_all(b"]")?;

    Ok(count)
}

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=NOT_EXIST_FILE"); // force rebuild to download the latest tld data file

    let out_dir: PathBuf = std::env::var("OUT_DIR").unwrap().into();

    let data = download_tld_data().unwrap();
    let tld = TLDData::parse(data).unwrap();

    let version_file_path = out_dir.join("tld_version.in");
    let count_file_path = out_dir.join("tld_count.in");
    let data_file_path = out_dir.join("tld_list.in");

    let mut op = OpenOptions::new();
    op.create(true).write(true).truncate(true);

    let mut version_file = op.open(version_file_path)?;
    version_file.write_all(format!("{}", tld.version()).as_bytes())?;

    let data_file = op.open(data_file_path)?;
    let domain_count = tld_domains_to_vec_source(&tld, data_file)?;

    let mut count_file = op.open(count_file_path).unwrap();
    count_file
        .write_all(format!("{}", domain_count).as_bytes())
        .unwrap();

    Ok(())
}
