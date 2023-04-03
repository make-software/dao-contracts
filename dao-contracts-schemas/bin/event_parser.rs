use blake2::{
    digest::{Update, VariableOutput},
    Digest,
    VarBlake2b,
};
use casper_dao_contracts::voting::voting_state_machine::VotingStateMachine;
// Development playground!
use casper_dao_contracts::{
    reputation_voter::ReputationVotingCreated,
    simple_voter::SimpleVotingCreated,
};
use casper_types::bytesrepr::{Bytes, FromBytes, ToBytes};

pub fn main() {
    // let hex_str = "9a0000001d0000006576656e745f52657075746174696f6e566f74696e674372656174656400ab69caaa1f0e920c5e3fe0e2a268f3187c99d6b1fcb51ba47ee7e11a640316ca000105040000007465737400ec8ac739798ed1ea18a9cde6f6c3ebb93fb4667a8b0befd98e4f1ba61c85e5790101010500000001000000809706000000000001000000809706000000000001020001088051010000000000";
    // let event: ReputationVotingCreated = from_bytes(hex_str);
    // println!("{:?}", event);

    // println!("{:?}", to_dictionary_item_key(&5u32));

    let hex_str = "01050000000000010100000000000000000000000000000000000000000000000000253c8701000000ec8ac739798ed1ea18a9cde6f6c3ebb93fb4667a8b0befd98e4f1ba61c85e579021027803a090000000000002f0d0000000000022c01016401e528b41c5c6aa9aae97eda0561723229a12d55a6dd78801522abefa245c87bea0102f40102f40102f40102f401809706000000000080970600000000008097060000000000809706000000000001805101000000000000a302000000000000010117fd44a38b0c7366c8a234e285c5a3dd046e9699ee4dff0249bb3f2989e17cf40164010880f40300000000000164012c39c7f53e5351b648d103a9305844948b1febf5cc23c21328fce8f6396381cf0000000100000001656abd87d67d617a19b69802c84381b384536aff4cd6bc7768a076d4cd2500b4040000006275726e02000000050000006f776e65722100000000ab69caaa1f0e920c5e3fe0e2a268f3187c99d6b1fcb51ba47ee7e11a640316ca0b06000000616d6f756e74020000000105080100010200";
    let event: Option<VotingStateMachine> = from_bytes(hex_str);
    let voting = event.unwrap();
    println!("Voting created: {:#?}", voting.created_at());

    let state = voting.state_in_time(voting.created_at());

    // let mut new_state = None;
    println!("{:#?}", state);
}

fn from_bytes<T: FromBytes>(bytes: &str) -> T {
    let bytes = hex::decode(bytes).unwrap();
    // let bytes = Bytes::from_bytes(&bytes).unwrap();
    T::from_bytes(&bytes).unwrap().0
}

fn to_dictionary_item_key<T: ToBytes>(key: &T) -> String {
    let preimage = key.to_bytes().unwrap();
    println!("Preimage: {:?}", &preimage);
    let hash = blake2b(preimage);
    hex::encode(hash)
}

fn blake2b<T: AsRef<[u8]>>(data: T) -> [u8; 32] {
    let mut result = [0; 32];
    let mut hasher = VarBlake2b::new(32).expect("should create hasher");

    hasher.update(data);
    hasher.finalize_variable(|slice| {
        result.copy_from_slice(slice);
    });
    result
}
