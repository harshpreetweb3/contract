import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey } from '@solana/web3.js';
// import { Program } from "@coral-xyz/anchor";
import { ADropApp } from "../target/types/a_drop_app";

describe("Airdrop Data!", () => {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.ADropApp as anchor.Program<ADropApp>;

  //generate a keypair for airdropInfo account
  const airdropInfoAccount = new Keypair();

  it('create the airdrop info account' async () => {
    console.log("Payer's public key : {}", payer.publicKey);
    console.log("Airdrop info Account : {}", payer.publicKey);

    //Instruction Ix data
    const airdropInfo = {
      drop_amount : 13 
    }

    const recepient_pk : PublicKey = new PublicKey("Fkth3sBewAfunPzzbUjGb5axoFCkPLJ3VDtgcDDuxF4H");

    const mint_account_pk;
    //Some mint account can be given here 

    //Phela ta main token transfer deploy kr k dekhda
    //localnet then devnet te 
    //pher mai vaapas aana

    const tx_signature = await program.methods
    .create_airdrop(airdropInfo.drop_amount)
    .accounts({

      //this address should own the tokens to send 
      sender_is_airdrop_creator : payer.publicKey,
      //Mai pretend krda k sender kol token haige aw.

      //we need to send tokens to the contract
      recipient : recepient_pk,
      //mai recepient de publicKey kiddaan access kr skda han?
      
      //token jehra send kita ja reha oda mint account
      mintAccount : 

    })



  })



  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
