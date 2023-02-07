use casper_dao_contracts::voting::Choice;
use casper_types::bytesrepr::ToBytes;

#[test]
fn test_enum_serialization() {
    assert_eq!(Choice::Against.to_bytes().unwrap(), vec![0u8]);
    assert_eq!(Choice::InFavor.to_bytes().unwrap(), vec![1u8]);
}
