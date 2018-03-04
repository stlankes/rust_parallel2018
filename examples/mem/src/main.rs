use std::sync::Arc;

fn test1() {
	let x;

	{
		let z = vec!("Hello para//el 2018!");

		x = z.clone();
		//x = &z;
	}

	println!("{}", x[0]);
}

fn test2() {
	let mut v = vec!("Hello para//el 2018!");

	{
		let x = &mut v;
		// Do something with x...
	}

	println!("{}", v[0]);
}

fn main()
{
	test1();
	test2();
}
