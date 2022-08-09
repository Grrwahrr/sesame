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
2) Create a paper wallet from some seed:  
`"open_sesame_" + event::key + secret`
3) Call ticket_issue instruction on chain
4) Charge [Payment](#payment-processor)
5) Create a QR code containing email & seat_id & secret & ?event::pubkey->MINT
6) [Mail](#mail-provider) ticket to customer
7) [Log](#log-db) customer & purchase details

---
### Domain  

sesamevent.xyz / io / ...

---
### Payment processor

- PayPal?
- Crypto.com?
  - https://crypto.com/eea/pay-merchant
  - https://pay-docs.crypto.com/#overview-accepting-payments

---
### Mail Provider

- Google Workspace
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
### Admin Backend

Requires the use of Phantom Wallet.  

Functionality:
- Create an organization
- Create events for an organization
- Update organization data
- Update event data
- Issue a refund & delete specific ticket
- Delete tickets in mass
- manually issue ticket
- manually check in
- manually delete ticket
- manually mint ticket


---
### Store Front

Allows users to purchase tickets.

---
### Check In App

Used in the venue to validate & update tickets.
- Scan a users QR code
- Verify & update ticket oon chain
- Display success / error
- Optimize: Read ticket state
  - indicate probably OK
  - display LOCKED CHECKBOX to confirm
    - is this an optimization?
    - test it 

WASM app?
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

- Website
  - SPA introduction site
    - Sales pitch
    - Donate link
    - Documentation
  - Admin panel
  - Mint panel

- WASM QR scanner

- Ticket sale


TIMEZONE to minute conversion
```
moment().utcOffset("+03:00").utcOffset() // returns 180
moment().utcOffset("-09:00").utcOffset() // returns -540
```

Keypair from seeds
```typescript
Keypair.fromSeed(Uint8Array.from([
    174, 47, 154, 16, 202, 193, 206, 113, 199, 190, 53, 133, 169, 175, 31, 56,
    222, 53, 138, 189, 224, 216, 117, 173, 10, 149, 53, 45, 73, 251, 237, 246,
    15, 185, 186, 82, 177, 240, 148, 69, 241, 227, 167, 80, 141, 89, 240, 121,
    121, 35, 172, 247, 68, 251, 226, 218, 48, 63, 176, 109, 168, 89, 238, 135,
]))

console.log(PublicKey.isOnCurve(key.toBytes()));
```

Sign messages  
```typescript
const message = "The quick brown fox jumps over the lazy dog";
const messageBytes = decodeUTF8(message);

const signature = nacl.sign.detached(messageBytes, keypair.secretKey);
const result = nacl.sign.detached.verify(
  messageBytes,
  signature,
  keypair.publicKey.toBytes()
);

console.log(result);

```



NFT docs  
https://docs.metaplex.com/programs/token-metadata/


---
Deploy local
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