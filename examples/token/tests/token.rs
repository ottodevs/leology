// This file will be autogenerated in the future.
use leology::*;

generate_bindings! {
    Token, {
        [
        { transfer_private, (sender: Balance, to: Address<Testnet3>, amount: u64), (Balance, Balance) },
        { mint_private, (to: Address<Testnet3>, amount: u64), (Balance) },
        ],
        [{ Balance, (amount: u64) }],
    }
}
