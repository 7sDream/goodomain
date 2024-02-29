use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn tld_version() -> u64 {
    goodomain::TLD_VERSION
}

#[wasm_bindgen]
pub fn tld_count() -> usize {
    goodomain::TLD_LIST.len()
}

#[wasm_bindgen]
pub fn tld_list() -> Vec<String> {
    goodomain::TLD_LIST
        .iter()
        .copied()
        .map(str::to_string)
        .collect()
}

#[wasm_bindgen]
pub fn is_tld(text: &str) -> bool {
    goodomain::is_tld(text)
}

#[wasm_bindgen]
pub struct TLDInWord {
    pub start: usize,
    pub end: usize,
    #[wasm_bindgen(getter_with_clone)]
    pub tld: String,
    #[wasm_bindgen(getter_with_clone)]
    pub domain: String,
    #[wasm_bindgen(getter_with_clone)]
    pub path: String,
    #[wasm_bindgen(getter_with_clone)]
    pub display: String,
}

impl TLDInWord {
    pub fn new(tld: &goodomain::TLDInWord) -> Self {
        let (start, end) = tld.index;
        Self {
            start,
            end,
            tld: tld.tld().to_string(),
            domain: tld.domain(),
            path: tld.path(),
            display: format!("{}", tld),
        }
    }
}

#[wasm_bindgen]
pub fn find(word: &str) -> Result<Vec<TLDInWord>, String> {
    let tlds = goodomain::find(word).map_err(|e| format!("{}", e))?;
    Ok(tlds.map(|tld| TLDInWord::new(&tld)).collect())
}
