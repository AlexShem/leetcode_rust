use super::Solve;

pub struct JudgeCircle;

impl JudgeCircle {
    pub fn judge_circle(moves: String) -> bool {
        let moves: Vec<char> = moves.chars().collect();
        let (mut x, mut y) = (0, 0);

        for action in moves {
            match action {
                'U' => y += 1,
                'D' => y -= 1,
                'L' => x -= 1,
                'R' => x += 1,
                _ => unreachable!()
            };
        }

        x == 0 && y == 0
    }
}

impl Solve<String, bool> for JudgeCircle {
    fn solve(input: String) -> bool {
        Self::judge_circle(input)
    }
}