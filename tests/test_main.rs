#[cfg(test)]
mod tests {

    fn average(numbers: &[i32]) -> f64 {
        let sum: i32 = numbers.iter().sum();
        sum as f64 / numbers.len() as f64
    }

    #[test]
    fn test_average() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = average(&numbers);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_average_empty() {
        let numbers: Vec<i32> = vec![];
        let result = average(&numbers);
        assert!(result.is_nan());
    }

    #[test]
    fn test_average_single() {
        let numbers = vec![10];
        let result = average(&numbers);
        assert_eq!(result, 10.0);
    }
}