use std::collections::{BTreeMap, HashMap};

fn print_repr(map: BTreeMap<i32, i32>) {
	for (k, v) in map {
		if v == 1 {
			print!("{}, ", k);
		} else {
			print!("{}^{}, ", k, v);
		}
	}
	println!("");
}

fn repr(num: i32, last: i32, mut map: BTreeMap<i32, i32>) {
	if last == 0 {
		if num == 0 {
			print_repr(map);
		}
	} else if last == 1 {
		if num == 0 {
			print_repr(map);
		} else if num == 1 {
			map.insert(1, 1);
			repr(num - 1, 0, map);
		}
	} else {
		let mut pow = 1;
		let mut val = last;
		while num - val >= 0 {
			let mut cmap = map.clone();
			cmap.insert(last, pow);
			repr(num - val, last - 1, cmap);
			pow += 1;
			val *= last;
		}
		repr(num, last - 1, map);
	}
}

fn count(num: i32, last: i32) -> i32 {
	if last == 0 {
		if num == 0 {
			1
		} else {
			0
		}
	} else if last == 1 {
		if num == 0 || num == 1 {
			1
		} else {
			0
		}
	} else {
		let mut sum = count(num, last - 1);
		let mut val = last;
		while num - val >= 0 {
			sum += count(num - val, last - 1);
			val *= last;
		}
		sum
	}
}

fn count_mem(num: i32, last: i32, mem: &mut HashMap<(i32, i32), i32>) -> i32 {
	if last == 0 {
		if num == 0 {
			1
		} else {
			0
		}
	} else if last == 1 {
		if num == 0 || num == 1 {
			1
		} else {
			0
		}
	} else {
		{
			let opt = mem.get(&(num, last));
			if opt.is_some() {
				return *opt.unwrap()
			}
		}

		let mut sum = count_mem(num, last - 1, mem);
		let mut val = last;
		while num - val >= 0 {
			sum += count_mem(num - val, last - 1, mem);
			val *= last;
		}
		mem.insert((num, last), sum);
		sum
	}
}

fn main() {
	let n = 15;

	let mut map = BTreeMap::<i32, i32>::new();
	repr(n, n, map);

	let mut mem = HashMap::<(i32, i32), i32>::new();
	println!("total: {}", count_mem(n, n, &mut mem));

	// println!("total: {}", count(n, n));
}