pub fn main() {
    println!("Hello World!");
}


#[cfg(test)]
mod tests {
    #[test]
    fn canary_test() {
        assert_eq!(true, true);
    }
}
