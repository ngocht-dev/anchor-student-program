import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorStudentProgram } from "../target/types/anchor_student_program";
import { expect } from "chai";
import { getAssociatedTokenAddress, getAccount } from "@solana/spl-token";

describe("anchor-student-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .AnchorStudentProgram as Program<AnchorStudentProgram>;

  const student = {
    name: "Pi Huynh",
    description: "I am a solana on-chain developer",
  };

  const [studentPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(student.name), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint")],
    program.programId
  );

  it("fetch a student account", async () => {
    try {
      const account = await program.account.student.fetch(studentPda);
      console.log(account);
    } catch (error) {
      console.log("fetch a student account failed: ", error);
    }
  });

  // it("Initializes the reward token", async () => {
  //   const tx = await program.methods.initializeTokenMint().rpc();
  // });

  // it("Student is added`", async () => {
  //   // Add your test here.
  //   const tokenAccount = await getAssociatedTokenAddress(
  //     mint,
  //     provider.wallet.publicKey
  //   );

  //   await program.methods
  //     .addStudent(student.name, student.description)
  //     .accounts([{ pubkey: tokenAccount, isSigner: false, isWritable: true }])
  //     .rpc();

  //   const account = await program.account.student.fetch(studentPda);
  //   expect(student.name === account.name);
  //   expect(student.description === account.description);
  //   expect(account.creator === provider.wallet.publicKey);
  // });

  // it("Student is updated`", async () => {
  //   const newDescription =
  //     "I am a solana on-chain developer and a full-stack developer";
  //   await program.methods.updateStudent(student.name, newDescription).rpc();

  //   const account = await program.account.student.fetch(studentPda);
  //   expect(student.name === account.name);
  //   expect(newDescription === account.description);
  //   expect(account.creator === provider.wallet.publicKey);
  // });

  // it("Deletes a student", async () => {
  //   await program.methods.deleteStudent(student.name).rpc();
  // });
});
