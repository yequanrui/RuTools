pub mod common;
pub mod console;
pub mod i18n;
pub mod reqwest;
pub mod winapi;
pub mod winreg;

pub fn add(left: u64, right: u64) -> u64 {
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
