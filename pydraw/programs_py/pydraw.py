# py-draw-with-frens
# Built with Seahorse v0.1.1

from seahorse.prelude import *

declare_id('Cncq6qR9Hd1xLveR6Z5vJWUkWYBPo75mqwHEfK2LRS17')


class Pixel(Account):
    pos_x: u8
    pos_y: u8
    col_r: u8
    col_g: u8
    col_b: u8
    bump: u8


class PixelChanged(Event):
    pos_x: u8
    pos_y: u8
    col_r: u8
    col_g: u8
    col_b: u8

    def __init__(
        self,
        pos_x: u8,
        pos_y: u8,
        col_r: u8,
        col_g: u8,
        col_b: u8
    ):
        self.pos_x = pos_x
        self.pos_y = pos_y
        self.col_r = col_r
        self.col_g = col_g
        self.col_b = col_b


def validate_pos(x: u8, y: u8):
    min_pos = 0
    max_pos = 99
    assert min_pos <= x and x <= max_pos, 'The given X co-ordinate is not between 0-99'
    assert min_pos <= y and y <= max_pos, 'The given Y co-ordinate is not between 0-99'


def validate_col(r: u8, g: u8, b: u8):
    min_col = 0
    max_col = 255
    assert min_col <= r and r <= max_col, 'The given red colour is not between 0-255'
    assert min_col <= g and g <= max_col, 'The given green colour is not between 0-255'
    assert min_col <= b and b <= max_col, 'The given blue colour is not between 0-255'


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
    # validation
    validate_pos(pos_x, pos_y)
    validate_col(init_col_r, init_col_g, init_col_b)

    # Initialise pixel account
    pixel_account = pixel.init(
        payer=user,
        seeds=["pixel", pos_x, pos_y]
    )

    # Set pixel fields
    pixel_account.pos_x = pos_x
    pixel_account.pos_y = pos_y
    pixel_account.col_r = init_col_r
    pixel_account.col_g = init_col_g
    pixel_account.col_b = init_col_b
    pixel_account.bump = pixel.bump()

    # emit event
    change_event = PixelChanged(
        pos_x,
        pos_y,
        init_col_r,
        init_col_g,
        init_col_b
    )
    change_event.emit()


@instruction
def update_pixel(
    pixel: Pixel,
    new_col_r: u8,
    new_col_g: u8,
    new_col_b: u8
):
    # Validation
    validate_col(new_col_r, new_col_g, new_col_b)

    # Update pixel fields
    pixel.col_r = new_col_r
    pixel.col_g = new_col_g
    pixel.col_b = new_col_b

    # emit event
    change_event = PixelChanged(
        pixel.pos_x,
        pixel.pos_y,
        new_col_r,
        new_col_g,
        new_col_b
    )
    change_event.emit()
