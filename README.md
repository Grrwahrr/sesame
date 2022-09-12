# Sesame
This ticketing service is Blockchain based, storing data and interacting with a Blockchain requires spending tokens on transaction fees.  
Thus, in order to use this ticketing system, a Blockchain wallet with a sufficient balance is required.  
Each ticket related operation has its own associated authority.  

### Ticket Authorities

Each authority can (and should) be a separate account. These accounts are stored in the event configuration. If no account is set, the related action can not be performed. I.e. without a delete authority account, tickets can not be deleted.

#### Issuer 
This account can issue new tickets. It will be used by the ticket vending application.  
It requires a sufficient _SOL_ balance to interact with the Blockchain.

#### Delete

This account can delete tickets. It will be used by an admin to delete issued tickets.  
Deleting tickets frees up space on the Blockchain, which refunds the associated storage fee. This account collects those refunds.

#### Check In
This account can verify a ticket and set it as checked in. 
Updating the Blockchain requires a sufficient _SOL_ balance to update the state.


---
### Ticket Issuance Flow

1) User visits web-shop of the organizer
2) Pays for ticket using [Payment Processor](#payment-processor)
3) Ticket is issued on chain
4) Create a QR code containing 
   - event::pubkey (verified in SCANNER; enables minting on global site)
   - seat_id (for finding PDA)
   - NAME & seat_name (or empty) & randomness (for Key Generation) + event::pubkey
5) [Mail](#mail-provider) ticket to customer
6) [Log](#log-db) payment, customer and ticket details

### Ticket deletion
- Delete the given ticket account
- Increments event.tickets_deleted
- Increments event.tickets_limit

### How is specific seating arranged for?
- Seating is printed on the ticket, stored in QR and used to derive owner key
- vendor website has to have seating info
- including the knowledge of what seats are booked and refunded
- weak point (database could fail and give out seat multiple times)


---
### Domain  

sesamevent.xyz / io / ...


---
### Payment processor

- [x] Stripe
- [ ] PayPal?
- [ ] SolanaPay?
  - https://solanapay.com/
- [ ] Crypto.com?
  - https://crypto.com/eea/pay-merchant
  - https://pay-docs.crypto.com/#overview-accepting-payments

---
### Mail Provider

- [x] Workspace GMail
- [ ] Mailchimp?


---
### Log DB

- [x] Workspace -> Sheets
- [ ] Google Big Query?
- [ ] SQL?
- [ ] Slack?


---
## User Interfaces


---
### Sesame WEBSITE

Requires the use of Phantom Wallet.  

- [x] Landing Page
- [x] Documentation
- [x] Admin:
  - [x] Create & update organizer
  - [x] Create & update events
  - [ ] Create & update event passes
  - [ ] Clean up event data
    - delete ticket co-signed by ticket / admin
  - [ ] Clean up event pass data

---
### Organizer integrated site

- Allows users to purchase tickets
  - If Seating is arranged / specific
    - this is a bit of a point of failure as that's not blockchain enforced
    - requires a DB to store sold and refunded seat names

---

- Allows users to purchase event passes

---

- Allows for manual issuance of tickets
  - must be here to send mail, log "sale"

---

- Allow issuance of refunds & deletion of the ticket
  - refund requires payment provider interaction

---

- Need a page to create tickets from event passes
  - URL will be specific to ONE EventPass (= page has event_pass::key)
  - user, upon buying receive a link to the redeem page
  - and some pass associated data
    - NAME ( HAS TO BE PRECISE :( )
    - pass offset (probably not required)
    - pass_holder offset
    - secret bytes (could be a random word, a u32, gibberish)
  - Opening the page shows all events the pass is valid for
  - With the pass user data
    - I'll show the tickets created / available for their pass
    - Will make sure that the key derived from seed is correct
  - User will pick the event -> I get the offset number for the event
  - need to call an API to issue the ticket
    - I'll generate the key from seed and co-sign
      - Seed is NAME + event_pass::key + randomness
---
### Check In App

Used in the venue to validate & update tickets.
- Scan a users QR code
- Verify
  - event_id on ticket must match app configured
  - use seat_id to find account with app configured event_id
  - make sure the seed generated KEY matches the stored one
- Update ticket state on chain
- Display success / error
- Optimize: Read ticket state
  - indicate probably OK
  - display LOCKED CHECKBOX to confirm
    - is this an optimization?
    - test it

Resources  
- https://www.reddit.com/r/WebAssembly/comments/mlxe0y/qrbar_code_scanner_for_the_browser/
- https://www.smithy.rs/examples/
- https://github.com/piderman314/bardecoder
- https://crates.io/crates/rqrr
- https://crates.io/crates/qrscan

---
### Mint POAP

Allows users to mint their tickets into NFTs.
- Requires the use of Phantom Wallet.
- Requires a positive balance to mint NFT.
- User must possess the original ticket
- The ticket must have been checked in at the event


---
## TODO

- WASM QR scanner
  - done
    - nothing
  - todo
    - WASM
    - scan QR code
    - decode JSON
    - query ticket status -> show red if checked in; yellow if not yet;
    - update ticket status on chain
    - show green on confirmation

---
## NFT docs  
https://docs.metaplex.com/programs/token-metadata/


---
## Deploy local
```bash
# run in home folder (depends on ./test-ledger folder
solana-test-validator
anchor deploy
# account receiving donations has to exist (be funded)
solana transfer JCsJe2cWR3wp9a4kvY9JK4qTR1FiBwxXSzsHyTZuZfFA 10 --allow-unfunded-recipient 
solana transfer 2WEXvXAiBVEcAD2gUAaurKpBvhpririWSEGZmZkdexzg 10 --allow-unfunded-recipient 
# well fuck that generated a new program id lol
# so tried to update ids everywhere and ran:
solana program deploy /Users/grrwahrr/IdeaProjects/sesame/target/deploy/sesame.so
# that seems to have worked
#Program Id: 2GTUkXFnABGVHFMqT1tVofBLPrBTAxzjb4Z2rpeMGsJG

```