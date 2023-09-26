use std::sync::Arc;
use std::thread;

use crate::philosopher::Philosopher;
use crate::stick::Stick;

pub struct Problem {
    size: usize,
}

impl Problem {
    pub fn new(size: usize) -> Self {
        Self { size }
    }

    pub fn start(self) {
        let forks = (0..self.size)
            .map(|_| Arc::new(Stick::new(1)))
            .collect::<Vec<_>>();

        let mut threads = Vec::with_capacity(self.size);

        for id in 0..self.size {
            let left_stick_id = wrapping_decrement(id, self.size - 1);
            let right_stick_id = id;

            let left_stick = forks[left_stick_id].clone();
            let right_stick = forks[right_stick_id].clone();

            let philosopher = Philosopher::new(id, left_stick, right_stick);

            let thread = thread::spawn(move || philosopher.work());
            threads.push(thread);
        }

        for thread in threads {
            thread.join().unwrap();
        }
    }
}

fn wrapping_decrement(number: usize, max: usize) -> usize {
    if number == 0 {
        return max;
    }

    return number - 1;
}
