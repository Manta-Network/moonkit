// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>


//! Autogenerated weights for pallet_foreign_asset_creator
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-09, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `girazoki-XPS-15-9530`, CPU: `13th Gen Intel(R) Core(TM) i9-13900H`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/tanssi-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_foreign_asset_creator
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --json-file
// raw.json
// --output
// weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_foreign_asset_creator.
pub trait WeightInfo {
	fn create_foreign_asset() -> Weight;
	fn change_existing_asset_type() -> Weight;
	fn remove_existing_asset_type() -> Weight;
	fn destroy_foreign_asset() -> Weight;
}

/// Weights for pallet_foreign_asset_creator using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `ForeignAssetsCreator::AssetIdToForeignAsset` (r:1 w:1)
	/// Proof: `ForeignAssetsCreator::AssetIdToForeignAsset` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssetsCreator::ForeignAssetToAssetId` (r:0 w:1)
	/// Proof: `ForeignAssetsCreator::ForeignAssetToAssetId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_foreign_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80`
		//  Estimated: `3673`
		// Minimum execution time: 17_654_000 picoseconds.
		Weight::from_parts(18_621_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `ForeignAssetsCreator::AssetIdToForeignAsset` (r:1 w:1)
	/// Proof: `ForeignAssetsCreator::AssetIdToForeignAsset` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ForeignAssetsCreator::ForeignAssetToAssetId` (r:0 w:2)
	/// Proof: `ForeignAssetsCreator::ForeignAssetToAssetId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn change_existing_asset_type() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `427`
		//  Estimated: `3880`
		// Minimum execution time: 17_469_000 picoseconds.
		Weight::from_parts(20_276_697, 3880)
			// Standard Error: 1_876
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `ForeignAssetsCreator::AssetIdToForeignAsset` (r:1 w:1)
	/// Proof: `ForeignAssetsCreator::AssetIdToForeignAsset` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ForeignAssetsCreator::ForeignAssetToAssetId` (r:0 w:1)
	/// Proof: `ForeignAssetsCreator::ForeignAssetToAssetId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_existing_asset_type() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `427`
		//  Estimated: `3880`
		// Minimum execution time: 15_165_000 picoseconds.
		Weight::from_parts(18_041_533, 3880)
			// Standard Error: 1_836
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssetsCreator::AssetIdToForeignAsset` (r:1 w:1)
	/// Proof: `ForeignAssetsCreator::AssetIdToForeignAsset` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssetsCreator::ForeignAssetToAssetId` (r:0 w:1)
	/// Proof: `ForeignAssetsCreator::ForeignAssetToAssetId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn destroy_foreign_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `981`
		//  Estimated: `4441`
		// Minimum execution time: 22_589_000 picoseconds.
		Weight::from_parts(26_897_574, 4441)
			// Standard Error: 3_872
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
		}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `ForeignAssetsCreator::AssetIdToForeignAsset` (r:1 w:1)
	/// Proof: `ForeignAssetsCreator::AssetIdToForeignAsset` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssetsCreator::ForeignAssetToAssetId` (r:0 w:1)
	/// Proof: `ForeignAssetsCreator::ForeignAssetToAssetId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_foreign_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80`
		//  Estimated: `3673`
		// Minimum execution time: 17_654_000 picoseconds.
		Weight::from_parts(18_621_000, 3673)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `ForeignAssetsCreator::AssetIdToForeignAsset` (r:1 w:1)
	/// Proof: `ForeignAssetsCreator::AssetIdToForeignAsset` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ForeignAssetsCreator::ForeignAssetToAssetId` (r:0 w:2)
	/// Proof: `ForeignAssetsCreator::ForeignAssetToAssetId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn change_existing_asset_type() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `427`
		//  Estimated: `3880`
		// Minimum execution time: 17_469_000 picoseconds.
		Weight::from_parts(20_276_697, 3880)
			// Standard Error: 1_876
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `ForeignAssetsCreator::AssetIdToForeignAsset` (r:1 w:1)
	/// Proof: `ForeignAssetsCreator::AssetIdToForeignAsset` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ForeignAssetsCreator::ForeignAssetToAssetId` (r:0 w:1)
	/// Proof: `ForeignAssetsCreator::ForeignAssetToAssetId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_existing_asset_type() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `427`
		//  Estimated: `3880`
		// Minimum execution time: 15_165_000 picoseconds.
		Weight::from_parts(18_041_533, 3880)
			// Standard Error: 1_836
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssetsCreator::AssetIdToForeignAsset` (r:1 w:1)
	/// Proof: `ForeignAssetsCreator::AssetIdToForeignAsset` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssetsCreator::ForeignAssetToAssetId` (r:0 w:1)
	/// Proof: `ForeignAssetsCreator::ForeignAssetToAssetId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn destroy_foreign_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `981`
		//  Estimated: `4441`
		// Minimum execution time: 22_589_000 picoseconds.
		Weight::from_parts(26_897_574, 4441)
			// Standard Error: 3_872
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}