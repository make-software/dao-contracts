use casper_dao_utils::casper_dao_macros::{CLTyped, FromBytes, ToBytes};

/// Enum for actions that (AdminContract)[crate::AdminContract] can perform
///
/// - `AddToWhitelists` - calls `add_to_whitelist` method
/// - `RemoveFromWhitelist` - calls `remove_from_whitelist` method
/// - `ChangeOwner` - calls `change_ownership` method
#[derive(CLTyped, PartialEq, Eq, Debug, FromBytes, ToBytes)]
pub enum Action {
    AddToWhitelist,
    RemoveFromWhitelist,
    ChangeOwner,
}

impl Action {
    pub(crate) fn get_entry_point(&self) -> String {
        match self {
            Action::AddToWhitelist => "add_to_whitelist",
            Action::RemoveFromWhitelist => "remove_from_whitelist",
            Action::ChangeOwner => "change_ownership",
        }
        .to_string()
    }

    pub(crate) fn get_arg(&self) -> &str {
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
