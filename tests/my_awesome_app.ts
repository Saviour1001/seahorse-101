import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MyAwesomeApp } from "../target/types/my_awesome_app";

describe("my_awesome_app", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MyAwesomeApp as Program<MyAwesomeApp>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
