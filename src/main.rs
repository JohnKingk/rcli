fn main() {
    println!("Hello, rust!");
}

mod tests {
    #[test]
    fn test_hello() {
        assert_eq!(2 + 2, 4);
    }
}
