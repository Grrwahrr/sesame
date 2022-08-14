import {Program} from "@project-serum/anchor";
import * as anchor from "@project-serum/anchor";
import {Sesame} from "../target/types/sesame";

const textEncoder = new TextEncoder();

export const deriveOrganizer = (program: Program<Sesame>, owner: anchor.web3.PublicKey) =>
    anchor.web3.PublicKey.findProgramAddress(
        [textEncoder.encode("Organizer"), owner.toBuffer()],
        program.programId
    );

export const deriveEvent = (program: Program<Sesame>, owner: anchor.web3.PublicKey, offset: number) =>
    anchor.web3.PublicKey.findProgramAddress(
        [textEncoder.encode("Event"), owner.toBuffer(), new anchor.BN(offset).toArrayLike(Buffer, "le", 4)],
        program.programId
    );

export const deriveTicket = (program: Program<Sesame>, event: anchor.web3.PublicKey, offset: number) =>
    anchor.web3.PublicKey.findProgramAddress(
        [textEncoder.encode("Ticket"), event.toBuffer(), new anchor.BN(offset).toArrayLike(Buffer, "le", 2)],
        program.programId
    );

export const deriveEventPass = (program: Program<Sesame>, owner: anchor.web3.PublicKey, offset: number) =>
    anchor.web3.PublicKey.findProgramAddress(
        [textEncoder.encode("EventPass"), owner.toBuffer(), new anchor.BN(offset).toArrayLike(Buffer, "le", 4)],
        program.programId
    );

export const deriveEventPassValidEvent = (program: Program<Sesame>, eventPass: anchor.web3.PublicKey, offset: number) =>
    anchor.web3.PublicKey.findProgramAddress(
        [textEncoder.encode("EventPassValidEvent"), eventPass.toBuffer(), new anchor.BN(offset).toArrayLike(Buffer, "le", 2)],
        program.programId
    );

export const deriveEventPassHolder = (program: Program<Sesame>, eventPass: anchor.web3.PublicKey, offset: number) =>
    anchor.web3.PublicKey.findProgramAddress(
        [textEncoder.encode("EventPassHolder"), eventPass.toBuffer(), new anchor.BN(offset).toArrayLike(Buffer, "le", 2)],
        program.programId
    );

export const deriveEventPassHolderTicket = (program: Program<Sesame>, eventPassHolder: anchor.web3.PublicKey, event: anchor.web3.PublicKey) =>
    anchor.web3.PublicKey.findProgramAddress(
        [textEncoder.encode("EventPassHolderTicket"), eventPassHolder.toBuffer(), event.toBuffer()],
        program.programId
    );