use std::{cmp::Reverse, collections::HashMap, iter::once};

#[derive(Debug)]
struct TeamStat {
    pub won: usize,
    pub lost: usize,
    pub drawn: usize,
    pub played: usize,
}

impl TeamStat {
    pub fn new() -> Self {
        Self {
            won: 0,
            lost: 0,
            drawn: 0,
            played: 0,
        }
    }

    pub fn points(&self) -> usize {
        self.won * 3 + self.drawn
    }
}

fn parse_game(game: &str) -> Option<(String, String, String)> {
    let mut line_iter = game.split(";");
    if line_iter.clone().count() == 3 {
        return Some((
            String::from(line_iter.next().unwrap()),
            String::from(line_iter.next().unwrap()),
            String::from(line_iter.next().unwrap()),
        ));
    }
    None
}

pub fn tally(match_results: &str) -> String {
    let mut table = HashMap::<String, TeamStat>::new();

    match_results
        .split("\n")
        .map(parse_game)
        .for_each(|game| match game {
            Some((home, away, result)) => {
                if !table.contains_key(&home) {
                    table.insert(home.clone(), TeamStat::new());
                }
                if !table.contains_key(&away) {
                    table.insert(away.clone(), TeamStat::new());
                }
                table.get_mut(&home).unwrap().played += 1;
                table.get_mut(&away).unwrap().played += 1;
                match result.as_str() {
                    "win" => {
                        table.get_mut(&home).unwrap().won += 1;
                        table.get_mut(&away).unwrap().lost += 1;
                    }
                    "loss" => {
                        table.get_mut(&away).unwrap().won += 1;
                        table.get_mut(&home).unwrap().lost += 1;
                    }
                    _ => {
                        table.get_mut(&home).unwrap().drawn += 1;
                        table.get_mut(&away).unwrap().drawn += 1;
                    }
                }
            }
            _ => {}
        });

    let mut points_table = table
        .iter()
        .map(|(team, stats)| (team, stats, stats.points()))
        .collect::<Vec<_>>();
    points_table.sort_by_key(|(name, _, points)| (Reverse(points.clone()), name.clone()));

    once(format!(
        "{: <30} | {: >2} | {: >2} | {: >2} | {: >2} | {: >2}",
        "Team", "MP", "W", "D", "L", "P"
    ))
    .chain(points_table.iter().map(|(team, stats, points)| {
        String::from(format!(
            "{: <30} | {: >2} | {: >2} | {: >2} | {: >2} | {: >2}",
            team, stats.played, stats.won, stats.drawn, stats.lost, points
        ))
    }))
    .collect::<Vec<String>>()
    .join("\n")
}
