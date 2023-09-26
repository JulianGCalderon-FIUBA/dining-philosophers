use std::sync::Arc;

use crate::stick::Stick;

pub struct Philosopher {
    id: usize,
    left_stick: Arc<Stick>,
    right_stick: Arc<Stick>,
}

impl Philosopher {
    pub fn new(id: usize, left_stick: Arc<Stick>, right_stick: Arc<Stick>) -> Self {
        Self {
            id,
            left_stick,
            right_stick,
        }
    }

    pub fn work(self) {
        loop {
            if self.right_handed() {
                self.say("Waiting for right stick.");
                self.right_stick.acquire();

                self.say("Got right stick. Waiting for left stick.");
                self.left_stick.acquire();
            } else {
                self.say("Waiting for left stick.");
                self.left_stick.acquire();

                self.say("Got left stick. Waiting for right stick.");
                self.right_stick.acquire();
            }

            self.say("Dining");
            self.dine();

            self.say("Releasing sticks.");
            self.left_stick.release();
            self.right_stick.release();

            self.say("Thinking");
            self.think();
        }
    }

    fn right_handed(&self) -> bool {
        self.id == 0
    }

    fn say(&self, phrase: &str) {
        println!("Philosopher {}: {}", self.id, phrase);
    }

    fn dine(&self) {
        // thread::sleep(Duration::from_millis(1000));
    }

    fn think(&self) {
        // thread::sleep(Duration::from_millis(1000));
    }
}
