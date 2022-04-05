use casper_dao_utils::casper_dao_macros::{CLTyped};
use casper_types::bytesrepr::{FromBytes, ToBytes};

#[derive(CLTyped, PartialEq, Debug)]
pub enum Action {
    AddToWhitelist,
    RemoveFromWhitelist,
    ChangeOwner,
}

impl FromBytes for Action {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), casper_types::bytesrepr::Error> {
        let (variant, bytes) = FromBytes::from_bytes(bytes)?;
        
        match variant {
            1 => {
                Ok((Action::AddToWhitelist, bytes))
            },
            2 => {
                Ok((Action::RemoveFromWhitelist, bytes))
            },
            3 => {
                Ok((Action::ChangeOwner, bytes))
            },
            _ => {
                Err(casper_types::bytesrepr::Error::Formatting)
            }
        }
    }
}

impl ToBytes for Action {
    fn to_bytes(&self) -> Result<Vec<u8>, casper_types::bytesrepr::Error> {
        let mut vec = Vec::with_capacity(self.serialized_length());
        vec.append(&mut match self {
            Action::AddToWhitelist => 1,
            Action::RemoveFromWhitelist => 2,
            Action::ChangeOwner => 3,
        }.to_bytes()?);
        Ok(vec)
    }

    fn serialized_length(&self) -> usize {
        return 1;
    }
}

impl Action {
    pub fn get_entry_point(&self) -> String {
        match self {
            Action::AddToWhitelist => {"add_to_whitelist".to_string()},
            Action::RemoveFromWhitelist => {"remove_from_whitelist".to_string()},
            Action::ChangeOwner => {"change_ownership".to_string()},
        }  
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
    let action = Action::ChangeOwner;
    let (deserialized_action, _) = Action::from_bytes(&action.to_bytes().unwrap()).unwrap();

    assert_eq!(action, deserialized_action);
    assert_eq!(deserialized_action.get_arg(), "owner");
    assert_eq!(deserialized_action.get_entry_point(), "change_ownership".to_string());
}
