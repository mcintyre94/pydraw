import { Program } from "@project-serum/anchor";
import { program } from "@project-serum/anchor/dist/cjs/spl/associated-token";
import { PublicKey } from "@solana/web3.js";
import { Pydraw } from "../idl/pydraw";

const getPixelAddress = (posX: number, posY: number, program: Program<Pydraw>) => {
  const [pixelPublicKey] = PublicKey.findProgramAddressSync(
    [Buffer.from("pixel"), Buffer.from([posX, posY])],
    program.programId,
  )
  return pixelPublicKey
}

export async function pixelUpdated(posX: number, posY: number, setFetchedPixels: any, program: Program<Pydraw>) {
  const pixelAddress = await getPixelAddress(posX, posY, program);
  const updatedPixelAccount = await program.account.pixel.fetch(pixelAddress);

  // Update the state
  setFetchedPixels(pixels => {
    const newPixels = [...pixels];
    const index = newPixels.findIndex(p => p.posX === posX && p.posY === posY);
    if (index >= 0) {
      // We already have pixel data at this position, so replace it
      newPixels[index] = updatedPixelAccount;
    } else {
      // We don't have pixel data at this position, so add it
      newPixels.push(updatedPixelAccount);
    }
    return newPixels;
  })
}