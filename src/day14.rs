use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Copy, Clone, Default, Debug)]
struct Polymer {
    rules: [usize; 2], // Index into polymer array
    element_id: usize,
    count: u64,
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^(\w\w) -> (\w)$").unwrap();
}

static INPUT: &str = include_str!("input/Day14.txt");

fn polymer_step(polymers: &[Polymer], elements: &mut [u64]) -> Vec<Polymer> {
    let mut new_polymers = polymers.to_vec();
    for polymer in new_polymers.iter_mut() {
        polymer.count = 0;
    }
    for polymer in polymers.iter() {
        elements[polymer.element_id] += polymer.count;
        for rule in polymer.rules {
            new_polymers[rule].count += polymer.count;
        }
    }
    new_polymers
}

pub fn day14() {
    let mut section_iter = INPUT.trim().split("\n\n");
    let start_polymer = section_iter.next().unwrap().trim();
    let rule_iter = section_iter.next().unwrap().trim().split('\n');
    assert!(section_iter.next().is_none());

    let mut elements = Vec::new();
    let mut element_lookup = HashMap::new();
    let mut polymers = Vec::new();
    let mut polymer_lookup = HashMap::new();
    for raw_rule in rule_iter {
        let captures = REGEX.captures(raw_rule).unwrap();
        let insert_char = captures[2].chars().next().unwrap();
        let prev_polymer_name = &captures[1];
        let prev_polymer_id = *polymer_lookup
            .entry(prev_polymer_name.to_string())
            .or_insert_with(|| {
                polymers.push(Polymer::default());
                polymers.len() - 1
            });
        let element_id = *element_lookup.entry(insert_char).or_insert_with(|| {
            elements.push(0);
            elements.len() - 1
        });

        let mut rules = [0; 2];
        for (i, rule) in rules.iter_mut().enumerate() {
            let mut new_polymer_name = String::new();
            if i == 0 {
                new_polymer_name.push(prev_polymer_name.chars().next().unwrap());
            }
            new_polymer_name.push(insert_char);
            if i == 1 {
                new_polymer_name.push(prev_polymer_name.chars().nth(1).unwrap());
            }
            let new_polymer_id = *polymer_lookup.entry(new_polymer_name).or_insert_with(|| {
                polymers.push(Polymer::default());
                polymers.len() - 1
            });

            *rule = new_polymer_id;
        }
        polymers[prev_polymer_id].rules = rules;
        polymers[prev_polymer_id].element_id = element_id;
    }

    for (char_a, char_b) in start_polymer.chars().zip(start_polymer.chars().skip(1)) {
        let mut polymer_name = String::new();
        polymer_name.push(char_a);
        polymer_name.push(char_b);
        let id = *polymer_lookup.get(&polymer_name).unwrap();
        polymers[id].count += 1;
    }

    for element in start_polymer.chars() {
        let element_id = *element_lookup.entry(element).or_insert_with(|| {
            elements.push(0);
            elements.len() - 1
        });
        elements[element_id] += 1;
    }

    for _ in 0..10 {
        polymers = polymer_step(&polymers, &mut elements);
    }

    let part1 = elements.iter().max().unwrap() - elements.iter().min().unwrap();
    println!("Part1: {}", part1);

    for _ in 0..30 {
        polymers = polymer_step(&polymers, &mut elements);
    }

    let part2 = elements.iter().max().unwrap() - elements.iter().min().unwrap();
    println!("Part2: {}", part2);

    /*let mut element_lookup_table = element_lookup
        .iter()
        .map(|(k, v)| (*k, *v))
        .collect::<Vec<_>>();
    element_lookup_table.sort_by_key(|x| x.1);
    let dbg_elements = elements
        .iter()
        .zip(element_lookup_table.into_iter().map(|(k, _)| k))
        .collect::<Vec<_>>();
    let mut polymer_lookup_table = polymer_lookup
        .iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect::<Vec<_>>();
    polymer_lookup_table.sort_by_key(|x| x.1);
    let dbg_polymers = polymers
        .iter()
        .zip(polymer_lookup_table.into_iter().map(|(k, _)| k))
        .collect::<Vec<_>>();

    eprintln!("{:?}", dbg_elements);
    eprintln!("{:?}", dbg_polymers);*/
}
