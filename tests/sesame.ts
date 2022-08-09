import * as anchor from "@project-serum/anchor";
import {Program} from "@project-serum/anchor";
import {Sesame} from "../target/types/sesame";

const textEncoder = new TextEncoder();

const deriveOrganizer = (program: Program<Sesame>, owner: anchor.web3.PublicKey) =>
    anchor.web3.PublicKey.findProgramAddress(
        [textEncoder.encode("Organizer"), owner.toBuffer()],
        program.programId
    );

const deriveEvent = (program: Program<Sesame>, owner: anchor.web3.PublicKey, offset: number) =>
    anchor.web3.PublicKey.findProgramAddress(
        [textEncoder.encode("Event"), owner.toBuffer(), new anchor.BN(offset).toArrayLike(Buffer, "le", 4)],
        program.programId
    );

const deriveTicket = (program: Program<Sesame>, event: anchor.web3.PublicKey, seatId: string) =>
    anchor.web3.PublicKey.findProgramAddress(
        [textEncoder.encode("Ticket"), event.toBuffer(), Buffer.from(seatId, "utf-8")],
        program.programId
    );


describe("sesame", async () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.Sesame as Program<Sesame>;

    // Generate some accounts
    const accProvider = (program.provider as anchor.AnchorProvider).wallet;
    const accWalletAuthorityIssuer = anchor.web3.Keypair.generate();
    const accWalletAuthorityDelete = anchor.web3.Keypair.generate();
    const accWalletAuthorityCheckIn = anchor.web3.Keypair.generate();

    const accWalletAlice = anchor.web3.Keypair.generate();


    // Find some PDA addresses
    const [accDataOrganizer, bumpOrganizer] = await deriveOrganizer(program, accProvider.publicKey);
    const [accDataEvent, bumpEvent] = await deriveEvent(program, accProvider.publicKey, 0);

    const [accDataTicket1, bumpTicket1] = await deriveTicket(program, accDataEvent, "1");
    const [accDataTicket2, bumpTicket2] = await deriveTicket(program, accDataEvent, "2");
    const [accDataTicket3, bumpTicket3] = await deriveTicket(program, accDataEvent, "3");

    it("Create Organizer", async () => {
        const tx = await program.methods
            .createOrganizer("The Organizers", "https://the.organizers.tldr")
            .accounts({
                payer: accProvider.publicKey,
                organizer: accDataOrganizer,
            })
            // .signers([accWalletAlice])
            .rpc();

        console.log("TX signature:", tx);

        // Get the newly created account
        const organizer = await program.account.organizer.fetch(accDataOrganizer);
        console.log("Organizer account", organizer);
    });

    it("Create Event", async () => {
        const tx = await program.methods
            .createEvent("The Event", "https://the.event.tldr", 2)
            .accounts({
                payer: accProvider.publicKey,
                donateTo: accWalletAlice.publicKey,
                organizer: accDataOrganizer,
                ticketAuthorityIssuer: accWalletAuthorityIssuer.publicKey,
                ticketAuthorityDelete: accWalletAuthorityDelete.publicKey,
                ticketAuthorityCheckIn: accWalletAuthorityCheckIn.publicKey,
                event: accDataEvent,
            })
            .rpc();

        console.log("TX signature:", tx);

        // Get the newly created account
        const event = await program.account.event.fetch(accDataEvent);
        console.log("Event account", event);
    });

    it("Issue ticket 1", async () => {
        // Airdrop to authority
        const airdrop = await program.provider.connection.requestAirdrop(accWalletAuthorityIssuer.publicKey, 100_000_000);
        await program.provider.connection.confirmTransaction(airdrop);

        const tx = await program.methods
            .ticketIssue("1")
            .accounts({
                payer: accWalletAuthorityIssuer.publicKey,
                event: accDataEvent,
                ticket: accDataTicket1,
                ticketOwner: accWalletAlice.publicKey,
            })
            .signers([accWalletAuthorityIssuer])
            .rpc();

        console.log("TX signature:", tx);

        // Get the newly created account
        const ticket = await program.account.ticket.fetch(accDataTicket1);
        console.log("Ticket account", ticket);
    });

    // it("Check in ticket 1", async () => {
    //     // Airdrop to authority
    //     const airdrop = await program.provider.connection.requestAirdrop(accWalletAuthorityCheckIn.publicKey, 100_000_000);
    //     await program.provider.connection.confirmTransaction(airdrop);
    //
    //     const tx = await program.methods
    //         .ticketCheckIn("1")
    //         .accounts({
    //             authority: accWalletAuthorityCheckIn.publicKey,
    //             ticketOwner: accWalletAlice.publicKey,
    //             event: accDataEvent,
    //             ticket: accDataTicket1,
    //         })
    //         .signers([accWalletAuthorityCheckIn, accWalletAlice])
    //         .rpc();
    //
    //     console.log("TX signature:", tx);
    //
    //     // Get the newly created account
    //     const ticket = await program.account.ticket.fetch(accDataTicket1);
    //     console.log("Ticket account", ticket);
    // });

    it("Delete ticket 1", async () => {
        // Airdrop to authority
        const airdrop = await program.provider.connection.requestAirdrop(accWalletAuthorityDelete.publicKey, 100_000_000);
        await program.provider.connection.confirmTransaction(airdrop);

        const tx = await program.methods
            .ticketDelete("1")
            .accounts({
                authority: accWalletAuthorityDelete.publicKey,
                ticketOwner: accWalletAlice.publicKey,
                event: accDataEvent,
                ticket: accDataTicket1,
            })
            .signers([accWalletAuthorityDelete, accWalletAlice])
            .rpc();

        console.log("TX signature:", tx);

        // Get the newly created account TODO will fail ccause its deleted :)
        const ticket = await program.account.ticket.fetch(accDataTicket1);
        console.log("Ticket account", ticket);
    });
});
