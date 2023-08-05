#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub mod dot;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token::{self, Mint, Token, TokenAccount},
};

use dot::program::*;
use std::{cell::RefCell, rc::Rc};

declare_id!("BAv1R7yGTcga43cjmsHMcZsXTsLTxdK4UT4nMg8qzhTh");

pub mod seahorse_util {
    use super::*;

    #[cfg(feature = "pyth-sdk-solana")]
    pub use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed};
    use std::{collections::HashMap, fmt::Debug, ops::Deref};

    pub struct Mutable<T>(Rc<RefCell<T>>);

    impl<T> Mutable<T> {
        pub fn new(obj: T) -> Self {
            Self(Rc::new(RefCell::new(obj)))
        }
    }

    impl<T> Clone for Mutable<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<T> Deref for Mutable<T> {
        type Target = Rc<RefCell<T>>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: Debug> Debug for Mutable<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl<T: Default> Default for Mutable<T> {
        fn default() -> Self {
            Self::new(T::default())
        }
    }

    impl<T: Clone> Mutable<Vec<T>> {
        pub fn wrapped_index(&self, mut index: i128) -> usize {
            if index >= 0 {
                return index.try_into().unwrap();
            }

            index += self.borrow().len() as i128;

            return index.try_into().unwrap();
        }
    }

    impl<T: Clone, const N: usize> Mutable<[T; N]> {
        pub fn wrapped_index(&self, mut index: i128) -> usize {
            if index >= 0 {
                return index.try_into().unwrap();
            }

            index += self.borrow().len() as i128;

            return index.try_into().unwrap();
        }
    }

    #[derive(Clone)]
    pub struct Empty<T: Clone> {
        pub account: T,
        pub bump: Option<u8>,
    }

    #[derive(Clone, Debug)]
    pub struct ProgramsMap<'info>(pub HashMap<&'static str, AccountInfo<'info>>);

    impl<'info> ProgramsMap<'info> {
        pub fn get(&self, name: &'static str) -> AccountInfo<'info> {
            self.0.get(name).unwrap().clone()
        }
    }

    #[derive(Clone, Debug)]
    pub struct WithPrograms<'info, 'entrypoint, A> {
        pub account: &'entrypoint A,
        pub programs: &'entrypoint ProgramsMap<'info>,
    }

    impl<'info, 'entrypoint, A> Deref for WithPrograms<'info, 'entrypoint, A> {
        type Target = A;

        fn deref(&self) -> &Self::Target {
            &self.account
        }
    }

    pub type SeahorseAccount<'info, 'entrypoint, A> =
        WithPrograms<'info, 'entrypoint, Box<Account<'info, A>>>;

    pub type SeahorseSigner<'info, 'entrypoint> = WithPrograms<'info, 'entrypoint, Signer<'info>>;

    #[derive(Clone, Debug)]
    pub struct CpiAccount<'info> {
        #[doc = "CHECK: CpiAccounts temporarily store AccountInfos."]
        pub account_info: AccountInfo<'info>,
        pub is_writable: bool,
        pub is_signer: bool,
        pub seeds: Option<Vec<Vec<u8>>>,
    }

    #[macro_export]
    macro_rules! seahorse_const {
        ($ name : ident , $ value : expr) => {
            macro_rules! $name {
                () => {
                    $value
                };
            }

            pub(crate) use $name;
        };
    }

    #[macro_export]
    macro_rules! assign {
        ($ lval : expr , $ rval : expr) => {{
            let temp = $rval;

            $lval = temp;
        }};
    }

    #[macro_export]
    macro_rules! index_assign {
        ($ lval : expr , $ idx : expr , $ rval : expr) => {
            let temp_rval = $rval;
            let temp_idx = $idx;

            $lval[temp_idx] = temp_rval;
        };
    }

    pub(crate) use assign;

    pub(crate) use index_assign;

    pub(crate) use seahorse_const;
}

#[program]
mod my_awesome_app {
    use super::*;
    use seahorse_util::*;
    use std::collections::HashMap;

    #[derive(Accounts)]
    # [instruction (content : String)]
    pub struct AddTask<'info> {
        #[account(mut)]
        pub owner: Signer<'info>,
        #[account(mut)]
        pub userprofile: Box<Account<'info, dot::program::UserProfile>>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: TodoAccount > () + 8 , payer = owner , seeds = ["todoaccount" . as_bytes () . as_ref () , owner . key () . as_ref () , userprofile . last_todo . to_le_bytes () . as_ref ()] , bump)]
        pub todoaccount: Box<Account<'info, dot::program::TodoAccount>>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    pub fn add_task(ctx: Context<AddTask>, content: String) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let owner = SeahorseSigner {
            account: &ctx.accounts.owner,
            programs: &programs_map,
        };

        let userprofile =
            dot::program::UserProfile::load(&mut ctx.accounts.userprofile, &programs_map);

        let todoaccount = Empty {
            account: dot::program::TodoAccount::load(&mut ctx.accounts.todoaccount, &programs_map),
            bump: ctx.bumps.get("todoaccount").map(|bump| *bump),
        };

        add_task_handler(
            owner.clone(),
            userprofile.clone(),
            todoaccount.clone(),
            content,
        );

        dot::program::UserProfile::store(userprofile);

        dot::program::TodoAccount::store(todoaccount.account);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (todoIndex : u8 , content : String)]
    pub struct EditTask<'info> {
        #[account(mut)]
        pub owner: Signer<'info>,
        #[account(mut)]
        pub userprofile: Box<Account<'info, dot::program::UserProfile>>,
        #[account(mut)]
        pub todoaccount: Box<Account<'info, dot::program::TodoAccount>>,
    }

    pub fn edit_task(ctx: Context<EditTask>, todoIndex: u8, content: String) -> Result<()> {
        let mut programs = HashMap::new();
        let programs_map = ProgramsMap(programs);
        let owner = SeahorseSigner {
            account: &ctx.accounts.owner,
            programs: &programs_map,
        };

        let userprofile =
            dot::program::UserProfile::load(&mut ctx.accounts.userprofile, &programs_map);

        let todoaccount =
            dot::program::TodoAccount::load(&mut ctx.accounts.todoaccount, &programs_map);

        edit_task_handler(
            owner.clone(),
            todoIndex,
            userprofile.clone(),
            todoaccount.clone(),
            content,
        );

        dot::program::UserProfile::store(userprofile);

        dot::program::TodoAccount::store(todoaccount);

        return Ok(());
    }

    #[derive(Accounts)]
    pub struct InitUserprofile<'info> {
        #[account(mut)]
        pub owner: Signer<'info>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: UserProfile > () + 8 , payer = owner , seeds = ["userprofile" . as_bytes () . as_ref () , owner . key () . as_ref ()] , bump)]
        pub userprofile: Box<Account<'info, dot::program::UserProfile>>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    pub fn init_userprofile(ctx: Context<InitUserprofile>) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let owner = SeahorseSigner {
            account: &ctx.accounts.owner,
            programs: &programs_map,
        };

        let userprofile = Empty {
            account: dot::program::UserProfile::load(&mut ctx.accounts.userprofile, &programs_map),
            bump: ctx.bumps.get("userprofile").map(|bump| *bump),
        };

        init_userprofile_handler(owner.clone(), userprofile.clone());

        dot::program::UserProfile::store(userprofile.account);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (todoIndex : u8)]
    pub struct MarkTask<'info> {
        #[account(mut)]
        pub owner: Signer<'info>,
        #[account(mut)]
        pub userprofile: Box<Account<'info, dot::program::UserProfile>>,
        #[account(mut)]
        pub todoaccount: Box<Account<'info, dot::program::TodoAccount>>,
    }

    pub fn mark_task(ctx: Context<MarkTask>, todoIndex: u8) -> Result<()> {
        let mut programs = HashMap::new();
        let programs_map = ProgramsMap(programs);
        let owner = SeahorseSigner {
            account: &ctx.accounts.owner,
            programs: &programs_map,
        };

        let userprofile =
            dot::program::UserProfile::load(&mut ctx.accounts.userprofile, &programs_map);

        let todoaccount =
            dot::program::TodoAccount::load(&mut ctx.accounts.todoaccount, &programs_map);

        mark_task_handler(
            owner.clone(),
            todoIndex,
            userprofile.clone(),
            todoaccount.clone(),
        );

        dot::program::UserProfile::store(userprofile);

        dot::program::TodoAccount::store(todoaccount);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (todoIndex : u8)]
    pub struct UnmarkTask<'info> {
        #[account(mut)]
        pub owner: Signer<'info>,
        #[account(mut)]
        pub userprofile: Box<Account<'info, dot::program::UserProfile>>,
        #[account(mut)]
        pub todoaccount: Box<Account<'info, dot::program::TodoAccount>>,
    }

    pub fn unmark_task(ctx: Context<UnmarkTask>, todoIndex: u8) -> Result<()> {
        let mut programs = HashMap::new();
        let programs_map = ProgramsMap(programs);
        let owner = SeahorseSigner {
            account: &ctx.accounts.owner,
            programs: &programs_map,
        };

        let userprofile =
            dot::program::UserProfile::load(&mut ctx.accounts.userprofile, &programs_map);

        let todoaccount =
            dot::program::TodoAccount::load(&mut ctx.accounts.todoaccount, &programs_map);

        unmark_task_handler(
            owner.clone(),
            todoIndex,
            userprofile.clone(),
            todoaccount.clone(),
        );

        dot::program::UserProfile::store(userprofile);

        dot::program::TodoAccount::store(todoaccount);

        return Ok(());
    }
}
