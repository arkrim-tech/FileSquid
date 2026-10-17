//! Extract EXIF and ID3 from a media file and map onto statements.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use exif::Reader;
use id3::TagLike;

use crate::record::Record;
use crate::MetadataNamespace;

/// Read embedded tags from `path` and merge them into `record`.
pub fn extract_into(path: &Path, record: &mut Record) {
    try_exif(path, record);
    try_id3(path, record);
}

fn try_exif(path: &Path, record: &mut Record) {
    let Ok(file) = File::open(path) else {
        return;
    };
    let mut reader = BufReader::new(file);
    let Ok(exif) = Reader::new().read_from_container(&mut reader) else {
        return;
    };
    for field in exif.fields() {
        let tag = format!("{}", field.tag);
        if tag.is_empty() || tag.starts_with("0x") {
            continue;
        }
        let value = field.display_value().with_unit(&exif).to_string();
        let _ = record.set(MetadataNamespace::Exif, &tag, value.clone());
        map_exif_to_dcterms(&tag, &value, record);
    }
}

fn map_exif_to_dcterms(tag: &str, value: &str, record: &mut Record) {
    let dcterms = match tag {
        "ImageDescription" | "UserComment" => Some("description"),
        "Artist" | "XPAuthor" => Some("creator"),
        "Copyright" => Some("rights"),
        "DateTimeOriginal" | "DateTime" => Some("created"),
        "ImageUniqueID" => Some("identifier"),
        _ => None,
    };
    if let Some(term) = dcterms {
        if record.get(&format!("dcterms:{term}")).is_none() {
            let _ = record.set(MetadataNamespace::Dcterms, term, value);
        }
    }
}

fn try_id3(path: &Path, record: &mut Record) {
    let Ok(tag) = id3::Tag::read_from_path(path) else {
        return;
    };
    if let Some(title) = tag.title() {
        let _ = record.set(MetadataNamespace::Dcterms, "title", title);
    }
    if let Some(artist) = tag.artist() {
        let _ = record.set(MetadataNamespace::Dcterms, "creator", artist);
    }
    if let Some(album) = tag.album() {
        let _ = record.set(MetadataNamespace::SchemaOrg, "album", album);
    }
    if let Some(year) = tag.year() {
        let _ = record.set(
            MetadataNamespace::Dcterms,
            "dateCopyrighted",
            year.to_string(),
        );
    }
}
