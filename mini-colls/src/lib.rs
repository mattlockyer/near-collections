use near_sdk::{
	borsh::{self, BorshDeserialize, BorshSerialize},
	env::{ storage_remove, storage_read, storage_write, storage_get_evicted }
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct DirectMap;

impl Default for DirectMap {
	fn default() -> Self {
		Self
	}
}

impl DirectMap {
    pub fn set(&mut self, key: &[u8], val: &[u8]) -> Option<Vec<u8>> {
        if storage_write(&key, val) {
			storage_get_evicted()
        } else {
            None
        }
    }

    pub fn del(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        if storage_remove(&key) {
            storage_get_evicted()
        } else {
            None
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        storage_read(&key)
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct IntMap {
	int: u64,
}

impl Default for IntMap {
	fn default() -> Self {
		Self {
			int: 0,
		}
	}
}

impl IntMap {
    pub fn set(&mut self, val: &[u8]) -> Option<u64> {
		let key = self.get_key(val);
        if key.is_some() {
			return key;
		}
		self.int += 1;
		let int_bytes = self.int.to_le_bytes();
		if storage_write(&int_bytes, val) || storage_write(val, &int_bytes) {
			return None;
		}
		Some(self.int)
    }

	pub fn update(&mut self, key: u64, val: &[u8]) -> bool {
		let int_bytes = key.to_le_bytes();
		if !storage_write(&int_bytes, val) || !storage_write(val, &int_bytes) {
			return false;
		}
		true
    }

    pub fn del(&mut self, key: u64) -> Option<Vec<u8>> {
        if !storage_remove(&key.to_le_bytes()) {
			return None;
		}
		let val = storage_get_evicted();
		if let Some(val) = val {
			if storage_remove(&val) {
				return Some(val);
			}
			None
		} else {
			None
		}
    }

    pub fn get(&self, key: u64) -> Option<Vec<u8>> {
        storage_read(&key.to_le_bytes())
    }

	pub fn get_key(&self, val: &[u8]) -> Option<u64> {
		let key = storage_read(&val);
        if let Some(key) = key {
			let mut word = [0u8; 8];
			word.copy_from_slice(&key);
			Some(u64::from_le_bytes(word))
		} else {
			None
		}
	}
}