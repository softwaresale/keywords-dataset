use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct GcsObject {
    pub(crate) bucket: String,
    pub(crate) content_type: String,
    pub(crate) id: String,
    pub(crate) md5_hash: String,
    pub(crate) media_link: String,
    pub(crate) name: String,
    pub(crate) self_link: String,
    size: String,
}

impl GcsObject {
    pub fn size(&self) -> usize {
        self.size.parse().expect("size should always be usize-parsable")
    }
}

#[derive(Deserialize)]
pub(crate) struct GcsListObjectResponse {
    pub(crate) items: Vec<GcsObject>,
    pub(crate) kind: String,
}

impl GcsListObjectResponse {
    pub fn take_most_recent(self) -> Option<GcsObject> {
        self.items.into_iter().last()
    }
}
