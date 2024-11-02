#![cfg(all(not(feature = "only-sig-bypass"), feature = "regular"))]
pub(crate) trait Replacer {
    fn replace(&self, original: &str) -> Option<String>;
}

pub(crate) struct GenericReplacer {
    pub(crate) regex: regex::Regex,
    pub(crate) replacement: String,
    pub(crate) scheme: String,
}

impl Replacer for GenericReplacer {
    fn replace(&self, original: &str) -> Option<String> {
        // Prepare output array
        let mut results: Vec<String> = vec![];
        // Perform the capture over input
        for (_, [path]) in self.regex.captures_iter(original).map(|c| c.extract()) {
            results.push(format!("{}://{}/{}", self.scheme, self.replacement, path));
        }
        // We are supposed to only parse one entry from text
        if 1 == results.len() {
            return Some(results.remove(0));
        } else if results.is_empty() {
            println!("No valid url match found so returning original url");
        } else {
            println!("Invalid number of entries parsed, expected 1, obtained {:?}", results.len());
        }
        None
    }
}