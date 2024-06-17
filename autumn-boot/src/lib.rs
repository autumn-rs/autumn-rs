use config::init_config;
use log::init_log;

mod config;
mod log;

pub struct App;

impl App {
    pub fn new() -> Self {
        init_log();
        init_config();
        App {}
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}