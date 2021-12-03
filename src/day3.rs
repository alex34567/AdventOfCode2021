use crate::util::list_of_integers_radix;

static INPUT: &str = include_str!("input/Day3.txt");

fn calc_gamma(nums: &[i32]) -> i32 {
    let mut bit_count = [0; 32];
    for num in nums {
        for (bit, bit_cnt) in bit_count.iter_mut().enumerate() {
            let bit_set = (num & (1 << bit)) != 0;
            *bit_cnt += bit_set as i32;
        }
    }
    let mut gamma = 0;
    for bit in bit_count.iter().rev() {
        gamma <<= 1;
        if *bit * 2 >= nums.len() as i32 {
            gamma |= 1;
        }
    }
    gamma
}

fn calc_bit_size(nums: &[i32]) -> i32 {
    nums.iter()
        .map(|x| (32 - x.leading_zeros()) as i32)
        .max()
        .unwrap_or(0)
}

fn calc_epsilon(nums: &[i32]) -> i32 {
    ((1 << calc_bit_size(nums)) - 1) ^ calc_gamma(nums)
}

fn life_support(in_nums: &[i32], mut gamma_func: impl FnMut(&[i32]) -> i32) -> i32 {
    let mut nums = in_nums.to_vec();
    let mut bit = 1 << (calc_bit_size(&nums) - 1);
    while nums.len() > 1 {
        let gamma = gamma_func(&nums);
        let mut i = 0;
        while i < nums.len() {
            let is_good = (nums[i] ^ gamma) & bit == 0;
            if is_good {
                i += 1;
            } else {
                nums.swap_remove(i);
            }
        }
        bit >>= 1;
    }
    nums[0]
}

pub fn day3() {
    let nums = list_of_integers_radix(INPUT, 2).collect::<Vec<_>>();

    println!("Part1: {}", calc_epsilon(&nums) * calc_gamma(&nums));
    println!(
        "Part2: {}",
        life_support(&nums, calc_epsilon) * life_support(&nums, calc_gamma)
    );
}
