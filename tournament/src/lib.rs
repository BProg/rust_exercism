use std::collections::HashMap;
use std::cmp::*;

const WIN: &'static str = "win";
const DRAW: &'static str = "draw";
const LOSE: &'static str = "loss";
const HEADER: &'static str = "Team                           | MP |  W |  D |  L |  P";

#[derive(Debug)]
enum MatchResult {
    Unknown,
    Win,
    Draw,
    Lose,
}

impl MatchResult {
    fn reverse(&self) -> Self {
        match self {
            Self::Win => Self::Lose,
            Self::Draw => Self::Draw,
            Self::Lose => Self::Win,
            Self::Unknown => Self::Unknown,
        }
    }
}

impl From<&str> for MatchResult {
    fn from(text: &str) -> Self {
        match text {
            WIN => Self::Win,
            DRAW => Self::Draw,
            LOSE => Self::Lose,
            _ => Self::Unknown,
        }
    }
}

struct TeamResults {
    pub name: String,
    pub matches_played: u16,
    pub wins: u16,
    pub draws: u16,
    pub loses: u16,
    pub points: u16,
}

impl TeamResults {
    pub fn new() -> Self {
        Self {
            name: "".into(),
            matches_played: 0,
            wins: 0,
            draws: 0,
            loses: 0,
            points: 0,
        }
    }

    pub fn with_name_and_result(name: String, result: MatchResult) -> Self {
        let mut team_result = Self::new();
        team_result.name = name;
        team_result.update(result);
        team_result
    }

    pub fn update(&mut self, result: MatchResult) {
        self.matches_played += 1;
        match result {
            MatchResult::Win => {
                self.wins += 1;
                self.points += 3;
            }
            MatchResult::Draw => {
                self.draws += 1;
                self.points += 1;
            }
            MatchResult::Lose => self.loses += 1,
            MatchResult::Unknown => (),
        }
    }
}

impl std::cmp::Ord for TeamResults {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.points.cmp(&other.points) == Ordering::Equal {
            return self.name.cmp(&other.name);
        }
        self.points.cmp(&other.points).reverse()
    }
}

impl PartialOrd for TeamResults {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for TeamResults {
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points && self.name == other.name
    }
}

impl Eq for TeamResults {}

pub fn tally(match_results: &str) -> String {
    let teams_results: HashMap<&str, TeamResults> =
        match_results
            .split("\n")
            .fold(HashMap::default(), |mut results, match_results| {
                let team_v_team = match_results.split(";").collect::<Vec<&str>>();
                if team_v_team.len() < 3 {
                    return results;
                }

                if let Some(team_results) = results.get_mut(&team_v_team[0]) {
                    team_results.update(team_v_team[2].into());
                } else {
                    let team_results = TeamResults::with_name_and_result(
                        String::from(team_v_team[0]),
                        team_v_team[2].into(),
                    );
                    results.insert(&team_v_team[0], team_results);
                }

                if let Some(team_results) = results.get_mut(&team_v_team[1]) {
                    team_results.update(MatchResult::from(team_v_team[2]).reverse());
                } else {
                    let team_results = TeamResults::with_name_and_result(
                        String::from(team_v_team[1]),
                        MatchResult::from(team_v_team[2]).reverse(),
                    );
                    results.insert(&team_v_team[1], team_results);
                }
                results
            });
    let mut ordered_results = teams_results.values().collect::<Vec<&TeamResults>>();
    ordered_results.sort();
    let formated_table =
        ordered_results
            .iter()
            .fold(String::from(HEADER), |mut table, team_results| {
                table.push_str("\n");
                table.push_str(&format!("{:<31}|", team_results.name));
                table.push_str(&format!("{:>3} |", team_results.matches_played));
                table.push_str(&format!("{:>3} |", team_results.wins));
                table.push_str(&format!("{:>3} |", team_results.draws));
                table.push_str(&format!("{:>3} |", team_results.loses));
                table.push_str(&format!("{:>3}", team_results.points));
                table
            });
    formated_table
}
