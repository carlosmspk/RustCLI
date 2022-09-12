use rust_cli::screens;

#[cfg(test)]
mod add_tests {
    use rust_cli::add;

    #[test]
    fn simple() {
        assert_eq!(add(1,2), 3)
    }
}
