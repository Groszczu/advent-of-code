use std::{
    cmp::{self, Ordering},
    fmt::Display,
    str::FromStr,
    vec,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketValue {
    Int(u32),
    List(Vec<PacketValue>),
}

impl Ord for PacketValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Int(left_int), Self::Int(right_int)) => left_int.cmp(right_int),
            (Self::List(left_list), Self::List(right_list)) => {
                cmp_packet_list(left_list, right_list)
            }
            (&Self::Int(left_int), Self::List(right_list)) => {
                cmp_packet_list(&vec![Self::Int(left_int)], right_list)
            }
            (Self::List(left_list), &Self::Int(right_int)) => {
                cmp_packet_list(left_list, &vec![Self::Int(right_int)])
            }
        }
    }
}

impl PartialOrd for PacketValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn cmp_packet_list(left_list: &Vec<PacketValue>, right_list: &Vec<PacketValue>) -> Ordering {
    let max_len = cmp::max(left_list.len(), right_list.len());
    for index in 0..max_len {
        let left_value = left_list.get(index);
        let right_value = right_list.get(index);

        match (left_value, right_value) {
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(left_value), Some(right_value)) => {
                let child_ordering = left_value.cmp(right_value);

                match child_ordering {
                    Ordering::Equal => continue,
                    ordering => return ordering,
                }
            }
            _ => unreachable!(),
        }
    }

    Ordering::Equal
}

impl Display for PacketValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Int(int_value) => write!(f, "{}", int_value)?,
            &Self::List(list_value) => {
                let list_result = list_value
                    .iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
                    .join(",");

                write!(f, "[{}]", list_result)?;
            }
        };

        Ok(())
    }
}

#[derive(Debug)]
pub struct ParsePacketValueError;

impl From<u32> for PacketValue {
    fn from(v: u32) -> Self {
        Self::Int(v)
    }
}

impl FromStr for PacketValue {
    type Err = ParsePacketValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut current_list: Option<Vec<PacketValue>> = None;
        let mut list_stack = vec![];

        let chars = s.chars().collect::<Vec<_>>();
        let mut input = chars.as_slice();

        while input.len() != 0 {
            match input {
                ['[', rest @ ..] => {
                    if let Some(list) = current_list {
                        list_stack.push(list);
                    }
                    current_list = Some(vec![]);
                    input = rest;
                }
                [value, rest @ ..] if value.is_digit(10) => {
                    let extra_digits = rest
                        .iter()
                        .take_while(|c| c.is_digit(10))
                        .collect::<Vec<_>>();
                    let number_of_extra_digits = extra_digits.len();

                    let digits = [&[value], extra_digits.as_slice()].concat();

                    let value = digits
                        .iter()
                        .map(|digit| digit.to_digit(10).unwrap())
                        .fold(0, |value, digit| 10 * value + digit);

                    current_list
                        .as_mut()
                        .ok_or(ParsePacketValueError)?
                        .push(Self::Int(value));

                    input = &rest[number_of_extra_digits..];
                }
                [']'] if list_stack.is_empty() => {
                    let complete_list = current_list.ok_or(ParsePacketValueError)?;
                    return Ok(Self::List(complete_list));
                }
                [']', rest @ ..] => {
                    let complete_list = current_list.ok_or(ParsePacketValueError)?;

                    let mut parent_list = list_stack.pop().ok_or(ParsePacketValueError)?;
                    parent_list.push(Self::List(complete_list));
                    current_list = Some(parent_list);

                    input = rest;
                }
                [',', rest @ ..] => {
                    input = rest;
                }
                _ => {
                    return Err(ParsePacketValueError);
                }
            };
        }

        Err(ParsePacketValueError)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packet {
    values: Vec<PacketValue>,
}

impl Packet {
    pub fn new(values: Vec<PacketValue>) -> Self {
        Self { values }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        let left_values = &self.values;
        let right_values = &other.values;

        cmp_packet_list(left_values, right_values)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        PacketValue::List(self.values.clone()).fmt(f)
    }
}

impl FromStr for Packet {
    type Err = ParsePacketValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = PacketValue::from_str(s)?;

        let values = match value {
            PacketValue::List(list) => list,
            value @ PacketValue::Int(_) => vec![value],
        };

        Ok(Self::new(values))
    }
}
