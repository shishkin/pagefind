use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use crate::options::PagefindServiceConfig;

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ServiceRequest {
    pub(super) message_id: u32,
    pub(super) payload: RequestAction,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub(super) enum RequestAction {
    NewIndex {
        config: Option<PagefindServiceConfig>,
    },
    AddFile {
        index_id: u32,
        file_path: String,
        file_contents: String,
    },
    AddRecord {
        index_id: u32,
        url: String,
        content: String,
        language: String,
        meta: Option<HashMap<String, String>>,
        filters: Option<HashMap<String, Vec<String>>>,
        sort: Option<HashMap<String, String>>,
    },
    AddDir {
        index_id: u32,
        path: String,
        glob: Option<String>,
    },
    BuildIndex {
        index_id: u32,
    },
    WriteFiles {
        index_id: u32,
        bundle_path: Option<String>,
    },
    GetFiles {
        index_id: u32,
    },
    DeleteIndex {
        index_id: u32,
    },
}
