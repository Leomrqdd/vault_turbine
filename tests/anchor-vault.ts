import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";

describe("anchor-vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.anchorVault as Program<AnchorVault>;

  const signer = new anchor.web3.Keypair()

  const [vaultState, stateBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault_state"),signer.publicKey.toBuffer()],
    program.programId
  )
  
  const [vault, vaultBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"),vaultState.toBuffer()],
    program.programId
  )



before(async() => {
  console.log("Airdropping to signer", signer.publicKey.toBase58());
  await provider.connection.requestAirdrop(signer.publicKey,100*anchor.web3.LAMPORTS_PER_SOL)
  await new Promise(resolve => setTimeout(resolve, 1000));
  console.log("Airdropped to signer", signer.publicKey.toBase58());
  console.log("User balance", (await provider.connection.getBalance(signer.publicKey))/anchor.web3.LAMPORTS_PER_SOL);
})

  it("Initialize the vault!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accountsStrict({
      user: signer.publicKey,
      vaultState:vaultState,
      vault:vault,
      systemProgram:anchor.web3.SystemProgram.programId,
    }).signers([signer]).rpc();

    console.log("Transaction signature", tx);
  });


  it("Deposit to the vault!", async () => {
    const amount = 1*anchor.web3.LAMPORTS_PER_SOL;
    const tx = await program.methods.deposit(new anchor.BN(amount)).accountsStrict({
      user: signer.publicKey,
      vaultState: vaultState,
      vault: vault,
      systemProgram:anchor.web3.SystemProgram.programId,
    }).signers([signer]).rpc();

    console.log("Transaction signature", tx);
  })
});
