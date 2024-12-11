import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey } from '@solana/web3.js';
import { getAssociatedTokenAddressSync } from '@solana/spl-token';
// import { Program } from "@coral-xyz/anchor";
import { AirdropPlatform } from "../target/types/airdrop_platform";

describe("Airdrop Data!", () => {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ADropApp as anchor.Program<AirdropPlatform>;
  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;

  const [airdropPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("airdrop")],
    program.programId
  );

  //generate a keypair for airdropInfo account
  const recipient_public_key = new PublicKey('Fkth3sBewAfunPzzbUjGb5axoFCkPLJ3VDtgcDDuxF4H');
  // new Keypair();

  //we must have a created mint account with us 

  const mint_account_public_key = new PublicKey("mint-account-address");

  // Derive the associated token address account for the mint and payer.
  const senderTokenAddress = getAssociatedTokenAddressSync(mint_account_public_key, wallet.publicKey);
  const recepientTokenAddress = getAssociatedTokenAddressSync(mint_account_public_key, recipient_public_key);

  it("create an airdrop!", async () => {
    try {

      console.log("Payer's public key : {}", wallet.publicKey);
      console.log("Airdrop info Account : {}", airdropPDA);

      //Instruction Ix data
      // const airdropInfo = {
      //   drop_amount: 13
      // }

      let drop_amount = new anchor.BN(13);

      // const txSig = await program.methods.create_airdrop().rpc();

      const tx_signature = await program.methods
        .createAirdrop(drop_amount)
        .accounts({
          senderIsAirdropCreator: wallet.publicKey,
          recipient: recipient_public_key,
          mintAccount: mint_account_public_key,
          senderTokenAccount: senderTokenAddress,
          recipientTokenAccount: recepientTokenAddress,
          airdropInfo: airdropPDA,
        })
        .rpc();

      console.log('Success!');
      console.log(`   Transaction Signature: ${tx_signature}`)

    } catch (error) {
      // If PDA Account already created, then we expect an error
      console.log(error);
    }
  });

});
