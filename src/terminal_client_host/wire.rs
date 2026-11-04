use knowledgehub::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Value as Json, json};
use std::io::{Read, Write};
pub const PROTOCOL: &str = "knowledgehub.client/1";
pub const FRAME: usize = 8 * 1024 * 1024;
pub const RESULT: usize = 1024 * 1024;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub id: String,
    pub method: String,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub args: Json,
    #[serde(default)]
    pub result_bytes: Option<String>,
}
pub fn read(input: &mut impl Read) -> Result<Option<Request>, Error> {
    let mut n = [0; 4];
    match input.read(&mut n[..1]) {
        Ok(0) => return Ok(None),
        Ok(_) => (),
        Err(_) => return Err(super::invalid("frame header unavailable")),
    };
    input
        .read_exact(&mut n[1..])
        .map_err(|_| super::invalid("truncated frame header"))?;
    let len = u32::from_be_bytes(n) as usize;
    if len == 0 || len > FRAME {
        return Err(super::invalid("frame length exceeds 8 MiB"));
    }
    let mut b = vec![0; len];
    input
        .read_exact(&mut b)
        .map_err(|_| super::invalid("truncated frame body"))?;
    let r: Request = serde_json::from_slice(&b)
        .map_err(|_| super::invalid("malformed request or unknown envelope field"))?;
    if r.id.is_empty() || r.id.len() > 128 || r.id.chars().any(char::is_control) {
        return Err(super::invalid("invalid correlation ID"));
    }
    Ok(Some(r))
}
pub fn decimal(v: &str) -> Result<u64, Error> {
    if v.is_empty() || (v.len() > 1 && v.starts_with('0')) || !v.bytes().all(|c| c.is_ascii_digit())
    {
        return Err(super::invalid("canonical unsigned decimal string required"));
    }
    v.parse().map_err(|_| super::invalid("decimal overflow"))
}
pub fn owned<T: Serialize>(value: &T) -> Result<Json, Error> {
    let mut j = serde_json::to_value(value).map_err(|_| super::invalid("native value encoding"))?;
    numbers(&mut j);
    Ok(j)
}
pub fn numbers(j: &mut Json) {
    match j {
        Json::Number(n) => *j = Json::String(n.to_string()),
        Json::Array(a) => a.iter_mut().for_each(numbers),
        Json::Object(m) => m.values_mut().for_each(numbers),
        _ => (),
    }
}
/// Convert only schema-defined request counters. Authored Text/Integer/Decimal strings are untouched.
pub fn request_numbers(j: &mut Json) -> Result<(), Error> {
    if let Json::Object(m) = j {
        for (k, v) in m.iter_mut() {
            if matches!(
                k.as_str(),
                "page_size"
                    | "depth"
                    | "subject_limit"
                    | "edge_limit"
                    | "examined_limit"
                    | "byte_limit"
                    | "owner_limit"
                    | "fact_limit"
                    | "hops"
                    | "starts"
                    | "states"
                    | "examined"
                    | "candidates"
                    | "targets"
                    | "witnesses"
                    | "bytes"
                    | "items"
                    | "nodes"
                    | "storage_bytes"
                    | "output_bytes"
            ) {
                if k == "starts" && v.is_array() {
                    continue;
                }
                let s = v
                    .as_str()
                    .ok_or_else(|| super::invalid("native counters require decimal strings"))?;
                *v = json!(decimal(s)?);
            } else if matches!(k.as_str(), "limits" | "budgets" | "preparation") {
                request_numbers(v)?;
            }
        }
    }
    Ok(())
}
pub fn base64(b: &[u8]) -> String {
    const A: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(b.len().div_ceil(3) * 4);
    for x in b.chunks(3) {
        let n = ((x[0] as u32) << 16)
            | ((x.get(1).copied().unwrap_or(0) as u32) << 8)
            | x.get(2).copied().unwrap_or(0) as u32;
        out.push(A[(n >> 18) as usize] as char);
        out.push(A[((n >> 12) & 63) as usize] as char);
        out.push(if x.len() > 1 {
            A[((n >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if x.len() > 2 {
            A[(n & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}
pub fn error(e: &Error) -> Json {
    match e {
        Error::Denied { action } => json!({"code":"Denied","action":format!("{action:?}")}),
        Error::Unavailable(s) => json!({"code":"Unavailable","message":s}),
        Error::Capacity => json!({"code":"Capacity"}),
        Error::Busy => json!({"code":"Busy"}),
        Error::Incompatible(s) => json!({"code":"Incompatible","message":s}),
        Error::InvalidInput(s) => json!({"code":"InvalidInput","message":s}),
        Error::Invalid(v) => json!({"code":"Invalid","violations":v}),
        Error::Conflict { expected, actual } => {
            json!({"code":"Conflict","expected":expected,"actual":actual})
        }
        Error::GuardConflict { id } => json!({"code":"GuardConflict","id":id}),
        Error::OperationReuse => json!({"code":"OperationReuse"}),
        Error::NotFound => json!({"code":"NotFound"}),
        Error::InvalidCursor => json!({"code":"InvalidCursor"}),
        Error::AlreadyExists => json!({"code":"AlreadyExists"}),
        Error::HistoryAnchorUnavailable { anchor } => {
            json!({"code":"HistoryAnchorUnavailable","anchor":anchor})
        }
        Error::HistoryLimit {
            resource,
            limit,
            work,
        } => {
            json!({"code":"HistoryLimit","resource":resource,"limit":limit.to_string(),"work":owned(work).expect("owned work")})
        }
        Error::Integrity(s) => json!({"code":"Integrity","message":s}),
        Error::Storage { stage } => json!({"code":"Storage","stage":stage}),
        Error::Uncertain {
            operation_id,
            stage,
        } => json!({"code":"Uncertain","operation_id":operation_id,"stage":stage}),
        Error::IndexNotReady { reason } => json!({"code":"IndexNotReady","reason":reason}),
    }
}
pub fn limit(r: &Request) -> Result<usize, Error> {
    let cap = match &r.result_bytes {
        None => RESULT,
        Some(s) => usize::try_from(decimal(s)?).map_err(|_| Error::Capacity)?,
    };
    if !(256..=FRAME - 1024).contains(&cap) {
        return Err(super::invalid("result_bytes outside 256..8387584"));
    }
    Ok(cap)
}
pub fn fits(r: &Request, data: &Json) -> Result<bool, Error> {
    let response = json!({"protocol":PROTOCOL,"id":r.id,"ok":true,"data":data});
    Ok(serde_json::to_vec(&response)
        .map_err(|_| super::invalid("response encoding"))?
        .len()
        <= limit(r)?)
}
pub fn write(
    output: &mut impl Write,
    r: &Request,
    result: Result<Json, Error>,
) -> Result<(), Error> {
    let (cap, result) = match limit(r) {
        Ok(cap) => (cap, result),
        Err(e) => (RESULT, Err(e)),
    };
    let mut response = match result {
        Ok(data) => json!({"protocol":PROTOCOL,"id":r.id,"ok":true,"data":data}),
        Err(e) => json!({"protocol":PROTOCOL,"id":r.id,"ok":false,"error":error(&e)}),
    };
    let mut bytes =
        serde_json::to_vec(&response).map_err(|_| super::invalid("response encoding"))?;
    // Native failures are bounded control responses. A small requested data cap
    // must not turn Denied/HistoryLimit into an unrelated Capacity failure.
    if bytes.len() > cap && response["ok"] == true {
        let work = if matches!(
            r.method.as_str(),
            "publication" | "publications" | "publication_affected" | "continue_history"
        ) {
            response.get("data").and_then(|d| d.get("work")).cloned()
        } else {
            None
        };
        response = json!({"protocol":PROTOCOL,"id":r.id,"ok":false,"error":{"code":"Capacity","message":"indivisible wire result exceeds requested limit","limit":cap.to_string(),"encoded_bytes":bytes.len().to_string(),"reference":r.args.get("reference"),"work":work}});
        bytes = serde_json::to_vec(&response).map_err(|_| Error::Capacity)?;
    }
    if bytes.len() > FRAME {
        return Err(Error::Capacity);
    }
    output
        .write_all(&(bytes.len() as u32).to_be_bytes())
        .and_then(|()| output.write_all(&bytes))
        .and_then(|()| output.flush())
        .map_err(|_| Error::Unavailable("response pipe closed".into()))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frames_precision_and_exact_numbers() {
        let j=owned(&json!({"n":u64::MAX,"text":"000123","v":{"kind":"Integer","value":"9007199254740993123"}})).unwrap();
        assert_eq!(j["n"], u64::MAX.to_string());
        assert_eq!(j["text"], "000123");
        assert_eq!(base64(&[0, 255, 128]), "AP+A");
        let request = Request {
            id: "limit".into(),
            method: "publications".into(),
            model: None,
            args: json!({}),
            result_bytes: Some("256".into()),
        };
        let work = json!({"visited_nodes":"18446744073709551615","output_bytes":"500"});
        let mut framed = vec![];
        write(
            &mut framed,
            &request,
            Ok(json!({"padding":"x".repeat(500),"work":work})),
        )
        .unwrap();
        let response: Json = serde_json::from_slice(&framed[4..]).unwrap();
        assert_eq!(response["error"]["work"], work);
        let denied = Request {
            id: "x".repeat(128),
            method: "publication".into(),
            model: None,
            args: json!({}),
            result_bytes: Some("256".into()),
        };
        let mut framed = vec![];
        write(
            &mut framed,
            &denied,
            Err(Error::Denied {
                action: knowledgehub::Action::ReadHistory,
            }),
        )
        .unwrap();
        let response: Json = serde_json::from_slice(&framed[4..]).unwrap();
        assert_eq!(response["error"]["code"], "Denied");
        assert!(decimal("01").is_err());
        assert!(decimal("18446744073709551616").is_err());
        assert!(read(&mut &b"\0\0\0\x0a{}"[..]).is_err());
        assert!(read(&mut &b"\x01\0\0\0"[..]).is_err());
    }
}
