pub mod http;
pub mod logger;
pub mod server;
pub mod service;
pub mod utils;

pub use ::uuid::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
