use std::{collections::HashMap, sync::Mutex};

#[cfg(unix)]
pub fn user_name(uid: u32) -> String {
    lazy_static! {
        static ref USERS_CACHE_MUTEX: Mutex<HashMap<u32, String>> = Mutex::new(HashMap::new());
    }
    let mut users_cache = USERS_CACHE_MUTEX.lock().unwrap();
    let name = users_cache
        .entry(uid)
        .or_insert_with(|| {
            users::get_user_by_uid(uid).map_or_else(
                || "????".to_string(),
                |u| u.name().to_string_lossy().to_string(),
            )
        });
    (*name).to_string()
}

#[cfg(unix)]
pub fn group_name(gid: u32) -> String {
    lazy_static! {
        static ref USERS_CACHE_MUTEX: Mutex<HashMap<u32, String>> = Mutex::new(HashMap::new());
    }
    let mut groups_cache = USERS_CACHE_MUTEX.lock().unwrap();
    let name = groups_cache
        .entry(gid)
        .or_insert_with(|| {
            users::get_group_by_gid(gid).map_or_else(
                || "????".to_string(),
                |u| u.name().to_string_lossy().to_string(),
            )
        });
    (*name).to_string()
}
