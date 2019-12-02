use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use crate::day09::ParseAction;

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D09-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", "{}", 1_u32)
            .case("Example 2", "{{{}}}", 6_u32)
            .case("Example 3", "{{},{}}", 5_u32)
            .case("Example 4", "{{{},{},{{}}}}", 16_u32)
            .case("Example 5", "{<a>,<a>,<a>,<a>}", 1_u32)
            .case("Example 6", "{{<ab>},{<ab>},{<ab>},{<ab>}}", 9_u32)
            .case("Example 7", "{{<!!>},{<!!>},{<!!>},{<!!>}}", 9_u32)
            .case("Example 8", "{{<a!>},{<a!>},{<a!>},{<ab>}}", 3_u32)
            .case("Solution", include_str!("input"), 17_390_u32)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        use crate::day09::ParseState::*;

        let mut total_score = 0;
        let mut state_stack = vec![];

        for c in input.trim().chars() {
            let next_action = match (state_stack.last(), c) {
                (Some(&s), '!') if s != Cancel => ParseAction::Push(Cancel),
                (Some(Cancel), _) | (Some(Garbage), '>') => ParseAction::Pop,
                (None, '{') => ParseAction::Push(InGroup(1)),
                (Some(InGroup(v)), '{') => ParseAction::Push(InGroup(v + 1)),
                (Some(&InGroup(v)), '}') => {
                    total_score += v;
                    ParseAction::Pop
                }
                (Some(InGroup(_)), '<') => ParseAction::Push(Garbage),
                (Some(InGroup(_)), ',') | (Some(Garbage), _) => ParseAction::Nothing,

                (s, c) => panic!("unexpected input '{}' in {:?}", c, s),
            };

            match next_action {
                ParseAction::Nothing => {}
                ParseAction::Push(v) => {
                    state_stack.push(v);
                }
                ParseAction::Pop => {
                    state_stack.pop();
                }
            }
        }

        assert_eq!(state_stack.len(), 0);

        total_score
    }
}