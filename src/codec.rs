//! RFC 8785 for this profile's I-JSON subset: safe integers, strings, bool, null, arrays, objects.
use crate::Error;
use serde::{
    Serialize,
    de::{self, DeserializeOwned, MapAccess, SeqAccess, Visitor},
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fmt;

pub(crate) fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub(crate) fn encoding(_: impl fmt::Display) -> Error {
    Error::InvalidInput("unsupported JSON encoding".into())
}

pub(crate) fn canonical(value: &Value) -> Result<Vec<u8>, Error> {
    fn emit(v: &Value, out: &mut Vec<u8>) -> Result<(), Error> {
        match v {
            Value::Null => out.extend(b"null"),
            Value::Bool(b) => out.extend(if *b {
                b"true".as_slice()
            } else {
                b"false".as_slice()
            }),
            Value::String(s) => out.extend(serde_json::to_vec(s).map_err(encoding)?),
            Value::Number(n) => {
                let valid = n.as_u64().is_some_and(|n| n <= 9_007_199_254_740_991)
                    || n.as_i64().is_some_and(|n| {
                        (-9_007_199_254_740_991..=9_007_199_254_740_991).contains(&n)
                    });
                if !valid {
                    return Err(encoding("non-safe-integer"));
                }
                out.extend(n.to_string().bytes());
            }
            Value::Array(a) => {
                out.push(b'[');
                for (i, v) in a.iter().enumerate() {
                    if i > 0 {
                        out.push(b',');
                    }
                    emit(v, out)?;
                }
                out.push(b']');
            }
            Value::Object(o) => {
                let mut entries: Vec<_> = o.iter().collect();
                entries.sort_by(|(a, _), (b, _)| a.encode_utf16().cmp(b.encode_utf16()));
                out.push(b'{');
                for (i, (k, v)) in entries.into_iter().enumerate() {
                    if i > 0 {
                        out.push(b',');
                    }
                    out.extend(serde_json::to_vec(k).map_err(encoding)?);
                    out.push(b':');
                    emit(v, out)?;
                }
                out.push(b'}');
            }
        }
        Ok(())
    }
    let mut bytes = Vec::new();
    emit(value, &mut bytes)?;
    Ok(bytes)
}

pub(crate) fn hash_value(tag: &str, record: &impl Serialize) -> Result<String, Error> {
    Ok(digest(&canonical(
        &serde_json::json!({"format":tag,"record":record}),
    )?))
}

pub(crate) fn omit_content_bytes(value: &mut Value) {
    match value {
        Value::Object(o) => {
            if o.contains_key("reference") && o.contains_key("bytes") {
                o.remove("bytes");
            }
            for v in o.values_mut() {
                omit_content_bytes(v);
            }
        }
        Value::Array(a) => {
            for v in a {
                omit_content_bytes(v);
            }
        }
        _ => {}
    }
}

pub(crate) fn record_revision(record: &impl Serialize) -> Result<String, Error> {
    let mut value = serde_json::to_value(record).map_err(encoding)?;
    value["reference"]
        .as_object_mut()
        .ok_or_else(|| encoding("reference"))?
        .remove("revision");
    omit_content_bytes(&mut value);
    hash_value("knowledgehub-record/1", &value)
}

// Parsing through Value normally loses duplicate keys. This visitor rejects them at every depth.
struct Strict(Value);
impl<'de> de::Deserialize<'de> for Strict {
    fn deserialize<D: de::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Strict;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("unambiguous I-JSON")
            }
            fn visit_bool<E: de::Error>(self, v: bool) -> Result<Strict, E> {
                Ok(Strict(v.into()))
            }
            fn visit_unit<E: de::Error>(self) -> Result<Strict, E> {
                Ok(Strict(Value::Null))
            }
            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Strict, E> {
                if !(-9_007_199_254_740_991..=9_007_199_254_740_991).contains(&v) {
                    return Err(E::custom("unsafe integer"));
                }
                Ok(Strict(v.into()))
            }
            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Strict, E> {
                if v > 9_007_199_254_740_991 {
                    return Err(E::custom("unsafe integer"));
                }
                Ok(Strict(v.into()))
            }
            fn visit_f64<E: de::Error>(self, _: f64) -> Result<Strict, E> {
                Err(E::custom("floating-point outside seed profile"))
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<Strict, E> {
                Ok(Strict(v.into()))
            }
            fn visit_string<E: de::Error>(self, v: String) -> Result<Strict, E> {
                Ok(Strict(v.into()))
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Strict, A::Error> {
                let mut out = Vec::new();
                while let Some(Strict(v)) = a.next_element()? {
                    out.push(v);
                }
                Ok(Strict(Value::Array(out)))
            }
            fn visit_map<A: MapAccess<'de>>(self, mut a: A) -> Result<Strict, A::Error> {
                let mut out = serde_json::Map::new();
                while let Some(k) = a.next_key::<String>()? {
                    if out.contains_key(&k) {
                        return Err(de::Error::custom("duplicate key"));
                    }
                    let Strict(v) = a.next_value()?;
                    out.insert(k, v);
                }
                Ok(Strict(Value::Object(out)))
            }
        }
        d.deserialize_any(V)
    }
}

pub(crate) fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, Error> {
    let Strict(value) = serde_json::from_slice::<Strict>(bytes).map_err(encoding)?;
    serde_json::from_value(value).map_err(encoding)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn canonical_rfc8785_subset_vectors() {
        let v = serde_json::json!({"\u{e000}":"\u{000f}\n\"\\/", "😀":false,"a":9_007_199_254_740_991u64});
        assert_eq!(
            String::from_utf8(canonical(&v).unwrap()).unwrap(),
            "{\"a\":9007199254740991,\"😀\":false,\"\u{e000}\":\"\\u000f\\n\\\"\\\\/\"}"
        );
        assert!(decode::<Value>(br#"{"a":1,"a":2}"#).is_err());
        assert!(decode::<Value>(br#"{"nested":{"a":1,"a":2}}"#).is_err());
        for bad in ["1.0", "9007199254740992", "\"\\ud800\""] {
            assert!(decode::<Value>(bad.as_bytes()).is_err());
        }
    }
}
