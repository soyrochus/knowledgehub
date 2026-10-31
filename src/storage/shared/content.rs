use super::*;
use super::{objects::Objects, trees::Tree};
use serde_json::json as value;

pub(super) const CHUNK: usize = 1_048_576;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Binding {
    pub content_oid: String,
    pub reference: ContentRef,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    schema: String,
    sha256: String,
    length: String,
    chunks_root: String,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Chunk {
    oid: String,
    length: String,
}

pub(super) fn put(objects: &Objects<'_>, content: &Content) -> Result<Binding, Error> {
    if ContentRef::of(&content.bytes) != content.reference {
        return Err(integrity("original content digest differs"));
    }
    let id = if content.bytes.len() <= CHUNK {
        objects.put("blob", &content.bytes)?
    } else {
        let chunks = content
            .bytes
            .chunks(CHUNK)
            .map(|bytes| {
                Ok(value!({"oid":objects.put("blob",bytes)?,"length":bytes.len().to_string()}))
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let root = Tree { objects }.sequence("chunks/1", &chunks)?;
        objects.put_json(
            "blob-manifest",
            &Manifest {
                schema: "knowledgehub-blob-manifest/1".into(),
                sha256: content.reference.sha256.clone(),
                length: content.reference.length.to_string(),
                chunks_root: root,
            },
        )?
    };
    Ok(Binding {
        content_oid: id,
        reference: content.reference.clone(),
    })
}

pub(super) fn range(
    objects: &Objects<'_>,
    binding: &Binding,
    start: u64,
    len: u64,
) -> Result<(Vec<u8>, bool), Error> {
    let total = binding.reference.length;
    if !model::hash(&binding.reference.sha256) || start > total || len > total - start {
        return Err(Error::InvalidInput("artifact range out of bounds".into()));
    }
    let object = objects.get(&binding.content_oid)?;
    if object.kind == "blob" {
        if total > CHUNK as u64 || ContentRef::of(&object.bytes) != binding.reference {
            return Err(integrity("blob original content binding"));
        }
        return Ok((
            object.bytes[start as usize..(start + len) as usize].to_vec(),
            true,
        ));
    }
    if object.kind != "blob-manifest" {
        return Err(integrity("expected original byte content"));
    }
    let manifest: Manifest = parse(&object.bytes)?;
    if manifest.schema != "knowledgehub-blob-manifest/1"
        || number(&manifest.length)? != total
        || manifest.sha256 != binding.reference.sha256
        || total <= CHUNK as u64
    {
        return Err(integrity("invalid content manifest"));
    }
    let tree = Tree { objects };
    let count = total.div_ceil(CHUNK as u64);
    if tree.seq_count(&manifest.chunks_root, "chunks/1")? != count {
        return Err(integrity("chunk count differs"));
    }
    if len == 0 {
        return Ok((vec![], false));
    }
    let first = start / CHUNK as u64;
    let last = (start + len - 1) / CHUNK as u64;
    let entries = tree.seq_range(&manifest.chunks_root, "chunks/1", first, last - first + 1)?;
    let capacity = usize::try_from(len).map_err(|_| Error::Capacity)?;
    // A public owned range is bounded; callers can stream larger artifacts by ranges.
    if capacity > objects::MAX_OBJECT {
        return Err(Error::Capacity);
    }
    let mut result = Vec::with_capacity(capacity);
    for (i, v) in entries.into_iter().enumerate() {
        let c: Chunk = serde_json::from_value(v).map_err(|_| integrity("chunk descriptor"))?;
        let index = first + i as u64;
        let offset = index * CHUNK as u64;
        let expected = (total - offset).min(CHUNK as u64);
        if number(&c.length)? != expected {
            return Err(integrity("fixed chunk length"));
        }
        let blob = objects.get(&c.oid)?;
        if blob.kind != "blob" || blob.bytes.len() as u64 != expected {
            return Err(integrity("chunk kind/length"));
        }
        let left = start.saturating_sub(offset) as usize;
        let right = ((start + len - offset).min(expected)) as usize;
        result.extend_from_slice(&blob.bytes[left..right]);
    }
    let complete = start == 0 && len == total;
    if result.len() != capacity || (complete && ContentRef::of(&result) != binding.reference) {
        return Err(integrity("original content reconstruction differs"));
    }
    Ok((result, complete))
}

pub(super) fn read(objects: &Objects<'_>, binding: &Binding) -> Result<Content, Error> {
    // Preserve full-content public contracts while bounding each decoded chunk and allocation.
    if binding.reference.length <= objects::MAX_OBJECT as u64 {
        return Ok(Content {
            reference: binding.reference.clone(),
            bytes: range(objects, binding, 0, binding.reference.length)?.0,
        });
    }
    let n = usize::try_from(binding.reference.length).map_err(|_| Error::Capacity)?;
    if n > 256 * 1024 * 1024 {
        return Err(Error::Capacity);
    }
    let mut bytes = Vec::with_capacity(n);
    let mut at = 0;
    while at < binding.reference.length {
        let len = (binding.reference.length - at).min(objects::MAX_OBJECT as u64);
        bytes.extend(range(objects, binding, at, len)?.0);
        at += len;
    }
    if ContentRef::of(&bytes) != binding.reference {
        return Err(integrity("complete original digest"));
    }
    Ok(Content {
        reference: binding.reference.clone(),
        bytes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chunks_ranges_and_append_reuse_original_bytes() {
        let (_s, b) = super::super::object_backend("shared-content");
        let o = Objects::new(&b.store);
        let original = Content::capture((0..CHUNK * 2 + 17).map(|i| (i % 251) as u8).collect());
        let binding = put(&o, &original).unwrap();
        assert_eq!(read(&Objects::new(&b.store), &binding).unwrap(), original);
        let (bytes, full) = range(&o, &binding, (CHUNK - 7) as u64, 31).unwrap();
        assert_eq!(bytes, original.bytes[CHUNK - 7..CHUNK + 24]);
        assert!(!full);
        let a: Manifest = o.get_json(&binding.content_oid, "blob-manifest").unwrap();
        let mut next = original.bytes.clone();
        next.extend(b"more");
        let next = put(&o, &Content::capture(next)).unwrap();
        let next: Manifest = o.get_json(&next.content_oid, "blob-manifest").unwrap();
        let tree = Tree { objects: &o };
        assert_eq!(
            tree.seq_range(&a.chunks_root, "chunks/1", 0, 2).unwrap(),
            tree.seq_range(&next.chunks_root, "chunks/1", 0, 2).unwrap()
        );
        let exact = Content::capture(vec![0; CHUNK * 2]);
        let exact = put(&o, &exact).unwrap();
        assert_eq!(read(&o, &exact).unwrap().bytes.len(), CHUNK * 2);
        assert!(range(&o, &binding, original.bytes.len() as u64, 1).is_err());
    }
}
