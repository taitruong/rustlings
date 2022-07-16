// tests1.rs
// Tests are important to ensure that your code does what you think it should do.
// Tests can be run on this file with the following command:
// rustlings run tests1

// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests1` for hints :)


#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        assert!(2 == 1 + 1, "optional error message in case of fail");
        // below won't compile, hence expecting boolean arguments, subsequent arguments are ignored
        // assert!(true);
    }

    #[test]
    #[should_panic]
    fn it_panics() {
        assert!(2 != 1 +1, "message ignored because it panics before");
    }
}
