#![feature(trivial_bounds)]

use near_sdk::{
	near_bindgen, PanicOnDefault,
	borsh::{self, BorshDeserialize, BorshSerialize},
};

use mini_colls::{
	IntMap
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
	map: IntMap,
}

#[near_bindgen]
impl Contract {

	#[init]
	pub fn new() -> Self {
		Self {
			map: Default::default()
		}
	}

	pub fn set(&mut self, val: String) -> Option<u64> {
		self.map.set(val.as_bytes())
	}

	pub fn update(&mut self, key: u64, val: String) -> bool {
		self.map.update(key, val.as_bytes())
	}

	pub fn del(&mut self, key: u64) -> String {
		String::from_utf8(self.map.del(key).unwrap()).unwrap()
	}

	pub fn get(&self, key: u64) -> String {
		String::from_utf8(self.map.get(key).unwrap()).unwrap()
	}

	pub fn get_key(&self, val: String) -> Option<u64> {
		self.map.get_key(val.as_bytes())
	}
}
    
	