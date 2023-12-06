use std::{collections::BTreeMap, ops::Not};

use nom::{
    bytes::complete::tag,
    character::complete::{
        self, alpha1, digit1, line_ending,
    },
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

use crate::custom_error::AocError;

#[derive(Debug)]
struct Cube<'a>{
    color: &'a str,
    amount: u32,
}

#[derive(Debug)]
struct Game<'a> {
    id: &'a str,
    rounds: Vec<Vec<Cube<'a>>>,
}

impl<'a> Game<'a>{
    fn valid_for_cube_set(
        &self,
        map: &BTreeMap<&str, u32>,
    ) -> Option<u32>{
        self.rounds
            .iter()
            .any(|round|){
                round.iter().any(|shown_cube|{
                    shown_cube.amount
                        > *map
                            .get(show_cube.color)
                            .expect("a valid cube")
                })
            }
    }
}