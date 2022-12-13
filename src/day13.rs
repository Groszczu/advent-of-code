mod packet;

use std::str::FromStr;

use crate::{shared::PuzzleResult, test_solvers};

use self::packet::Packet;

pub fn part1(input: &str) -> PuzzleResult {
    let packet_pairs = transform_input_for_part_1(input);

    let packets_in_right_order = packet_pairs
        .iter()
        .enumerate()
        .map(|(index, (left, right))| (index + 1, left.cmp(right)))
        .filter(|(_, ord)| ord.is_lt());

    let pairs_in_right_order_indices_sum = packets_in_right_order
        .map(|(index, _)| index)
        .sum::<usize>();

    (pairs_in_right_order_indices_sum as i64).into()
}

pub fn part2(input: &str) -> PuzzleResult {
    let mut packets = transform_input_for_part_2(input);

    let divider_packets = [
        Packet::from_str("[[2]]").unwrap(),
        Packet::from_str("[[6]]").unwrap(),
    ];

    packets.extend(divider_packets.iter().cloned());

    packets.sort();

    let result = packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| divider_packets.contains(packet))
        .fold(1, |acc, (index, _)| acc * (index + 1)) as i64;

    result.into()
}

fn transform_input_for_part_1(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\n\n")
        .map(|pairs| {
            let pairs = pairs.lines().collect::<Vec<_>>();

            let left_packet = Packet::from_str(pairs[0]).expect("first packet should be valid");
            let right_packet = Packet::from_str(pairs[1]).expect("second packet should be valid");

            (left_packet, right_packet)
        })
        .collect()
}

fn transform_input_for_part_2(input: &str) -> Vec<Packet> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|packet| Packet::from_str(packet).expect("packet should be valid"))
        .collect()
}

test_solvers!(13, 140);
