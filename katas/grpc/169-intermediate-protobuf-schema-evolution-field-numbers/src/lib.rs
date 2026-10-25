use prost::Message;

pub mod pb {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/account.v1.rs"));
    }
    pub mod v2 {
        include!(concat!(env!("OUT_DIR"), "/account.v2.rs"));
    }
}

/// An account as this service sees it, whichever schema version sent it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub id: String,
    pub display_name: String,
    pub phone: String,
    pub balance_cents: u64,
    /// `None`: the sender never said, so the server applies its default. `Some(0)`: no overdraft at all.
    pub overdraft_limit_cents: Option<u64>,
}

/// What a v2 sender puts on the wire.
pub fn encode_v2(account: &Account) -> Vec<u8> {
    pb::v2::Account {
        id: account.id.clone(),
        display_name: account.display_name.clone(),
        phone: account.phone.clone(),
        balance_cents: account.balance_cents,
        overdraft_limit_cents: account.overdraft_limit_cents.unwrap_or_default(),
    }
    .encode_to_vec()
}

/// Reads bytes from any sender, old or new, as a v2 reader.
/// CLUE: Field 3 used to be `legacy_email`, and old clients still send it. When a v2 reader decodes their bytes, which field gets that string?
/// CLUE: In proto3 a plain `uint64` that is 0 is never written to the wire. How can a reader tell "no overdraft" from "the sender never said"?
/// CLUE: The proto3 language guide has a "Reserved Fields" section and a field-presence label. Which one stops the next person from reusing 3, and which one makes prost generate an `Option`?
pub fn decode_v2(bytes: &[u8]) -> Result<Account, prost::DecodeError> {
    let msg = pb::v2::Account::decode(bytes)?;
    Ok(Account {
        id: msg.id,
        display_name: msg.display_name,
        phone: msg.phone,
        balance_cents: msg.balance_cents,
        overdraft_limit_cents: if msg.overdraft_limit_cents == 0 {
            None
        } else {
            Some(msg.overdraft_limit_cents)
        },
    })
}
