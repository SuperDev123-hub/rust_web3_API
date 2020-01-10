use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub enum JsonRpcVersion {
    V2,
    Other(String),
}

impl Default for JsonRpcVersion {
    #[inline]
    fn default() -> Self {
        JsonRpcVersion::V2
    }
}

impl Serialize for JsonRpcVersion {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match self {
            JsonRpcVersion::V2 => "2.0",
            JsonRpcVersion::Other(version) => version,
        })
    }
}

impl<'de> Deserialize<'de> for JsonRpcVersion {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).and_then(|version| {
            Ok(match &*version {
                "2.0" => JsonRpcVersion::V2,
                _ => JsonRpcVersion::Other(version),
            })
        })
    }
}
