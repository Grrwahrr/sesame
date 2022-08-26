import {BN, Program} from "@project-serum/anchor";
import * as anchor from "@project-serum/anchor";
import {Sesame} from "../target/types/sesame";

export const createOrganizer = async(program: Program<Sesame>, payer: anchor.web3.PublicKey, organizer: anchor.web3.PublicKey) => {
    let tx;
    try {
        tx = await program.methods
            .createOrganizer("Test Organizer", "https://www.testorganizer.tld")
            .accounts({
                payer: payer,
                organizer: organizer,
            })
            .rpc();
    }
    catch (e) {
        console.log("Error ", e);
        return undefined;
    }

    return await program.account.organizer.fetch(organizer);
}

export const createEvent = async(program: Program<Sesame>, payer: anchor.web3.PublicKey, donateTo: anchor.web3.PublicKey, organizer: anchor.web3.PublicKey, authorityIssuer: anchor.web3.PublicKey, authorityDelete: anchor.web3.PublicKey, authorityCheckIn: anchor.web3.PublicKey, event: anchor.web3.PublicKey) => {
    let tx;
    try {
        tx = await program.methods
            .createEvent("The Event", "https://the.event.tld", 2, new BN(1746421203), 0, "Earth, Milky Way", "https://some.broken.url/image.jpg")
            .accounts({
                payer: payer,
                donateTo: donateTo,
                organizer: organizer,
                ticketAuthorityIssuer: authorityIssuer,
                ticketAuthorityDelete: authorityDelete,
                ticketAuthorityCheckIn: authorityCheckIn,
                event: event,
            })
            .rpc();
    }
    catch (e) {
        console.log("Error ", e);
        return undefined;
    }

    return await program.account.event.fetch(event);
}

export const ticketIssue = async(program: Program<Sesame>, authorityIssuer: anchor.web3.Signer, event: anchor.web3.PublicKey, ticket: anchor.web3.PublicKey, owner: anchor.web3.PublicKey, logError: boolean) => {
    let tx;
    try {
        tx = await program.methods
            .ticketIssue()
            .accounts({
                payer: authorityIssuer.publicKey,
                event: event,
                ticket: ticket,
                ticketOwner: owner,
            })
            .signers([authorityIssuer])
            .rpc();
    }
    catch (e) {
        if ( logError ) {
            console.log("Error ", e);
        }
        return undefined;
    }

    return await program.account.ticket.fetch(ticket);
}

export const ticketCheckIn = async(program: Program<Sesame>, ticket_offset: number, authorityCheckIn: anchor.web3.Signer, ticketOwner: anchor.web3.Signer, event: anchor.web3.PublicKey, ticket: anchor.web3.PublicKey, logError: boolean) => {
    let tx;
    try {
        tx = await program.methods
            .ticketCheckIn(ticket_offset)
            .accounts({
                authority: authorityCheckIn.publicKey,
                ticketOwner: ticketOwner.publicKey,
                event: event,
                ticket: ticket,
            })
            .signers([authorityCheckIn, ticketOwner])
            .rpc();
    }
    catch (e) {
        if ( logError ) {
            console.log("Error ", e);
        }
        return undefined;
    }

    return await program.account.ticket.fetch(ticket);
}

export const ticketDelete = async(program: Program<Sesame>, ticket_offset: number, authorityDelete: anchor.web3.Signer, ticketOwner: anchor.web3.Signer, event: anchor.web3.PublicKey, ticket: anchor.web3.PublicKey, logError: boolean) => {
    let tx;
    try {
        tx = await program.methods
            .ticketDelete(ticket_offset)
            .accounts({
                authority: authorityDelete.publicKey,
                ticketOwner: ticketOwner.publicKey,
                event: event,
                ticket: ticket,
            })
            .signers([authorityDelete, ticketOwner])
            .rpc();
    }
    catch (e) {
        if ( logError ) {
            console.log("Error ", e);
        }
        return "FAILED TO DELETE";
    }

    // After account deletion, this SHOULD cause an exception
    let account;
    try {
        account = await program.account.ticket.fetch(ticket);
    }
    catch (e) {}

    return account;
}

export const createEventPass = async(program: Program<Sesame>, payer: anchor.web3.PublicKey, organizer: anchor.web3.PublicKey, authorityIssuer: anchor.web3.PublicKey, authorityDelete: anchor.web3.PublicKey, eventPass: anchor.web3.PublicKey) => {
    let tx;
    try {
        tx = await program.methods
            .createEventPass("The Event Pass", "https://the.eventpass.tld", "https://some.broken.url/image.jpg", 2, 100)
            .accounts({
                payer: payer,
                organizer: organizer,
                passAuthorityIssuer: authorityIssuer,
                passAuthorityDelete: authorityDelete,
                eventPass: eventPass,
            })
            .rpc();
    }
    catch (e) {
        console.log("Error ", e);
        return undefined;
    }

    return await program.account.eventPass.fetch(eventPass);
}

export const eventPassAddEvent = async(program: Program<Sesame>, authorityIssuer: anchor.web3.PublicKey, eventPass: anchor.web3.PublicKey, event: anchor.web3.PublicKey, eventPassValidEvent: anchor.web3.PublicKey) => {
    let tx;
    try {
        tx = await program.methods
            .eventPassAddEvent()
            .accounts({
                payer: authorityIssuer,
                eventPass: eventPass,
                event: event,
                eventPassValidEvent: eventPassValidEvent,
            })
            .signers([])
            .rpc();
    }
    catch (e) {
        console.log("Error ", e);
        return undefined;
    }

    return await program.account.eventPassValidEvent.fetch(eventPassValidEvent);
}

export const eventPassCreateHolder = async(program: Program<Sesame>, authorityIssuer: anchor.web3.Signer, eventPass: anchor.web3.PublicKey, eventPassHolder: anchor.web3.PublicKey, owner: anchor.web3.PublicKey) => {
    let tx;
    try {
        tx = await program.methods
            .eventPassHolderCreate()
            .accounts({
                payer: authorityIssuer.publicKey,
                eventPass: eventPass,
                eventPassHolder: eventPassHolder,
                eventPassOwner: owner,
            })
            .signers([authorityIssuer])
            .rpc();
    }
    catch (e) {
        console.log("Error ", e);
        return undefined;
    }

    return await program.account.eventPassHolder.fetch(eventPassHolder);
}

export const ticketIssueForEventPass = async(program: Program<Sesame>, eventOffset: number, holderOffset: number, authorityIssuer: anchor.web3.Signer, eventPass: anchor.web3.PublicKey, eventPassValidEvent: anchor.web3.PublicKey, eventPassHolder: anchor.web3.PublicKey, passOwner: anchor.web3.Signer, eventPassHolderTicket: anchor.web3.PublicKey, event: anchor.web3.PublicKey, ticket: anchor.web3.PublicKey, owner: anchor.web3.PublicKey, logError: boolean) => {
    let tx;
    try {
        tx = await program.methods
            .ticketIssueForEventPass(eventOffset, holderOffset)
            .accounts({
                payer: authorityIssuer.publicKey,
                eventPass: eventPass,
                eventPassValidEvent: eventPassValidEvent,
                eventPassHolder: eventPassHolder,
                eventPassOwner: passOwner.publicKey,
                eventPassHolderTicket: eventPassHolderTicket,
                event: event,
                ticket: ticket,
                ticketOwner: owner
            })
            .signers([authorityIssuer, passOwner])
            .rpc();
    }
    catch (e) {
        if ( logError ) {
            console.log("Error ", e);
        }
        return undefined;
    }

    return await program.account.ticket.fetch(ticket);
}