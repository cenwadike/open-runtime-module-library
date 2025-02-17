#![cfg(test)]

use super::*;

use frame_support::{
	construct_runtime, parameter_types,
	traits::{ConstU32, ConstU64, Everything, SortedMembers},
};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

use std::cell::RefCell;

mod oracle {
	pub use super::super::*;
}

pub type AccountId = u128;
type Key = u32;
type Value = u32;

impl frame_system::Config for Test {
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type BlockWeights = ();
	type BlockLength = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = Everything;
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

thread_local! {
	static TIME: RefCell<u32> = RefCell::new(0);
}

pub struct Timestamp;
impl Time for Timestamp {
	type Moment = u32;

	fn now() -> Self::Moment {
		TIME.with(|v| *v.borrow())
	}
}

impl Timestamp {
	pub fn set_timestamp(val: u32) {
		TIME.with(|v| *v.borrow_mut() = val);
	}
}

parameter_types! {
	pub const RootOperatorAccountId: AccountId = 4;
	pub static OracleMembers: Vec<AccountId> = vec![1, 2, 3];
}

pub struct Members;

impl SortedMembers<AccountId> for Members {
	fn sorted_members() -> Vec<AccountId> {
		OracleMembers::get()
	}
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type OnNewData = ();
	type CombineData = DefaultCombineData<Self, ConstU32<3>, ConstU32<600>>;
	type Time = Timestamp;
	type OracleKey = Key;
	type OracleValue = Value;
	type RootOperatorAccountId = RootOperatorAccountId;
	type Members = Members;
	type WeightInfo = ();
	type MaxHasDispatchedSize = ConstU32<100>;
	type MaxFeedValues = ConstU32<5>;
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
		ModuleOracle: oracle::{Pallet, Storage, Call, Event<T>},
	}
);

// This function basically just builds a genesis storage key/value store
// according to our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let storage = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	let mut t: sp_io::TestExternalities = storage.into();

	t.execute_with(|| {
		Timestamp::set_timestamp(12345);
	});

	t
}
