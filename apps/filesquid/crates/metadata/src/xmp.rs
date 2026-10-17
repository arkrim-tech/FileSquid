//! Minimal XMP sidecar interchange for dcterms statements.

use crate::record::Record;
use crate::MetadataNamespace;

/// Serialize populated `dcterms:` statements as a small XMP packet.
pub fn to_xmp(record: &Record) -> String {
    let mut out = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
<x:xmpmeta xmlns:x=\"adobe:ns:meta/\">\n\
 <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">\n\
  <rdf:Description xmlns:dcterms=\"http://purl.org/dc/terms/\">\n",
    );
    for s in record
        .statements
        .iter()
        .filter(|s| s.namespace == MetadataNamespace::Dcterms)
    {
        out.push_str("   <dcterms:");
        out.push_str(&s.term);
        out.push('>');
        out.push_str(&xml_escape(&s.value));
        out.push_str("</dcterms:");
        out.push_str(&s.term);
        out.push_str(">\n");
    }
    out.push_str("  </rdf:Description>\n </rdf:RDF>\n</x:xmpmeta>\n");
    out
}

/// Read `dcterms:` and `dc:` elements from a sidecar produced by [`to_xmp`].
pub fn from_xmp(text: &str) -> Record {
    let mut record = Record::default();
    pull_tags(text, "dcterms:", MetadataNamespace::Dcterms, &mut record);
    pull_tags(text, "dc:", MetadataNamespace::Dcterms, &mut record);
    record
}

fn pull_tags(text: &str, prefix: &str, ns: MetadataNamespace, record: &mut Record) {
    let open = format!("<{prefix}");
    let mut rest = text;
    while let Some(start) = rest.find(&open) {
        rest = &rest[start + open.len()..];
        let Some(gt) = rest.find('>') else {
            break;
        };
        let term = rest[..gt].trim().trim_end_matches('/');
        if term.contains(' ') || term.is_empty() {
            rest = &rest[gt + 1..];
            continue;
        }
        rest = &rest[gt + 1..];
        let close = format!("</{prefix}{term}>");
        let Some(end) = rest.find(&close) else {
            continue;
        };
        let value = xml_unescape(&rest[..end]);
        let term = crate::map_dc_to_dcterms(term).unwrap_or(term);
        let _ = record.set(ns, term, value);
        rest = &rest[end + close.len()..];
    }
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn xml_unescape(value: &str) -> String {
    value
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}
