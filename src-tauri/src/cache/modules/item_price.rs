use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex, Weak},
};

use crate::{
    cache::{client::CacheState, types::item_price_info::ItemPriceInfo},
    emit_startup,
    utils::ErrorFromExt,
};
use qf_api::Client as QFClient;
use utils::SubType;
use utils::{get_location, info, read_json_file_optional, Error, LoggerOptions};

/// Index key: (wfm_url or wfm_id, sub_type). One price row per item + sub type.
type PriceKey = (String, Option<SubType>);

#[derive(Debug, Default)]
struct PriceIndex {
    by_url: HashMap<PriceKey, usize>,
    by_id: HashMap<PriceKey, usize>,
}

impl PriceIndex {
    fn build(items: &[ItemPriceInfo]) -> Self {
        let mut index = PriceIndex {
            by_url: HashMap::with_capacity(items.len()),
            by_id: HashMap::with_capacity(items.len()),
        };
        for (i, item) in items.iter().enumerate() {
            // First occurrence wins, matching the previous linear `find` semantics.
            index
                .by_url
                .entry((item.wfm_url.clone(), item.sub_type.clone()))
                .or_insert(i);
            index
                .by_id
                .entry((item.wfm_id.clone(), item.sub_type.clone()))
                .or_insert(i);
        }
        index
    }
}

#[derive(Debug)]
pub struct ItemPriceModule {
    path: PathBuf,
    /// Price rows plus lookup indexes, kept under one lock so they never drift apart.
    items: Mutex<(Vec<ItemPriceInfo>, PriceIndex)>,
    client: Weak<CacheState>,
}

impl ItemPriceModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("items/ItemPrices.json"),
            items: Mutex::new((Vec::new(), PriceIndex::default())),
            client: Arc::downgrade(&client),
        })
    }
    pub async fn check_update(&self, qf_client: &QFClient) -> Result<(bool, String), Error> {
        let client = self.client.upgrade().expect("Client should not be dropped");
        let current_version = client.version.id_price.clone();
        let remote_version = match qf_client.cache().get_cache_id("item_price").await {
            Ok(id) => id,
            Err(e) => {
                let err = Error::from_qf(
                    "Cache:ItemPrice:CheckUpdate",
                    "Failed to get item price cache ID",
                    e,
                    get_location!(),
                );
                err.log("cache_version.json");
                return Err(err);
            }
        };

        if !self.path.exists() {
            Ok((true, remote_version))
        } else {
            Ok((current_version != remote_version, remote_version))
        }
    }

    pub async fn load(
        &self,
        qf_client: &QFClient,
        price_require_update: bool,
    ) -> Result<(), Error> {
        let _client = self.client.upgrade().expect("Client should not be dropped");
        if price_require_update {
            match self.extract(qf_client).await {
                Ok(()) => {
                    info(
                        "Cache:ItemPrice:Load",
                        "Item price cache extracted successfully.",
                        &LoggerOptions::default(),
                    );
                }
                Err(e) => {
                    e.log("cache_version.json");
                    return Err(e);
                }
            }
        }
        match read_json_file_optional::<Vec<ItemPriceInfo>>(&self.path) {
            Ok(items) => {
                let index = PriceIndex::build(&items);
                let count = items.len();
                let mut items_lock = self.items.lock().unwrap();
                *items_lock = (items, index);
                info(
                    "Cache:ItemPrice:Load",
                    format!(
                        "Item price cache loaded successfully with {} items.",
                        count
                    ),
                    &LoggerOptions::default(),
                );
            }
            Err(e) => return Err(e.with_location(get_location!())),
        }
        Ok(())
    }
    async fn extract(&self, qf_client: &QFClient) -> Result<(), Error> {
        emit_startup!("cache.item_price_updating", json!({}));
        let content = qf_client
            .cache()
            .download_cache("item_price")
            .await
            .map_err(|e| {
                Error::from_qf(
                    "Cache:ItemPrice",
                    "Failed to download cache",
                    e,
                    get_location!(),
                )
            })?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                Error::from_io(
                    "Cache:ItemPrice",
                    &parent.to_path_buf(),
                    "Failed to create parent directory",
                    e,
                    get_location!(),
                )
            })?;
        }

        let mut file = File::create(self.path.clone()).map_err(|e| {
            Error::from_io(
                "Cache:ItemPrice",
                &self.path,
                "Failed to create file",
                e,
                get_location!(),
            )
        })?;

        file.write_all(&content).map_err(|e| {
            Error::from_io(
                "Cache:ItemPrice",
                &self.path,
                "Failed to write file",
                e,
                get_location!(),
            )
        })?;
        Ok(())
    }

    /// O(1) lookup by Warframe Market URL + sub type. Clones only the matching row.
    pub fn find_by(
        &self,
        url: impl Into<String>,
        sub_type: Option<SubType>,
    ) -> Result<Option<ItemPriceInfo>, Error> {
        let key: PriceKey = (url.into(), sub_type);
        let guard = self.items.lock().expect("Failed to lock items mutex");
        let (items, index) = &*guard;
        Ok(index.by_url.get(&key).map(|&i| items[i].clone()))
    }

    /// O(1) lookup by Warframe Market item id + sub type. Clones only the matching row.
    pub fn find_by_id(
        &self,
        id: impl Into<String>,
        sub_type: Option<SubType>,
    ) -> Result<Option<ItemPriceInfo>, Error> {
        let key: PriceKey = (id.into(), sub_type);
        let guard = self.items.lock().expect("Failed to lock items mutex");
        let (items, index) = &*guard;
        Ok(index.by_id.get(&key).map(|&i| items[i].clone()))
    }

    /// Filters under the lock and clones only the rows that pass.
    pub fn get_by_filter<F>(&self, predicate: F) -> Vec<ItemPriceInfo>
    where
        F: Fn(&ItemPriceInfo) -> bool,
    {
        let guard = self.items.lock().expect("Failed to lock items mutex");
        guard
            .0
            .iter()
            .filter(|item| predicate(item))
            .cloned()
            .collect::<Vec<ItemPriceInfo>>()
    }
}
