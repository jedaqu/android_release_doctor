use std::{error::Error, fmt};

pub const ANDROID_NAMESPACE: &str = "http://schemas.android.com/apk/res/android";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestInfo {
    pub package_name: Option<String>,
    pub version_code: Option<u32>,
    pub version_name: Option<String>,
    pub min_sdk: Option<u32>,
    pub target_sdk: Option<u32>,
    pub debuggable: Option<bool>,
    pub permissions: Vec<String>,
    pub components: Vec<ComponentInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentInfo {
    pub kind: String,
    pub name: String,
    pub exported: Option<bool>,
    pub has_intent_filters: bool,
}

#[derive(Debug)]
pub enum AxmlError {
    Truncated,
    Invalid(&'static str),
}

impl fmt::Display for AxmlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated => write!(f, "truncated Android binary XML"),
            Self::Invalid(message) => write!(f, "invalid Android binary XML: {message}"),
        }
    }
}

impl Error for AxmlError {}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.position)
    }

    fn read_u8(&mut self) -> Result<u8, AxmlError> {
        if self.remaining() < 1 {
            return Err(AxmlError::Truncated);
        }
        let value = self.bytes[self.position];
        self.position += 1;
        Ok(value)
    }

    fn read_u16(&mut self) -> Result<u16, AxmlError> {
        if self.remaining() < 2 {
            return Err(AxmlError::Truncated);
        }
        let value = u16::from_le_bytes([self.bytes[self.position], self.bytes[self.position + 1]]);
        self.position += 2;
        Ok(value)
    }

    fn read_u32(&mut self) -> Result<u32, AxmlError> {
        if self.remaining() < 4 {
            return Err(AxmlError::Truncated);
        }
        let value = u32::from_le_bytes(
            self.bytes[self.position..self.position + 4]
                .try_into()
                .map_err(|_| AxmlError::Truncated)?,
        );
        self.position += 4;
        Ok(value)
    }

    fn skip(&mut self, bytes: usize) -> Result<(), AxmlError> {
        if self.remaining() < bytes {
            return Err(AxmlError::Truncated);
        }
        self.position += bytes;
        Ok(())
    }

    fn take(&mut self, bytes: usize) -> Result<&'a [u8], AxmlError> {
        if self.remaining() < bytes {
            return Err(AxmlError::Truncated);
        }
        let value = &self.bytes[self.position..self.position + bytes];
        self.position += bytes;
        Ok(value)
    }
}

#[derive(Debug, Clone)]
enum TypedValue {
    String(String),
    Int(u32),
    Bool(bool),
    Reference,
    Other,
}

#[derive(Debug, Clone)]
struct Attribute {
    namespace: Option<String>,
    name: String,
    value: TypedValue,
}

#[derive(Debug)]
struct Tag {
    name: String,
    attributes: Vec<Attribute>,
    has_intent_filters: bool,
}

fn decode_u8_length(cursor: &mut Cursor<'_>) -> Result<usize, AxmlError> {
    let first = cursor.read_u8()? as usize;
    if first & 0x80 == 0 {
        Ok(first)
    } else {
        let second = cursor.read_u8()? as usize;
        Ok(((first & 0x7f) << 7) | (second & 0x7f))
    }
}

fn decode_u16_length(cursor: &mut Cursor<'_>) -> Result<usize, AxmlError> {
    let first = cursor.read_u16()? as usize;
    if first & 0x8000 == 0 {
        Ok(first)
    } else {
        let second = cursor.read_u16()? as usize;
        Ok(((first & 0x7fff) << 15) | (second & 0x7fff))
    }
}

fn parse_string_pool(
    bytes: &[u8],
    chunk_start: usize,
    chunk_size: usize,
) -> Result<Vec<String>, AxmlError> {
    if chunk_size < 28 {
        return Err(AxmlError::Invalid("string pool header is too small"));
    }

    let mut header = Cursor::new(&bytes[chunk_start + 8..chunk_start + chunk_size]);
    let string_count = header.read_u32()? as usize;
    let _style_count = header.read_u32()?;
    let flags = header.read_u32()?;
    let strings_start = header.read_u32()? as usize;
    let _styles_start = header.read_u32()?;

    if strings_start < 28 || strings_start > chunk_size {
        return Err(AxmlError::Invalid(
            "string pool string data offset is invalid",
        ));
    }

    let offset_bytes = string_count
        .checked_mul(4)
        .ok_or(AxmlError::Invalid("string pool count overflows"))?;
    let offsets_start = chunk_start + 28;
    let offsets_end = offsets_start
        .checked_add(offset_bytes)
        .ok_or(AxmlError::Invalid("string pool offsets overflow"))?;

    if offsets_end > chunk_start + chunk_size {
        return Err(AxmlError::Truncated);
    }

    let data_start = chunk_start + strings_start;
    let data_end = chunk_start + chunk_size;
    let utf8 = flags & 0x100 != 0;
    let mut strings = Vec::with_capacity(string_count);

    for index in 0..string_count {
        let offset_position = offsets_start + index * 4;
        let offset = u32::from_le_bytes(
            bytes[offset_position..offset_position + 4]
                .try_into()
                .map_err(|_| AxmlError::Truncated)?,
        ) as usize;

        let string_position = data_start
            .checked_add(offset)
            .ok_or(AxmlError::Invalid("string offset overflows"))?;

        if string_position >= data_end {
            return Err(AxmlError::Truncated);
        }

        let mut cursor = Cursor::new(&bytes[string_position..data_end]);

        let value = if utf8 {
            let _character_count = decode_u8_length(&mut cursor)?;
            let byte_count = decode_u8_length(&mut cursor)?;
            let raw = cursor.take(byte_count)?;
            String::from_utf8(raw.to_vec())
                .map_err(|_| AxmlError::Invalid("string pool contains invalid UTF-8"))?
        } else {
            let character_count = decode_u16_length(&mut cursor)?;
            let byte_count = character_count
                .checked_mul(2)
                .ok_or(AxmlError::Invalid("UTF-16 string length overflows"))?;
            let raw = cursor.take(byte_count)?;
            let mut units = Vec::with_capacity(character_count);
            for pair in raw.as_chunks::<2>().0 {
                units.push(u16::from_le_bytes(*pair));
            }
            String::from_utf16(&units)
                .map_err(|_| AxmlError::Invalid("string pool contains invalid UTF-16"))?
        };

        strings.push(value);
    }

    Ok(strings)
}

fn pool_string(pool: &[String], index: u32) -> Result<String, AxmlError> {
    if index == u32::MAX {
        return Err(AxmlError::Invalid("unexpected null string index"));
    }

    pool.get(index as usize)
        .cloned()
        .ok_or(AxmlError::Invalid("string index is out of range"))
}

fn pool_optional(pool: &[String], index: u32) -> Result<Option<String>, AxmlError> {
    if index == u32::MAX {
        Ok(None)
    } else {
        pool_string(pool, index).map(Some)
    }
}

fn parse_typed_value(
    pool: &[String],
    raw_index: u32,
    data_type: u8,
    data: u32,
) -> Result<TypedValue, AxmlError> {
    match data_type {
        0x01 => Ok(TypedValue::Reference),
        0x03 => {
            if raw_index != u32::MAX {
                pool_string(pool, raw_index).map(TypedValue::String)
            } else {
                pool_string(pool, data).map(TypedValue::String)
            }
        }
        0x10 | 0x11 => Ok(TypedValue::Int(data)),
        0x12 => Ok(TypedValue::Bool(data != 0)),
        _ => Ok(TypedValue::Other),
    }
}

fn parse_attributes(
    pool: &[String],
    cursor: &mut Cursor<'_>,
    attribute_start: usize,
    attribute_size: usize,
    attribute_count: usize,
) -> Result<Vec<Attribute>, AxmlError> {
    if attribute_start < 20 {
        return Err(AxmlError::Invalid(
            "attribute start precedes attribute extension",
        ));
    }
    if attribute_size < 20 {
        return Err(AxmlError::Invalid(
            "attribute size is smaller than 20 bytes",
        ));
    }

    cursor.skip(attribute_start - 20)?;

    let total_bytes = attribute_size
        .checked_mul(attribute_count)
        .ok_or(AxmlError::Invalid("attribute count overflows"))?;
    if total_bytes > cursor.remaining() {
        return Err(AxmlError::Truncated);
    }

    let mut attributes = Vec::with_capacity(attribute_count);

    for _ in 0..attribute_count {
        let namespace_index = cursor.read_u32()?;
        let name_index = cursor.read_u32()?;
        let raw_value_index = cursor.read_u32()?;
        let value_size = cursor.read_u16()?;
        let _reserved = cursor.read_u8()?;
        let data_type = cursor.read_u8()?;
        let data = cursor.read_u32()?;

        if value_size < 8 {
            return Err(AxmlError::Invalid("typed value is smaller than 8 bytes"));
        }
        if value_size > 8 {
            cursor.skip((value_size - 8) as usize)?;
        }

        attributes.push(Attribute {
            namespace: pool_optional(pool, namespace_index)?,
            name: pool_string(pool, name_index)?,
            value: parse_typed_value(pool, raw_value_index, data_type, data)?,
        });
    }

    Ok(attributes)
}

fn is_component(tag: &str) -> bool {
    matches!(
        tag,
        "activity" | "activity-alias" | "service" | "receiver" | "provider"
    )
}

fn component_from_tag(tag: Tag) -> Option<ComponentInfo> {
    if !is_component(&tag.name) {
        return None;
    }

    let mut name = None;
    let mut exported = None;

    for attribute in tag.attributes {
        if attribute.namespace.as_deref() != Some(ANDROID_NAMESPACE) {
            continue;
        }

        match (attribute.name.as_str(), attribute.value) {
            ("name", TypedValue::String(value)) => name = Some(value),
            ("exported", TypedValue::Bool(value)) => exported = Some(value),
            _ => {}
        }
    }

    Some(ComponentInfo {
        kind: tag.name,
        name: name.unwrap_or_else(|| "<unnamed>".to_string()),
        exported,
        has_intent_filters: tag.has_intent_filters,
    })
}

fn chunk_bounds(
    bytes: &[u8],
    offset: usize,
    total_size: usize,
) -> Result<(u16, usize, usize), AxmlError> {
    if offset
        .checked_add(8)
        .ok_or(AxmlError::Invalid("chunk offset overflow"))?
        > total_size
    {
        return Err(AxmlError::Truncated);
    }

    let chunk_type = u16::from_le_bytes(
        bytes[offset..offset + 2]
            .try_into()
            .map_err(|_| AxmlError::Truncated)?,
    );
    let header_size = u16::from_le_bytes(
        bytes[offset + 2..offset + 4]
            .try_into()
            .map_err(|_| AxmlError::Truncated)?,
    ) as usize;
    let chunk_size = u32::from_le_bytes(
        bytes[offset + 4..offset + 8]
            .try_into()
            .map_err(|_| AxmlError::Truncated)?,
    ) as usize;

    if header_size < 8 || chunk_size < header_size || chunk_size == 0 {
        return Err(AxmlError::Invalid("invalid XML chunk bounds"));
    }

    if offset
        .checked_add(chunk_size)
        .ok_or(AxmlError::Invalid("chunk size overflows"))?
        > total_size
    {
        return Err(AxmlError::Truncated);
    }

    Ok((chunk_type, header_size, chunk_size))
}

pub fn parse_manifest(data: &[u8]) -> Result<ManifestInfo, AxmlError> {
    if data.len() < 8 {
        return Err(AxmlError::Truncated);
    }

    let root_type = u16::from_le_bytes(data[0..2].try_into().map_err(|_| AxmlError::Truncated)?);
    let root_header_size =
        u16::from_le_bytes(data[2..4].try_into().map_err(|_| AxmlError::Truncated)?);
    let total_size =
        u32::from_le_bytes(data[4..8].try_into().map_err(|_| AxmlError::Truncated)?) as usize;

    if root_type != 0x0003 {
        return Err(AxmlError::Invalid("missing RES_XML_TYPE root chunk"));
    }
    if root_header_size != 8 {
        return Err(AxmlError::Invalid("unexpected XML root header size"));
    }
    if total_size < 8 || total_size > data.len() {
        return Err(AxmlError::Truncated);
    }

    let mut string_pool = None;
    let mut offset = 8usize;

    while offset < total_size {
        let (chunk_type, _, chunk_size) = chunk_bounds(data, offset, total_size)?;
        if chunk_type == 0x0001 {
            string_pool = Some(parse_string_pool(data, offset, chunk_size)?);
            break;
        }
        offset += chunk_size;
    }

    let string_pool = string_pool.ok_or(AxmlError::Invalid("manifest has no string pool chunk"))?;

    let mut info = ManifestInfo {
        package_name: None,
        version_code: None,
        version_name: None,
        min_sdk: None,
        target_sdk: None,
        debuggable: None,
        permissions: Vec::new(),
        components: Vec::new(),
    };

    let mut stack = Vec::<Tag>::new();
    offset = 8;

    while offset < total_size {
        let (chunk_type, _header_size, chunk_size) = chunk_bounds(data, offset, total_size)?;

        match chunk_type {
            0x0102 => {
                let mut cursor = Cursor::new(&data[offset + 8..offset + chunk_size]);
                let _line = cursor.read_u32()?;
                let _comment = cursor.read_u32()?;
                let _namespace_index = cursor.read_u32()?;
                let name_index = cursor.read_u32()?;
                let attribute_start = cursor.read_u16()? as usize;
                let attribute_size = cursor.read_u16()? as usize;
                let attribute_count = cursor.read_u16()? as usize;
                let _id_index = cursor.read_u16()?;
                let _class_index = cursor.read_u16()?;
                let _style_index = cursor.read_u16()?;

                let name = pool_string(&string_pool, name_index)?;

                if name == "intent-filter" {
                    if let Some(parent) = stack.last_mut() {
                        parent.has_intent_filters = true;
                    }
                }

                let attributes = parse_attributes(
                    &string_pool,
                    &mut cursor,
                    attribute_start,
                    attribute_size,
                    attribute_count,
                )?;

                stack.push(Tag {
                    name,
                    attributes,
                    has_intent_filters: false,
                });
            }
            0x0103 => {
                let mut cursor = Cursor::new(&data[offset + 8..offset + chunk_size]);
                let _line = cursor.read_u32()?;
                let _comment = cursor.read_u32()?;
                let _namespace_index = cursor.read_u32()?;
                let name_index = cursor.read_u32()?;
                let end_name = pool_string(&string_pool, name_index)?;

                let tag = stack
                    .pop()
                    .ok_or(AxmlError::Invalid("end tag without a start tag"))?;

                if tag.name != end_name {
                    return Err(AxmlError::Invalid("start/end tag names do not match"));
                }

                match end_name.as_str() {
                    "manifest" => {
                        for attribute in tag.attributes {
                            match (
                                attribute.namespace.as_deref(),
                                attribute.name.as_str(),
                                attribute.value,
                            ) {
                                (None, "package", TypedValue::String(value)) => {
                                    info.package_name = Some(value)
                                }
                                (
                                    Some(ANDROID_NAMESPACE),
                                    "versionCode",
                                    TypedValue::Int(value),
                                ) => info.version_code = Some(value),
                                (
                                    Some(ANDROID_NAMESPACE),
                                    "versionName",
                                    TypedValue::String(value),
                                ) => info.version_name = Some(value),
                                _ => {}
                            }
                        }
                    }
                    "uses-sdk" => {
                        for attribute in tag.attributes {
                            match (
                                attribute.namespace.as_deref(),
                                attribute.name.as_str(),
                                attribute.value,
                            ) {
                                (
                                    Some(ANDROID_NAMESPACE),
                                    "minSdkVersion",
                                    TypedValue::Int(value),
                                ) => info.min_sdk = Some(value),
                                (
                                    Some(ANDROID_NAMESPACE),
                                    "targetSdkVersion",
                                    TypedValue::Int(value),
                                ) => info.target_sdk = Some(value),
                                _ => {}
                            }
                        }
                    }
                    "uses-permission" => {
                        for attribute in tag.attributes {
                            if attribute.namespace.as_deref() == Some(ANDROID_NAMESPACE)
                                && attribute.name == "name"
                            {
                                if let TypedValue::String(value) = attribute.value {
                                    info.permissions.push(value);
                                }
                            }
                        }
                    }
                    "application" => {
                        for attribute in tag.attributes {
                            if attribute.namespace.as_deref() == Some(ANDROID_NAMESPACE)
                                && attribute.name == "debuggable"
                            {
                                if let TypedValue::Bool(value) = attribute.value {
                                    info.debuggable = Some(value);
                                }
                            }
                        }
                    }
                    _ => {
                        if let Some(component) = component_from_tag(tag) {
                            info.components.push(component);
                        }
                    }
                }
            }
            _ => {}
        }

        offset += chunk_size;
    }

    if !stack.is_empty() {
        return Err(AxmlError::Invalid(
            "manifest contains unterminated elements",
        ));
    }

    info.components.reverse();
    info.permissions.sort();
    info.permissions.dedup();

    Ok(info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_plain_text_xml() {
        assert!(parse_manifest(br#"<manifest package="x"/>"#).is_err());
    }

    #[test]
    fn rejects_truncated_binary_xml() {
        assert!(parse_manifest(&[0x03, 0x00, 0x08]).is_err());
    }
}
