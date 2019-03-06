/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/gav-template/srml/example/src/lib.rs

use support::{decl_module, decl_storage, decl_event, StorageMap, ensure, dispatch::Result};
use system::ensure_signed;
use parity_codec::{Encode,Decode};
use balances;
use runtime_primitives::traits::Hash;

#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
pub struct Channel<AccountId, Balance> {
    sender: AccountId,
    receiver: AccountId,
    deposit: Balance,
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
pub struct ChannelOffchainState<Hash, Balance> {
    channel_id: Hash,
    balance: Balance,
}

#[derive(Encode, Decode, Clone, PartialEq, Debug)]
pub enum ChannelStatus {
    Active,
}

/// The module's configuration trait.
pub trait Trait: system::Trait + balances::Trait {
	// TODO: Add other types and constants required configure this module.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

/// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as ChannelsModule {
		State get(state): map T::Hash => Option<ChannelStatus>;
	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event<T>() = default;

		pub fn channel_open(origin, channel: Channel<T::AccountId, T::Balance>) -> Result {
			let sender = ensure_signed(origin)?;
                        ensure!(sender == channel.sender, "channel sender is ok");
                        let channel_id = T::Hashing::hash_of(&channel);
			<State<T>>::insert(channel_id, ChannelStatus::Active);
			//Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}
	}
}

decl_event!(
	/// An event in this module.
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
		SomethingStored(u32, AccountId),
	}
);

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok};
	use runtime_primitives::{
		BuildStorage,
		traits::{BlakeTwo256, IdentityLookup},
		testing::{Digest, DigestItem, Header}
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	impl system::Trait for Test {
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type Digest = Digest;
		type AccountId = u64;
		type Lookup = IdentityLookup<u64>;
		type Header = Header;
		type Event = ();
		type Log = DigestItem;
	}
	impl Trait for Test {
		type Event = ();
	}
	type ChannelsModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
	}

	#[test]
	fn it_works_for_default_value() {
		with_externalities(&mut new_test_ext(), || {
			let channel = Channel { sender: 1, receiver: 2, deposit: 10 };
                        assert_ok!(ChannelsModule::channel_open(Origin::signed(1), channel.to_owned()));
			assert_eq!(ChannelsModule::state(T::Hashing::hash_of(&channel)), Some(()));
		});
	}
}
