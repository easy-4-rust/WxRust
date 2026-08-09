//! 腾讯企点服务实现。
//!
//! 对应 Java `me.chanjar.weixin.qidian.api.impl.WxQidianServiceImpl`（继承
//! `WxQidianServiceHttpComponentsImpl` → `BaseWxQidianServiceImpl`）：
//! 组合门面 trait 的默认实现 + 多企点配置管理（对应 Java `configStorageMap`
//! + `WxQidianConfigStorageHolder`）。子服务以 `Weak<dyn WxQidianService>`
//! 注入（对应 Java `new WxQidianDialServiceImpl(this)` 的循环引用，Rust
//! 用弱引用打破）。重试参数（对应 Java `retrySleepMillis`/`maxRetryTimes`
//! 字段）以 `Mutex` 承载内部可变性。

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::api::r#impl::{WxQidianCallDataServiceImpl, WxQidianDialServiceImpl};
use crate::api::{WxQidianCallDataService, WxQidianDialService, WxQidianService};
use crate::config::WxQidianConfigStorage;
use crate::util::wx_qidian_config_storage_holder::WxQidianConfigStorageHolder;

/// 子服务集合（对应 Java `BaseWxQidianServiceImpl` 的 dial/callData 字段）。
struct SubServices {
    dial: Arc<dyn WxQidianDialService>,
    call_data: Arc<dyn WxQidianCallDataService>,
}

/// 腾讯企点服务实现（reqwest HTTP 后端）。
pub struct WxQidianServiceImpl {
    client: reqwest::Client,
    config_storages: Mutex<HashMap<String, Arc<dyn WxQidianConfigStorage>>>,
    default_mp_id: String,
    retry_sleep_millis: Mutex<i32>,
    max_retry_times: Mutex<i32>,
    sub_services: OnceLock<SubServices>,
}

impl WxQidianServiceImpl {
    /// 构建服务（子服务注入 `Weak<dyn WxQidianService>` 打破循环引用）。
    ///
    /// # 参数
    /// - `config`：初始企点配置（对应 Java `setWxMpConfigStorage`）
    pub fn new_arc(config: Arc<dyn WxQidianConfigStorage>) -> Arc<Self> {
        let mp_id = config.app_id().to_string();
        let mut storages = HashMap::new();
        storages.insert(mp_id.clone(), config);
        let arc = Arc::new(Self {
            client: reqwest::Client::new(),
            config_storages: Mutex::new(storages),
            default_mp_id: mp_id,
            retry_sleep_millis: Mutex::new(1000),
            max_retry_times: Mutex::new(5),
            sub_services: OnceLock::new(),
        });
        // 先转 Arc<dyn WxQidianService> 再降级为 Weak<dyn WxQidianService>
        let dyn_arc: Arc<dyn WxQidianService> = arc.clone();
        let weak = Arc::downgrade(&dyn_arc);
        let _ = arc.sub_services.set(SubServices {
            dial: Arc::new(WxQidianDialServiceImpl::new(weak.clone())),
            call_data: Arc::new(WxQidianCallDataServiceImpl::new(weak)),
        });
        arc
    }

    /// 子服务集合。
    fn services(&self) -> &SubServices {
        self.sub_services.get().expect("子服务已在构建时安装")
    }
}

impl WxQidianService for WxQidianServiceImpl {
    fn config_storage(&self) -> Arc<dyn WxQidianConfigStorage> {
        let map = self.config_storages.lock().unwrap();
        if map.len() == 1 {
            // 只有一个企点配置，直接返回其配置（对应 Java 逻辑）
            return map.values().next().unwrap().clone();
        }
        let holder = WxQidianConfigStorageHolder::get();
        map.get(&holder)
            .cloned()
            .or_else(|| map.get(&self.default_mp_id).cloned())
            .unwrap_or_else(|| map.values().next().unwrap().clone())
    }

    fn set_config_storage(&self, config: Arc<dyn WxQidianConfigStorage>) {
        // 对应 Java `setWxMpConfigStorage`：以 appId 为 key 单配置注入
        let default_mp_id = config.app_id().to_string();
        let mut map = self.config_storages.lock().unwrap();
        map.clear();
        map.insert(default_mp_id.clone(), config);
        WxQidianConfigStorageHolder::set(default_mp_id);
    }

    fn add_config_storage(&self, mp_id: &str, config_storage: Arc<dyn WxQidianConfigStorage>) {
        // 对应 Java synchronized addConfigStorage
        let mut map = self.config_storages.lock().unwrap();
        map.insert(mp_id.to_string(), config_storage);
    }

    fn remove_config_storage(&self, mp_id: &str) {
        // 对应 Java synchronized removeConfigStorage：删除最后一个配置时
        // 仅警告；删除当前默认配置时自动切换剩余首个配置
        let mut map = self.config_storages.lock().unwrap();
        if map.len() == 1 {
            map.remove(mp_id);
            return;
        }
        if WxQidianConfigStorageHolder::get() == mp_id {
            map.remove(mp_id);
            if let Some(next) = map.keys().next() {
                WxQidianConfigStorageHolder::set(next.clone());
            }
            return;
        }
        map.remove(mp_id);
    }

    fn set_multi_config_storages(
        &self,
        config_storages: Vec<(String, Arc<dyn WxQidianConfigStorage>)>,
    ) {
        // 对应 Java `setMultiConfigStorages(Map)`：默认取首个 key
        let default_mp_id = config_storages
            .first()
            .map(|(id, _)| id.clone())
            .unwrap_or_default();
        self.set_multi_config_storages_with_default(config_storages, &default_mp_id);
    }

    fn set_multi_config_storages_with_default(
        &self,
        config_storages: Vec<(String, Arc<dyn WxQidianConfigStorage>)>,
        default_mp_id: &str,
    ) {
        // 对应 Java `setMultiConfigStorages(Map, String)`：整表替换并设
        // 默认 mpId
        let map: HashMap<String, Arc<dyn WxQidianConfigStorage>> =
            config_storages.into_iter().collect();
        *self.config_storages.lock().unwrap() = map;
        WxQidianConfigStorageHolder::set(default_mp_id.to_string());
    }

    fn switchover(&self, mp_id: &str) -> bool {
        let map = self.config_storages.lock().unwrap();
        if map.contains_key(mp_id) {
            WxQidianConfigStorageHolder::set(mp_id.to_string());
            true
        } else {
            false
        }
    }

    fn switchover_to(&self, mp_id: &str) -> Result<(), String> {
        let map = self.config_storages.lock().unwrap();
        if map.contains_key(mp_id) {
            WxQidianConfigStorageHolder::set(mp_id.to_string());
            Ok(())
        } else {
            // 对应 Java `WxRuntimeException`
            Err(format!("无法找到对应【{mp_id}】的公众号配置信息，请核实！"))
        }
    }

    fn http_client(&self) -> reqwest::Client {
        self.client.clone()
    }

    fn retry_sleep_millis(&self) -> i32 {
        *self.retry_sleep_millis.lock().unwrap()
    }

    fn max_retry_times(&self) -> i32 {
        *self.max_retry_times.lock().unwrap()
    }

    fn set_retry_sleep_millis(&self, retry_sleep_millis: i32) {
        *self.retry_sleep_millis.lock().unwrap() = retry_sleep_millis;
    }

    fn set_max_retry_times(&self, max_retry_times: i32) {
        *self.max_retry_times.lock().unwrap() = max_retry_times;
    }

    fn dial_service(&self) -> Option<Arc<dyn WxQidianDialService>> {
        Some(self.services().dial.clone())
    }

    fn call_data_service(&self) -> Option<Arc<dyn WxQidianCallDataService>> {
        Some(self.services().call_data.clone())
    }
}
