use super::super::scanner::Scanner;

fn main() {
	let mut scanner = Scanner::default();

    // Read a Vec with n elements.
	let n = scanner.next::<usize>();
	let a: Vec<i64> = (0..n).map(|_| scanner.next()).collect();

    // Calculate prefix sum of a vector: prefix[i] = sum(a[0]..a[i])
	let prefix_sum = a.iter().scan(0, |sum, &x| {
		*sum += x;
		Some(*sum)
	}).collect::<Vec<_>>();
}

