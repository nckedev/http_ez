use std::collections::HashMap;

#[derive(Debug)]
pub struct Path<'a> {
    full_path: &'a str,
    resource_path: &'a str,
    params_path: Option<&'a str>,
    params_map: Option<HashMap<&'a str, &'a str>>,
}

impl<'a> Path<'a> {
    pub fn new(full_path: &'a str) -> Self {
        let (resource_path, params_path) = parse_path(full_path);
        let params_map = parse_params(params_path);
        println!("{resource_path}");

        Path {
            full_path,
            resource_path,
            params_path,
            params_map,
        }
    }
}

fn parse_path(full_path: &str) -> (&str, Option<&str>) {
    match full_path.split('?').collect::<Vec<&str>>()[..] {
        [p] => (p, None),
        [p, par] => (p, Some(par)),
        _ => ("", None),
    }
}

fn parse_params(params: Option<&str>) -> Option<HashMap<&str, &str>> {
    let Some(params) = params else {
        return None;
    };

    let mut map = HashMap::new();
    for splitted_params in params.split('&').collect::<Vec<&str>>() {
        let (k, v) = match splitted_params.split('=').collect::<Vec<&str>>().as_slice() {
            [k] => (*k, ""),
            [k, v] => (*k, *v),
            _ => continue,
        };
        map.insert(k, v);
        // if let Some((k, v)) = splitted_params.split_once('=') {
        //     map.insert(k, v);
        // } // else bad request ?
    }
    match map.len() {
        0 => None,
        _ => Some(map),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_path_without_params() {
        let request_path_raw = "/path/to/re";
        let request_path = Path::new(request_path_raw);

        assert_eq!(request_path.resource_path, request_path_raw);
        assert_eq!(request_path.full_path, request_path_raw);
        assert_eq!(request_path.params_path, None);
    }
    #[test]
    fn request_path_with_params() {
        let request_path_raw = "/path/to/re?param1=value1&param2=value2";
        let request_path = Path::new(request_path_raw);

        assert_eq!(request_path.resource_path, "/path/to/re");
        assert_eq!(request_path.full_path, request_path_raw);
        assert_eq!(
            request_path.params_path,
            Some("param1=value1&param2=value2")
        );
        assert_eq!(request_path.params_map.as_ref().unwrap().len(), 2);
        assert_eq!(
            request_path.params_map.as_ref().unwrap().get("param1"),
            Some(&"value1")
        );
        assert_eq!(
            request_path.params_map.as_ref().unwrap().get("param2"),
            Some(&"value2")
        );
    }
    #[test]
    fn request_path_with_param_without_value() {
        let request_path_raw = "/path/to/re?param1";
        let request_path = Path::new(request_path_raw);

        assert_eq!(request_path.resource_path, "/path/to/re");
        assert_eq!(request_path.full_path, request_path_raw);
        assert_eq!(request_path.params_path, Some("param1"));
        assert_eq!(request_path.params_map.as_ref().unwrap().len(), 1);
        assert_eq!(
            request_path.params_map.as_ref().unwrap().get("param1"),
            Some(&"")
        );
    }
}
