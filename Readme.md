Useful cmds for geyser plugin

First, fund your default local wallet:
solana --url localhost airdrop 2


This requests 2 SOL from your local test validator.

Next, get your wallet address:
solana address


This is the sender's public key.

Create another wallet to receive the transfer:
solana-keygen new --no-bip39-passphrase --outfile /tmp/geyser-recipient.json


Get its public key:
solana-keygen pubkey /tmp/geyser-recipient.json


Copy the resulting address.

Finally, transfer 1 SOL to that address:
solana --url localhost transfer <RECIPIENT_ADDRESS> 1 --allow-unfunded-recipient
solana --url localhost transfer 7FDFKctXSEsEdRk83pzAaiudnBHk7ReYNzCL1jaR5cJD 1 --allow-unfunded-recipient

Replace <RECIPIENT_ADDRESS> with the address you just generated.


SanitizedTransaction {
    message: Legacy(
        LegacyMessage {
            message: Message {
                header: MessageHeader {
                    num_required_signatures: 1,
                    num_readonly_signed_accounts: 0,
                    num_readonly_unsigned_accounts: 1
                },

                account_keys: [
                    CDu6x7LTF6quqrdJHinxMUA6YpGsSuVfYenXvFx6DCtT,
                    7FDFKctXSEsEdRk83pzAaiudnBHk7ReYNzCL1jaR5cJD,
                    11111111111111111111111111111111
                ],

                recent_blockhash: HmSvvYTBfBYtPE6L3GiZ46AWY5HxgG3Vxpv5vwbfoxLG,

                instructions: [...]
            }
        }
    ),

    message_hash: D7HFJojPAoqVADJjfxW7eXf6bxJNU1AwtmUgDcNXx8LP,

    is_simple_vote_tx: false,

    signatures: [
        3UfsttdteBfTsNLHCeoD9JfzWF3J6MndsetJUzi2aV33K6tYWxY4WgAYVCGmpGUFyHSF3qMLSPXsoMzLrW5Q81dT
    ]
}