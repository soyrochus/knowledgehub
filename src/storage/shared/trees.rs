//! Persistent typed maps and sequences.
use super::objects::Objects;
use super::*;
use serde_json::Value;

const MAX_ITEMS: usize = 1_000_000;
const MAP_SCHEMA: &str = "knowledgehub-map-node/1";
const SEQ_SCHEMA: &str = "knowledgehub-sequence-node/1";
fn spaces() -> [&'static str; 10] {
    [
        "selection/1",
        "revisions/1",
        "exact-reference-set/1",
        "artifacts/1",
        "operations/1",
        "ancestors/1",
        "neighbour-endpoints/1",
        "neighbour-selectors/1",
        "text-grams/1",
        "text-fields/1",
    ]
}
fn item_types() -> [&'static str; 6] {
    [
        "exact-refs/1",
        "legacy-refs/1",
        "gap-oids/1",
        "chunks/1",
        "neighbour-postings/1",
        "text-postings/1",
    ]
}
fn item_valid(t: &str, v: &Value) -> bool {
    match t {
        "text-postings/1" => {
            serde_json::from_value::<crate::ontology::impact::text::FieldRef>(v.clone())
                .is_ok_and(|f| f.valid())
        }
        "neighbour-postings/1" => {
            serde_json::from_value::<crate::ontology::neighbours::Posting>(v.clone())
                .is_ok_and(|p| p.valid())
        }
        "exact-refs/1" => serde_json::from_value::<crate::ontology::ExactRef>(v.clone())
            .is_ok_and(|r| r.is_valid()),
        "legacy-refs/1" => serde_json::from_value::<RevisionRef>(v.clone())
            .is_ok_and(|r| !r.id.trim().is_empty() && !r.revision.trim().is_empty()),
        "gap-oids/1" => v.as_str().is_some_and(model::hash),
        "chunks/1" => v.as_object().is_some_and(|m| {
            m.len() == 2
                && m.get("oid")
                    .and_then(Value::as_str)
                    .is_some_and(model::hash)
                && m.get("length")
                    .and_then(Value::as_str)
                    .is_some_and(|s| number(s).is_ok_and(|n| n > 0 && n <= 1_048_576))
        }),
        _ => false,
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    key: Value,
    value: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "variant", rename_all = "lowercase", deny_unknown_fields)]
enum MapNode {
    Leaf {
        schema: String,
        keyspace: String,
        prefix: String,
        count: String,
        entries: Vec<Entry>,
    },
    Branch {
        schema: String,
        keyspace: String,
        prefix: String,
        count: String,
        children: BTreeMap<String, String>,
    },
}
impl MapNode {
    fn count(&self) -> Result<u64, Error> {
        number(match self {
            Self::Leaf { count, .. } | Self::Branch { count, .. } => count,
        })
    }
}

pub(super) struct Tree<'a, 'b> {
    pub objects: &'a Objects<'b>,
}

fn key_valid(space: &str, key: &Value) -> bool {
    match space {
        "text-fields/1" => {
            serde_json::from_value::<crate::ontology::impact::text::FieldRef>(key.clone())
                .is_ok_and(|f| f.valid())
        }
        "text-grams/1" => key.as_str().is_some_and(|s| {
            s.len() == 6
                && s.bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        }),
        "selection/1"
        | "revisions/1"
        | "exact-reference-set/1"
        | "artifacts/1"
        | "neighbour-endpoints/1" => key.as_array().is_some_and(|a| {
            a.len() == (if space == "selection/1" { 2 } else { 3 })
                && a.iter().all(|v| {
                    v.as_str()
                        .is_some_and(|s| !s.trim().is_empty() && !s.contains('\0'))
                })
        }),
        "operations/1" => key.as_str().is_some_and(|s| !s.trim().is_empty()),
        "ancestors/1" => key.as_str().is_some_and(model::hash),
        "neighbour-selectors/1" => {
            serde_json::from_value::<crate::ontology::NeighbourSelector>(key.clone())
                .is_ok_and(|s| s.valid())
        }
        _ => false,
    }
}
fn route(key: &Value) -> Result<String, Error> {
    Ok(codec::digest(&json(key)?))
}
pub(super) fn ref_key(r: &crate::ontology::ExactRef) -> Value {
    serde_json::json!([r.model, r.id, r.revision])
}
pub(super) fn identity_key(r: &crate::ontology::ExactRef) -> Value {
    serde_json::json!([r.model, r.id])
}
pub(super) fn exact_key(key: &Value) -> Result<crate::ontology::ExactRef, Error> {
    let a = key
        .as_array()
        .filter(|a| a.len() == 3)
        .ok_or_else(|| integrity("exact key"))?;
    let strings = a
        .iter()
        .map(|v| v.as_str().ok_or_else(|| integrity("exact key component")))
        .collect::<Result<Vec<_>, _>>()?;
    let r = crate::ontology::ExactRef::new(strings[0], strings[1], strings[2]);
    if !r.is_valid() {
        return Err(integrity("invalid exact key"));
    }
    Ok(r)
}

impl Tree<'_, '_> {
    /// Batch path copying emits only final changed trie nodes, rather than intermediate roots.
    pub fn update_many(
        &self,
        root: &str,
        space: &str,
        changes: Vec<(Value, Option<Value>)>,
    ) -> Result<String, Error> {
        let mut unique = BTreeMap::new();
        for (key, value) in changes {
            if !key_valid(space, &key) {
                return Err(integrity("invalid batch map key"));
            }
            unique.insert(json(&key)?, (key, value));
        }
        self.batch_at(root, space, "", unique.into_values().collect())
    }
    fn batch_at(
        &self,
        id: &str,
        space: &str,
        prefix: &str,
        changes: Vec<(Value, Option<Value>)>,
    ) -> Result<String, Error> {
        if changes.is_empty() {
            return Ok(id.into());
        }
        match self.map_node(id, space, prefix)? {
            MapNode::Leaf { entries, .. } => {
                let mut entries = entries
                    .into_iter()
                    .map(|e| Ok((json(&e.key)?, e)))
                    .collect::<Result<BTreeMap<_, _>, Error>>()?;
                for (key, value) in changes {
                    let k = json(&key)?;
                    if let Some(value) = value {
                        entries.insert(k, Entry { key, value });
                    } else {
                        entries.remove(&k);
                    }
                }
                if entries.len() > MAX_ITEMS {
                    return Err(Error::Capacity);
                }
                self.build(space, prefix, entries.into_values().collect())
            }
            MapNode::Branch {
                mut children,
                count,
                ..
            } => {
                let mut groups: BTreeMap<String, Vec<(Value, Option<Value>)>> = BTreeMap::new();
                for (key, value) in changes {
                    groups
                        .entry(route(&key)?[prefix.len()..prefix.len() + 1].into())
                        .or_default()
                        .push((key, value));
                }
                let mut count = number(&count)?;
                for (digit, changes) in groups {
                    let p = format!("{prefix}{digit}");
                    let old = children.get(&digit).cloned();
                    let old_count = old
                        .as_ref()
                        .map(|id| self.map_node(id, space, &p)?.count())
                        .transpose()?
                        .unwrap_or(0);
                    let child = match old {
                        Some(old) => self.batch_at(&old, space, &p, changes)?,
                        None => self.build(
                            space,
                            &p,
                            changes
                                .into_iter()
                                .filter_map(|(key, value)| value.map(|value| Entry { key, value }))
                                .collect(),
                        )?,
                    };
                    let new_count = self.map_node(&child, space, &p)?.count()?;
                    count = count
                        .checked_sub(old_count)
                        .and_then(|n| n.checked_add(new_count))
                        .ok_or_else(|| integrity("batch map count"))?;
                    if new_count == 0 {
                        children.remove(&digit);
                    } else {
                        children.insert(digit, child);
                    }
                }
                if count > MAX_ITEMS as u64 {
                    return Err(Error::Capacity);
                }
                if count <= 64 {
                    let mut entries = Vec::new();
                    for (digit, id) in children {
                        entries.extend(self.collect(&id, space, &format!("{prefix}{digit}"))?);
                    }
                    entries.sort_by_cached_key(|e| json(&e.key).expect("validated key"));
                    self.build(space, prefix, entries)
                } else {
                    self.objects.put_json(
                        "map-node",
                        &MapNode::Branch {
                            schema: MAP_SCHEMA.into(),
                            keyspace: space.into(),
                            prefix: prefix.into(),
                            count: count.to_string(),
                            children,
                        },
                    )
                }
            }
        }
    }
    pub fn map_count(&self, root: &str, space: &str) -> Result<u64, Error> {
        self.map_node(root, space, "")?.count()
    }
    fn map_node(&self, id: &str, space: &str, prefix: &str) -> Result<MapNode, Error> {
        let n: MapNode = self.objects.get_json(id, "map-node")?;
        let (schema, keyspace, p) = match &n {
            MapNode::Leaf {
                schema,
                keyspace,
                prefix,
                ..
            }
            | MapNode::Branch {
                schema,
                keyspace,
                prefix,
                ..
            } => (schema, keyspace, prefix),
        };
        if schema != MAP_SCHEMA
            || keyspace != space
            || !spaces().contains(&space)
            || p != prefix
            || prefix.len() > 64
            || n.count()? > MAX_ITEMS as u64
        {
            return Err(integrity("map schema/prefix/count"));
        }
        match &n {
            MapNode::Leaf { entries, .. } => {
                if entries.len() as u64 != n.count()? || (entries.len() > 64 && prefix.len() != 64)
                {
                    return Err(integrity("leaf count/size"));
                }
                let mut previous: Option<Vec<u8>> = None;
                for e in entries {
                    let key = json(&e.key)?;
                    if !key_valid(space, &e.key)
                        || !route(&e.key)?.starts_with(prefix)
                        || previous.as_ref().is_some_and(|p| p >= &key)
                    {
                        return Err(integrity("unordered, duplicate or misplaced map key"));
                    }
                    previous = Some(key);
                }
            }
            MapNode::Branch { children, .. } => {
                if prefix.len() == 64
                    || n.count()? <= 64
                    || children.is_empty()
                    || children.len() > 16
                    || children.iter().any(|(k, v)| {
                        k.len() != 1
                            || !k
                                .bytes()
                                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
                            || !model::hash(v)
                    })
                {
                    return Err(integrity("invalid map branch"));
                }
            }
        }
        Ok(n)
    }
    pub fn map(
        &self,
        space: &str,
        values: impl IntoIterator<Item = (Value, Value)>,
    ) -> Result<String, Error> {
        if !spaces().contains(&space) {
            return Err(integrity("unknown keyspace"));
        }
        let mut entries = BTreeMap::new();
        for (key, value) in values {
            if !key_valid(space, &key)
                || entries.insert(json(&key)?, Entry { key, value }).is_some()
            {
                return Err(integrity("invalid/duplicate map input"));
            }
            if entries.len() > MAX_ITEMS {
                return Err(Error::Capacity);
            }
        }
        self.build(space, "", entries.into_values().collect())
    }
    fn build(&self, space: &str, prefix: &str, entries: Vec<Entry>) -> Result<String, Error> {
        let count = entries.len().to_string();
        let node = if entries.len() <= 64 || prefix.len() == 64 {
            MapNode::Leaf {
                schema: MAP_SCHEMA.into(),
                keyspace: space.into(),
                prefix: prefix.into(),
                count,
                entries,
            }
        } else {
            let mut groups: BTreeMap<String, Vec<Entry>> = BTreeMap::new();
            for e in entries {
                groups
                    .entry(route(&e.key)?[prefix.len()..prefix.len() + 1].into())
                    .or_default()
                    .push(e);
            }
            let children = groups
                .into_iter()
                .map(|(digit, items)| {
                    Ok((
                        digit.clone(),
                        self.build(space, &format!("{prefix}{digit}"), items)?,
                    ))
                })
                .collect::<Result<_, Error>>()?;
            MapNode::Branch {
                schema: MAP_SCHEMA.into(),
                keyspace: space.into(),
                prefix: prefix.into(),
                count,
                children,
            }
        };
        self.objects.put_json("map-node", &node)
    }
    pub fn get(&self, root: &str, space: &str, key: &Value) -> Result<Option<Value>, Error> {
        if !key_valid(space, key) {
            return Err(integrity("invalid lookup key"));
        }
        let hash = route(key)?;
        let mut id = root.to_string();
        let mut prefix = String::new();
        loop {
            match self.map_node(&id, space, &prefix)? {
                MapNode::Leaf { entries, .. } => {
                    return Ok(entries.into_iter().find(|e| e.key == *key).map(|e| e.value));
                }
                MapNode::Branch { children, .. } => {
                    let digit = &hash[prefix.len()..prefix.len() + 1];
                    let Some(child) = children.get(digit) else {
                        return Ok(None);
                    };
                    id = child.clone();
                    prefix.push_str(digit);
                }
            }
        }
    }
    pub fn entries(&self, root: &str, space: &str) -> Result<Vec<(Value, Value)>, Error> {
        let mut entries = self.collect(root, space, "")?;
        entries.sort_by_cached_key(|e| json(&e.key).expect("validated key"));
        Ok(entries.into_iter().map(|e| (e.key, e.value)).collect())
    }
    fn collect(&self, id: &str, space: &str, prefix: &str) -> Result<Vec<Entry>, Error> {
        match self.map_node(id, space, prefix)? {
            MapNode::Leaf { entries, .. } => Ok(entries),
            MapNode::Branch {
                children, count, ..
            } => {
                let mut entries = Vec::new();
                for (digit, id) in children {
                    entries.extend(self.collect(&id, space, &format!("{prefix}{digit}"))?);
                    if entries.len() > MAX_ITEMS {
                        return Err(Error::Capacity);
                    }
                }
                if entries.len() as u64 != number(&count)? {
                    return Err(integrity("map subtree count differs"));
                }
                Ok(entries)
            }
        }
    }
    pub fn update(
        &self,
        root: &str,
        space: &str,
        key: Value,
        value: Option<Value>,
    ) -> Result<String, Error> {
        if !key_valid(space, &key) {
            return Err(integrity("invalid update key"));
        }
        self.update_at(root, space, "", key, value)
    }
    fn update_at(
        &self,
        id: &str,
        space: &str,
        prefix: &str,
        key: Value,
        value: Option<Value>,
    ) -> Result<String, Error> {
        match self.map_node(id, space, prefix)? {
            MapNode::Leaf { mut entries, .. } => {
                entries.retain(|e| e.key != key);
                if let Some(value) = value {
                    entries.push(Entry { key, value });
                }
                entries.sort_by_cached_key(|e| json(&e.key).expect("valid key"));
                self.build(space, prefix, entries)
            }
            MapNode::Branch {
                mut children,
                count,
                ..
            } => {
                let digit = route(&key)?[prefix.len()..prefix.len() + 1].to_string();
                let p = format!("{prefix}{digit}");
                let old = children.get(&digit).cloned();
                let old_count = old
                    .as_ref()
                    .map(|id| self.map_node(id, space, &p)?.count())
                    .transpose()?
                    .unwrap_or(0);
                let child = match old {
                    Some(id) => self.update_at(&id, space, &p, key, value)?,
                    None => self.build(
                        space,
                        &p,
                        value
                            .into_iter()
                            .map(|v| Entry {
                                key: key.clone(),
                                value: v,
                            })
                            .collect(),
                    )?,
                };
                let new_count = self.map_node(&child, space, &p)?.count()?;
                if new_count == 0 {
                    children.remove(&digit);
                } else {
                    children.insert(digit, child);
                }
                let count = number(&count)?
                    .checked_sub(old_count)
                    .and_then(|n| n.checked_add(new_count))
                    .ok_or_else(|| integrity("map count overflow"))?;
                if count <= 64 {
                    let mut entries = Vec::new();
                    for (digit, id) in children {
                        entries.extend(self.collect(&id, space, &format!("{prefix}{digit}"))?);
                    }
                    entries.sort_by_cached_key(|e| json(&e.key).expect("valid key"));
                    self.build(space, prefix, entries)
                } else {
                    self.objects.put_json(
                        "map-node",
                        &MapNode::Branch {
                            schema: MAP_SCHEMA.into(),
                            keyspace: space.into(),
                            prefix: prefix.into(),
                            count: count.to_string(),
                            children,
                        },
                    )
                }
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Child {
    oid: String,
    count: String,
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "variant", rename_all = "lowercase", deny_unknown_fields)]
enum Sequence {
    Leaf {
        schema: String,
        item_type: String,
        count: String,
        items: Vec<Value>,
    },
    Branch {
        schema: String,
        item_type: String,
        count: String,
        children: Vec<Child>,
    },
}
impl Sequence {
    fn count(&self) -> Result<u64, Error> {
        number(match self {
            Self::Leaf { count, .. } | Self::Branch { count, .. } => count,
        })
    }
}

impl Tree<'_, '_> {
    fn sequence_node(&self, id: &str, item_type: &str) -> Result<Sequence, Error> {
        let n: Sequence = self.objects.get_json(id, "sequence-node")?;
        let (s, t) = match &n {
            Sequence::Leaf {
                schema, item_type, ..
            }
            | Sequence::Branch {
                schema, item_type, ..
            } => (schema, item_type),
        };
        if s != SEQ_SCHEMA
            || t != item_type
            || !item_types().contains(&item_type)
            || n.count()? > MAX_ITEMS as u64
        {
            return Err(integrity("sequence schema/count"));
        }
        match &n {
            Sequence::Leaf { items, .. } => {
                if items.len() > 128
                    || items.len() as u64 != n.count()?
                    || items.iter().any(|v| !item_valid(item_type, v))
                {
                    return Err(integrity("sequence leaf count/items"));
                }
            }
            Sequence::Branch { children, .. } => {
                if children.is_empty()
                    || children.len() > 128
                    || children.iter().any(|c| !model::hash(&c.oid))
                {
                    return Err(integrity("sequence children"));
                }
                let total = children.iter().try_fold(0u64, |a, c| {
                    a.checked_add(number(&c.count)?)
                        .ok_or_else(|| integrity("sequence overflow"))
                })?;
                if total != n.count()? {
                    return Err(integrity("sequence subtree count"));
                }
            }
        }
        Ok(n)
    }
    fn seq_node(&self, t: &str, items: Vec<Value>) -> Result<Child, Error> {
        if !item_types().contains(&t) || items.iter().any(|v| !item_valid(t, v)) {
            return Err(integrity("unsupported sequence items"));
        }
        let count = items.len().to_string();
        let oid = self.objects.put_json(
            "sequence-node",
            &Sequence::Leaf {
                schema: SEQ_SCHEMA.into(),
                item_type: t.into(),
                count: count.clone(),
                items,
            },
        )?;
        Ok(Child { oid, count })
    }
    fn seq_branch(&self, t: &str, children: Vec<Child>) -> Result<Child, Error> {
        let count = children
            .iter()
            .try_fold(0u64, |n, c| {
                n.checked_add(number(&c.count)?)
                    .ok_or_else(|| integrity("sequence overflow"))
            })?
            .to_string();
        let oid = self.objects.put_json(
            "sequence-node",
            &Sequence::Branch {
                schema: SEQ_SCHEMA.into(),
                item_type: t.into(),
                count: count.clone(),
                children,
            },
        )?;
        Ok(Child { oid, count })
    }
    pub fn sequence(&self, t: &str, items: &[Value]) -> Result<String, Error> {
        if items.len() > MAX_ITEMS {
            return Err(Error::Capacity);
        }
        let mut children = items
            .chunks(128)
            .map(|items| self.seq_node(t, items.to_vec()))
            .collect::<Result<Vec<_>, _>>()?;
        if children.is_empty() {
            return Ok(self.seq_node(t, vec![])?.oid);
        }
        while children.len() > 1 {
            children = children
                .chunks(128)
                .map(|c| self.seq_branch(t, c.to_vec()))
                .collect::<Result<Vec<_>, _>>()?;
        }
        Ok(children.pop().unwrap().oid)
    }
    pub fn seq_range(&self, id: &str, t: &str, start: u64, len: u64) -> Result<Vec<Value>, Error> {
        self.range_at(id, t, start, len, 0)
    }
    fn range_at(
        &self,
        id: &str,
        t: &str,
        start: u64,
        len: u64,
        depth: usize,
    ) -> Result<Vec<Value>, Error> {
        if depth > 64 || len > MAX_ITEMS as u64 {
            return Err(Error::Capacity);
        }
        let n = self.sequence_node(id, t)?;
        let count = n.count()?;
        if start > count || len > count - start {
            return Err(integrity("sequence range"));
        }
        match n {
            Sequence::Leaf { items, .. } => {
                Ok(items[start as usize..(start + len) as usize].to_vec())
            }
            Sequence::Branch { children, .. } => {
                let mut result = Vec::new();
                let mut offset = 0;
                for c in children {
                    let count = number(&c.count)?;
                    let left = start.max(offset);
                    let right = (start + len).min(offset + count);
                    if left < right {
                        if self.sequence_node(&c.oid, t)?.count()? != count {
                            return Err(integrity("child sequence count"));
                        }
                        result.extend(self.range_at(
                            &c.oid,
                            t,
                            left - offset,
                            right - left,
                            depth + 1,
                        )?);
                    }
                    offset += count;
                }
                Ok(result)
            }
        }
    }
    pub fn items(&self, id: &str, t: &str) -> Result<Vec<Value>, Error> {
        self.seq_range(id, t, 0, self.sequence_node(id, t)?.count()?)
    }
    pub fn seq_count(&self, id: &str, t: &str) -> Result<u64, Error> {
        self.sequence_node(id, t)?.count()
    }
    pub fn reconcile(&self, old: Option<&str>, t: &str, items: &[Value]) -> Result<String, Error> {
        let Some(root) = old else {
            return self.sequence(t, items);
        };
        let previous = self.items(root, t)?;
        if previous == items {
            return Ok(root.into());
        }
        let prefix = previous
            .iter()
            .zip(items)
            .take_while(|(a, b)| a == b)
            .count();
        if prefix == previous.len() {
            if items.len() > MAX_ITEMS {
                return Err(Error::Capacity);
            }
            let mut nodes = self.append_at(root, t, &items[prefix..], 0)?;
            while nodes.len() > 1 {
                nodes = nodes
                    .chunks(128)
                    .map(|c| self.seq_branch(t, c.to_vec()))
                    .collect::<Result<_, _>>()?;
            }
            return Ok(nodes
                .pop()
                .ok_or_else(|| integrity("append sequence root"))?
                .oid);
        }
        let suffix = previous[prefix..]
            .iter()
            .rev()
            .zip(items[prefix..].iter().rev())
            .take_while(|(a, b)| a == b)
            .count();
        let mut root = root.to_string();
        for _ in prefix..previous.len() - suffix {
            root = self.edit(&root, t, prefix as u64, None, true)?;
        }
        for (i, v) in items[prefix..items.len() - suffix].iter().enumerate() {
            root = self.edit(&root, t, (prefix + i) as u64, Some(v.clone()), false)?;
        }
        Ok(root)
    }
    fn append_at(
        &self,
        id: &str,
        t: &str,
        added: &[Value],
        depth: usize,
    ) -> Result<Vec<Child>, Error> {
        if depth > 64 {
            return Err(Error::Capacity);
        }
        let node = self.sequence_node(id, t)?;
        if added.is_empty() {
            return Ok(vec![Child {
                oid: id.into(),
                count: node.count()?.to_string(),
            }]);
        }
        match node {
            Sequence::Leaf { mut items, .. } => {
                items.extend_from_slice(added);
                items
                    .chunks(128)
                    .map(|items| self.seq_node(t, items.to_vec()))
                    .collect()
            }
            Sequence::Branch { mut children, .. } => {
                let last = children.pop().ok_or_else(|| integrity("append child"))?;
                children.extend(self.append_at(&last.oid, t, added, depth + 1)?);
                children
                    .chunks(128)
                    .map(|c| self.seq_branch(t, c.to_vec()))
                    .collect()
            }
        }
    }
    fn edit(
        &self,
        id: &str,
        t: &str,
        index: u64,
        value: Option<Value>,
        remove: bool,
    ) -> Result<String, Error> {
        let mut nodes = self.edit_at(id, t, index, value, remove, 0)?;
        if nodes.is_empty() {
            return Ok(self.seq_node(t, vec![])?.oid);
        }
        while nodes.len() > 1 {
            nodes = nodes
                .chunks(128)
                .map(|c| self.seq_branch(t, c.to_vec()))
                .collect::<Result<_, _>>()?;
        }
        let mut root = nodes.pop().unwrap().oid;
        while let Sequence::Branch { children, .. } = self.sequence_node(&root, t)? {
            if children.len() != 1 {
                break;
            }
            root = children[0].oid.clone();
        }
        Ok(root)
    }
    fn edit_at(
        &self,
        id: &str,
        t: &str,
        index: u64,
        value: Option<Value>,
        remove: bool,
        depth: usize,
    ) -> Result<Vec<Child>, Error> {
        if depth > 64 {
            return Err(Error::Capacity);
        }
        let n = self.sequence_node(id, t)?;
        let total = n.count()?;
        if index > total || (remove && index == total) {
            return Err(integrity("sequence edit range"));
        }
        match n {
            Sequence::Leaf { mut items, .. } => {
                if remove {
                    items.remove(index as usize);
                }
                if let Some(v) = value {
                    items.insert(index as usize, v);
                }
                items
                    .chunks(128)
                    .map(|v| self.seq_node(t, v.to_vec()))
                    .collect()
            }
            Sequence::Branch { mut children, .. } => {
                let mut offset = 0;
                let mut chosen = None;
                for (i, c) in children.iter().enumerate() {
                    let n = number(&c.count)?;
                    if index < offset + n || i == children.len() - 1 {
                        chosen = Some((i, index - offset));
                        break;
                    }
                    offset += n;
                }
                let (i, local) = chosen.ok_or_else(|| integrity("sequence edit child"))?;
                let changed = self.edit_at(&children[i].oid, t, local, value, remove, depth + 1)?;
                children.splice(i..i + 1, changed);
                children
                    .chunks(128)
                    .map(|c| self.seq_branch(t, c.to_vec()))
                    .collect()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn batched_maps_and_sequence_appends_emit_only_final_changed_nodes() {
        let (_s, b) = super::super::object_backend("batch-index-nodes");
        let o = Objects::new(&b.store);
        let tree = Tree { objects: &o };
        let key = |i| serde_json::json!(["m", format!("key:{i:04}"), "1"]);
        let root = tree
            .map(
                "exact-reference-set/1",
                (0..1000).map(|i| (key(i), Value::Null)),
            )
            .unwrap();
        let before = b.file_work();
        let changes = (0..250)
            .map(|i| (key(i), None))
            .chain((1000..1250).map(|i| (key(i), Some(Value::Null))))
            .collect();
        let changed = tree
            .update_many(&root, "exact-reference-set/1", changes)
            .unwrap();
        assert!(b.file_work().objects_written - before.objects_written < 200);
        assert_eq!(
            changed,
            tree.map(
                "exact-reference-set/1",
                (250..1250).map(|i| (key(i), Value::Null))
            )
            .unwrap()
        );
        assert_eq!(
            tree.entries(&root, "exact-reference-set/1").unwrap().len(),
            1000
        );
        let refs = (0..300)
            .map(|i| serde_json::json!({"model":"m","id":format!("r{i}"),"revision":"1"}))
            .collect::<Vec<_>>();
        let root = tree.sequence("exact-refs/1", &refs).unwrap();
        let mut next = refs.clone();
        next.extend(
            (300..2300)
                .map(|i| serde_json::json!({"model":"m","id":format!("r{i}"),"revision":"1"})),
        );
        let before = b.file_work();
        let changed = tree.reconcile(Some(&root), "exact-refs/1", &next).unwrap();
        assert!(b.file_work().objects_written - before.objects_written < 32);
        assert_eq!(tree.items(&changed, "exact-refs/1").unwrap(), next);
        assert_eq!(tree.items(&root, "exact-refs/1").unwrap(), refs);
    }
    #[test]
    fn persistent_maps_share_paths_and_preserve_prior_roots() {
        let (_s, b) = super::super::object_backend("shared-maps");
        let o = Objects::new(&b.store);
        let tree = Tree { objects: &o };
        let values =
            (0..1000).map(|i| (serde_json::json!(["m", format!("r{i}"), "1"]), Value::Null));
        let root = tree.map("exact-reference-set/1", values).unwrap();
        let before = b.store.work.snapshot();
        let key = serde_json::json!(["m", "later", "1"]);
        let new = tree
            .update(
                &root,
                "exact-reference-set/1",
                key.clone(),
                Some(Value::Null),
            )
            .unwrap();
        assert!(b.store.work.snapshot().objects_written - before.objects_written < 10);
        assert_eq!(
            tree.get(&root, "exact-reference-set/1", &key).unwrap(),
            None
        );
        assert_eq!(
            tree.get(&new, "exact-reference-set/1", &key).unwrap(),
            Some(Value::Null)
        );
        assert_eq!(
            tree.update(&new, "exact-reference-set/1", key, None)
                .unwrap(),
            root
        );
        assert!(
            tree.get(&root, "selection/1", &serde_json::json!(["m", "r0"]))
                .is_err()
        );
        assert_eq!(
            tree.entries(&root, "exact-reference-set/1").unwrap().len(),
            1000
        );
        let mut bad: serde_json::Value =
            serde_json::from_slice(&o.get(&root).unwrap().bytes).unwrap();
        bad["count"] = "1".into();
        let bad = o.put_json("map-node", &bad).unwrap();
        assert!(tree.entries(&bad, "exact-reference-set/1").is_err());
    }
    #[test]
    fn sequences_preserve_order_duplicates_and_share_edits() {
        let (_s, b) = super::super::object_backend("shared-sequences");
        let o = Objects::new(&b.store);
        let tree = Tree { objects: &o };
        let values = (0..1000)
            .map(|i| serde_json::json!({"model":"m","id":format!("r{}",i%7),"revision":"1"}))
            .collect::<Vec<_>>();
        let root = tree.sequence("exact-refs/1", &values).unwrap();
        let before = b.store.work.snapshot();
        let mut next = values.clone();
        next.insert(
            127,
            serde_json::json!({"model":"m","id":"new","revision":"1"}),
        );
        let changed = tree.reconcile(Some(&root), "exact-refs/1", &next).unwrap();
        assert!(b.store.work.snapshot().objects_written - before.objects_written < 10);
        assert_eq!(tree.items(&root, "exact-refs/1").unwrap(), values);
        assert_eq!(tree.items(&changed, "exact-refs/1").unwrap(), next);
        assert_eq!(
            tree.seq_range(&changed, "exact-refs/1", 126, 3).unwrap(),
            next[126..129]
        );
        assert_eq!(
            tree.items(
                &tree
                    .reconcile(Some(&changed), "exact-refs/1", &values)
                    .unwrap(),
                "exact-refs/1"
            )
            .unwrap(),
            values
        );
    }
}
