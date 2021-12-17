use std::fmt::Display;
use std::iter;
use std::ops;

pub mod adjacent_iter;

pub fn list_of_integers_radix(input: &str, radix: u32) -> impl Iterator<Item = i32> + '_ {
    input.split('\n').filter_map(move |line| {
        let trimed_line = line.trim();
        i32::from_str_radix(trimed_line, radix).ok()
    })
}

pub fn list_of_integers(input: &str) -> impl Iterator<Item = i32> + '_ {
    list_of_integers_radix(input, 10)
}

pub fn grid_parse<T>(input: &str, mut num_to_entry: impl FnMut(u8) -> T) -> Vec<Vec<T>> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| {
                    let mut tmp = [0; 4];
                    let num = c.encode_utf8(&mut tmp).parse().unwrap();
                    num_to_entry(num)
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>()
}

pub fn print_grid<T, L: ops::Deref<Target = [T]>, D: Display>(
    grid: &[L],
    mut display_fn: impl FnMut(&T) -> D,
) {
    for line in grid.iter() {
        for column in line.iter() {
            print!("{}", display_fn(column))
        }
        println!();
    }
}

#[allow(dead_code)]
pub fn dbg_print_grid<T, L: ops::Deref<Target = [T]>, D: Display>(
    grid: &[L],
    mut display_fn: impl FnMut(&T) -> D,
) {
    for line in grid.iter() {
        for column in line.iter() {
            eprint!("{}", display_fn(column))
        }
        eprintln!();
    }
}

pub struct BitIter<T: Iterator<Item = u8>> {
    front_byte: u8,
    front_byte_left: u8,
    back_byte: u8,
    back_byte_left: u8,
    iter: T,
}

macro_rules! take_into_impl {
    ( $name: ident, $type: ty) => {
        pub fn $name(&mut self, n: u8) -> Result<$type, usize> {
            assert!(n as u32 <= <$type>::BITS);

            let mut ret = 0;
            for i in 0..n {
                if let Some(bit) = self.next() {
                    ret <<= 1;
                    ret |= bit as $type;
                } else {
                    return Err(i as usize);
                }
            }
            Ok(ret)
        }
    };
}

impl<T: Iterator<Item = u8>> BitIter<T> {
    pub fn new<I: IntoIterator<IntoIter = T>>(iter: I) -> Self {
        Self {
            front_byte: 0,
            front_byte_left: 0,
            back_byte: 0,
            back_byte_left: 0,
            iter: iter.into_iter(),
        }
    }

    take_into_impl!(take_into_u8, u8);
    take_into_impl!(take_into_u16, u16);
    take_into_impl!(take_into_u32, u32);
    take_into_impl!(take_into_u64, u64);
}

impl<T: Iterator<Item = u8>> Iterator for BitIter<T> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.front_byte_left == 0 {
            if let Some(byte) = self.iter.next() {
                self.front_byte_left = 8;
                self.front_byte = byte;
            } else if self.back_byte_left != 0 {
                let bit = 1 << (self.back_byte_left - 1);
                let ret = self.back_byte & bit != 0;
                self.back_byte_left -= 1;
                return Some(ret);
            } else {
                return None;
            }
        }

        let bit = 1 << (self.front_byte_left - 1);
        let ret = self.front_byte & bit != 0;
        self.front_byte_left -= 1;
        Some(ret)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (mut low, mut high) = self.iter.size_hint();
        low = low.saturating_mul(8);
        low = low.saturating_add((self.front_byte_left + self.back_byte_left).into());
        high = high
            .and_then(|high| high.checked_mul(8))
            .and_then(|high| high.checked_add((self.front_byte_left + self.back_byte_left).into()));
        (low, high)
    }

    fn last(mut self) -> Option<bool> {
        if self.back_byte_left == 0 {
            if let Some(iter_last) = self.iter.last() {
                self.front_byte_left = 8;
                self.front_byte = iter_last;
            }
        } else {
            self.front_byte_left = 0;
        }

        let new_iter = BitIter {
            front_byte: self.front_byte,
            front_byte_left: self.front_byte_left,
            back_byte: self.back_byte,
            back_byte_left: self.back_byte_left,
            iter: iter::empty(),
        };

        let mut last = None;
        for item in new_iter {
            last = Some(item)
        }
        last
    }
}

impl<T: DoubleEndedIterator + Iterator<Item = u8>> DoubleEndedIterator for BitIter<T> {
    fn next_back(&mut self) -> Option<bool> {
        if self.back_byte_left == 0 {
            if let Some(byte) = self.iter.next_back() {
                self.back_byte_left = 8;
                self.back_byte = byte;
            } else if self.front_byte_left != 0 {
                let ret = self.front_byte & 1 == 1;
                self.front_byte >>= 1;
                self.front_byte_left -= 1;
                return Some(ret);
            } else {
                return None;
            }
        }

        let ret = self.back_byte & 1 == 1;
        self.back_byte >>= 1;
        self.back_byte_left -= 1;
        Some(ret)
    }
}
