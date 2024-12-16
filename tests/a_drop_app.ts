import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey } from '@solana/web3.js';
// import { Program } from "@coral-xyz/anchor";
import { getAssociatedTokenAddressSync } from '@solana/spl-token';
import { AirdropPlatform } from "../target/types/a_drop_app";

describe("Airdrop Data!", () => {

  //accessed and set provider
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  //who is the payer? seems not to be a address from solana address cmd 
  const payer = provider.wallet as anchor.Wallet;

  //program to interact with
  const program = anchor.workspace.ADropApp as anchor.Program<AirdropPlatform>;

  //a connection let's see
  const connection = provider.connection;

  //airdropInfo PDA
  const [airdropPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("airdrop")],
    program.programId
  );

  //say no to this bro!
  //generate a keypair for airdropInfo account
  //const airdropInfoAccount = new Keypair();

  //we must have a created mint account with us 
  const mint_account_public_key = new PublicKey("EL2YDdsBFqse1nucZBDnLRdMCYQoVjgsKFCezxzAaYht");

  const recipient_public_key = program.programId;

  // Derive the associated token address account for the mint and payer.
  const senderTokenAddress = getAssociatedTokenAddressSync(mint_account_public_key, payer.publicKey); //ho skda payer kol token he na hove jida oh mint address use kr reha va
  //tan payer address dekhna bhut jruri hai k ke aaw

  //if recepient token account is being created? then will below line be same
  const recepientTokenAddress = getAssociatedTokenAddressSync(mint_account_public_key, program.programId);

  it('create the airdrop info account', async () => {

    //let's see if it is the address from solana address cmd
    console.log("Payer's public key : {}", payer.publicKey);

    //it is ok to generate a keyPair for airdropInfo account
    //but
    //we studied PDA as well
    //let's implement that
    //IMPLEMENTED.........................
    console.log("Airdrop info Account : {}", airdropPDA);

    let drop_amount = new anchor.BN(13);

    const tx_signature = await program.methods
        .createAirdrop(drop_amount)
        .accounts({
          senderIsAirdropCreator: payer.publicKey,
          recipient: recipient_public_key,
          mintAccount: mint_account_public_key,
          senderTokenAccount: senderTokenAddress,
          recipientTokenAccount: recepientTokenAddress,
          airdropInfo: airdropPDA
        })
        .rpc();

      console.log('Success!');
      console.log(`   Transaction Signature: ${tx_signature}`)

  })



  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
