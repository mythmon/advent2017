use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::str::FromStr;

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2017-D20-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .case(
                "Example",
                "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>\np=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>",
                0_usize,
            )
            .case("Solution", include_str!("input"), 308_usize)
            .collect())
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        input
            .lines()
            .map(|l| l.parse::<Particle>().unwrap())
            .enumerate()
            .min_by_key(|&(_, p)| p.position_at_time(1_000_000).manhattan())
            .unwrap()
            .0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Particle {
    p: Vec3,
    v: Vec3,
    a: Vec3,
}

impl Particle {
    fn position_at_time(&self, time: i64) -> Vec3 {
        let x = self.p.x + self.v.x * time + self.a.x * time.pow(2) / 2;
        let y = self.p.y + self.v.y * time + self.a.y * time.pow(2) / 2;
        let z = self.p.z + self.v.z * time + self.a.z * time.pow(2) / 2;
        (x, y, z).into()
    }
}

impl FromStr for Particle {
    type Err = std::num::ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // p=<-4897,3080,2133>, v=<-58,-15,-78>, a=<17,-7,0>
        // -0------|-1--|-2---|-3-----|-4-|-5--|-6----|7-|8-
        let parts: Vec<&str> = input.split(',').collect();
        assert_eq!(parts.len(), 9);

        Ok(Self {
            p: (
                parts[0][3..].trim().parse()?,
                parts[1].trim().parse()?,
                parts[2][..(parts[2].len() - 1)].trim().parse()?,
            )
                .into(),
            v: (
                parts[3][4..].trim().parse()?,
                parts[4].trim().parse()?,
                parts[5][..(parts[5].len() - 1)].trim().parse()?,
            )
                .into(),
            a: (
                parts[6][4..].trim().parse()?,
                parts[7].trim().parse()?,
                parts[8][..(parts[8].len() - 1)].trim().parse()?,
            )
                .into(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    const fn manhattan(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl From<(i64, i64, i64)> for Vec3 {
    fn from(t: (i64, i64, i64)) -> Self {
        Self {
            x: t.0,
            y: t.1,
            z: t.2,
        }
    }
}

#[test]
fn test_particle_from_str() {
    let s = "p=<-4897,3080,2133>, v=<-58,-15,-78>, a=<17,-7,0>";
    let p: Particle = s.parse().unwrap();
    assert_eq!(p.p, (-4897, 3080, 2133).into());
    assert_eq!(p.v, (-58, -15, -78).into());
    assert_eq!(p.a, (17, -7, 0).into());
}
