use std::cmp::Ordering;

use indexmap::IndexMap;

pub fn tally(match_results: &str) -> String {
    let mut teams: IndexMap<&str, TeamResults> = IndexMap::new();

    for match_result in match_results.split('\n') {
        let match_data: Vec<_> = match_result.split(';').collect();
        if let [home, away, result] = match_data[..] {
            let result: MatchResult = result.try_into().unwrap();
            teams.entry(home).or_default().update(result);
            teams.entry(away).or_default().update(result.opponent());
        }
    }

    teams.sort_unstable_by(|team_1, results_1, team_2, results_2| {
        match results_2.points().cmp(&results_1.points()) {
            Ordering::Equal => team_1.cmp(team_2),
            order => order,
        }
    });

    let header = table_row("Team", "MP", "W", "D", "L", "P");
    let teams_table = create_teams_table(&teams);

    if teams_table.is_empty() {
        header
    } else {
        format!("{header}\n{teams_table}")
    }
}

fn create_teams_table(teams: &IndexMap<&str, TeamResults>) -> String {
    teams
        .iter()
        .map(|(team, results)| {
            table_row(
                team,
                &results.matches_played().to_string(),
                &results.wins.to_string(),
                &results.draws.to_string(),
                &results.defeats.to_string(),
                &results.points().to_string(),
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn table_row(team: &str, played: &str, wins: &str, draws: &str, losses: &str, pts: &str) -> String {
    format!("{team:<30} | {played:>2} | {wins:>2} | {draws:>2} | {losses:>2} | {pts:>2}")
}

#[derive(Debug)]
struct InvalidMatchResult;

#[derive(Clone, Copy)]
enum MatchResult {
    Win,
    Draw,
    Loss,
}

impl MatchResult {
    fn opponent(&self) -> Self {
        match *self {
            Self::Win => Self::Loss,
            Self::Draw => Self::Draw,
            Self::Loss => Self::Win,
        }
    }
}

impl TryFrom<&str> for MatchResult {
    type Error = InvalidMatchResult;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "win" => Ok(Self::Win),
            "draw" => Ok(Self::Draw),
            "loss" => Ok(Self::Loss),
            _ => Err(InvalidMatchResult),
        }
    }
}

#[derive(Default)]
struct TeamResults {
    wins: u8,
    draws: u8,
    defeats: u8,
}

impl TeamResults {
    fn update(&mut self, result: MatchResult) {
        match result {
            MatchResult::Win => self.wins += 1,
            MatchResult::Draw => self.draws += 1,
            MatchResult::Loss => self.defeats += 1,
        }
    }

    fn points(&self) -> u8 {
        self.wins * 3 + self.draws
    }

    fn matches_played(&self) -> u8 {
        self.wins + self.draws + self.defeats
    }
}
