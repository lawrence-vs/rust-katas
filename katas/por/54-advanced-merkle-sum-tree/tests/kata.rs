use kata_por_54::{
    Account, MerkleSumTree, Node, PorError, Proof, Side, Step, VerifyError, leaf_hash, node_hash,
    verify,
};

fn acct(id: u64, balance: u64) -> Account {
    Account { id, balance }
}

fn leaf(a: &Account) -> Node {
    Node {
        hash: leaf_hash(a),
        sum: a.balance,
    }
}

fn five() -> [Account; 5] {
    [
        acct(1, 100),
        acct(2, 250),
        acct(3, 0),
        acct(4, 7),
        acct(5, 1000),
    ]
}

#[test]
fn every_client_can_verify_their_balance() {
    let accounts = five();
    let tree = MerkleSumTree::build(&accounts).expect("five small balances build");
    for (i, account) in accounts.iter().enumerate() {
        let proof = tree.proof(i).expect("every account has a proof");
        assert_eq!(
            verify(&tree.root(), account, &proof),
            Ok(()),
            "every client can verify their balance"
        );
    }
}

#[test]
fn the_root_sum_is_the_total_liabilities() {
    let tree = MerkleSumTree::build(&five()).expect("five small balances build");
    assert_eq!(
        tree.root().sum,
        1357,
        "the root sum is the total liabilities"
    );
    assert_eq!(
        MerkleSumTree::build(&[]).map(|t| t.root()),
        Err(PorError::Empty),
        "an empty book is an error"
    );
    assert_eq!(
        tree.proof(5),
        None,
        "there is no proof past the last account"
    );
}

#[test]
fn a_total_that_overflows_is_rejected() {
    let accounts = [acct(1, 100), acct(2, u64::MAX - 49)];
    assert_eq!(
        MerkleSumTree::build(&accounts).map(|t| t.root().sum),
        Err(PorError::Overflow),
        "a total that overflows is rejected"
    );
}

#[test]
fn a_proof_checks_the_clients_own_leaf() {
    let accounts = five();
    let tree = MerkleSumTree::build(&accounts).expect("five small balances build");
    let alices = tree.proof(0).expect("alice has a proof");
    assert_eq!(
        verify(&tree.root(), &acct(2, 250), &alices),
        Err(VerifyError::LeafMismatch),
        "a proof checks the client's own leaf"
    );

    let mut lied = five();
    lied[0] = acct(1, 40);
    let tree = MerkleSumTree::build(&lied).expect("lied-about balances build");
    let proof = tree.proof(0).expect("account 1 has a proof");
    assert_eq!(
        verify(&tree.root(), &acct(1, 100), &proof),
        Err(VerifyError::LeafMismatch),
        "a proof for a smaller balance does not verify"
    );
}

#[test]
fn the_root_hash_commits_to_the_sums() {
    let tree = MerkleSumTree::build(&five()).expect("five small balances build");
    let root = tree.root();
    let mut proof = tree.proof(0).expect("alice has a proof");
    proof.path[0].sibling.sum = 50;
    let forged_root = Node {
        hash: root.hash,
        sum: root.sum - 200,
    };
    assert_eq!(
        verify(&forged_root, &five()[0], &proof),
        Err(VerifyError::RootMismatch),
        "the root hash commits to the sums"
    );
}

#[test]
fn verify_rejects_a_proof_that_overflows() {
    let fake = Node {
        hash: leaf_hash(&acct(99, u64::MAX - 49)),
        sum: u64::MAX - 49,
    };
    let a = leaf(&acct(1, 100));
    let root = Node {
        hash: node_hash(&a, &fake),
        sum: 50,
    };
    let proof = Proof {
        leaf: a,
        path: vec![Step {
            sibling: fake,
            side: Side::Right,
        }],
    };
    assert_eq!(
        verify(&root, &acct(1, 100), &proof),
        Err(VerifyError::Overflow),
        "verify rejects a proof that overflows"
    );
}
