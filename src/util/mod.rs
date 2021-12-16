use std::fmt::Display;
use std::marker::PhantomData;
use std::ops;

pub fn list_of_integers_radix(input: &str, radix: u32) -> impl Iterator<Item = i32> + '_ {
    input.split('\n').filter_map(move |line| {
        let trimed_line = line.trim();
        i32::from_str_radix(trimed_line, radix).ok()
    })
}

pub fn list_of_integers(input: &str) -> impl Iterator<Item = i32> + '_ {
    list_of_integers_radix(input, 10)
}

struct AdjEnuIter<'a: 'b, 'b, T: 'a, O, OReader: FnMut(&O) -> isize> {
    list: &'a [T],
    x: isize,
    offsets: &'b [O],
    offset_reader: OReader,
}

impl<'a: 'b, 'b, T: 'a, O, OReader: FnMut(&O) -> isize> Iterator
    for AdjEnuIter<'a, 'b, T, O, OReader>
{
    type Item = (usize, &'a T, &'b O);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.offsets.is_empty() {
                return None;
            }
            let raw_offset = &self.offsets[0];
            let offset = (self.offset_reader)(raw_offset);
            self.offsets = &self.offsets[1..];

            let index =
                if let Some(idx) = self.x.checked_add(offset).and_then(|x| x.try_into().ok()) {
                    idx
                } else {
                    continue;
                };

            let item = if let Some(it) = self.list.get(index) {
                it
            } else {
                continue;
            };

            return Some((index, item, raw_offset));
        }
    }
}

pub fn adjacent_enumerated_iter<'a: 'b, 'b, T: 'a, O>(
    list: &'a [T],
    x: usize,
    offsets: &'b [O],
    offset_reader: impl FnMut(&O) -> isize + 'b,
) -> impl Iterator<Item = (usize, &'a T, &'b O)> + 'b {
    AdjEnuIter {
        list,
        x: x.try_into().unwrap(),
        offsets,
        offset_reader,
    }
}

pub fn two_d_adjacent_enumerated_iter<
    'a: 'b,
    'b,
    T: 'a,
    S: ops::Deref<Target = [T]>,
    O: ops::Deref<Target = [isize]>,
>(
    list: &'a [S],
    x: usize,
    y: usize,
    offsets: &'b [(O, isize)],
) -> impl Iterator<Item = (usize, usize, &'a T)> + 'b {
    // (x, y, Item)
    let outer_iter = adjacent_enumerated_iter(list, y, offsets, |(_, y)| *y);
    outer_iter.flat_map(move |(y, line, (x_offsets, _))| {
        adjacent_enumerated_iter(line, x, x_offsets, |x| *x).map(move |(x, item, _)| (x, y, item))
    })
}

pub fn two_d_straight_adjacent_enumerated_iter<'a: 'b, 'b, T: 'a, S: ops::Deref<Target = [T]>>(
    list: &'a [S],
    x: usize,
    y: usize,
) -> impl Iterator<Item = (usize, usize, &'a T)> + 'b {
    let offsets: &'static [(&'static [isize], isize)] = &[(&[0], -1), (&[-1, 1], 0), (&[0], 1)];
    two_d_adjacent_enumerated_iter(list, x, y, offsets)
}

pub fn two_d_straight_adjacent_iter<'a, T: 'a, S: ops::Deref<Target = [T]>>(
    list: &'a [S],
    x: usize,
    y: usize,
) -> impl Iterator<Item = &T> {
    two_d_straight_adjacent_enumerated_iter(list, x, y).map(|adj| adj.2)
}

//SAFETY: Offsets cannot contain dupes
struct AdjEnuIterMut<'a: 'b, 'b, T: 'a, O, OReader: FnMut(&O) -> isize> {
    list: *mut [T],
    list_phantom: PhantomData<&'a mut [T]>,
    x: isize,
    offsets: &'b [O],
    offset_reader: OReader,
}

impl<'a: 'b, 'b, T: 'a, O, OReader: FnMut(&O) -> isize> Iterator
    for AdjEnuIterMut<'a, 'b, T, O, OReader>
{
    type Item = (usize, &'a mut T, &'b O);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.offsets.is_empty() {
                return None;
            }
            let raw_offset = &self.offsets[0];
            let offset = (self.offset_reader)(raw_offset);
            self.offsets = &self.offsets[1..];
            //SAFETY: Does not create a double reference because () is size zero.
            let bound_list = unsafe { &*(self.list as *mut [()]) };

            let index =
                if let Some(idx) = self.x.checked_add(offset).and_then(|x| x.try_into().ok()) {
                    idx
                } else {
                    continue;
                };

            if bound_list.get(index).is_none() {
                continue;
            }

            //SAFETY: If the offsets have no dupes than the mut references cannot overlap
            let item = unsafe { &mut *(self.list as *mut T).add(index) };

            return Some((index, item, raw_offset));
        }
    }
}

//SAFETY: Offsets cannot contain dupes
pub unsafe fn adjacent_enumerated_iter_mut<'a: 'b, 'b, T: 'a, O>(
    list: &'a mut [T],
    x: usize,
    offsets: &'b [O],
    offset_reader: impl FnMut(&O) -> isize + 'b,
) -> impl Iterator<Item = (usize, &'a mut T, &'b O)> + 'b {
    AdjEnuIterMut {
        list,
        list_phantom: PhantomData,
        x: x.try_into().unwrap(),
        offsets,
        offset_reader,
    }
}

//SAFETY: Offsets cannot contain dupes
pub unsafe fn two_d_adjacent_enumerated_iter_mut<
    'a: 'b,
    'b,
    T: 'a,
    S: ops::DerefMut<Target = [T]>,
    O: ops::Deref<Target = [isize]>,
>(
    list: &'a mut [S],
    x: usize,
    y: usize,
    offsets: &'b [(O, isize)],
) -> impl Iterator<Item = (usize, usize, &'a mut T)> + 'b {
    // (x, y, Item)
    let outer_iter = unsafe { adjacent_enumerated_iter_mut(list, y, offsets, |(_, y)| *y) };
    outer_iter.flat_map(move |(y, line, (x_offsets, _))| unsafe {
        adjacent_enumerated_iter_mut(line, x, x_offsets, |x| *x)
            .map(move |(x, item, _)| (x, y, item))
    })
}

pub fn two_d_straight_adjacent_enumerated_iter_mut<
    'a: 'b,
    'b,
    T: 'a,
    S: ops::DerefMut<Target = [T]>,
>(
    list: &'a mut [S],
    x: usize,
    y: usize,
) -> impl Iterator<Item = (usize, usize, &'a mut T)> + 'b {
    let offsets: &'static [(&'static [isize], isize)] = &[(&[0], -1), (&[-1, 1], 0), (&[0], 1)];
    // Safety offsets do not overlap
    unsafe { two_d_adjacent_enumerated_iter_mut(list, x, y, offsets) }
}

pub fn two_d_diagonal_adjacent_enumerated_iter_mut<
    'a: 'b,
    'b,
    T: 'a,
    S: ops::DerefMut<Target = [T]>,
>(
    list: &'a mut [S],
    x: usize,
    y: usize,
) -> impl Iterator<Item = (usize, usize, &'a mut T)> + 'b {
    let offsets: &'static [(&'static [isize], isize)] =
        &[(&[-1, 0, 1], -1), (&[-1, 1], 0), (&[-1, 0, 1], 1)];
    // Safety offsets do not overlap
    unsafe { two_d_adjacent_enumerated_iter_mut(list, x, y, offsets) }
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
