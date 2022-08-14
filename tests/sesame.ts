import * as anchor from "@project-serum/anchor";
import {Program} from "@project-serum/anchor";
import {Sesame} from "../target/types/sesame";
import {
    deriveEvent,
    deriveEventPass,
    deriveEventPassHolder, deriveEventPassHolderTicket,
    deriveEventPassValidEvent,
    deriveOrganizer,
    deriveTicket
} from "./pda";
import {
    createEvent,
    ticketIssue,
    createOrganizer,
    ticketCheckIn,
    ticketDelete,
    createEventPass,
    eventPassAddEvent, eventPassCreateHolder, ticketIssueForEventPass
} from "./instructions";
import {expect} from "chai";



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
    const [accDataEventPass, bumpEventPass] = await deriveEventPass(program, accProvider.publicKey, 0);

    const [accDataTicket1, bumpTicket1] = await deriveTicket(program, accDataEvent, 0);
    const [accDataTicket2, bumpTicket2] = await deriveTicket(program, accDataEvent, 1);
    const [accDataTicket3, bumpTicket3] = await deriveTicket(program, accDataEvent, 2);

    const [accDataEventPassValidEvent1, EventPassValidEvent1] = await deriveEventPassValidEvent(program, accDataEventPass, 0);
    const [accDataEventPassValidEvent2, EventPassValidEvent2] = await deriveEventPassValidEvent(program, accDataEventPass, 1);

    const [accDataEventPassHolder1, EventPassHolder1] = await deriveEventPassHolder(program, accDataEventPass, 0);
    const [accDataEventPassHolder2, EventPassHolder2] = await deriveEventPassHolder(program, accDataEventPass, 1);

    const [accDataEventPassHolderTicket1, EventPassHolderTicket1] = await deriveEventPassHolderTicket(program, accDataEventPassHolder1, accDataEvent);

    it("Create organizer", async () => {
        const organizer = await createOrganizer(program, accProvider.publicKey, accDataOrganizer);
        expect(organizer).to.not.be.undefined;
        // console.log("Organizer account", organizer);
        //TODO: update organizer
    });

    it("Create event", async () => {
        const event = await createEvent(program, accProvider.publicKey, accWalletAlice.publicKey, accDataOrganizer, accWalletAuthorityIssuer.publicKey, accWalletAuthorityDelete.publicKey, accWalletAuthorityCheckIn.publicKey, accDataEvent);
        expect(event).to.not.be.undefined;
        // console.log("Event account", event);
        //TODO: update event
    });

    it("Issue tickets", async () => {
        // Airdrop to authority
        const airdrop = await program.provider.connection.requestAirdrop(accWalletAuthorityIssuer.publicKey, 100_000_000);
        await program.provider.connection.confirmTransaction(airdrop);

        // Issue 1st ticket
        const ticket1 = await ticketIssue(program, accWalletAuthorityIssuer, accDataEvent, accDataTicket1, accWalletAlice.publicKey, true);
        expect(ticket1).to.not.be.undefined;

        // Issue 2nd ticket
        const ticket2 = await ticketIssue(program, accWalletAuthorityIssuer, accDataEvent, accDataTicket2, accWalletAlice.publicKey, true);
        expect(ticket2).to.not.be.undefined;

        // Issue 3rd ticket; should fail as event only allows for 2 tickets
        const ticket3 = await ticketIssue(program, accWalletAuthorityIssuer, accDataEvent, accDataTicket3, accWalletAlice.publicKey, false);
        expect(ticket3).to.be.undefined;

        // Attempt to re-issue 1st ticket; should fail
        const ticket1re = await ticketIssue(program, accWalletAuthorityIssuer, accDataEvent, accDataTicket1, accWalletAlice.publicKey, false);
        expect(ticket1re).to.be.undefined;
    });

    it("Ticket check in", async () => {
        // Airdrop to authority
        const airdrop = await program.provider.connection.requestAirdrop(accWalletAuthorityCheckIn.publicKey, 100_000_000);
        await program.provider.connection.confirmTransaction(airdrop);

        // Check in works
        const ticket1 = await ticketCheckIn(program, 0, accWalletAuthorityCheckIn, accWalletAlice, accDataEvent, accDataTicket1, true);
        expect(ticket1).to.not.be.undefined;
        expect(ticket1.state).to.have.key("checkedIn");

        // Can not check in again
        const ticket1re = await ticketCheckIn(program, 0, accWalletAuthorityCheckIn, accWalletAlice, accDataEvent, accDataTicket1, false);
        expect(ticket1re).to.be.undefined;
    });

    it("Ticket delete", async () => {
        // Airdrop to authority
        const airdrop = await program.provider.connection.requestAirdrop(accWalletAuthorityDelete.publicKey, 100_000_000);
        await program.provider.connection.confirmTransaction(airdrop);

        const ticket2 = await ticketDelete(program, 1, accWalletAuthorityDelete, accWalletAlice, accDataEvent, accDataTicket2, true);
        expect(ticket2).to.be.undefined;
    });





    it("Create event pass", async () => {
        const eventPass = await createEventPass(program, accProvider.publicKey, accDataOrganizer, accWalletAuthorityIssuer.publicKey, accWalletAuthorityDelete.publicKey, accDataEventPass);
        expect(eventPass).to.not.be.undefined;
        // console.log("Event pass account", eventPass);
    });

    it("Event pass add valid event", async () => {
        const eventPassValidEvent = await eventPassAddEvent(program, accProvider.publicKey, accDataEventPass, accDataEvent, accDataEventPassValidEvent1);
        expect(eventPassValidEvent).to.not.be.undefined;
        // console.log("Event pass valid event account", eventPassValidEvent);
    });

    it("Event pass crete holder", async () => {
        const eventPassHolder = await eventPassCreateHolder(program, accWalletAuthorityIssuer, accDataEventPass, accDataEventPassHolder1, accWalletAlice.publicKey);
        expect(eventPassHolder).to.not.be.undefined;
        // console.log("Event pass holder account", eventPassHolder);
    });

    it("Issue tickets for event pass holder", async () => {
        // Issue 1st ticket
        const ticket3 = await ticketIssueForEventPass(program, 0, 0, accWalletAuthorityIssuer, accDataEventPass, accDataEventPassValidEvent1, accDataEventPassHolder1, accWalletAlice, accDataEventPassHolderTicket1, accDataEvent, accDataTicket3, accWalletAlice.publicKey, true);
        expect(ticket3).to.not.be.undefined;
        // console.log("Event pass holder created ticket account", ticket3);
    });
});
