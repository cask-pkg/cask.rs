#![deny(warnings)]

pub fn get_iso8601() -> String {
    format!("{:?}", chrono::offset::Local::now())
}

#[cfg(test)]
mod tests {
    use crate::util;

    #[test]
    fn test_to_iso8601() {
        let result = util::get_iso8601();

        println!("{}", result)
    }
}
