use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};

#[derive(Debug)]
pub struct Day23Part2;

impl PuzzleRunner for Day23Part2 {
    type Input = (u32, u32);
    type Output = u32;

    fn name(&self) -> String {
        "2017-D23-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("part 1", (67, 67), 0)
            .case("Solution", (106_700, 123_700), 905)
            .collect()
    }

    fn run_puzzle((min, max): Self::Input) -> Self::Output {
        let mut composite_count = 0;

        for n in (min..=max + 1).step_by(17) {
            // let max = ((n + 1) as f32).sqrt().ceil() as u32;
            for d in 2..n {
                if n % d == 0 {
                    composite_count += 1;
                    break;
                }
            }
        }

        composite_count
    }
}