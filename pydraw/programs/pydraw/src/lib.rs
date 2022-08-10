use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::token;
use std::convert::TryFrom;

declare_id!("Cncq6qR9Hd1xLveR6Z5vJWUkWYBPo75mqwHEfK2LRS17");

#[derive(Debug)]
#[account]
pub struct Pixel {
    pos_x: u8,
    pos_y: u8,
    col_r: u8,
    col_g: u8,
    col_b: u8,
    bump: u8,
}

pub fn create_pixel_handler(
    mut ctx: Context<CreatePixel>,
    mut pos_x: u8,
    mut pos_y: u8,
    mut init_col_r: u8,
    mut init_col_g: u8,
    mut init_col_b: u8,
) -> Result<()> {
    let mut pixel = &mut ctx.accounts.pixel;
    let mut user = &mut ctx.accounts.user;
    let mut pixel_account = pixel;
    let mut MIN_POS: u8 = <u8 as TryFrom<_>>::try_from(0).unwrap();
    let mut MAX_POS: u8 = <u8 as TryFrom<_>>::try_from(99).unwrap();
    let mut MIN_COL: u8 = <u8 as TryFrom<_>>::try_from(0).unwrap();
    let mut MAX_COL: u8 = <u8 as TryFrom<_>>::try_from(255).unwrap();

    require!((MIN_POS <= pos_x) && (pos_x <= MAX_POS), ProgramError::E000);

    require!((MIN_POS <= pos_y) && (pos_y <= MAX_POS), ProgramError::E001);

    require!(
        (MIN_COL <= init_col_r) && (init_col_r <= MAX_COL),
        ProgramError::E002
    );

    require!(
        (MIN_COL <= init_col_g) && (init_col_g <= MAX_COL),
        ProgramError::E003
    );

    require!(
        (MIN_COL <= init_col_b) && (init_col_b <= MAX_COL),
        ProgramError::E004
    );

    pixel_account.pos_x = pos_x;

    pixel_account.pos_y = pos_y;

    pixel_account.col_r = init_col_r;

    pixel_account.col_g = init_col_g;

    pixel_account.col_b = init_col_b;

    pixel_account.bump = *ctx.bumps.get("pixel").unwrap();

    Ok(())
}

pub fn update_pixel_handler(
    mut ctx: Context<UpdatePixel>,
    mut new_col_r: u8,
    mut new_col_g: u8,
    mut new_col_b: u8,
) -> Result<()> {
    let mut pixel = &mut ctx.accounts.pixel;
    let mut MIN_COL: u8 = <u8 as TryFrom<_>>::try_from(0).unwrap();
    let mut MAX_COL: u8 = <u8 as TryFrom<_>>::try_from(255).unwrap();

    require!(
        (MIN_COL <= new_col_r) && (new_col_r <= MAX_COL),
        ProgramError::E002
    );

    require!(
        (MIN_COL <= new_col_g) && (new_col_g <= MAX_COL),
        ProgramError::E003
    );

    require!(
        (MIN_COL <= new_col_b) && (new_col_b <= MAX_COL),
        ProgramError::E004
    );

    pixel.col_r = new_col_r;

    pixel.col_g = new_col_g;

    pixel.col_b = new_col_b;

    Ok(())
}

#[derive(Accounts)]
# [instruction (pos_x : u8 , pos_y : u8)]
pub struct CreatePixel<'info> {
    #[account(
        init,
        payer = user,
        seeds = [
            "pixel".as_bytes().as_ref(),
            pos_x.to_le_bytes().as_ref(),
            pos_y.to_le_bytes().as_ref()
        ],
        bump,
        space = 8 + std::mem::size_of::<Pixel>()
    )]
    pub pixel: Box<Account<'info, Pixel>>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePixel<'info> {
    #[account(mut)]
    pub pixel: Box<Account<'info, Pixel>>,
}

#[program]
pub mod pydraw {
    use super::*;

    pub fn create_pixel(
        ctx: Context<CreatePixel>,
        pos_x: u8,
        pos_y: u8,
        init_col_r: u8,
        init_col_g: u8,
        init_col_b: u8,
    ) -> Result<()> {
        create_pixel_handler(ctx, pos_x, pos_y, init_col_r, init_col_g, init_col_b)
    }

    pub fn update_pixel(
        ctx: Context<UpdatePixel>,
        new_col_r: u8,
        new_col_g: u8,
        new_col_b: u8,
    ) -> Result<()> {
        update_pixel_handler(ctx, new_col_r, new_col_g, new_col_b)
    }
}

#[error_code]
pub enum ProgramError {
    #[msg("The given X co-ordinate is not between 0-99")]
    E000,
    #[msg("The given Y co-ordinate is not between 0-99")]
    E001,
    #[msg("The given red colour is not between 0-255")]
    E002,
    #[msg("The given green colour is not between 0-255")]
    E003,
    #[msg("The given blue colour is not between 0-255")]
    E004,
}
