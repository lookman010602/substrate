// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

use frame_support::storage::unhashed;
use codec::Encode;
use frame_support::{StorageDoubleMap, StorageMap, StorageValue, StoragePrefixedMap};
use sp_io::{TestExternalities, hashing::{twox_32, twox_64, blake2_64}};

mod no_instance {
	use codec::{Encode, Decode, EncodeLike};

	pub trait Trait {
		type Origin;
		type BlockNumber: Encode + Decode + EncodeLike + Default + Clone;
	}

	frame_support::decl_module! {
		pub struct Module<T: Trait> for enum Call where origin: T::Origin {}
	}

	frame_support::decl_storage!{
		trait Store for Module<T: Trait> as FinalKeysNone {
			pub Value config(value): u32;

			pub Map: map hasher(blake2_64_concat) u32 => u32;
			pub Map2: map hasher(twox_32_concat) u32 => u32;

			pub DoubleMap: double_map hasher(blake2_64_concat) u32, hasher(blake2_64_concat) u32 => u32;
			pub DoubleMap2: double_map hasher(twox_32_concat) u32, hasher(twox_32_concat) u32 => u32;

			pub TestGenericValue get(fn test_generic_value) config(): Option<T::BlockNumber>;
			pub TestGenericDoubleMap get(fn foo2) config(test_generic_double_map):
				double_map hasher(blake2_64_concat) u32, hasher(blake2_64_concat) T::BlockNumber => Option<u32>;
		}
	}
}

mod instance {
	pub trait Trait<I = DefaultInstance>: super::no_instance::Trait {}

	frame_support::decl_module! {
		pub struct Module<T: Trait<I>, I: Instantiable = DefaultInstance>
			for enum Call where origin: T::Origin {}
	}

	frame_support::decl_storage!{
		trait Store for Module<T: Trait<I>, I: Instantiable = DefaultInstance>
			as FinalKeysSome
		{
			pub Value config(value): u32;

			pub Map: map hasher(blake2_64_concat) u32 => u32;
			pub Map2: map hasher(twox_32_concat) u32 => u32;

			pub DoubleMap: double_map hasher(blake2_64_concat) u32, hasher(blake2_64_concat) u32 => u32;
			pub DoubleMap2: double_map hasher(twox_32_concat) u32, hasher(twox_32_concat) u32 => u32;

			pub TestGenericValue get(fn test_generic_value) config(): Option<T::BlockNumber>;
			pub TestGenericDoubleMap get(fn foo2) config(test_generic_double_map):
				double_map hasher(blake2_64_concat) u32, hasher(blake2_64_concat) T::BlockNumber => Option<u32>;
		}
		add_extra_genesis {
			// See `decl_storage` limitation.
			config(phantom): core::marker::PhantomData<I>;
		}
	}
}

fn twox_32_concat(d: &[u8]) -> Vec<u8> {
	let mut v = twox_32(d).to_vec();
	v.extend_from_slice(d);
	v
}

fn blake2_64_concat(d: &[u8]) -> Vec<u8> {
	let mut v = blake2_64(d).to_vec();
	v.extend_from_slice(d);
	v
}

#[test]
fn final_keys_no_instance() {
	TestExternalities::default().execute_with(|| {
		no_instance::Value::put(1);
		let k = twox_64(b"FinalKeysNoneValue").to_vec();
		assert_eq!(unhashed::get::<u32>(&k), Some(1u32));

		no_instance::Map::insert(1, 2);
		let mut k = twox_32(b"FinalKeysNoneMap").to_vec();
		k.extend(1u32.using_encoded(blake2_64_concat));
		assert_eq!(unhashed::get::<u32>(&k), Some(2u32));
		assert_eq!(&k[..32], &<no_instance::Map>::final_prefix());

		no_instance::Map2::insert(1, 2);
		let mut k = twox_32(b"FinalKeysNoneMap2").to_vec();
		k.extend(1u32.using_encoded(twox_32_concat));
		assert_eq!(unhashed::get::<u32>(&k), Some(2u32));
		assert_eq!(&k[..32], &<no_instance::Map2>::final_prefix());

		no_instance::DoubleMap::insert(&1, &2, &3);
		let mut k = twox_32(b"FinalKeysNoneDoubleMap").to_vec();
		k.extend(1u32.using_encoded(blake2_64_concat));
		k.extend(2u32.using_encoded(blake2_64_concat));
		assert_eq!(unhashed::get::<u32>(&k), Some(3u32));
		assert_eq!(&k[..32], &<no_instance::DoubleMap>::final_prefix());

		no_instance::DoubleMap2::insert(&1, &2, &3);
		let mut k = twox_32(b"FinalKeysNoneDoubleMap2").to_vec();
		k.extend(1u32.using_encoded(twox_32_concat));
		k.extend(2u32.using_encoded(twox_32_concat));
		assert_eq!(unhashed::get::<u32>(&k), Some(3u32));
		assert_eq!(&k[..32], &<no_instance::DoubleMap2>::final_prefix());
	});
}

#[test]
fn final_keys_default_instance() {
	TestExternalities::default().execute_with(|| {
		<instance::Value<instance::DefaultInstance>>::put(1);
		let k = twox_64(b"FinalKeysSomeValue").to_vec();
		assert_eq!(unhashed::get::<u32>(&k), Some(1u32));

		<instance::Map<instance::DefaultInstance>>::insert(1, 2);
		let mut k = twox_32(b"FinalKeysSomeMap").to_vec();
		k.extend(1u32.using_encoded(blake2_64_concat));
		assert_eq!(unhashed::get::<u32>(&k), Some(2u32));
		assert_eq!(&k[..32], &<instance::Map<instance::DefaultInstance>>::final_prefix());

		<instance::Map2<instance::DefaultInstance>>::insert(1, 2);
		let mut k = twox_32(b"FinalKeysSomeMap2").to_vec();
		k.extend(1u32.using_encoded(twox_32_concat));
		assert_eq!(unhashed::get::<u32>(&k), Some(2u32));
		assert_eq!(&k[..32], &<instance::Map2<instance::DefaultInstance>>::final_prefix());

		<instance::DoubleMap<instance::DefaultInstance>>::insert(&1, &2, &3);
		let mut k = twox_32(b"FinalKeysSomeDoubleMap").to_vec();
		k.extend(1u32.using_encoded(blake2_64_concat));
		k.extend(2u32.using_encoded(blake2_64_concat));
		assert_eq!(unhashed::get::<u32>(&k), Some(3u32));
		assert_eq!(&k[..32], &<instance::DoubleMap<instance::DefaultInstance>>::final_prefix());

		<instance::DoubleMap2<instance::DefaultInstance>>::insert(&1, &2, &3);
		let mut k = twox_32(b"FinalKeysSomeDoubleMap2").to_vec();
		k.extend(1u32.using_encoded(twox_32_concat));
		k.extend(2u32.using_encoded(twox_32_concat));
		assert_eq!(unhashed::get::<u32>(&k), Some(3u32));
		assert_eq!(&k[..32], &<instance::DoubleMap2<instance::DefaultInstance>>::final_prefix());
	});
}

#[test]
fn final_keys_instance_2() {
	TestExternalities::default().execute_with(|| {
		<instance::Value<instance::Instance2>>::put(1);
		let k = twox_64(b"Instance2FinalKeysSomeValue").to_vec();
		assert_eq!(unhashed::get::<u32>(&k), Some(1u32));

		<instance::Map<instance::Instance2>>::insert(1, 2);
		let mut k = twox_32(b"Instance2FinalKeysSomeMap").to_vec();
		k.extend(1u32.using_encoded(blake2_64_concat));
		assert_eq!(unhashed::get::<u32>(&k), Some(2u32));
		assert_eq!(&k[..32], &<instance::Map<instance::Instance2>>::final_prefix());

		<instance::Map2<instance::Instance2>>::insert(1, 2);
		let mut k = twox_32(b"Instance2FinalKeysSomeMap2").to_vec();
		k.extend(1u32.using_encoded(twox_32_concat));
		assert_eq!(unhashed::get::<u32>(&k), Some(2u32));
		assert_eq!(&k[..32], &<instance::Map2<instance::Instance2>>::final_prefix());

		<instance::DoubleMap<instance::Instance2>>::insert(&1, &2, &3);
		let mut k = twox_32(b"Instance2FinalKeysSomeDoubleMap").to_vec();
		k.extend(1u32.using_encoded(blake2_64_concat));
		k.extend(2u32.using_encoded(blake2_64_concat));
		assert_eq!(unhashed::get::<u32>(&k), Some(3u32));
		assert_eq!(&k[..32], &<instance::DoubleMap<instance::Instance2>>::final_prefix());

		<instance::DoubleMap2<instance::Instance2>>::insert(&1, &2, &3);
		let mut k = twox_32(b"Instance2FinalKeysSomeDoubleMap2").to_vec();
		k.extend(1u32.using_encoded(twox_32_concat));
		k.extend(2u32.using_encoded(twox_32_concat));
		assert_eq!(unhashed::get::<u32>(&k), Some(3u32));
		assert_eq!(&k[..32], &<instance::DoubleMap2<instance::Instance2>>::final_prefix());
	});
}
