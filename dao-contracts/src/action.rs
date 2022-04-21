use casper_dao_utils::casper_dao_macros::{CLTyped, FromBytes, ToBytes};

#[derive(CLTyped, PartialEq, Debug, FromBytes, ToBytes)]
pub enum Action {
    AddToWhitelist,
    RemoveFromWhitelist,
    ChangeOwner,
}

impl Action {
    pub fn get_entry_point(&self) -> String {
        match self {
            Action::AddToWhitelist => "add_to_whitelist",
            Action::RemoveFromWhitelist => "remove_from_whitelist",
            Action::ChangeOwner => "change_ownership",
        }
        .to_string()
    }

    pub fn get_arg(&self) -> &str {
        match self {
            Action::AddToWhitelist => "address",
            Action::RemoveFromWhitelist => "address",
            Action::ChangeOwner => "owner",
        }
    }
}

#[test]
fn test_action() {
    use casper_types::bytesrepr::FromBytes;
    use casper_types::bytesrepr::ToBytes;
    let action = Action::ChangeOwner;
    let (deserialized_action, _) = Action::from_bytes(&action.to_bytes().unwrap()).unwrap();

    assert_eq!(action, deserialized_action);
    assert_eq!(deserialized_action.get_arg(), "owner");
    assert_eq!(
        deserialized_action.get_entry_point(),
        "change_ownership".to_string()
    );
}
