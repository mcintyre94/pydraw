import { IdlAccounts, Program } from "@project-serum/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { Pydraw } from "../idl/pydraw";
import { Color } from "../lib/colors";

type PixelAccount = IdlAccounts<Pydraw>['pixel']

interface Props {
  posX: number,
  posY: number,
  program: Program<Pydraw>,
  pixelData?: PixelAccount,
  selectedColor: Color,
}

export default function Pixel({ posX, posY, program, pixelData, selectedColor }: Props) {
  const { colR, colG, colB } = pixelData || {};
  const color = pixelData ? `rgb(${colR}, ${colG}, ${colB})` : "white"

  const getPixelAddress = () => {
    const [pixelPublicKey] = PublicKey.findProgramAddressSync(
      [Buffer.from("pixel"), Buffer.from([posX, posY])],
      program.programId,
    )
    return pixelPublicKey
  }

  const createPixel = async () => {
    await program.methods
      .createPixel(posX, posY, selectedColor.r, selectedColor.g, selectedColor.b)
      .accounts({
        pixel: getPixelAddress(),
        user: program.provider.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
  }

  const updatePixel = async () => {
    await program.methods
      .updatePixel(selectedColor.r, selectedColor.g, selectedColor.b)
      .accounts({
        pixel: getPixelAddress(),
      })
      .rpc();
  }

  return <td
    className="h-4 min-w-[1rem]"
    style={{ backgroundColor: color }}
    onClick={pixelData ? updatePixel : createPixel}
  />
}
