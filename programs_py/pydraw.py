# py-draw-with-frens
# Built with Seahorse v0.1.1

from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')

class Pixel(Account):
  pos_x: u8
  pos_y: u8
  col_r: u8
  col_g: u8
  col_b: u8
  bump: u8

@instruction
def create_pixel(
  pixel: Empty[Pixel],
  user: Signer,
  pos_x: u8,
  pos_y: u8,
  init_col_r: u8,
  init_col_g: u8,
  init_col_b: u8
):
  # Initialise pixel account
  pixel_account = pixel.init(
    payer = user,
    seeds = ["pixel", pos_x, pos_y]
  )

  # Validation
  MIN_POS: u8 = 0
  MAX_POS: u8 = 99
  MIN_COL: u8 = 0
  MAX_COL: u8 = 255


  assert MIN_POS <= pos_x and pos_x <= MAX_POS, 'The given X co-ordinate is not between 0-99'
  assert MIN_POS <= pos_y and pos_y <= MAX_POS, 'The given Y co-ordinate is not between 0-99'
  assert MIN_COL <= init_col_r and init_col_r <= MAX_COL, 'The given red colour is not between 0-255'
  assert MIN_COL <= init_col_g and init_col_g <= MAX_COL, 'The given green colour is not between 0-255'
  assert MIN_COL <= init_col_b  and init_col_b <= MAX_COL, 'The given blue colour is not between 0-255'

  # Set pixel fields
  pixel_account.pos_x = pos_x
  pixel_account.pos_y = pos_y
  pixel_account.col_r = init_col_r
  pixel_account.col_g = init_col_g
  pixel_account.col_b = init_col_b
  pixel_account.bump = pixel.bump()

  # TODO: emit event!


@instruction
def update_pixel(
  pixel: Pixel,
  new_col_r: u8,
  new_col_g: u8,
  new_col_b: u8
):
  # Validation
  MIN_COL: u8 = 0
  MAX_COL: u8 = 255

  assert MIN_COL <= new_col_r and new_col_r <= MAX_COL, 'The given red colour is not between 0-255'
  assert MIN_COL <= new_col_g and new_col_g <= MAX_COL, 'The given green colour is not between 0-255'
  assert MIN_COL <= new_col_b  and new_col_b <= MAX_COL, 'The given blue colour is not between 0-255'

  # Update pixel fields
  pixel.col_r = new_col_r
  pixel.col_g = new_col_g
  pixel.col_b = new_col_b

  # TODO: emit event!