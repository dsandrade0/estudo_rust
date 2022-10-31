
use serde_json::{Map, Value};
use crate::todo::ItemTypes;
use crate::todo::structs::done::Done;
use crate::todo::structs::traits::create::Create;
use crate::todo::structs::traits::delete::Delete;
use crate::todo::structs::traits::edit::Edit;
use crate::todo::structs::traits::get::Get;
use super::todo::structs::pending::Pending;


fn process_pending(item: Pending, command: &str, state: &Map<String, Value>) {
    let mut state = state.clone();
    println!("Processing pending...");
    println!("Status - {}", &item.super_struct.status);

    match command {
        "get" => item.get(&item.super_struct.title, &state),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        "create" => item.create(&item.super_struct.title, &item.super_struct.status, &mut state),
        "edit" => item.set_to_done(&item.super_struct.title, &mut state),
        _ => println!("Command {} not supported", command)
    }
}

fn process_done(item: Done, command: &str, state: &Map<String, Value>) {
    let mut state = state.clone();

    match command {
        "get" => item.get(&item.super_struct.title, &state),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        "edit" => item.set_to_pending(&item.super_struct.title, &mut state),
        _ => println!("Command {} not supported", command)
    }
}

pub fn process_input(item: ItemTypes, command: &str, state: &Map<String, Value>) {
    match item {
        ItemTypes::Pending(item) => process_pending(item, command, state),
        ItemTypes::Done(item) => process_done(item, command, state)
    }
}