import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HabitTracker } from "../target/types/habit_tracker";
import { expect } from "chai";

describe("habit-tracker", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.HabitTracker as Program<HabitTracker>;
  const habitName = "Morning Workout";

  it("Initializes a habit counter", async () => {
    const [habitCounterPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("habit_counter"),
        provider.wallet.publicKey.toBuffer(),
        Buffer.from(habitName),
      ],
      program.programId
    );

    await program.methods
      .initializeCounter(habitName)
      .accounts({
        habitCounter: habitCounterPda,
        owner: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const counter = await program.account.habitCounter.fetch(habitCounterPda);
    expect(counter.count.toNumber()).to.equal(0);
    expect(counter.habitName).to.equal(habitName);
  });

  it("Increments the counter", async () => {
    const [habitCounterPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("habit_counter"),
        provider.wallet.publicKey.toBuffer(),
        Buffer.from(habitName),
      ],
      program.programId
    );

    await program.methods
      .incrementCounter()
      .accounts({
        habitCounter: habitCounterPda,
        owner: provider.wallet.publicKey,
      })
      .rpc();

    const counter = await program.account.habitCounter.fetch(habitCounterPda);
    expect(counter.count.toNumber()).to.equal(1);
  });
});