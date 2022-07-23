import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { StakingTool } from "../target/types/staking_tool";

import fs from "fs";

import {
  AccountLayout,
  TOKEN_PROGRAM_ID,
  createAccount,
  createMint,
  setAuthority,
  AuthorityType,
  getMint,
  getOrCreateAssociatedTokenAccount,
  getAccount,
  mintTo,
  createInitializeAccountInstruction,
  transfer,
} from "@solana/spl-token";

import { ConfirmOptions } from "@solana/web3.js";
import { token } from "@project-serum/anchor/dist/cjs/utils";
import { BN } from "bn.js";
const {
  SystemProgram,
  Keypair,
  PublicKey,
  LAMPORTS_PER_SOL,
  clusterApiUrl,
  SYSVAR_RENT_PUBKEY,
  SYSVAR_CLOCK_PUBKEY,
  Transaction,
  sendAndConfirmTransaction,
} = anchor.web3;

import jsonFile from "/home/messi/zilla_staking/zilla_dev.json";
import jsonFile1 from "/home/messi/zilla_staking/zilla_dev.json";
// import jsonFile from "/home/kts/.config/solana/id.json";
// import jsonFile1 from "/home/kts/.config/solana/id1.json";
// var jsonFile = "/home/Guardian/dope-pirates-staking-contract-v3/client.json";
// var parsed = JSON.parse(fs.readFileSync(jsonFile));
// var parsed1 = JSON.parse(fs.readFileSync(jsonFile1));

// doesn't work
const sleep = (ms: number): Promise<void> => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};

describe("staking-tool", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.StakingTool as Program<StakingTool>;

  it("Is initialized!", async () => {
    // Add your test here.
    const provider = anchor.AnchorProvider.env();
    const signer = Keypair.fromSecretKey(Uint8Array.from(jsonFile));
    const signer1 = Keypair.fromSecretKey(Uint8Array.from(jsonFile1));
    const provider1 = new anchor.AnchorProvider(
      provider.connection,
      new anchor.Wallet(signer1),
      "confirmed" as anchor.web3.ConfirmOptions
    );

    let bal = await provider.connection.getBalance(
      signer.publicKey,
      "confirmed"
    );
    console.log("bal = ", bal);
    console.log("wallet = ", signer.publicKey.toBase58());

    // var transaction = new Transaction().add(
    //   SystemProgram.transfer({
    //     fromPubkey: signer.publicKey,
    //     toPubkey: signer1.publicKey,
    //     lamports: 10 * LAMPORTS_PER_SOL, //Investing 1 SOL. Remember 1 Lamport = 10^-9 SOL.
    //   })
    // );

    // Setting the variables for the transaction
    // transaction.feePayer = await signer.publicKey;
    // let blockhashObj = await provider.connection.getRecentBlockhash();
    // transaction.recentBlockhash = await blockhashObj.blockhash;

    // // Request creator to sign the transaction (allow the transaction)
    // let signed = await provider.signTransaction(transaction);
    // // The signature is generated
    // let signature = await connection.sendRawTransaction(signed.serialize());
    // // Confirm whether the transaction went through or not

    // Transaction constructor initialized successfully
    // if (transaction) {
    //   console.log("Txn created successfully");
    // }

    // let signature = await sendAndConfirmTransaction(
    //   provider.connection,
    //   transaction,
    //   [signer]
    // );
    // await provider.connection.confirmTransaction(signature);

    // let sign = await provider.sendAndConfirm(transaction);
    // await provider.connection.confirmTransaction(sign);
    // console.log("sign = ", sign);

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(signer1.publicKey, 1000000000),
      "confirmed"
    );

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(signer.publicKey, 1000000000),
      "confirmed"
    );

    bal = await provider.connection.getBalance(signer1.publicKey, "confirmed");
    console.log("bal = ", bal);
    console.log("wallet = ", signer1.publicKey.toBase58());

    const token_mint = await createMint(
      provider.connection,
      signer1,
      signer1.publicKey,
      signer1.publicKey,
      0
    );
    console.log("mintkey = ", token_mint.toBase58());

    let mintInfo = await getMint(provider.connection, token_mint);
    console.log("mintInfo = ", mintInfo.mintAuthority.toBase58());

    await setAuthority(
      provider.connection,
      signer1,
      mintInfo.address,
      signer1.publicKey,
      AuthorityType.MintTokens,
      signer.publicKey
    );

    mintInfo = await getMint(provider.connection, token_mint);
    console.log("mintInfo = ", mintInfo.mintAuthority.toBase58());

    const ownerTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      signer,
      token_mint,
      signer.publicKey
    );
    console.log("ownerTokenAccount = ", ownerTokenAccount.address.toBase58());

    await mintTo(
      provider.connection,
      signer,
      token_mint,
      ownerTokenAccount.address,
      signer.publicKey,
      100
    );

    let ownerTokenAccountInfo = await getAccount(
      provider.connection,
      ownerTokenAccount.address
    );
    console.log(
      "ownerTokenAccountinfo.amount = ",
      ownerTokenAccountInfo.amount
    );

    let initialized_key = new Keypair();
    console.log("initializedKey = ", initialized_key.publicKey.toBase58());

    let [stakePDA, _stake_nonce] = await PublicKey.findProgramAddress(
      [
        Buffer.from("staking_instance"),
        signer.publicKey.toBuffer(),
        initialized_key.publicKey.toBuffer(),
      ],
      program.programId
    );
    console.log("stakinginstancePDA = ", stakePDA.toBase58());

    let [vaultPDA, _vault_nonce] = await PublicKey.findProgramAddress(
      [Buffer.from("reward_vault"), stakePDA.toBuffer()],
      program.programId
    );
    console.log("vaultPDA = ", vaultPDA.toBase58());

    let slot = await provider.connection.getSlot("finalized");
    let time = await provider.connection.getBlockTime(slot);
    console.log("time = ", time);
    await provider.connection.confirmTransaction(
      await program.rpc.createStake(new BN(time), new BN(1), new BN(10), true, {
        accounts: {
          authority: signer.publicKey,
          rewardTokenMint: token_mint,
          rewardTokenVault: vaultPDA,
          stakingInstance: stakePDA,
          initializeKey: initialized_key.publicKey,
          burnWallet: signer.publicKey,
          burnTokenAccount: ownerTokenAccount.address,
          unstakingWallet: signer1.publicKey,
          rentSysvar: SYSVAR_RENT_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        },
      })
    );

    mintInfo = await getMint(provider.connection, token_mint);
    console.log("mintInfo = ", mintInfo.mintAuthority.toBase58());

    await provider.connection.confirmTransaction(
      await transfer(
        provider.connection,
        signer,
        ownerTokenAccount.address,
        vaultPDA,
        signer.publicKey,
        50
      )
    );

    let [stakingPoolPDA, _pool_nonce] = await PublicKey.findProgramAddress(
      [
        Buffer.from("staking_pool"),
        signer.publicKey.toBuffer(),
        token_mint.toBuffer(),
      ],
      program.programId
    );
    console.log("stakingpoolPDA = ", stakingPoolPDA.toBase58());

    let [escrowPDA, _escrow_nonce] = await PublicKey.findProgramAddress(
      [
        Buffer.from("escrow"),
        signer.publicKey.toBuffer(),
        stakingPoolPDA.toBuffer(),
      ],
      program.programId
    );
    console.log("escrowPDA = ", escrowPDA.toBase58());

    let res = await provider.connection.confirmTransaction(
      await program.rpc.enterStaking(true, new BN(0), new BN(1), 2000, {
        accounts: {
          authority: signer.publicKey,
          nftTokenMint: token_mint,
          nftTokenAccount: ownerTokenAccount.address,
          stakingInstance: stakePDA,
          stakingPool: stakingPoolPDA,
          escrow: escrowPDA,
          rentSysvar: SYSVAR_RENT_PUBKEY,
          clockSysvar: SYSVAR_CLOCK_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        },
      })
    );
    console.log("res = ", res);

    let escrowAccount = await provider.connection.getTokenAccountsByOwner(
      escrowPDA,
      {
        // programId: TOKEN_PROGRAM_ID,
        mint: token_mint,
      }
    );
    // console.log("escrowAccounts = ", escrowAccount);
    console.log("Token                                         Balance");
    console.log("------------------------------------------------------------");
    escrowAccount.value.forEach((e) => {
      const accountInfo = AccountLayout.decode(e.account.data);
      console.log(`${new PublicKey(accountInfo.mint)}   ${accountInfo.amount}`);
    });

    // doesn't work
    await sleep(10000);
    slot = await provider.connection.getSlot("finalized");
    time = await provider.connection.getBlockTime(slot);
    console.log("time = ", time);

    res = await provider.connection.confirmTransaction(
      await program.rpc.claimRewards({
        accounts: {
          authority: signer.publicKey,
          stakingPool: stakingPoolPDA,
          stakingInstance: stakePDA,
          receiveRewardTokenAccount: ownerTokenAccount.address,
          rewardTokenMint: token_mint,
          rewardTokenVault: vaultPDA,
          escrow: escrowPDA,
          rentSysvar: SYSVAR_RENT_PUBKEY,
          clockSysvar: SYSVAR_CLOCK_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        },
      })
    );

    escrowAccount = await provider.connection.getTokenAccountsByOwner(
      signer.publicKey,
      {
        // programId: TOKEN_PROGRAM_ID,
        mint: token_mint,
      }
    );
    // console.log("escrowAccounts = ", escrowAccount);
    console.log("Token                                         Balance");
    console.log("------------------------------------------------------------");
    escrowAccount.value.forEach((e) => {
      const accountInfo = AccountLayout.decode(e.account.data);
      console.log(`${new PublicKey(accountInfo.mint)}   ${accountInfo.amount}`);
    });

    res = await provider.connection.confirmTransaction(
      await program.rpc.cancelStaking({
        accounts: {
          authority: signer.publicKey,
          rewardTokenMint: token_mint,
          rewardTokenVault: vaultPDA,
          nftTokenMint: token_mint,
          receiveNftTokenAccount: ownerTokenAccount.address,
          receiveRewardTokenAccount: ownerTokenAccount.address,
          burnTokenAccount: ownerTokenAccount.address,
          unstakingWallet: signer1.publicKey,
          stakingInstance: stakePDA,
          stakingPool: stakingPoolPDA,
          escrow: escrowPDA,
          rentSysvar: SYSVAR_RENT_PUBKEY,
          clockSysvar: SYSVAR_CLOCK_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        },
      })
    );

    escrowAccount = await provider.connection.getTokenAccountsByOwner(
      signer.publicKey,
      {
        // programId: TOKEN_PROGRAM_ID,
        mint: token_mint,
      }
    );
    // console.log("escrowAccounts = ", escrowAccount);
    console.log("Token                                         Balance");
    console.log("------------------------------------------------------------");
    escrowAccount.value.forEach((e) => {
      const accountInfo = AccountLayout.decode(e.account.data);
      console.log(`${new PublicKey(accountInfo.mint)}   ${accountInfo.amount}`);
    });

    // show the stakingInstance
    let staking_instance = await program.account.stakingInstance.fetch(
      stakePDA
    );
    console.log("staking_instance = ", staking_instance);

    // show the staking_pool
    let staking_pool = await program.account.stakingPool.fetch(stakingPoolPDA);
    console.log("staking_pool = ", staking_pool);
  });
});
