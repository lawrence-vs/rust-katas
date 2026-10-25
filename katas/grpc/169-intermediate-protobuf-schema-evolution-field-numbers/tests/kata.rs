use kata_grpc_169::pb::v1;
use kata_grpc_169::{Account, decode_v2, encode_v2};
use prost::Message;
use prost_types::DescriptorProto;

fn schema(package: &str) -> DescriptorProto {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/proto");
    let fds = protox::compile(["v1/account.proto", "v2/account.proto"], [dir]).unwrap();
    let file = fds
        .file
        .into_iter()
        .find(|f| f.package() == package)
        .unwrap();
    file.message_type
        .into_iter()
        .find(|m| m.name() == "Account")
        .unwrap()
}

fn numbers(message: &DescriptorProto) -> Vec<(String, i32)> {
    message
        .field
        .iter()
        .map(|f| (f.name().to_string(), f.number()))
        .collect()
}

fn number_of(message: &DescriptorProto, name: &str) -> Option<i32> {
    message
        .field
        .iter()
        .find(|f| f.name() == name)
        .map(|f| f.number())
}

fn old_sender_bytes() -> Vec<u8> {
    v1::Account {
        id: "acc-1".into(),
        display_name: "Ada".into(),
        legacy_email: "ada@example.com".into(),
        balance_cents: 1_000,
    }
    .encode_to_vec()
}

fn new_account(overdraft_limit_cents: Option<u64>) -> Account {
    Account {
        id: "acc-2".into(),
        display_name: "Grace".into(),
        phone: "+27 21 555 0100".into(),
        balance_cents: 2_500,
        overdraft_limit_cents,
    }
}

#[test]
fn old_sender_removed_field_does_not_leak_into_a_new_field() {
    let account = decode_v2(&old_sender_bytes()).unwrap();
    assert_eq!(account.id, "acc-1");
    assert_eq!(account.display_name, "Ada");
    assert_eq!(account.balance_cents, 1_000);
    assert_eq!(
        account.phone, "",
        "old sender's legacy_email decoded as phone"
    );
}

#[test]
fn old_sender_leaves_new_optional_field_absent() {
    let account = decode_v2(&old_sender_bytes()).unwrap();
    assert_eq!(
        account.overdraft_limit_cents, None,
        "a field the old sender never sent must decode as absent"
    );
}

#[test]
fn explicit_zero_survives_a_round_trip() {
    for limit in [None, Some(0), Some(25_000)] {
        let sent = new_account(limit);
        let got = decode_v2(&encode_v2(&sent)).unwrap();
        assert_eq!(
            got.overdraft_limit_cents, limit,
            "explicit zero overdraft limit decoded as unset"
        );
        assert_eq!(got, sent);
    }
}

#[test]
fn old_reader_ignores_fields_it_does_not_know() {
    let bytes = encode_v2(&new_account(Some(25_000)));
    let old = match v1::Account::decode(&bytes[..]) {
        Ok(old) => old,
        Err(e) => panic!("old reader rejected a newer message: {e}"),
    };
    assert_eq!(old.id, "acc-2");
    assert_eq!(old.display_name, "Grace");
    assert_eq!(old.balance_cents, 2_500);
    assert_eq!(
        old.legacy_email, "",
        "new sender's phone decoded as the old reader's legacy_email"
    );
}

#[test]
fn removed_field_number_and_name_are_reserved() {
    let v2 = schema("account.v2");
    assert!(
        v2.reserved_range
            .iter()
            .any(|r| r.start() <= 3 && 3 < r.end()),
        "field number 3 (legacy_email) is not reserved"
    );
    assert!(
        v2.reserved_name.iter().any(|n| n == "legacy_email"),
        "name legacy_email is not reserved"
    );
    assert!(
        v2.field.iter().all(|f| f.number() != 3),
        "a v2 field reuses number 3"
    );
}

#[test]
fn overdraft_limit_is_declared_optional() {
    let v2 = schema("account.v2");
    let field = v2
        .field
        .iter()
        .find(|f| f.name() == "overdraft_limit_cents")
        .unwrap();
    assert!(
        field.proto3_optional(),
        "overdraft_limit_cents must be declared optional"
    );
}

#[test]
fn v2_keeps_every_number_it_already_had() {
    let v2 = schema("account.v2");
    for (name, number) in [
        ("id", 1),
        ("display_name", 2),
        ("balance_cents", 4),
        ("overdraft_limit_cents", 5),
    ] {
        assert_eq!(
            number_of(&v2, name),
            Some(number),
            "{name} changed its field number"
        );
    }
    assert!(number_of(&v2, "phone").is_some(), "v2 lost its phone field");
}

#[test]
fn v1_is_frozen() {
    let v1 = schema("account.v1");
    let want: Vec<(String, i32)> = [
        ("id", 1),
        ("display_name", 2),
        ("legacy_email", 3),
        ("balance_cents", 4),
    ]
    .iter()
    .map(|&(n, k)| (n.to_string(), k))
    .collect();
    assert_eq!(
        numbers(&v1),
        want,
        "proto/v1 is what old clients run; don't edit it"
    );
}
