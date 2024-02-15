use std::borrow::Borrow;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use clap::Parser;
use anyhow::{Context, Result};


#[derive(PartialEq, Eq, Debug, Clone)]
struct PaintballPlayer {
    player_name: String,
    rounds_played: u32,
}

struct PaintballGame {
    round_counter: u32,
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

impl PaintballPlayer {
    fn new(name: &String) -> PaintballPlayer {
        PaintballPlayer { player_name: name.to_string(), rounds_played: 1 }
    }

    fn get_name(&self) -> &str {
        self.player_name.as_str()
    }

    fn get_rounds_played(&self) -> u32 {
        self.rounds_played
    }

    fn add_player_rounds_played(&mut self) {
        self.rounds_played = self.rounds_played +1;
    }
}

impl PaintballGame {
    fn new() -> PaintballGame {
        PaintballGame { round_counter: 0}
    }

    fn add_round(&mut self) {
        self.round_counter+=1;
    }

    fn get_rounds(&self) -> u32 {
        self.round_counter
    }
}

fn main() -> Result<()>{
    let mut pb_game = PaintballGame::new();
    let mut pb_players: Vec<PaintballPlayer> = vec![];

    let args = Args::parse();
    let _file = std::fs::read_to_string(&args.path).with_context(|| format!("could not read file '{}'", args.path.display()))?;

    


    if let Ok(lines) = read_lines(&args.path) {
        for line in lines {
            if let Ok(_line) = line {
                if _line.as_str().contains(" : playing") {
                    let mut player_str = _line.as_str().replace(" : playing", "");
                    player_str = player_str.as_str().split_at(60).1.to_string();

                    if !does_pb_player_exits(&pb_players, player_str.clone()) {
                        let res_player = create_new_pb_player(&player_str);
                        let new_player;
                        match res_player {
                            Some(ref _v) => new_player = res_player,
                            None => continue,
                        };
                        pb_players.push(new_player.unwrap());
                    } else {
                        // else 
                        // get PaintballPlayer from player Name
                        // to prevent that multiple /pb list commands affect round counter
                        // if PaintballPlayer.get_rounds_played > pb_game.get_rounds 
                        //      PaintballPlayer.add_player_rounds_played
                        let option = return_pb_player_from_vector(&pb_players, player_str.clone());
                        let pb_player ;
                        match option {
                            Some(ref _v) => pb_player = option.unwrap(),
                            None => continue,
                        };
                        if pb_player.get_rounds_played() < pb_game.get_rounds() {
                            update_vector(&mut pb_players, pb_player);
                        }
                        
                        //pb_player.add_player_rounds_played();
                        //println!("{:?}", pb_player.get_rounds_played());
                        //println!("{:?}", pb_players);
                        
                    }
                   

                                        
                } else if _line.as_str().contains("Match status: Round started! GO GO GO!") {
                    pb_game.add_round();
                }
            }
        }
    }
    println!("Rounds played: {:?}", pb_game.get_rounds());
    let mut i = pb_game.get_rounds();
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);
    while i > 0 {
        writeln!(handle, "{:?} Runden ", i)?;
    
        for players in pb_players.iter() {
            if players.get_rounds_played() == i {
                write!(handle, "{}, ", players.get_name())?;
            }
        }
        writeln!(handle,"")?;

        i = i-1;
    }
    //println!("{:?}", pb_players);

    Ok(())
}

fn does_pb_player_exits<'a>(players: &'a Vec<PaintballPlayer>, name: String) -> bool {
    let mut exists = false;
    for player in players {
        if player.get_name() == name {
            exists = true;
            break;
        }
    }
    return exists;
}


fn return_pb_player_from_vector<'a>(players: &'a Vec<PaintballPlayer>, name: String) -> Option<PaintballPlayer> {
    for player in players {
        if player.get_name() == name {
            return Some(player.clone());
        }
    }
    return None
}

fn create_new_pb_player(name: &String) -> Option<PaintballPlayer>{
    let player; 
    player = PaintballPlayer::new(name);
    return Some(player);  
}

fn update_vector(player_vec: &mut Vec<PaintballPlayer>, pb_player: PaintballPlayer) {
    let index = player_vec.iter().position(|r| r == pb_player.clone().borrow());
    let i; 
    match index {
        Some(_v) => i = index.unwrap(),
        None => return,
    }
    player_vec[i].add_player_rounds_played();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
