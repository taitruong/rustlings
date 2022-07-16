// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)


#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(2, 2, "values are not equal!");
        assert_eq!(2, 1 + 1, "values are not equal!");

        // fail example
        // assert_eq!(2, 1, "values are not equal!");

        // compile error example due to different types
        // assert_eq!(2, "2", "values are not equal!");
        // assert_eq!(2, 2.0, "values are not equal!");
    }
}
