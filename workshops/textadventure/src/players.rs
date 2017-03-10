extern crate rand;

use board;
use board::Board;
use board::Position;
use inventory;
use inventory::Thing;
use std::io;
use std::collections::VecDeque;
use self::rand::Rng;

pub type Players = VecDeque<Player>;

pub enum Player {
    Explorer(ExplorerData),  // user
    Gnome(GnomeData),  // NPC
    Leprechaun(LeprechaunData)  // NPC
}

#[derive(Clone, PartialEq, Eq)]
pub enum Direction { North, South, East, West }

pub struct ExplorerData {
    pos: Position,
    energy: i32,
    things: Vec<Thing>
}

pub struct GnomeData {
    pos: Position,
    energy: i32,
    things: Vec<Thing>
}

pub struct LeprechaunData {
    pos: Position,
    things: Vec<Thing>
}

pub fn build_players(board: &Board) -> Players {
    let mut players: Players = VecDeque::new();

    let explorer = Player::Explorer(
        ExplorerData { pos: Position::new(0, 0, board),
                       energy: 65,
                       things: vec![Thing::Torch,
                                    Thing::GoldCoin { denom: 5 },
                                    Thing::GoldCoin { denom: 10 },
                                    Thing::GoldCoin { denom: 25 }] }
    );

    let gnome_a = Player::Gnome(
        GnomeData { pos: Position::new(0, 4, board),
                    energy: 41,
                    things: vec![Thing::GoldCoin { denom: 25 }; 3] }
    );

    let gnome_b = Player::Gnome(
        GnomeData { pos: Position::new(2, 2, board),
                    energy: 37,
                    things: vec![Thing::GoldCoin { denom: 25 }; 3] }
    );

    let mut lep_things = vec![];

    lep_things.append(&mut vec![Thing::GoldCoin { denom: 5 }; 8]);
    lep_things.append(&mut vec![Thing::GoldCoin { denom: 10 }; 8]);
    lep_things.append(&mut vec![Thing::GoldCoin { denom: 25 }; 8]);
    lep_things.append(&mut vec![Thing::FakeCoin { denom: 5 }; 5]);
    lep_things.append(&mut vec![Thing::FakeCoin { denom: 10 }; 5]);
    lep_things.append(&mut vec![Thing::FakeCoin { denom: 25 }; 5]);
    lep_things.append(&mut inventory::all_magic_words(board));
    lep_things.append(&mut inventory::all_fake_words(board));
    // FIXME rand::Rng shuffle lep_things

    let leprechaun = Player::Leprechaun(
        LeprechaunData { pos: Position::new(4, 4, board),
                         things: lep_things }
    );

    players.push_back(explorer);
    players.push_back(gnome_a);
    players.push_back(gnome_b);
    players.push_back(leprechaun);

    players
}

pub fn is_game_over(players: &Players) -> bool {
    players.iter()
           .filter(|&player| is_explorer(player))
           .fold(true, |acc, explorer| acc & is_dead(explorer))
}

pub fn move_player(player: Player, board: &Board) -> Player {
    let _player : Player;

    match player {
        Player::Explorer(data) => {
            _player = Player::Explorer(move_exp(data, board));
        },
        Player::Gnome(data) => {
            _player = Player::Gnome(move_gnome(data, board));
        },
        Player::Leprechaun(data) => {
            _player = Player::Leprechaun(move_lep(data, board));
        }
    }

    _player
}

pub fn is_occupant(other: &Player, pos: &Position) -> bool {
    match *other {
        Player::Explorer(ref data) => data.pos == *pos,
        Player::Gnome(ref data) => data.pos == *pos,
        Player::Leprechaun(ref data) => data.pos == *pos
    }
}

pub fn get_exp_pos(data: &ExplorerData) -> Position {
    data.pos
}

pub fn get_gnome_pos(data: &GnomeData) -> Position {
    data.pos
}

pub fn get_lep_pos(data: &LeprechaunData) -> Position {
    data.pos
}

fn is_explorer(player: &Player) -> bool {
    match *player {
        Player::Explorer(_) => true,
        _ => false
    }
}

fn is_dead(player: &Player) -> bool {
    match *player {
        Player::Explorer(ref data) => data.energy <= 0,
        _ => false
    }
}

fn move_exp(data: ExplorerData, board: &Board) -> ExplorerData {
    let mut _data = data;
    let mut input = String::new();

    loop {
        println!("Enter letter command: [N]orth [S]outh [E]ast [W]est [T]eleport");

        match io::stdin().read_line(&mut input) {
            Ok(n) => (),
            Err(why) => { println!("Failed to read line: {:?}", why); continue; }
        }

        match input.trim().chars().nth(0) {
            Some(command) => {
                match command {
                    'N' => { move_exp_north(&mut _data, board); break; },
                    'S' => { move_exp_south(&mut _data, board); break; },
                    'E' => { move_exp_east(&mut _data, board); break; },
                    'W' => { move_exp_west(&mut _data, board); break; },
                    'T' => if teleport_exp(&mut _data, board) { break; }
                           else { println!("Cannot teleport"); },
                    _ => println!("Invalid command")
                }
            },
            None => println!("Ignoring leading whitespace")
        }
    }

    _data
}

fn dir_to_dx_dy(direction: &Direction) -> (i32, i32) {
    match *direction {
        Direction::North => (0, 1),
        Direction::South => (0, -1),
        Direction::East => (1, 0),
        Direction::West => (-1, 0)
    }
}

fn move_gnome(data: GnomeData, board: &Board) -> GnomeData {
    let pos = data.pos;
    let choices = [Direction::North, Direction::South, Direction::East, Direction::West];
    let (dx, dy) : (i32, i32);

    loop {
        let index = rand::thread_rng().gen_range(0, choices.len());
        let (_dx, _dy) = dir_to_dx_dy(&choices[index]);

        if board::move_in_bounds(&pos, &_dx, &_dy, board) {
            dx = _dx;
            dy = _dy;
            break;
        }
    }

    GnomeData { pos: board::move_pos(pos, dx, dy, board),
                energy: data.energy - 1,
                things: data.things }
}

fn move_lep(data: LeprechaunData, board: &Board) -> LeprechaunData {
    let mut _data = data;

    teleport_lep(&mut _data, board);

    _data
}   

// TODO
fn teleport_lep(data: &mut LeprechaunData, board: &Board) {
}

// TODO
fn teleport_exp(data: &mut ExplorerData, board: &Board) -> bool {
    false
}

// TODO
fn is_opening(room: &Position, wall: &Direction, board: &Board) -> bool {
    false
}

// TODO
fn move_exp_north(data: &mut ExplorerData, board: &Board) {
}

// TODO
fn move_exp_south(data: &mut ExplorerData, board: &Board) {
}

// TODO
fn move_exp_east(data: &mut ExplorerData, board: &Board) {
}

// TODO
fn move_exp_west(data: &mut ExplorerData, board: &Board) {
}

// TODO
fn find_word<'a, 'b>(word: &'a String, things: &'b Vec<Thing>) -> Option<&'b Thing> {
    None
}

// TODO
fn is_magic_word(word: &Thing, board: &Board) -> bool {
    false
}

// TODO
fn open_sesame(word: &Thing, board: &Board) -> bool {
    false
}
