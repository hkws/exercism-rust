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
        // 本当はlet (teama, teamb, result) = line.split(',').collect();がしたいが
        //   let tokens: Vec<&str> = line.split(';').collect();
        //   let (teama, teamb, result) = (tokens[0], tokens[1], tokens[2]);
        // しかなさそう
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
    // cloneせずにsortする方法が知りたい
    // let mut teamscores: Vec<&TeamScore> = table.values().collect::<Vec<&TeamScore>>();
    // これで行けた、参照さえ取得すれば十分。たしかに。
    teamscores.sort_by_key(|k| k.name.clone());
    teamscores.reverse();
    teamscores.sort_by_key(|k| k.get_point());
    teamscores.reverse();
    // 名前でsortした後pointでsortができない。。。
    // teamscores.sort_by(|a, b| b.get_point().cmp(&a.get_point()).
    //                             then_with(|| a.name.cmp(&b.name)));
    // これでいけるみたい。sort_byとthen_with。
    let mut result_table: String = "Team                           | MP |  W |  D |  L |  P".to_string();
    for team in teamscores {
        result_table += "\n";
        result_table += &team.stringify();
        // 自前のstringifyはあんまrustっぽくない気がする
        // rust的に書くなら
        // impl From<&TeamScore> for String {
        //     fn from(origin: &TeamScore) -> String {
        //         format!("{: <31}|  {} |  {} |  {} |  {} |  {}", &self.name, self.get_mp(), self.won, self.drawn, self.lost, self.get_point())
        //     }
        // }
        // このように、型どうしの変換を実装するときは、From<変換元> for 変換先をimplする
        // 逆変換としてIntoがあり、これはFromを実装したら自動で使えるようになる
        
    }
    result_table
}
