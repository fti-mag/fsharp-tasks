use std::env;
use std::collections::HashMap;

fn fibm(mem: &mut HashMap<i32, i32>, n: i32) -> i32 {
	if n < 2 {
		n
	} else {
		{
			let opt = mem.get(&n);
			if opt.is_some() {
				return *opt.unwrap()
			}
		}

		let res: i32 = (fibm(mem, n - 1) + fibm(mem, n - 2)) % 100000007;
		mem.insert(n, res);
		res
	}
}

fn main() {
	let mut mem = HashMap::<i32, i32>::new();
	let args: Vec<_> = env::args().collect();
	for arg in &args[1..] {
		let n: i32 = arg.parse::<i32>().unwrap();
		println!("fib({}) = {}", n, fibm(&mut mem, n));
	}
}