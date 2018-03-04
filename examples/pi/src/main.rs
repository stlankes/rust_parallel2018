extern crate rayon;

use std::sync::mpsc;
use std::thread;
use std::time::Instant;
use std::sync::{Mutex,Arc};
use rayon::prelude::*;
//use rayon;

const NUM_STEPS: u64 = 100000000;

fn pi_seq()
{
	let step = 1.0 / NUM_STEPS as f64;
	let mut sum = 0.0;
	let now = Instant::now();

	for i  in 0..NUM_STEPS {
        let x = (i as f64 + 0.5) * step;
        sum += 4.0 / (1.0 + x * x);
    }

	let duration = now.elapsed();

	println!("Time to calculate (serial): {}", duration.as_secs() as f64
			+ (duration.subsec_nanos() as f64 / 1000000000.0));
	println!("Pi: {}", sum * (1.0 / NUM_STEPS as f64));
}

fn pi_naive(nthreads: u64)
{
	let step = 1.0 / NUM_STEPS as f64;
	let sum = Arc::new(Mutex::new(0.0 as f64));
	let now = Instant::now();

	let threads: Vec<_> = (0..nthreads)
        .map(|tid| {
			let sum = sum.clone();

			thread::spawn(move || {
				let start = (NUM_STEPS / nthreads) * tid;
				let end = (NUM_STEPS / nthreads) * (tid+1);

				for i  in start..end {
			        let x = (i as f64 + 0.5) * step;
					let partial_sum = 4.0 / (1.0 + x * x);
			        *sum.lock().unwrap() += partial_sum
			    }
			})
		}).collect();

	for t in threads {
        t.join().unwrap();
    }

	let duration = now.elapsed();

	println!("Time to calculate (naive): {}", duration.as_secs() as f64
			+ (duration.subsec_nanos() as f64 / 1000000000.0));
	println!("Pi: {}", *(sum.lock().unwrap()) * (1.0 / NUM_STEPS as f64));
}

fn pi_local_sum(nthreads: u64)
{
	let step = 1.0 / NUM_STEPS as f64;
	let mut sum = 0.0 as f64;
	let now = Instant::now();

	let threads: Vec<_> = (0..nthreads)
        .map(|tid| {
			thread::spawn(move || {
				let mut partial_sum = 0 as f64;
				let start = (NUM_STEPS / nthreads) * tid;
				let end = (NUM_STEPS / nthreads) * (tid+1);

				for i  in start..end {
			        let x = (i as f64 + 0.5) * step;
					partial_sum += 4.0 / (1.0 + x * x);
			    }

				partial_sum
			})
		}).collect();

	for t in threads {
		sum += t.join().unwrap();
    }

	let duration = now.elapsed();

	println!("Time to calculate (local sum): {}", duration.as_secs() as f64
			+ (duration.subsec_nanos() as f64 / 1000000000.0));
	println!("Pi: {}", sum * (1.0 / NUM_STEPS as f64));
}

fn term(start: u64, end: u64) -> f64
{
	let step = 1.0 / NUM_STEPS as f64;
	let mut sum = 0.0;

	for i  in start..end {
        let x = (i as f64 + 0.5) * step;
        sum += 4.0 / (1.0 + x * x);
    }

	sum
}

fn pi_channel(nthreads: u64)
{
	let (tx, rx) = mpsc::channel();
	let mut sum = 0.0;
	let now = Instant::now();

	for id in 0..nthreads {
		// The sender endpoint can be copied
        let thread_tx = tx.clone();
		let start = (NUM_STEPS / nthreads as u64) * id;
		let end = (NUM_STEPS / nthreads as u64) * (id+1);

		// Each thread will send its partial sum via the channel
        thread::spawn(move || {
			let partial_sum = term(start, end);
			thread_tx.send(partial_sum).unwrap();
		});
	};

    for _ in 0..nthreads {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there no messages available
        sum = sum + rx.recv().unwrap();
    }

	let duration = now.elapsed();

	println!("Time to calculate (channel): {}", duration.as_secs() as f64
			+ (duration.subsec_nanos() as f64 / 1000000000.0));
	println!("Pi: {}", sum * (1.0 / NUM_STEPS as f64));
}

fn pi_iter()
{
	let step = 1.0 / NUM_STEPS as f64;
	let now = Instant::now();

	let sum: f64 = (0..NUM_STEPS).into_iter()
		.map(|i| {
			let x = (i as f64 + 0.5) * step;
			4.0 / (1.0 + x * x)
		}).sum();

	let duration = now.elapsed();

	println!("Time to calculate (serial iterator): {}", duration.as_secs() as f64
			+ (duration.subsec_nanos() as f64 / 1000000000.0));
	println!("Pi: {}", sum * (1.0 / NUM_STEPS as f64));
}

fn pi_rayon()
{
	let step = 1.0 / NUM_STEPS as f64;
	let now = Instant::now();

	let sum: f64 = (0..NUM_STEPS).into_par_iter()
		.map(|i| {
			let x = (i as f64 + 0.5) * step;
			4.0 / (1.0 + x * x)
		}).sum();

	let duration = now.elapsed();

	println!("Time to calculate (parallel iterator): {}", duration.as_secs() as f64
			+ (duration.subsec_nanos() as f64 / 1000000000.0));
	println!("Pi: {}", sum * (1.0 / NUM_STEPS as f64));
}

fn main() {
	println!("Demo to explain concurrent programming with Rust!");

	pi_seq();
	// do not use the naive implementation, it takes too much time
	//pi_naive(2);
	pi_local_sum(2);
	pi_channel(2);
	pi_iter();
	rayon::initialize(rayon::Configuration::new().num_threads(2)).unwrap();
	pi_rayon();
}
