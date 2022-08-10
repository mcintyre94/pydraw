# Pydraw

This is a clone of the tutorial app https://github.com/pointer-gg/anchor-tutorial with Seahorse

It's a collaborative pixel art app on Solana

## Programs

There is one program, `pydraw`. Its source code is at **programs_py/pydraw.py**

The rust source for it, **programs/pydraw/src/lib.rs** is build by running `seahorse build`.

From then on Anchor can be used as normal.

## Tests

Tests are in the **tests/** directory, and are normal Anchor tests. The test suite is copied from the original tutorial app.

## Differences from the original

- It doesn't have events yet: https://github.com/ameliatastic/seahorse-lang/issues/1

- The program name is changed because it needs to be snake case for Seahorse (was `draw-with-frens`)

- We're not checking the precise PDA seeds + bump on the update instruction. But this is ok because it's still required to be a `Pixel` account and the seeds we're checking are stored on-chain in that account anyway. Basically if `create_pixel` works correctly then `update_pixel` doesn't need to check the account beyond making sure it's a `Pixel`.

- The PDA seeds are different, because Seahorse doesn't allow lists yet.

Original (Rust):
`seeds = [b"pixel".as_ref(), [pos_x, pos_y].as_ref()],`

Original (TS):
`[Buffer.from("pixel"), Buffer.from([x, y])],`

Seahorse (Python):
`seeds = ["pixel", pos_x, pos_y]`

Seahorse (TS):
`[Buffer.from("pixel"), Buffer.from([x]), Buffer.from([y])],`
