import ColorSelector from "../components/ColorSelector";
import Title from "../components/Title";
import { Color, colors } from "../lib/colors";
import { useMemo, useState } from "react";
import Canvas from "../components/Canvas";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { AnchorWallet, useAnchorWallet, useConnection } from "@solana/wallet-adapter-react";
import { AnchorProvider, Program } from "@project-serum/anchor";
import { Pydraw, IDL } from "../idl/pydraw";
import { PublicKey } from "@solana/web3.js";

export default function Home() {
  const [selectedColor, setSelectedColor] = useState<Color>(colors[0])

  const { connection } = useConnection();

  const anchorWallet: AnchorWallet | undefined = useAnchorWallet()

  const anchorProvider: AnchorProvider | undefined = useMemo(() => {
    if (anchorWallet) {
      return new AnchorProvider(connection, anchorWallet, { commitment: 'confirmed' })
    } else {
      return undefined
    }
  }, [connection, anchorWallet])

  // Make sure you use your program ID!
  const programId = new PublicKey("Cncq6qR9Hd1xLveR6Z5vJWUkWYBPo75mqwHEfK2LRS17");

  const anchorProgram: Program<Pydraw> | undefined = useMemo(() => {
    if (anchorProvider) {
      return new Program(IDL, programId, anchorProvider)
    } else {
      return undefined
    }
  }, [anchorProvider])

  return (
    <div className="flex flex-col items-stretch gap-8 px-4 pt-24 mx-auto max-w-max">
      <main className="flex flex-col gap-4">
        <Title>Pydraw</Title>

        <div className="basis-1/4">
          <WalletMultiButton className='!bg-gray-900 hover:scale-105' />
        </div>

        <ColorSelector selectedColor={selectedColor} setSelectedColor={setSelectedColor} />

        <Canvas program={anchorProgram} selectedColor={selectedColor} />
      </main>
    </div>
  )
}
