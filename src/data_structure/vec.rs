use super::super::scanner::Scanner;

/// Read n space-separated i64 into a Vec.
fn read_vec<T: std::str::FromStr>(scanner: &mut Scanner, n: usize) -> Vec<T> {
    (0..n).map(|_| scanner.next::<T>()).collect()
}

/// Returns prefix sum of a Vec.
fn prefix_sum(a: &Vec<i64>) -> Vec<i64> {
	return a.iter().scan(0, |sum, &x| {
		*sum += x;
		Some(*sum)
	}).collect::<Vec<_>>();
}

#[cfg(test)]
mod test {
    use super::*;

	#[test]
	fn test_prefix_sum() {
		assert_eq!(prefix_sum(&vec![]), vec![]);
		assert_eq!(prefix_sum(&vec![10, 20, 30]), vec![10, 30, 60]);
		assert_eq!(prefix_sum(&vec![10, -20, 20]), vec![10, -10, 10]);
	}
}
