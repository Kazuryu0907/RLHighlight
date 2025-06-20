use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum MugiCmd {
    Init,
    EndReplay,
    EndStats,
    TeamNames,
    Demolished,
    Scored,
    MatchId,
    Start,
    End,
    Stats,
    Goals,
    EpicSave,
    Dbg,
    DisplayNames,
    PlayerTable,
    Time,
    Boost,
    SubScore,
    Score,
    Player,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TeamNames {
    blue: String,
    orange: String,
    #[serde(rename = "matchId")]
    match_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Demolished {
    #[serde(rename = "receiverIndex")]
    receiver_index: u32,
    #[serde(rename = "victimIndex")]
    victim_index: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct MatchId {
    #[serde(rename = "matchId")]
    match_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct _Stats {
    id: String,
    teams: u32,
    scores: u32,
    goals: u32,
    assists: u32,
    saves: u32,
    shots: u32,
    demos: u32,
    #[serde(rename = "ballTouches")]
    ball_touches: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Goals {
    team: String,
    #[serde(rename = "scoreId")]
    score_id: String,
    #[serde(rename = "assistId")]
    assist_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Time {
    time: u32,
    #[serde(rename = "isOvertime")]
    is_overtime: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct Boost {
    boost: u32,
    index: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct SubScore {
    goals: u32,
    shots: u32,
    assists: u32,
    saves: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Score {
    score: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Player {
    #[serde(rename = "playerIndex")]
    player_index: usize,
    team: String,
    #[serde(rename = "playerName")]
    player_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MugiData<T> {
    cmd: String,
    // Any
    data: Option<T>,
}

pub fn parse_cmd(json: &str) -> Result<MugiCmd> {
    let data: MugiData<serde_json::Value> = serde_json::from_str(json)?;
    let cmd = data.cmd.as_str();
    let mugi_cmd = match cmd {
        "init" => MugiCmd::Init,
        "endReplay" => MugiCmd::EndReplay,
        "endStats" => MugiCmd::EndStats,
        "teamNames" => MugiCmd::TeamNames,
        "demolished" => MugiCmd::Demolished,
        "scored" => MugiCmd::Scored,
        "matchId" => MugiCmd::MatchId,
        "start" => MugiCmd::Start,
        "end" => MugiCmd::End,
        "stats" => MugiCmd::Stats,
        "goals" => MugiCmd::Goals,
        "epicSave" => MugiCmd::EpicSave,
        "dbg" => MugiCmd::Dbg,
        "displayNames" => MugiCmd::DisplayNames,
        "playerTable" => MugiCmd::PlayerTable,
        "time" => MugiCmd::Time,
        "boost" => MugiCmd::Boost,
        "subScore" => MugiCmd::SubScore,
        "score" => MugiCmd::Score,
        "player" => MugiCmd::Player,
        _ => return Err(anyhow!("mugi parse failed")),
    };
    Ok(mugi_cmd)
}

#[cfg(test)]
mod test {

    use super::*;

    // #[test]
    // fn test_team_names(){
    //     let msg = r#"{"cmd":"teamNames","data":{"blue":"","matchId":"DA3FB72C11F00213D67A6E8E78296A08","orange":""}}"#;
    //     let parse = parse_cmd(msg).unwrap();
    //     assert_eq!(parse,MugiCmd::TeamNames);
    //     let msg:MugiData<TeamNames> = serde_json::from_str(msg).unwrap();
    //     assert_eq!(msg.data,TeamNames{blue:"".to_owned(),orange:"".to_owned(),matchId:"DA3FB72C11F00213D67A6E8E78296A08".to_owned()});
    // }
    // #[test]
    // fn test_display_names(){
    //     let msg = r#"{"cmd":"displayNames","data":["Player_Bot_Tex","Player_Bot_Sabretooth","Player_Bot_Boomer","Player_Bot_Mountain","Player_Bot_Casper","Player_Bot_Bandit"]}"#;
    //     let parse = parse_cmd(msg).unwrap();
    //     assert_eq!(parse,MugiCmd::DisplayNames);
    //     let msg:MugiData<DisplayNames> = serde_json::from_str(msg).unwrap();
    //     let expect: DisplayNames = vec!["Player_Bot_Tex".to_string(),
    //             "Player_Bot_Sabretooth".to_string(),
    //             "Player_Bot_Boomer".to_string(),
    //             "Player_Bot_Mountain".to_string(),
    //             "Player_Bot_Casper".to_string(),
    //             "Player_Bot_Bandit".to_string(),];
    //     assert_eq!(msg.data,expect);
    // }
    #[test]
    fn test_all() {
        use std::fs::read_to_string;
        let path = r#"F:\Github\Moca_rust\src-tauri\mugi_log.txt"#;
        let lines: Vec<String> = read_to_string(path)
            .unwrap()
            .lines()
            .map(String::from)
            .collect();

        for msg in lines {
            println!("{}", msg);
            let cmd = parse_cmd(&msg).unwrap();
            match cmd {
                MugiCmd::Init => {}
                MugiCmd::EndReplay => {}
                MugiCmd::EndStats => {}
                MugiCmd::TeamNames => {
                    serde_json::from_str::<MugiData<TeamNames>>(&msg).unwrap();
                }
                MugiCmd::Demolished => {
                    serde_json::from_str::<MugiData<Demolished>>(&msg).unwrap();
                }
                MugiCmd::Scored => {}
                MugiCmd::MatchId => {
                    serde_json::from_str::<MugiData<MatchId>>(&msg).unwrap();
                }
                MugiCmd::Start => {}
                MugiCmd::End => {}
                MugiCmd::Stats => {
                    serde_json::from_str::<MugiData<Vec<_Stats>>>(&msg).unwrap();
                }
                MugiCmd::Goals => {
                    serde_json::from_str::<MugiData<Goals>>(&msg).unwrap();
                }
                MugiCmd::Dbg => {
                    serde_json::from_str::<MugiData<String>>(&msg).unwrap();
                }
                MugiCmd::DisplayNames => {
                    serde_json::from_str::<MugiData<Vec<String>>>(&msg).unwrap();
                }
                MugiCmd::PlayerTable => {
                    serde_json::from_str::<MugiData<Vec<String>>>(&msg).unwrap();
                }
                MugiCmd::Time => {
                    serde_json::from_str::<MugiData<Time>>(&msg).unwrap();
                }
                MugiCmd::Boost => {
                    serde_json::from_str::<MugiData<Boost>>(&msg).unwrap();
                }
                MugiCmd::SubScore => {
                    serde_json::from_str::<MugiData<SubScore>>(&msg).unwrap();
                }
                MugiCmd::Score => {
                    serde_json::from_str::<MugiData<Score>>(&msg).unwrap();
                }
                MugiCmd::Player => {
                    serde_json::from_str::<MugiData<Player>>(&msg).unwrap();
                }
                _ => panic!(),
            }
        }
    }
}
