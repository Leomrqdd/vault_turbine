import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorError } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import { expect, assert } from "chai";

describe("anchor-vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.anchorVault as Program<AnchorVault>;

  const signer = new anchor.web3.Keypair()
  const user_2 = new anchor.web3.Keypair()

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
    const state = await program.account.vaultState.fetch(vaultState);
    expect(state.stateBump).to.equal(stateBump);
    expect(state.vaultBump).to.equal(vaultBump);
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
    assert.equal(await provider.connection.getBalance(vault), amount);
  })

  it("Should fail to withdraw if the user is not authorized!", async () => {
    try {
      const amount = 1*anchor.web3.LAMPORTS_PER_SOL;
      await program.methods.withdraw(new anchor.BN(amount)).accountsStrict({
        user:user_2.publicKey,
        vaultState:vaultState,
        vault:vault,
        systemProgram:anchor.web3.SystemProgram.programId,
      }).signers([user_2]).rpc();
    } catch (error) {
      expect(error).to.be.instanceOf(AnchorError);
      const anchorError = error as AnchorError;
      expect(anchorError.error.errorCode.code).to.equal("ConstraintSeeds");
      return;
    }
    assert.fail("Withdrawal should have failed");
  }
  );

  it("Withdraw from the vault!", async () => {
    const amount = 1*anchor.web3.LAMPORTS_PER_SOL;
    const tx = await program.methods.withdraw(new anchor.BN(amount)).accountsStrict({
      user:signer.publicKey,
      vaultState:vaultState,
      vault:vault,
      systemProgram:anchor.web3.SystemProgram.programId,
    }).signers([signer]).rpc();

    console.log("Transaction signature", tx);
    assert.equal(await provider.connection.getBalance(vault), 0);
  })



  it("Close the vault!", async () => {
    const tx = await program.methods.close().accountsStrict({
      user:signer.publicKey,
      vaultState:vaultState,
      vault:vault,
      systemProgram:anchor.web3.SystemProgram.programId,
    }).signers([signer]).rpc();

    assert.equal(await provider.connection.getBalance(vaultState),0);
    assert.equal(await provider.connection.getBalance(vault),0);
    expect(await provider.connection.getAccountInfo(vaultState)).to.be.null;
    
    console.log("Transaction signature", tx);
  })
});
