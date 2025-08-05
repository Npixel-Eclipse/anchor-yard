use std::{cell::RefCell, collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use shipyard::{AllStoragesViewMut, Component, EntityId, IntoIter, View, World};

type Serializer = Box<dyn Fn(&World) -> HashMap<EntityId, Vec<u8>> + Send + Sync>;
type Deserializer = Box<dyn Fn(&mut World, &HashMap<EntityId, Vec<u8>>) + Send + Sync>;

pub struct SnapshotRegistry {
    components: HashMap<String, (Serializer, Deserializer)>,
}

impl SnapshotRegistry {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    /// 스냅샷에 포함할 컴포넌트를 등록합니다.
    pub fn register<T: Component + Serialize + for<'de> Deserialize<'de> + 'static>(&mut self) {
        let type_name = std::any::type_name::<T>().to_string();

        let serializer: Serializer = Box::new(|world| {
            let mut data = HashMap::new();
            if let Ok(view) = world.borrow::<View<T>>() {
                for (id, component) in view.iter().with_id() {
                    if let Ok(bytes) =
                        bincode::serde::encode_to_vec(&component, bincode::config::standard())
                    {
                        data.insert(id, bytes);
                    }
                }
            }
            data
        });

        let deserializer: Deserializer = Box::new(|world, data| {
            let mut all_storages = world.borrow::<AllStoragesViewMut>().unwrap();
            for (_, bytes) in data {
                if let Ok((c, _)) =
                    bincode::serde::decode_from_slice::<T, _>(bytes, bincode::config::standard())
                {
                    all_storages.add_entity((c,));
                }
            }
        });

        self.components
            .insert(type_name, (serializer, deserializer));
    }
}

/// 전역 레지스트리. `main` 함수에서 컴포넌트를 등록할 때 사용합니다.
pub static REGISTRY: Lazy<Mutex<SnapshotRegistry>> =
    Lazy::new(|| Mutex::new(SnapshotRegistry::new()));

thread_local! {
    static CURRENT_WORLD: RefCell<Option<*const World>> = RefCell::new(None);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SystemSnapshot {
    pub system_name: String,
    pub execution_time_ms: u64,
    pub timestamp: u64,
    pub component_data: HashMap<String, HashMap<EntityId, Vec<u8>>>,
}

impl SystemSnapshot {
    pub fn capture_current_world(system_name: &str, execution_time_ms: u64) -> Option<Self> {
        CURRENT_WORLD.with(|world_ref| {
            if let Some(world_ptr) = *world_ref.borrow() {
                let world = unsafe { &*world_ptr };
                let mut component_data = HashMap::new();
                let registry = REGISTRY.lock().unwrap();

                for (name, (serializer, _)) in &registry.components {
                    component_data.insert(name.clone(), serializer(world));
                }

                Some(SystemSnapshot {
                    system_name: system_name.to_string(),
                    execution_time_ms,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    component_data,
                })
            } else {
                None
            }
        })
    }

    pub fn save_to_file(&self) -> Result<(), anyhow::Error> {
        std::fs::create_dir_all("snapshots")?;
        let filename = format!("snapshots/{}_{}.snapshot", self.system_name, self.timestamp);
        let data = bincode::serde::encode_to_vec(self, bincode::config::standard())?;
        std::fs::write(filename, data)?;
        Ok(())
    }

    pub fn load_from_file(path: &std::path::Path) -> Result<Self, anyhow::Error> {
        let data = std::fs::read(path)?;
        let (snapshot, _) = bincode::serde::decode_from_slice(&data, bincode::config::standard())?;
        Ok(snapshot)
    }

    pub fn restore_world(&self) -> Result<World, anyhow::Error> {
        let mut world = World::new();
        let mut all_entities = std::collections::HashSet::new();
        for component_map in self.component_data.values() {
            for entity_id in component_map.keys() {
                all_entities.insert(entity_id);
            }
        }

        let registry = REGISTRY.lock().unwrap();
        for (name, data) in &self.component_data {
            if let Some((_, deserializer)) = registry.components.get(name) {
                deserializer(&mut world, data);
            }
        }
        Ok(world)
    }
}

pub trait WorldSnapshotExt {
    fn run_with_snapshot<F, R>(&self, system_closure: F) -> R
    where
        F: FnOnce() -> R;
    fn run_workload_with_snapshot(
        &self,
        workload_name: impl ToString,
    ) -> Result<(), shipyard::error::RunWorkload>;
}

impl WorldSnapshotExt for World {
    fn run_with_snapshot<F, R>(&self, system_closure: F) -> R
    where
        F: FnOnce() -> R,
    {
        CURRENT_WORLD.with(|world_ref| {
            *world_ref.borrow_mut() = Some(self as *const World);
        });

        let result = system_closure();

        CURRENT_WORLD.with(|world_ref| {
            *world_ref.borrow_mut() = None;
        });

        result
    }

    fn run_workload_with_snapshot(
        &self,
        workload_name: impl ToString,
    ) -> Result<(), shipyard::error::RunWorkload> {
        CURRENT_WORLD.with(|world_ref| {
            *world_ref.borrow_mut() = Some(self as *const World);
        });

        let result = self.run_workload(workload_name.to_string());

        CURRENT_WORLD.with(|world_ref| {
            *world_ref.borrow_mut() = None;
        });

        result
    }
}
