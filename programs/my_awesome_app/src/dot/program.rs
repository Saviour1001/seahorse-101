#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct Note {
    pub owner: Pubkey,
    pub index: u8,
    pub title: String,
    pub content: String,
}

impl<'info, 'entrypoint> Note {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedNote<'info, 'entrypoint>> {
        let owner = account.owner.clone();
        let index = account.index;
        let title = account.title.clone();
        let content = account.content.clone();

        Mutable::new(LoadedNote {
            __account__: account,
            __programs__: programs_map,
            owner,
            index,
            title,
            content,
        })
    }

    pub fn store(loaded: Mutable<LoadedNote>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let index = loaded.index;

        loaded.__account__.index = index;

        let title = loaded.title.clone();

        loaded.__account__.title = title;

        let content = loaded.content.clone();

        loaded.__account__.content = content;
    }
}

#[derive(Debug)]
pub struct LoadedNote<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Note>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
    pub index: u8,
    pub title: String,
    pub content: String,
}

#[account]
#[derive(Debug)]
pub struct User {
    pub owner: Pubkey,
    pub note_count: u8,
    pub last_note: u8,
}

impl<'info, 'entrypoint> User {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedUser<'info, 'entrypoint>> {
        let owner = account.owner.clone();
        let note_count = account.note_count;
        let last_note = account.last_note;

        Mutable::new(LoadedUser {
            __account__: account,
            __programs__: programs_map,
            owner,
            note_count,
            last_note,
        })
    }

    pub fn store(loaded: Mutable<LoadedUser>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let note_count = loaded.note_count;

        loaded.__account__.note_count = note_count;

        let last_note = loaded.last_note;

        loaded.__account__.last_note = last_note;
    }
}

#[derive(Debug)]
pub struct LoadedUser<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, User>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
    pub note_count: u8,
    pub last_note: u8,
}

pub fn create_note_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut user: Mutable<LoadedUser<'info, '_>>,
    mut note: Empty<Mutable<LoadedNote<'info, '_>>>,
    mut title: String,
    mut content: String,
) -> () {
    let mut note = note.account.clone();

    assign!(note.borrow_mut().owner, owner.key());

    assign!(note.borrow_mut().index, user.borrow().note_count);

    assign!(note.borrow_mut().title, title);

    assign!(note.borrow_mut().content, content);

    assign!(user.borrow_mut().note_count, user.borrow().note_count + 1);

    assign!(user.borrow_mut().last_note, note.borrow().index);
}

pub fn delete_note_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut note: Mutable<LoadedNote<'info, '_>>,
) -> () {
    if !(note.borrow().owner == owner.key()) {
        panic!("You are not the owner of this note");
    }

    assign!(note.borrow_mut().content, "".to_string());

    assign!(note.borrow_mut().title, "".to_string());
}

pub fn init_user_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut user: Empty<Mutable<LoadedUser<'info, '_>>>,
) -> () {
    let mut user = user.account.clone();

    assign!(user.borrow_mut().owner, owner.key());

    assign!(user.borrow_mut().note_count, 0);

    assign!(user.borrow_mut().last_note, 0);
}

pub fn update_note_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut noteIndex: u8,
    mut note: Mutable<LoadedNote<'info, '_>>,
    mut title: String,
    mut content: String,
) -> () {
    if !(note.borrow().owner == owner.key()) {
        panic!("You are not the owner of this note");
    }

    assign!(note.borrow_mut().title, title);

    assign!(note.borrow_mut().content, content);
}
