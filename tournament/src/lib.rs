#[derive(Clone)]
struct TeamScore {
    name: String,
    won: u32,
    drawn: u32,
    lost: u32,
}

impl TeamScore {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            won: 0,
            drawn: 0,
            lost: 0
        }
    }
    pub fn won(&mut self){
        self.won += 1;
    }
    pub fn drawn(&mut self){
        self.drawn += 1;
    }
    pub fn lost(&mut self){
        self.lost += 1;
    }
    pub fn get_mp(&self) -> u32 {
        self.won+self.drawn+self.lost
    }
    pub fn get_point(&self) -> u32 {
        self.won*3+self.drawn
    }
    pub fn stringify(&self) -> String {
        format!("{: <31}|  {} |  {} |  {} |  {} |  {}", &self.name, self.get_mp(), self.won, self.drawn, self.lost, self.get_point())
    }
}

use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let mut table = HashMap::new();
    for line in match_results.lines() {
        let tokens: Vec<&str> = line.split(';').collect();
        let (teama, teamb, result) = (tokens[0], tokens[1], tokens[2]);
        table.entry(teama).or_insert(TeamScore::new(teama.to_string()));
        table.entry(teamb).or_insert(TeamScore::new(teamb.to_string()));
        match result {
            "win" => {
                table.get_mut(teama).unwrap().won();
                table.get_mut(teamb).unwrap().lost();
            },
            "loss" => {
                table.get_mut(teama).unwrap().lost();
                table.get_mut(teamb).unwrap().won();
            },
            "draw" => {
                table.get_mut(teama).unwrap().drawn();
                table.get_mut(teamb).unwrap().drawn();
            },
            _ => unimplemented!()
        }
    }
    let mut teamscores: Vec<TeamScore> = table.values().cloned().collect::<Vec<TeamScore>>();
    teamscores.sort_by_key(|k| k.name.clone());
    teamscores.reverse();
    teamscores.sort_by_key(|k| k.get_point());
    teamscores.reverse();
    let mut result_table: String = "Team                           | MP |  W |  D |  L |  P".to_string();
    for team in teamscores {
        result_table += "\n";
        result_table += &team.stringify();
    }
    result_table
}

// let (teama, teamb, result) = line.split(',').collect();がしたい
// 名前でsortした後pointでsortができない。。。
// cloneせずにsortする方法が知りたい