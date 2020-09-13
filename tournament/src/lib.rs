use std::collections::HashMap;

struct Team<'a> {
    name: &'a str,
    wins: usize,
    loses: usize,
    draws: usize,
    matches_played: usize,
    points: usize,
}

impl Team {
    fn new(name: &str) -> Self {
        Team {name, wins: 0, loses: 0, draws: 0, matches_played: 0, points: 0 }
    }
}


const TEAMS: HashMap<&str, Team> = HashMap::new();

fn get_team(name: &str) -> Team {
    if !TEAMS.contains_key(name) { TEAMS.insert(name, Team::new(name))}
    Team[name]
}

pub fn tally(match_results: &str) -> String {

    for mut result in match_results.lines().map(|x| x.split(';')) {
        let (home, away, outcome) =
            (result.next().unwrap(), result.next().unwrap(), result.next().unwrap());

        println!("{} : {} : {}", home, away, outcome);
        match outcome {
            "draw" => {},
            "win" => {},
            "lose" => home.win(away),
            _ => panic!("Unknown outcome ->: {}", outcome),
        };

    }

    String::new()
}
