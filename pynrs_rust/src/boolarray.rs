#[derive(Debug)]
pub struct BoolArray {
    size: u64,
    ints: Vec<usize>,
}

/// An Array of boolean values which are stored usize integers for saving space.
impl BoolArray {
    const BITS64: u64 = usize::BITS as u64;

    pub fn new(init_value: bool, size: u64) -> Self {
        let array_size = (size + Self::BITS64 - 1) / Self::BITS64;
        let mut array_init_value = 0;
        if init_value {
            array_init_value = usize::MAX;
        }
        BoolArray {
            size: size,
            ints: vec![array_init_value; array_size.try_into().unwrap()],
        }
    }

    pub fn len(self: &Self) -> u64 {
        self.size
    }

    pub fn set(self: &mut Self, pos: u64, value: bool) {
        let (array_pos, int_pos) = self.positions(pos);
        let mask: usize = 1 << int_pos;
        self.ints[array_pos] |= mask;
        if !value {
            self.ints[array_pos] ^= mask;
        }
    }

    pub fn get(self: &Self, pos: u64) -> bool {
        let (array_pos, int_pos) = self.positions(pos);
        let mask = 1 << int_pos;
        mask & self.ints[array_pos] == mask
    }

    pub fn iter(&self) -> BoolArrayIterator {
        BoolArrayIterator::new(&self.ints, self.size)
    }

    fn positions(self: &Self, pos: u64) -> (usize, usize) {
        if pos >= self.size {
            panic!("index out of bounds: the len is {} but the index is {}", self.size, pos);
        }
        (
            (pos / Self::BITS64).try_into().unwrap(),
            (pos % Self::BITS64).try_into().unwrap()
        )
    }

}

pub struct BoolArrayIterator<'a> {
    ints: std::slice::Iter<'a, usize>,
    int: usize,
    int_pos: usize,
    size: u64,
}

impl<'a> BoolArrayIterator<'a> {
    fn new(ints: &'a Vec<usize>, size: u64) -> Self {
        BoolArrayIterator {
            ints: ints.iter(),
            int: 0,
            int_pos: usize::BITS as usize,
            size: size,
        }
    }    
}

impl Iterator for BoolArrayIterator<'_> {
    type Item = bool;
    
    fn next(&mut self) -> Option<bool> {
        if self.size == 0 {
            return None;
        }
        if self.int_pos == usize::BITS as usize {
            match self.ints.next() {
                None => {
                    panic!("iterator exhausted before time");
                },
                Some(int) => {
                    self.int = int.clone();
                    self.int_pos = 0;
                },
            }
        }
        let mask = 1 << self.int_pos;
        self.int_pos += 1;
        self.size -= 1;
        Some(self.int & mask == mask)
    }
}
