impl SplitKvExt for &str {
    /// Splits a `&str` into a `(key, value)`
    /// ```rust
    ///# use crate::http_ez::utils::str_ext::SplitKvExt;
    /// let str = "key;value";
    /// assert_eq!(str.split_kv(';', ""), Some(("key", "value")));
    ///
    /// //if no value is present the default_value will be used.
    /// let str = "key;";
    /// assert_eq!(str.split_kv(';', ""), Some(("key", "")));
    /// let str = "key";
    /// assert_eq!(str.split_kv(';', ""), Some(("key", "")));
    ///
    /// //if no key is present None vill be returned.
    /// let str = ";value";
    /// assert_eq!(str.split_kv(';', ""), None);
    ///
    /// //if there is more than two values after split None is returned
    /// let str = "key;value1;value2";
    /// assert_eq!(str.split_kv(';', ""), None);
    /// ````
    fn split_kv<'a>(&'a self, delimiter: char, default_value: &'a str) -> Option<(&str, &str)> {
        match self.split(delimiter).collect::<Vec<&'a str>>().as_slice() {
            [k] => Some((k, default_value)),
            [k, v] if k != &"" => Some((k, v)),
            _ => None,
        }
    }
}

pub trait SplitKvExt {
    fn split_kv<'a>(&'a self, delimiter: char, default_value: &'a str) -> Option<(&str, &str)>;
}
