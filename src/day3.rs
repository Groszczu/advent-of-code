use std::collections::HashSet;

use crate::test_puzzle;

pub fn part1(input: &str) -> i64 {
    let rucksack = transform_input_for_part1(input);

    rucksack
        .iter()
        .map(|compartments| {
            let common_item = get_common_item(compartments)
                .expect("rucksack compartments should have a common item");
            let priority =
                get_item_priority(common_item).expect("common item should have priority") as i32;
            priority as i64
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let groups = transform_input_for_part2(input);

    groups
        .iter()
        .map(|groups| {
            let common_item =
                get_common_item(groups).expect("rucksack groups should have a common item");
            let priority =
                get_item_priority(common_item).expect("common item should have priority") as i32;
            priority as i64
        })
        .sum()
}

fn get_common_item(item_groups: &[String]) -> Option<char> {
    let mut unique_groups_items = item_groups
        .iter()
        .map(|group| group.chars().collect::<HashSet<_>>());

    let mut common_items = unique_groups_items
        .next()
        .expect("there should be at least one group");

    for group_items in unique_groups_items {
        common_items = common_items.intersection(&group_items).copied().collect();
    }

    common_items.iter().next().copied()
}

fn get_item_priority(item: char) -> Option<u8> {
    const LOWERCASE_A_ASCII_VALUE: u8 = 97;
    const UPPERCASE_A_ASCII_VALUE: u8 = 65;

    const LOWERCASE_ITEMS_START_PRIORITY: u8 = 1;
    const UPPERCASE_ITEMS_START_PRIORITY: u8 = 27;

    let ascii_value = item as u8;
    if !ascii_value.is_ascii_alphabetic() {
        return None;
    }
    let priority = if ascii_value.is_ascii_lowercase() {
        ascii_value - LOWERCASE_A_ASCII_VALUE + LOWERCASE_ITEMS_START_PRIORITY
    } else {
        ascii_value - UPPERCASE_A_ASCII_VALUE + UPPERCASE_ITEMS_START_PRIORITY
    };

    Some(priority)
}

fn transform_input_for_part1(input: &str) -> Vec<[String; 2]> {
    input
        .lines()
        .map(|rucksack_items| {
            let number_of_items = rucksack_items.len();
            let number_of_items_in_compartment = number_of_items / 2;
            let first_compartment_items =
                rucksack_items[..number_of_items_in_compartment].to_owned();
            let second_compartment_items =
                rucksack_items[number_of_items_in_compartment..].to_owned();
            [first_compartment_items, second_compartment_items]
        })
        .collect()
}

fn transform_input_for_part2(input: &str) -> Vec<[String; 3]> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            [
                group[0].to_owned(),
                group[1].to_owned(),
                group[2].to_owned(),
            ]
        })
        .collect()
}

test_puzzle!(3, 157, 70);
