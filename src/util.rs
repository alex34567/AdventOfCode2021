pub fn list_of_integers_radix(input: &str, radix: u32) -> impl Iterator<Item = i32> + '_ {
    input.split('\n').filter_map(move |line| {
        let trimed_line = line.trim();
        i32::from_str_radix(trimed_line, radix).ok()
    })
}

pub fn list_of_integers(input: &str) -> impl Iterator<Item = i32> + '_ {
    list_of_integers_radix(input, 10)
}
