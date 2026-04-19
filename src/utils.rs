use std::{sync::{Arc, Mutex}, thread, time::Duration};
use super::Chip8;

pub struct Hertz {
    period: Duration,
}

impl Hertz {
    pub fn new(freq: u64) -> Self {
        Self { period: Duration::from_millis(1/freq * 1000)}
    }

    pub fn period(&self) -> Duration {
        self.period
    }

    pub fn set_frequency(&mut self, freq: u64) {
        self.period = Duration::from_millis(1000/freq);
    }
}

pub(crate) struct Timer {
    pub(crate) value: Arc<Mutex<u8>>,
}

impl Timer {

    pub fn new() -> Self {
        let mut timer = Timer { value: Arc::new(Mutex::new(0)) };
        timer.start_timer_thread();
        timer
    }

    pub fn activate(&mut self, value: u8) {
        *self.value.lock().unwrap() = value;
    }

    fn start_timer_thread(&mut self) {
        let value = Arc::clone(&self.value);
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(16));
                let mut v = value.lock().unwrap();
                if *v > 0 {
                    *v -= 1;
                }
            }
        });
    }
}

impl Chip8 {
    pub fn set_clock(&mut self, freq: u64) {
        self.clock_rate.set_frequency(freq);
    }

    pub(crate) fn log_state(&self) {
        println!();
        println!("PC: {:#06X}", self.program_counter);
        println!("V:  {:?}", self.register); // your registers array
        println!("I:  {:#06X}", self.index_register);
        println!("SP: {}", self.stack_pointer);
        println!("KEYPAD: {:?}", self.keypad);
        println!();
    }
}
