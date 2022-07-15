use std::convert::{TryFrom, TryInto};
use std::sync::atomic::Ordering;
use std::time::{Instant, SystemTime};
use bcs::to_bytes;
use dashmap::DashSet;
use once_cell::sync::Lazy;
use aptos_crypto::{hash::DefaultHasher, HashValue, PrivateKey, Uniform};
use serde::Serialize;
//use lru::LruCache;
use aptos_crypto::ed25519::{Ed25519PrivateKey, Ed25519Signature};
use aptos_types::account_address::AccountAddress;
use aptos_types::chain_id::ChainId;
use aptos_types::transaction::{RawTransaction, Script, SignedTransaction};
use parking_lot::RwLock;
//use aptos_infallible::RwLock;
use rayon::prelude::*;

extern crate lru;


static RAYON_EXEC_POOL: Lazy<rayon::ThreadPool> = Lazy::new(|| {
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap()
});

struct CacheState {
    caches: [DashSet<HashValue>; 2],
    current_idx: usize,
}

impl CacheState {
    fn new() -> CacheState {
        CacheState {
            caches: [DashSet::new(), DashSet::new()],
            current_idx: 0,
        }
    }

    /// Returns a bool whether the item was in cache. After executing,
    /// the item will be guaranteed to be in the current cache. If the item
    /// was added to the current cache, its new size is also returned, o.w. None.
    fn lru_update(&self, hash: HashValue) -> (bool, Option<usize>) {
        if !self.caches[self.current_idx].insert(hash) {
            // hash was in the current cache, no need to return size.
            (true, None)
        } else {
            // hash was added to active.
            //Some(self.caches[self.current_idx].len())
            (self.caches[1 - self.current_idx].contains(&hash), None)
        }
    }

    fn switch(&mut self) {
        self.caches[self.current_idx].clear();
        self.current_idx = 1 - self.current_idx;
    }
}

pub struct ConcurrentTxnCache {
    max_size: usize,
    state: RwLock<CacheState>,
}

impl ConcurrentTxnCache {
    pub fn new(cache_size: usize) -> ConcurrentTxnCache {
        ConcurrentTxnCache {
            max_size: cache_size,
            state: RwLock::new(CacheState::new()),
        }
    }

    fn hash<U: Clone + Serialize>(&self, element: U) -> HashValue {
        let bytes = to_bytes(&element).unwrap();
        let mut hasher = DefaultHasher::new(b"CacheTesting");
        hasher.update(&bytes);
        let hash_res = hasher.finish();
        hash_res
    }

    pub fn insert<U: Clone + Serialize>(&mut self, element: U) {
        let key = self.hash(element);
        let mut state = self.state.write();
        if let Some(cur_size) = state.lru_update(key).1 {
            if cur_size > self.max_size {
                state.switch();
            }
        }
    }

    // /// Filter the set of elements in items according to the cache.
    // /// If an element is in cache, it is removed from the filtered set and marked as recently used.
    // pub fn filter_and_update<U: Clone + Serialize>(&mut self, items: &Vec<U>) -> Vec<U> {
    //     let mut ret = Vec::new();
    //     for i in items {
    //         let key = self.hash(i);
    //         let (in_cache, cur_size) = self.state.read().lru_update(key);
    //
    //         if let Some(cur_size) = cur_size {
    //             if cur_size > self.max_size {
    //                 self.state.write().switch();
    //             }
    //         }
    //
    //         if !in_cache {
    //             ret.push(i.clone());
    //         }
    //     };
    //     ret
    // }

    /// Filter the set of elements in items according to the cache.
    /// If an element is in cache, it is removed from the filtered set and marked as recently used.
    pub fn filter_and_update<U: Clone + Serialize + Sync + Send>(&mut self, items: &Vec<U>) -> Vec<U> {
        // let mut ret = Vec::new();

        // .collect::<Vec<U>>()

        let chunk_size = 100;
        RAYON_EXEC_POOL.install(|| {
            items
                .par_chunks(chunk_size)
                .flat_map(&|chunk:&[U]| {
                    let mut ret = Vec::new();
                    for i in chunk.iter() {
                        let key = self.hash(i);
                        let (in_cache, cur_size) = self.state.read().lru_update(key);
                        if let Some(cur_size) = cur_size {
                            if cur_size > self.max_size {
                                self.state.write().switch();
                            }
                        }

                        if !in_cache {
                            ret.push(i.clone());
                        }
                    }
                    ret
                })
                .collect::<Vec<U>>()
        })
    }

}


// pub struct TxnCache {
//     cache: LruCache<HashValue, bool>,
// }
//
// impl TxnCache {
//     pub fn new(cache_size: usize) -> TxnCache {
//         TxnCache {
//             cache: LruCache::new(cache_size),
//         }
//     }
//
//     fn hash<T: Clone + Serialize>(&self, element: T) -> HashValue {
//         let bytes = to_bytes(&element).unwrap();
//         let mut hasher = DefaultHasher::new(b"CacheTesting");
//         hasher.update(&bytes);
//         let hash_res = hasher.finish();
//         hash_res
//     }
//
//     /// inserts element to the cache
//     pub fn insert<T: Clone + Serialize>(&mut self, element: T) -> () {
//         let key = self.hash(&element);
//         self.cache.put(key, true);
//     }
//
//     /// This returns a boolean whether the item exists in cache, and also moves the key to
//     /// the head of the LRU list if it exists
//     pub fn in_cache<T: Clone + Serialize>(&mut self, element: T) -> bool {
//         let key = self.hash(&element);
//         let val = self.cache.get(&key);
//         let res = match val {
//             None => false,
//             _ => true,
//         };
//         res
//     }
//
//     /// Filter the set of elements in items according to the cache.
//     /// If an element is in cache, it is removed from the filtered set and marked as recently used.
//     pub fn filter_and_update<U: Clone + Serialize>(&mut self, mut items: &mut Vec<U>) -> Vec<U> {
//         // items.retain(|i| !self.in_cache(i));
//
//         //
//         // let (in_cache,not_in_cache):(Vec<U>,Vec<U>)=list
//         //     .into_iter()
//         //     .partition(|x|self.in_cache(i));
//
//
//         // let mut add_to_cache=Vec::new();
//         // // let mut not_in_cache=Vec::new();
//         // for i in items{
//         //     if !self.in_cache(i) {
//         //         add_to_cache.push(i);
//         //     }
//         // }
//         // items.retain(|i| if self.in_cache(i) {
//         //     false
//         // } else {
//         //     add_to_cache.push(i);
//         //     true
//         // });
//
//         items.retain(|i| {
//             //let delete = {
//             let remove = self.in_cache(i);
//             if !remove {
//                 self.insert(i);
//             }
//             //};
//             !remove
//         });
//
//         // TODO: update cache as a seperated step (can also be done for each item during the iteration)
//         // not_in_cache
//         items.to_vec()
//     }
// }

// #[test]
// fn int_test() {
//     let mut my_cache: TxnCache = TxnCache::new(2);
//     my_cache.insert(1);
//     assert_eq!(my_cache.in_cache(1), true);
//     assert_eq!(my_cache.in_cache(10), false);
//
//     let mut txns = vec![1];
//     my_cache.filter_and_update(&mut txns);
//     let empty_list: Vec<i32> = vec![];
//     assert_eq!(txns, empty_list);
//
//     my_cache.insert(2);
//     my_cache.insert(3); // 1 is evicted after the insertion of 2,3 due to cache size =2
//
//     assert_eq!(my_cache.in_cache(1), false);
//     assert_eq!(my_cache.in_cache(2), true);
//     assert_eq!(my_cache.in_cache(3), true);
//
//     let mut new_txns = vec![1, 3, 2];
//     my_cache.filter_and_update(&mut new_txns);
//     assert_eq!(new_txns, vec![1, 2]); // after cheking 1 it is inserted and override the oldest which is 2 at the time
// }


// #[test]
// fn update_priority_test() {
//     let mut my_cache: TxnCache = TxnCache::new(2);
//     my_cache.insert(1);
//     my_cache.insert(2);
//     assert_eq!(my_cache.in_cache(1), true);
//     my_cache.insert(3);
//     assert_eq!(my_cache.in_cache(1), true);
//     assert_eq!(my_cache.in_cache(2), false);
//     assert_eq!(my_cache.in_cache(3), true);
// }


fn generate_txn(id: u64) -> SignedTransaction {
    let txn: SignedTransaction = SignedTransaction::new(
        RawTransaction::new_script(
            AccountAddress::random(),
            0,
            Script::new(vec![], vec![], vec![]),
            0,
            0,
            100 * id,
            ChainId::test(),
        ),
        Ed25519PrivateKey::generate_for_testing().public_key(),
        Ed25519Signature::try_from(&[1u8; 64][..]).unwrap(),
    );
    txn
}


// #[test]
// fn txn_test() {
//     let mut my_cache: TxnCache = TxnCache::new(2);
//     let txn1 = generate_txn(1);
//     let txn2 = generate_txn(2);
//     let txn3 = generate_txn(3);
//
//     my_cache.insert(&txn1);
//     assert_eq!(my_cache.in_cache(&txn1), true);
//     assert_eq!(my_cache.in_cache(&txn2), false);
//
//     let mut txns = vec![&txn1];
//     my_cache.filter_and_update(&mut txns);
//     let empty_list: Vec<&SignedTransaction> = vec![];
//     assert_eq!(txns, empty_list);
//
//     my_cache.insert(&txn2);
//     my_cache.insert(&txn3);
//
//     assert_eq!(my_cache.in_cache(&txn1), false);
//     assert_eq!(my_cache.in_cache(&txn2), true);
//     assert_eq!(my_cache.in_cache(&txn3), true);
//
//     let mut new_txns = vec![&txn2, &txn3, &txn1];
//     my_cache.filter_and_update(&mut new_txns);
//     assert_eq!(new_txns, vec![&txn1]); //2,3 are checked before the insertion of 1
// }


fn fill_cache(cache: &mut ConcurrentTxnCache, cache_size: usize) -> () {
    for i in 1..cache_size + 1 {
        let txn = generate_txn(i.try_into().unwrap());
        cache.insert(&txn);
    }
}

fn create_batch(batch_size: u32, hit_limit: u32, cache_size: u32) -> Vec<SignedTransaction> {
    let mut txn_batch: Vec<SignedTransaction> = Vec::new();

    let residue = batch_size - hit_limit;
    for i in 1..hit_limit + 1 {
        let txn = generate_txn(i.try_into().unwrap());
        txn_batch.push(txn);
    }
    for i in 1..residue + 1 {
        let txn = generate_txn((i + cache_size).into());
        txn_batch.push(txn);
    }
    txn_batch
}


fn test_hit_rate(hit_rate: u32) -> () {
    let cache_size = 70000;
    let batch_size = 10000;

    let hit_limit = hit_rate / 100 * batch_size;


    let mut my_cache= ConcurrentTxnCache::new(cache_size);
    let u_cache_size: u32 = cache_size as u32;

    fill_cache(&mut my_cache, cache_size);

    let mut batch = create_batch(batch_size, hit_limit, u_cache_size);

    let start = Instant::now();
    my_cache.filter_and_update(&mut batch);
    let duration = start.elapsed();
    println!("Time elapsed in filtering for hit rate:  {} is:  {:?}", hit_rate, duration);
}


#[test]
fn rati_test() {
    // let hit_rate=10; // percentages to be filtered
    let hit_rates = vec![0, 10, 50]; // percentages to be filtered
    for i in hit_rates {
        test_hit_rate(i);
    }
}


