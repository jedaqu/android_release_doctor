use std::{error::Error, fmt};

use crate::axml::{ComponentInfo, ManifestInfo, ANDROID_NAMESPACE};

#[derive(Debug)]
pub enum ProtoManifestError {
    Truncated,
    Invalid(&'static str),
    InvalidUtf8,
}

impl fmt::Display for ProtoManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated => write!(f, "truncated Android manifest protobuf"),
            Self::Invalid(message) => write!(f, "invalid Android manifest protobuf: {message}"),
            Self::InvalidUtf8 => write!(f, "invalid UTF-8 in Android manifest protobuf"),
        }
    }
}

impl Error for ProtoManifestError {}

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

    fn is_empty(&self) -> bool {
        self.position == self.bytes.len()
    }

    fn read_u8(&mut self) -> Result<u8, ProtoManifestError> {
        if self.remaining() < 1 {
            return Err(ProtoManifestError::Truncated);
        }
        let value = self.bytes[self.position];
        self.position += 1;
        Ok(value)
    }

    fn read_varint(&mut self) -> Result<u64, ProtoManifestError> {
        let mut value = 0_u64;
        for shift in (0..=63).step_by(7) {
            let byte = self.read_u8()?;
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err(ProtoManifestError::Invalid("varint exceeds 10 bytes"))
    }

    fn read_key(&mut self) -> Result<(u32, u8), ProtoManifestError> {
        let key = self.read_varint()?;
        let field_number = u32::try_from(key >> 3)
            .map_err(|_| ProtoManifestError::Invalid("protobuf field number overflows"))?;
        let wire_type = u8::try_from(key & 0x07)
            .map_err(|_| ProtoManifestError::Invalid("protobuf wire type overflows"))?;
        if field_number == 0 {
            return Err(ProtoManifestError::Invalid("protobuf field number is zero"));
        }
        Ok((field_number, wire_type))
    }

    fn read_length_delimited(&mut self) -> Result<&'a [u8], ProtoManifestError> {
        let length = usize::try_from(self.read_varint()?)
            .map_err(|_| ProtoManifestError::Invalid("protobuf length overflows"))?;
        if self.remaining() < length {
            return Err(ProtoManifestError::Truncated);
        }
        let start = self.position;
        self.position += length;
        Ok(&self.bytes[start..start + length])
    }

    fn read_string(&mut self) -> Result<String, ProtoManifestError> {
        let bytes = self.read_length_delimited()?;
        String::from_utf8(bytes.to_vec()).map_err(|_| ProtoManifestError::InvalidUtf8)
    }
}

fn skip_field(cursor: &mut Cursor<'_>, wire_type: u8) -> Result<(), ProtoManifestError> {
    match wire_type {
        0 => {
            cursor.read_varint()?;
            Ok(())
        }
        1 => {
            if cursor.remaining() < 8 {
                return Err(ProtoManifestError::Truncated);
            }
            cursor.position += 8;
            Ok(())
        }
        2 => {
            cursor.read_length_delimited()?;
            Ok(())
        }
        5 => {
            if cursor.remaining() < 4 {
                return Err(ProtoManifestError::Truncated);
            }
            cursor.position += 4;
            Ok(())
        }
        3 | 4 => Err(ProtoManifestError::Invalid(
            "protobuf groups are not supported",
        )),
        _ => Err(ProtoManifestError::Invalid("unknown protobuf wire type")),
    }
}

#[derive(Debug, Default)]
struct ProtoPrimitive {
    value_type: Option<u32>,
    data: Option<u32>,
}

#[derive(Debug, Default)]
struct ProtoItem {
    string_value: Option<String>,
    raw_string_value: Option<String>,
    primitive: Option<ProtoPrimitive>,
}

#[derive(Debug, Default)]
struct ProtoAttribute {
    namespace_uri: Option<String>,
    name: Option<String>,
    value: Option<String>,
    compiled_item: Option<ProtoItem>,
}

#[derive(Debug, Default)]
struct ProtoElement {
    name: Option<String>,
    attributes: Vec<ProtoAttribute>,
    children: Vec<ProtoNode>,
}

#[derive(Debug, Default)]
struct ProtoNode {
    element: Option<ProtoElement>,
}

fn parse_primitive(bytes: &[u8]) -> Result<ProtoPrimitive, ProtoManifestError> {
    let mut cursor = Cursor::new(bytes);
    let mut primitive = ProtoPrimitive::default();

    while !cursor.is_empty() {
        let (field, wire_type) = cursor.read_key()?;
        match field {
            1 if wire_type == 0 => {
                primitive.value_type = Some(
                    u32::try_from(cursor.read_varint()?)
                        .map_err(|_| ProtoManifestError::Invalid("primitive type overflows"))?,
                );
            }
            2 if wire_type == 0 => {
                primitive.data = Some(
                    u32::try_from(cursor.read_varint()?)
                        .map_err(|_| ProtoManifestError::Invalid("primitive data overflows"))?,
                );
            }
            _ => skip_field(&mut cursor, wire_type)?,
        }
    }

    Ok(primitive)
}

fn parse_item(bytes: &[u8]) -> Result<ProtoItem, ProtoManifestError> {
    let mut cursor = Cursor::new(bytes);
    let mut item = ProtoItem::default();

    while !cursor.is_empty() {
        let (field, wire_type) = cursor.read_key()?;
        match field {
            2 if wire_type == 2 => {
                item.string_value = Some(cursor.read_string()?);
            }
            3 if wire_type == 2 => {
                item.raw_string_value = Some(cursor.read_string()?);
            }
            7 if wire_type == 2 => {
                item.primitive = Some(parse_primitive(cursor.read_length_delimited()?)?);
            }
            _ => skip_field(&mut cursor, wire_type)?,
        }
    }

    Ok(item)
}

fn parse_attribute(bytes: &[u8]) -> Result<ProtoAttribute, ProtoManifestError> {
    let mut cursor = Cursor::new(bytes);
    let mut attribute = ProtoAttribute::default();

    while !cursor.is_empty() {
        let (field, wire_type) = cursor.read_key()?;
        match field {
            1 if wire_type == 2 => {
                attribute.namespace_uri = Some(cursor.read_string()?);
            }
            2 if wire_type == 2 => {
                attribute.name = Some(cursor.read_string()?);
            }
            3 if wire_type == 2 => {
                attribute.value = Some(cursor.read_string()?);
            }
            6 if wire_type == 2 => {
                attribute.compiled_item = Some(parse_item(cursor.read_length_delimited()?)?);
            }
            _ => skip_field(&mut cursor, wire_type)?,
        }
    }

    Ok(attribute)
}

fn parse_node(bytes: &[u8]) -> Result<ProtoNode, ProtoManifestError> {
    let mut cursor = Cursor::new(bytes);
    let mut node = ProtoNode::default();

    while !cursor.is_empty() {
        let (field, wire_type) = cursor.read_key()?;
        match field {
            1 if wire_type == 2 => {
                node.element = Some(parse_element(cursor.read_length_delimited()?)?);
            }
            2 if wire_type == 2 => {
                cursor.read_length_delimited()?;
            }
            _ => skip_field(&mut cursor, wire_type)?,
        }
    }

    Ok(node)
}

fn parse_element(bytes: &[u8]) -> Result<ProtoElement, ProtoManifestError> {
    let mut cursor = Cursor::new(bytes);
    let mut element = ProtoElement::default();

    while !cursor.is_empty() {
        let (field, wire_type) = cursor.read_key()?;
        match field {
            2 | 3 if wire_type == 2 => {
                let value = cursor.read_string()?;
                if field == 3 {
                    element.name = Some(value);
                }
            }
            4 if wire_type == 2 => {
                element
                    .attributes
                    .push(parse_attribute(cursor.read_length_delimited()?)?);
            }
            5 if wire_type == 2 => {
                element
                    .children
                    .push(parse_node(cursor.read_length_delimited()?)?);
            }
            _ => skip_field(&mut cursor, wire_type)?,
        }
    }

    Ok(element)
}

fn attr_string<'a>(
    attributes: &'a [ProtoAttribute],
    namespace: &str,
    name: &str,
) -> Option<&'a str> {
    attributes.iter().find_map(|attribute| {
        if attribute.namespace_uri.as_deref().unwrap_or_default() == namespace
            && attribute.name.as_deref() == Some(name)
        {
            attribute.value.as_deref().or_else(|| {
                attribute
                    .compiled_item
                    .as_ref()
                    .and_then(|item| item.string_value.as_deref())
                    .or_else(|| {
                        attribute
                            .compiled_item
                            .as_ref()
                            .and_then(|item| item.raw_string_value.as_deref())
                    })
            })
        } else {
            None
        }
    })
}

fn attr_primitive<'a>(
    attributes: &'a [ProtoAttribute],
    namespace: &str,
    name: &str,
) -> Option<&'a ProtoPrimitive> {
    attributes.iter().find_map(|attribute| {
        if attribute.namespace_uri.as_deref().unwrap_or_default() == namespace
            && attribute.name.as_deref() == Some(name)
        {
            attribute
                .compiled_item
                .as_ref()
                .and_then(|item| item.primitive.as_ref())
        } else {
            None
        }
    })
}

fn attr_u32(attributes: &[ProtoAttribute], name: &str) -> Option<u32> {
    attr_primitive(attributes, ANDROID_NAMESPACE, name)
        .and_then(|primitive| primitive.data)
        .or_else(|| {
            attr_string(attributes, ANDROID_NAMESPACE, name)?
                .parse()
                .ok()
        })
}

fn attr_bool(attributes: &[ProtoAttribute], name: &str) -> Option<bool> {
    attr_primitive(attributes, ANDROID_NAMESPACE, name)
        .and_then(|primitive| match primitive.value_type {
            Some(0x12) => primitive.data.map(|value| value != 0),
            _ => None,
        })
        .or_else(|| match attr_string(attributes, ANDROID_NAMESPACE, name)? {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        })
}

fn collect_manifest_info(element: &ProtoElement, info: &mut ManifestInfo, in_application: bool) {
    let name = element.name.as_deref().unwrap_or_default();

    match name {
        "manifest" => {
            if let Some(value) = element.attributes.iter().find_map(|attribute| {
                if attribute
                    .namespace_uri
                    .as_deref()
                    .unwrap_or_default()
                    .is_empty()
                    && attribute.name.as_deref() == Some("package")
                {
                    attr_string(std::slice::from_ref(attribute), "", "package")
                } else {
                    None
                }
            }) {
                info.package_name = Some(value.to_string());
            }

            if let Some(value) = attr_u32(&element.attributes, "versionCode") {
                info.version_code = Some(value);
            }
            if let Some(value) = attr_string(&element.attributes, ANDROID_NAMESPACE, "versionName")
            {
                info.version_name = Some(value.to_string());
            }
        }
        "uses-sdk" => {
            info.min_sdk = attr_u32(&element.attributes, "minSdkVersion");
            info.target_sdk = attr_u32(&element.attributes, "targetSdkVersion");
        }
        "uses-permission" => {
            if let Some(value) = attr_string(&element.attributes, ANDROID_NAMESPACE, "name") {
                info.permissions.push(value.to_string());
            }
        }
        "application" => {
            info.debuggable = attr_bool(&element.attributes, "debuggable");
        }
        _ => {}
    }

    let application_scope = in_application || name == "application";

    if application_scope
        && matches!(
            name,
            "activity" | "activity-alias" | "service" | "receiver" | "provider"
        )
    {
        let component_name =
            attr_string(&element.attributes, ANDROID_NAMESPACE, "name").unwrap_or("<unnamed>");
        let has_intent_filters = element.children.iter().any(|child| {
            child
                .element
                .as_ref()
                .and_then(|child_element| child_element.name.as_deref())
                == Some("intent-filter")
        });

        info.components.push(ComponentInfo {
            kind: name.to_string(),
            name: component_name.to_string(),
            exported: attr_bool(&element.attributes, "exported"),
            has_intent_filters,
        });
    }

    for child in &element.children {
        if let Some(child) = &child.element {
            collect_manifest_info(child, info, application_scope);
        }
    }
}

pub fn parse_manifest(data: &[u8]) -> Result<ManifestInfo, ProtoManifestError> {
    let root = parse_node(data)?;
    let element = root.element.ok_or(ProtoManifestError::Invalid(
        "manifest root node is not an element",
    ))?;

    if element.name.as_deref() != Some("manifest") {
        return Err(ProtoManifestError::Invalid(
            "manifest root element is not <manifest>",
        ));
    }

    let mut info = ManifestInfo::default();
    collect_manifest_info(&element, &mut info, false);
    Ok(info)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode_varint(value: u64, output: &mut Vec<u8>) {
        let mut value = value;
        while value >= 0x80 {
            output.push((value as u8 & 0x7f) | 0x80);
            value >>= 7;
        }
        output.push(value as u8);
    }

    fn encode_bytes(field: u32, value: &[u8], output: &mut Vec<u8>) {
        encode_varint((u64::from(field) << 3) | 2, output);
        encode_varint(value.len() as u64, output);
        output.extend_from_slice(value);
    }

    fn encode_string(field: u32, value: &str, output: &mut Vec<u8>) {
        encode_bytes(field, value.as_bytes(), output);
    }

    fn compiled_primitive(value_type: u32, data: u32) -> Vec<u8> {
        let mut primitive = Vec::new();
        encode_varint(8, &mut primitive);
        encode_varint(u64::from(value_type), &mut primitive);
        encode_varint(16, &mut primitive);
        encode_varint(u64::from(data), &mut primitive);

        let mut item = Vec::new();
        encode_bytes(7, &primitive, &mut item);
        item
    }

    fn attribute(
        namespace: &str,
        name: &str,
        value: &str,
        primitive: Option<(u32, u32)>,
    ) -> Vec<u8> {
        let mut output = Vec::new();
        encode_string(1, namespace, &mut output);
        encode_string(2, name, &mut output);
        encode_string(3, value, &mut output);
        if let Some((value_type, data)) = primitive {
            encode_bytes(6, &compiled_primitive(value_type, data), &mut output);
        }
        output
    }

    fn element(name: &str, attributes: &[Vec<u8>], children: &[Vec<u8>]) -> Vec<u8> {
        let mut output = Vec::new();
        encode_string(3, name, &mut output);
        for attribute in attributes {
            encode_bytes(4, attribute, &mut output);
        }
        for child in children {
            encode_bytes(5, child, &mut output);
        }
        let mut node = Vec::new();
        encode_bytes(1, &output, &mut node);
        node
    }

    #[test]
    fn parses_proto_manifest_fields() {
        let manifest = element(
            "manifest",
            &[
                attribute("", "package", "com.example.proto", None),
                attribute(ANDROID_NAMESPACE, "versionCode", "7", Some((0x10, 7))),
                attribute(ANDROID_NAMESPACE, "versionName", "1.2.3", None),
            ],
            &[
                element(
                    "uses-sdk",
                    &[
                        attribute(ANDROID_NAMESPACE, "minSdkVersion", "24", Some((0x10, 24))),
                        attribute(
                            ANDROID_NAMESPACE,
                            "targetSdkVersion",
                            "35",
                            Some((0x10, 35)),
                        ),
                    ],
                    &[],
                ),
                element(
                    "uses-permission",
                    &[attribute(
                        ANDROID_NAMESPACE,
                        "name",
                        "android.permission.INTERNET",
                        None,
                    )],
                    &[],
                ),
                element(
                    "application",
                    &[attribute(
                        ANDROID_NAMESPACE,
                        "debuggable",
                        "false",
                        Some((0x12, 0)),
                    )],
                    &[element(
                        "activity",
                        &[attribute(
                            ANDROID_NAMESPACE,
                            "name",
                            "com.example.MainActivity",
                            None,
                        )],
                        &[element("intent-filter", &[], &[])],
                    )],
                ),
            ],
        );

        let report = parse_manifest(&manifest).expect("proto manifest should parse");
        assert_eq!(report.package_name.as_deref(), Some("com.example.proto"));
        assert_eq!(report.version_code, Some(7));
        assert_eq!(report.version_name.as_deref(), Some("1.2.3"));
        assert_eq!(report.min_sdk, Some(24));
        assert_eq!(report.target_sdk, Some(35));
        assert_eq!(report.debuggable, Some(false));
        assert_eq!(report.permissions, vec!["android.permission.INTERNET"]);
        assert_eq!(report.components.len(), 1);
        assert_eq!(report.components[0].kind, "activity");
        assert_eq!(report.components[0].name, "com.example.MainActivity");
        assert!(report.components[0].has_intent_filters);
    }

    #[test]
    fn rejects_non_manifest_proto_root() {
        let node = element("application", &[], &[]);
        let error = parse_manifest(&node).expect_err("non-manifest root should fail");
        assert!(error.to_string().contains("manifest root element"));
    }
}
