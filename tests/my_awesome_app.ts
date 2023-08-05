import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";
import { MyAwesomeApp } from "../target/types/my_awesome_app";

describe("my_awesome_app", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.MyAwesomeApp as Program<MyAwesomeApp>;

  // setting up a common owner to be used in all tests
  const owner = provider.wallet.publicKey;

  const MyAwesomeApp = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("my_awesome_app"), owner.toBuffer()],
    program.programId
  );

  console.log("MyAwesomeApp", MyAwesomeApp);

  it("init a user", async () => {
    const tx = await program.methods
      .initUserprofile()
      .accounts({
        owner: owner,
        userprofile: MyAwesomeApp[0],
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
