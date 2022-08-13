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
### Ticket Issuance Workflow

1) Lock [Payment](#payment-processor)
3) Call ticket_issue instruction on chain
4) Charge [Payment](#payment-processor)
5) Create a QR code containing 
   - seat_id (for finding PDA)
   - NAME & seat_name (or empty) & randomness (for Key Generation) + event::pubkey
   - event::pubkey (enables minting on global site)
   - should probably base64 encode or something
6) [Mail](#mail-provider) ticket to customer
7) [Log](#log-db) customer & purchase details

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

- Stripe
  - https://stripe.com/docs/payments
- PayPal?
- Crypto.com?
  - https://crypto.com/eea/pay-merchant
  - https://pay-docs.crypto.com/#overview-accepting-payments
- USDC on Solana?

---
### Mail Provider

- GMail
  - https://developers.google.com/gmail/api/guides/sending

- Mailchimp?
---
### Log DB

- Data
  - customer info
  - payment details
  - event pubkey
  - ticket pubkey
  - secret?

- Privileges
  - INSERT on log DB

- Google Workspace
  - https://developers.google.com/sheets/api
  - https://developers.google.com/sheets/api/quickstart/nodejs
  - https://developers.google.com/sheets/api/samples/writing#append_values
  - https://developers.google.com/sheets/api/guides/values#node.js_4

- Google Big Query?  
- SQL?  
- Slack?  


---
## User Interfaces


---
### Sesame WEBSITE

Requires the use of Phantom Wallet.  

- Admin:
  - Create an organization
  - Create events for an organization
  - Update organization data
  - Update event data
  - ? Manually delete single ticket
    - requires seat_id, co-signed by ticket / admin
  - ? Delete all tickets for an event
    - co-signed by ticket / admin
- User
  - mint NFT for ticket

---
### Organizer integrated site

- Allows users to purchase tickets
  - may require list of seat names
- Issue ticket manually
  - must be here to send mail, log "sale"
- Issue refunds & delete ticket
  - refund only here


- Need a page to create tickets for event passes
  - is a page for a specific event_pass account (has the key)
  - user needs to provide
    - NAME ( HAS TO BE PRECISE :( )
    - pass offset (probably not requried())
    - pass_holder offset
    - secret bytes (could be a random word also)
  - I'll mail a link to a specific redeem website that knows the event_pass::key
  - I'll show the list of events the user can create tickets for
  - I'll show the tickets created / available
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

### WASM app?
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

- Ticket sale

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
# well fuck that generated a new program id lol
# so tried to update ids everywhere and ran:
solana program deploy /Users/grrwahrr/IdeaProjects/sesame/target/deploy/sesame.so
# that seems to have worked
#Program Id: 2GTUkXFnABGVHFMqT1tVofBLPrBTAxzjb4Z2rpeMGsJG

```