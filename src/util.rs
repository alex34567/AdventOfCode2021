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

pub fn two_d_adjacent_enumerated_iter<'a, T: 'a, S: ops::Deref<Target = [T]>>(
    list: &'a [S],
    x: usize,
    y: usize,
) -> impl Iterator<Item = (usize, usize, &T)> { // (x, y, Item)
    y.checked_sub(1)
        .map(|y| (x, y, &list[y][x]))
        .into_iter()
        .chain(list[y].get(x + 1).map(|item| (x + 1, y, item)))
        .chain(list.get(y + 1).map(|line| (x, y + 1, &line[x])))
        .chain(x.checked_sub(1).map(|x| (x, y, &list[y][x])))
}

pub fn two_d_adjacent_iter<'a, T: 'a, S: ops::Deref<Target = [T]>>(
    list: &'a [S],
    x: usize,
    y: usize,
) -> impl Iterator<Item = &T> {
    two_d_adjacent_enumerated_iter(list, x, y).map(|adj| adj.2)
}

pub fn two_d_adjacent_enumerated_iter_mut<'a, T: 'a, S: ops::DerefMut<Target = [T]>>(
    list: &'a mut [S],
    x: usize,
    y: usize,
) -> impl Iterator<Item = (usize, usize, &mut T)> { // (x, y, Item)
    let (lines_before, lines_cur_after) = list.split_at_mut(y);
    let (curr_lines, lines_after) = lines_cur_after.split_at_mut(1);
    let curr_line = &mut curr_lines[0];

    let (items_before, items_cur_after) = curr_line.split_at_mut(x);
    y.checked_sub(1)
        .map(|y| (x, y, &mut lines_before[y][x]))
        .into_iter()
        .chain(items_cur_after.get_mut(1).map(|item| (x + 1, y, item)))
        .chain(lines_after.get_mut(0).map(|line| (x, y + 1, &mut line[x])))
        .chain(x.checked_sub(1).map(|x| (x, y, &mut items_before[x])))
}

pub fn _two_d_adjacent_iter_mut<'a, T: 'a, S: ops::DerefMut<Target = [T]>>(
    list: &'a mut [S],
    x: usize,
    y: usize,
) -> impl Iterator<Item = &mut T> {
    two_d_adjacent_enumerated_iter_mut(list, x, y).map(|adj| adj.2)
}
