fn silly_function() -> i32 {
    42
}

fn main() {
    println!("Hello, ci/cd!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_silly_function() {
        assert_eq!(silly_function(), 42);
    }
}
