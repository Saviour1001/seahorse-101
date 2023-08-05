#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{id, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct TodoAccount {
    pub owner: Pubkey,
    pub idx: u8,
    pub content: String,
    pub marked: bool,
}

impl<'info, 'entrypoint> TodoAccount {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedTodoAccount<'info, 'entrypoint>> {
        let owner = account.owner.clone();
        let idx = account.idx;
        let content = account.content.clone();
        let marked = account.marked.clone();

        Mutable::new(LoadedTodoAccount {
            __account__: account,
            __programs__: programs_map,
            owner,
            idx,
            content,
            marked,
        })
    }

    pub fn store(loaded: Mutable<LoadedTodoAccount>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let idx = loaded.idx;

        loaded.__account__.idx = idx;

        let content = loaded.content.clone();

        loaded.__account__.content = content;

        let marked = loaded.marked.clone();

        loaded.__account__.marked = marked;
    }
}

#[derive(Debug)]
pub struct LoadedTodoAccount<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, TodoAccount>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
    pub idx: u8,
    pub content: String,
    pub marked: bool,
}

#[account]
#[derive(Debug)]
pub struct UserProfile {
    pub owner: Pubkey,
    pub last_todo: u8,
    pub todo_count: u8,
}

impl<'info, 'entrypoint> UserProfile {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedUserProfile<'info, 'entrypoint>> {
        let owner = account.owner.clone();
        let last_todo = account.last_todo;
        let todo_count = account.todo_count;

        Mutable::new(LoadedUserProfile {
            __account__: account,
            __programs__: programs_map,
            owner,
            last_todo,
            todo_count,
        })
    }

    pub fn store(loaded: Mutable<LoadedUserProfile>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let last_todo = loaded.last_todo;

        loaded.__account__.last_todo = last_todo;

        let todo_count = loaded.todo_count;

        loaded.__account__.todo_count = todo_count;
    }
}

#[derive(Debug)]
pub struct LoadedUserProfile<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, UserProfile>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
    pub last_todo: u8,
    pub todo_count: u8,
}

pub fn add_task_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut userprofile: Mutable<LoadedUserProfile<'info, '_>>,
    mut todoaccount: Empty<Mutable<LoadedTodoAccount<'info, '_>>>,
    mut content: String,
) -> () {
    let mut todoaccount = todoaccount.account.clone();

    assign!(todoaccount.borrow_mut().content, content);

    assign!(todoaccount.borrow_mut().idx, userprofile.borrow().last_todo);

    assign!(todoaccount.borrow_mut().owner, owner.key());

    assign!(
        userprofile.borrow_mut().last_todo,
        userprofile.borrow().last_todo + 1
    );

    assign!(
        userprofile.borrow_mut().todo_count,
        userprofile.borrow().todo_count + 1
    );
}

pub fn edit_task_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut todoIndex: u8,
    mut userprofile: Mutable<LoadedUserProfile<'info, '_>>,
    mut todoaccount: Mutable<LoadedTodoAccount<'info, '_>>,
    mut content: String,
) -> () {
    assign!(todoaccount.borrow_mut().content, content);
}

pub fn init_userprofile_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut userprofile: Empty<Mutable<LoadedUserProfile<'info, '_>>>,
) -> () {
    let mut userprofile = userprofile.account.clone();

    assign!(userprofile.borrow_mut().owner, owner.key());

    assign!(userprofile.borrow_mut().last_todo, 0);

    assign!(userprofile.borrow_mut().todo_count, 0);
}

pub fn mark_task_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut todoIndex: u8,
    mut userprofile: Mutable<LoadedUserProfile<'info, '_>>,
    mut todoaccount: Mutable<LoadedTodoAccount<'info, '_>>,
) -> () {
    assign!(todoaccount.borrow_mut().marked, true);

    solana_program::msg!("{}", todoaccount.borrow().marked);
}

pub fn unmark_task_handler<'info>(
    mut owner: SeahorseSigner<'info, '_>,
    mut todoIndex: u8,
    mut userprofile: Mutable<LoadedUserProfile<'info, '_>>,
    mut todoaccount: Mutable<LoadedTodoAccount<'info, '_>>,
) -> () {
    assign!(todoaccount.borrow_mut().marked, false);

    solana_program::msg!("{}", todoaccount.borrow().marked);
}
