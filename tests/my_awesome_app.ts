import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";
import { MyAwesomeApp } from "../target/types/my_awesome_app";
import { assert } from "chai";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";

describe("my_awesome_app", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.MyAwesomeApp as Program<MyAwesomeApp>;

  const myKey = provider.wallet.publicKey;

  const seed1 = Buffer.from("user");
  const seed2 = myKey.toBuffer();

  const [userPDA, _bump] = findProgramAddressSync(
    [seed1, seed2],
    program.programId
  );

  const user = userPDA;

  it("Creating user", async () => {
    await program.methods.initUser().accounts({ user, owner: myKey }).rpc();
  });
});
