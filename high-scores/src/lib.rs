pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> HighScores<'a> {
        HighScores { scores: scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.len() {
            0 => None,
            n => Some(self.scores[n - 1]),
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_three: Vec<u32> = Vec::with_capacity(3);
        for score in self.scores {
            if top_three.len() < 3 {
                top_three.push(*score);
            } else {
                let min = top_three
                    .iter()
                    .enumerate()
                    .fold((0 as usize, u32::MAX), |min, this| {
                        if *this.1 < min.1 {
                            (this.0, *this.1)
                        } else {
                            min
                        }
                    });
                if *score > min.1 {
                    top_three[min.0] = *score;
                }
            }
        }
        top_three.sort_unstable_by(|a, b| b.cmp(a));
        top_three
    }
}
