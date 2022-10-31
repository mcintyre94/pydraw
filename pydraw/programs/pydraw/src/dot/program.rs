#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{assign, index_assign, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct Pixel {
    pub pos_x: u8,
    pub pos_y: u8,
    pub col_r: u8,
    pub col_g: u8,
    pub col_b: u8,
    pub bump: u8,
}

impl<'info, 'entrypoint> Pixel {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedPixel<'info, 'entrypoint>> {
        let pos_x = account.pos_x;
        let pos_y = account.pos_y;
        let col_r = account.col_r;
        let col_g = account.col_g;
        let col_b = account.col_b;
        let bump = account.bump;

        Mutable::new(LoadedPixel {
            __account__: account,
            __programs__: programs_map,
            pos_x,
            pos_y,
            col_r,
            col_g,
            col_b,
            bump,
        })
    }

    pub fn store(loaded: Mutable<LoadedPixel>) {
        let mut loaded = loaded.borrow_mut();
        let pos_x = loaded.pos_x;

        loaded.__account__.pos_x = pos_x;

        let pos_y = loaded.pos_y;

        loaded.__account__.pos_y = pos_y;

        let col_r = loaded.col_r;

        loaded.__account__.col_r = col_r;

        let col_g = loaded.col_g;

        loaded.__account__.col_g = col_g;

        let col_b = loaded.col_b;

        loaded.__account__.col_b = col_b;

        let bump = loaded.bump;

        loaded.__account__.bump = bump;
    }
}

#[derive(Debug)]
pub struct LoadedPixel<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Pixel>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub pos_x: u8,
    pub pos_y: u8,
    pub col_r: u8,
    pub col_g: u8,
    pub col_b: u8,
    pub bump: u8,
}

#[event]
#[derive(Clone, Debug, Default)]
pub struct PixelChanged {
    pub pos_x: u8,
    pub pos_y: u8,
    pub col_r: u8,
    pub col_g: u8,
    pub col_b: u8,
}

impl Mutable<PixelChanged> {
    pub fn __init__(
        &self,
        mut pos_x: u8,
        mut pos_y: u8,
        mut col_r: u8,
        mut col_g: u8,
        mut col_b: u8,
    ) -> () {
        assign!(self.borrow_mut().pos_x, pos_x);

        assign!(self.borrow_mut().pos_y, pos_y);

        assign!(self.borrow_mut().col_r, col_r);

        assign!(self.borrow_mut().col_g, col_g);

        assign!(self.borrow_mut().col_b, col_b);
    }

    fn __emit__(&self) {
        emit!(PixelChanged {
            pos_x: self.borrow().pos_x,
            pos_y: self.borrow().pos_y,
            col_r: self.borrow().col_r,
            col_g: self.borrow().col_g,
            col_b: self.borrow().col_b
        })
    }
}

impl PixelChanged {
    pub fn __new__(pos_x: u8, pos_y: u8, col_r: u8, col_g: u8, col_b: u8) -> Mutable<Self> {
        let obj = Mutable::new(PixelChanged::default());

        obj.__init__(pos_x, pos_y, col_r, col_g, col_b);

        return obj;
    }
}

pub fn validate_pos(mut x: u8, mut y: u8) -> () {
    let mut min_pos = 0;
    let mut max_pos = 99;

    if !((min_pos <= x) && (x <= max_pos)) {
        panic!("The given X co-ordinate is not between 0-99");
    }

    if !((min_pos <= y) && (y <= max_pos)) {
        panic!("The given Y co-ordinate is not between 0-99");
    }
}

pub fn validate_col(mut r: u8, mut g: u8, mut b: u8) -> () {
    let mut min_col = 0;
    let mut max_col = 255;

    if !((min_col <= r) && (r <= max_col)) {
        panic!("The given red colour is not between 0-255");
    }

    if !((min_col <= g) && (g <= max_col)) {
        panic!("The given green colour is not between 0-255");
    }

    if !((min_col <= b) && (b <= max_col)) {
        panic!("The given blue colour is not between 0-255");
    }
}

pub fn update_pixel_handler<'info>(
    mut pixel: Mutable<LoadedPixel<'info, '_>>,
    mut new_col_r: u8,
    mut new_col_g: u8,
    mut new_col_b: u8,
) -> () {
    validate_col(new_col_r, new_col_g, new_col_b);

    assign!(pixel.borrow_mut().col_r, new_col_r);

    assign!(pixel.borrow_mut().col_g, new_col_g);

    assign!(pixel.borrow_mut().col_b, new_col_b);

    let mut change_event = PixelChanged::__new__(
        pixel.borrow().pos_x,
        pixel.borrow().pos_y,
        new_col_r,
        new_col_g,
        new_col_b,
    );

    change_event.__emit__();
}

pub fn create_pixel_handler<'info>(
    mut pixel: Empty<Mutable<LoadedPixel<'info, '_>>>,
    mut user: SeahorseSigner<'info, '_>,
    mut pos_x: u8,
    mut pos_y: u8,
    mut init_col_r: u8,
    mut init_col_g: u8,
    mut init_col_b: u8,
) -> () {
    validate_pos(pos_x, pos_y);

    validate_col(init_col_r, init_col_g, init_col_b);

    let mut pixel_account = pixel.account.clone();

    assign!(pixel_account.borrow_mut().pos_x, pos_x);

    assign!(pixel_account.borrow_mut().pos_y, pos_y);

    assign!(pixel_account.borrow_mut().col_r, init_col_r);

    assign!(pixel_account.borrow_mut().col_g, init_col_g);

    assign!(pixel_account.borrow_mut().col_b, init_col_b);

    assign!(pixel_account.borrow_mut().bump, pixel.bump.unwrap());

    let mut change_event = PixelChanged::__new__(pos_x, pos_y, init_col_r, init_col_g, init_col_b);

    change_event.__emit__();
}
