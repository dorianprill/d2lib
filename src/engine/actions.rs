use std::mem::transmute;

use game::coordinate::*;
use game::Player::*;

#[repr(u8)]
enum Action {
    Walk                    = 0x01,
    Run                     = 0x03,
    Relocate                = 0x5f,
    CastOnCoord             = 0x0c,
    CastOnObject            = 0x0d,
    RequestReassignment     = 0x4b,
    TerminateEntityDialogue = 0x30,
    MakeEntityMove          = 0x59,
    SwitchSkill             = 0x3c,
    DrinkPotion             = 0x26
}

fn u32_to_byte_array(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >>  8) & 0xff) as u8;
    let b4 : u8 =        (x  & 0xff) as u8;
    return [b1, b2, b3, b4]
}

fn u16_to_byte_array(x:u32) -> [u8;2] {
    let b1 : u8 = ((x >>  8) & 0xff) as u8;
    let b2 : u8 =        (x  & 0xff) as u8;
    return [b1, b2]
}


fn build_packet(command: u8, args: &[u8]) -> &[u8] {
    let packet: Vec<u8> = new Vec<u8>;
    let boxed:  Box<[u8]> = packet.into_boxed_slice();
    boxed.push(command);
    boxed.extend_from_slice(args);
    return boxed;
}

pub fn walk(coords: Coordinate) -> &[u8] {
    return build_packet(Action::Walk,
                        u16_to_byte_array(coords.x),
                        u16_to_byte_array(coords.y));
}

pub fn run(coords: Coordinate) -> &[u8] {
    return build_packet(Action::Run,
                        u16_to_byte_array(coords.x),
                        u16_to_byte_array(coords.y));
}

pub fn relocate(coords: Coordinate) -> &[u8] {
    return build_packet(Action::Relocate,
                        u16_to_byte_array(coords.x),
                        u16_to_byte_array(coords.y));
}

pub fn cast_on_coord(coords: Coordinate) -> &[u8] {
    return build_packet(Action::CastOnCoord,
                        u16_to_byte_array(coords.x),
                        u16_to_byte_array(coords.y));
}

pub fn cast_on_player(player: Player) -> &[u8] {
    return build_packet(Action::CastOnCoord,
                        u16_to_byte_array(player.entity.location.x),
                        u16_to_byte_array(player.entity.location.y));
}

pub fn cast_on_object(id: u32) -> &[u8] {
    return build_packet(Action::CastOnObject,
                        Connections.GenericDispatcher.one,
                        u32_to_byte_array(id));
}

pub fn request_reassignment(id: u32) -> &[u8] {
    return build_packet(Action::RequestReassignment,
                        Connections.GenericDispatcher.nulls,
                        u32_to_byte_array(id));
}

pub fn terminate_entity_dialogue(id: u32) -> &[u8] {
    return build_packet(Action::TerminateEntityDialogue,
                        Connections.GenericDispatcher.one,
                        u32_to_byte_array(id));
}

pub fn make_entity_move(id: u32, coords: Coordinate) -> &[u8] {
    return build_packet(Action::MakeEntityMove,
                        Connections.GenericDispatcher.one,
                        u32_to_byte_array(id),
                        u32_to_byte_array(coords.x),
                        Connections.GenericDispatcher.zero,
                        Connections.GenericDispatcher.zero,
                        u32_to_byte_array(coords.y),
                        Connections.GenericDispatcher.zero,
                        Connections.GenericDispatcher.zero);
}

pub fn switch_skill(skill: u32) -> &[u8] {
    let temp: [u8] = { 0xFF, 0xFF, 0xFF, 0xFF };
    //little endian on linux and win but NOT mac32!
    //let bytes: [u8; 4] = unsafe { transmute(skill.to_le()) };
    return build_packet(Action::SwitchSkill, u32_to_byte_array(id), temp);
}

pub fn drink_potion(id: u32) -> &[u8] {
    return build_packet(Action::DrinkPotion,
                        u32_to_byte_array(id),
                        Connections.GenericDispatcher.nulls,
                        Connections.GenericDispatcher.nulls);
}