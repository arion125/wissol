import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { IDL, Wissol } from "../target/types/wissol";
import { 
  PublicKey, 
  Keypair,
  Commitment,
  SystemProgram, 
  LAMPORTS_PER_SOL 
} from "@solana/web3.js";

import {
  getOrCreateAssociatedTokenAccount,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  mintTo,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";

import {Connection} from "@solana/web3.js";
import { BN } from "bn.js";

describe("Wissol", () => {
  const commitment: Commitment = "confirmed"; // processed, confirmed, finalized
  const connection = new Connection("http://localhost:8899", {
      commitment,
      wsEndpoint: "ws://localhost:8900/",
  });
  // Configure the client to use the local cluster.
  const keypair = anchor.web3.Keypair.generate();

  const provider = new anchor.AnchorProvider(connection, new anchor.Wallet(keypair), { commitment });

  const programId = new PublicKey("64drnnxTSEe9fpeTEQ3MeVtm1Vztzf7bR4Cigs9X7S5j");

  const program = new anchor.Program<Wissol>(IDL, programId, provider);

  // Helpers
  const wait = (ms: number): Promise<void> => {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }
  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...block,
    })
    return signature
  }

  const log = async(signature: string): Promise<string> => {
    console.log(`Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`);
    return signature;
  }

  // Variables
  let nftMint: PublicKey;;
  let referralAccount: PublicKey;
  let nftEscrow: PublicKey;
  let mint: PublicKey;
  let mintAta: PublicKey;
  let project = Keypair.generate();
  let feeAccount = Keypair.generate();

  // Instructions
  it("Airdrop", async () => {
    await connection.requestAirdrop(keypair.publicKey, LAMPORTS_PER_SOL * 10).then(confirm).then(log)
  })

  it("Set up", async () => {
    mint = await createMint(connection, keypair, keypair.publicKey, null, 0);
  });

  it("Initiazlize Referral", async () => {
    nftMint = await createMint(connection, keypair, keypair.publicKey, null, 0);

    referralAccount = PublicKey.findProgramAddressSync([Buffer.from("referral"), nftMint.toBuffer()], programId)[0];
    nftEscrow = PublicKey.findProgramAddressSync([Buffer.from("nft_escrow"), nftMint.toBuffer()], programId)[0];

    await program.methods
    .initializeReferral(10)
    .accounts({
      payer: keypair.publicKey,
      nftMint: nftMint,
      referralAccount,
      nftEscrow,
      systemProgram: SystemProgram.programId,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([keypair]).rpc().then(confirm).then(log)
  })

  it("Transfer Referral", async () => {

    mintAta = (await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey)).address
    await mintTo(connection, keypair, mint, mintAta, keypair.publicKey, 1e6);
    const payerRefereeAccount = PublicKey.findProgramAddressSync([Buffer.from("referee"), keypair.publicKey.toBuffer()], programId)[0]

    await program.methods
    .transferRef(new BN(1))
    .accounts({
      payer: keypair.publicKey,
      payerTokenAta: mintAta,
      payerRefereeAccount,
      referralAccount,
      mint: mint,
      nftEscrow,
      escrowMintAta: getAssociatedTokenAddressSync(mint, nftEscrow, true),
      project: project.publicKey,
      projectMintAta: getAssociatedTokenAddressSync(mint, project.publicKey),
      feeAccount: feeAccount.publicKey,
      feeMintAta: getAssociatedTokenAddressSync(mint, feeAccount.publicKey),
      systemProgram: SystemProgram.programId,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([keypair]).rpc().then(confirm).then(log)
  })

  it("Withdraw", async () => {

    await program.methods
    .withdraw()
    .accounts({
      payer: keypair.publicKey,
      mint: mint,
      payerMintAta: mintAta,
      nftEscrow,
      escrowMintAta: getAssociatedTokenAddressSync(mint, nftEscrow, true),
      systemProgram: SystemProgram.programId,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([keypair]).rpc({skipPreflight: true}).then(confirm).then(log)

  })
  

});
