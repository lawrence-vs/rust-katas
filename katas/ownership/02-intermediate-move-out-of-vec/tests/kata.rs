use kata_ownership_02::equip;

#[test]
fn equip_leaves_the_inventory_whole() {
    let inventory = vec![String::from("Sword"), String::from("Shield")];
    assert_eq!(equip(&inventory), "Sword");
    assert_eq!(inventory.len(), 2);
}
