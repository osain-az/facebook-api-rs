use seed::prelude::IndexMap;
use seed::prelude::Url;

/// Extract data from  from the url fragment and return an `IndexMap`
/// for the Enum Variant.
/// # Panics
/// The function will panic a key that has no value.
/// # Warns
/// with no query. Theses choices are opinionated for now.
pub fn extract_query_fragments(url : Url) -> IndexMap<String, String> {
    let mut query: IndexMap<String, String> = IndexMap::new();
    if let Some(hash) = url.hash() {
        let key_value: Vec<&str> = hash.split('&').collect();
        
        for pair in key_value {
            let mut sub = pair.split('=');
            let key = sub.next().unwrap_or_else(|| {
                panic!(
                    "we should have a key for the parameter key but got {}",
                    hash
                )
            });
            let value = sub.next().unwrap_or_else(|| {
                panic!("we should have a value for the key but got {}", hash)
            });
            query.insert(key.to_string(), value.to_string());
            
            
        }
    }
    query
}

