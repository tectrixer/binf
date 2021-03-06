pub trait BfStorageSimu {
    fn right(&mut self);
    fn left(&mut self);
    fn add(&mut self);
    fn sub(&mut self);
    fn get(&mut self) -> u8;
    fn set(&mut self, value: u8);
}

const ARR_SIZE: usize = 30000;

pub struct BfArrayImplementation {
    index: usize,
    storage: [u8; ARR_SIZE],
}

impl BfArrayImplementation {
    pub fn new() -> BfArrayImplementation {
        BfArrayImplementation {
            index: 0,
            storage: [0; ARR_SIZE],
        }
    }
}

impl BfStorageSimu for BfArrayImplementation {
    fn right(&mut self) {
        self.index = (self.index + 1) % ARR_SIZE;
    }
    fn left(&mut self) {
        if self.index == 0 {
            self.index = ARR_SIZE - 1;
        } else {
            self.index -= 1;
        }
    }
    fn add(&mut self) {
        self.storage[self.index] += 1;
    }
    fn sub(&mut self) {
        self.storage[self.index] -= 1;
    }
    fn get(&mut self) -> u8 {
        self.storage[self.index]
    }
    fn set(&mut self, value: u8) {
        self.storage[self.index] = value;
    }
}
