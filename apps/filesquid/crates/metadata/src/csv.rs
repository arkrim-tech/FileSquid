//! CSV interchange: one row per object, headers `prefix:term`.

use crate::{reject_file_dcterms_term, MetadataError, MetadataNamespace};

/// A CSV column header.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CsvHeader {
    /// Column namespace.
    pub namespace: MetadataNamespace,
    /// Local name.
    pub term: String,
}

/// Errors from CSV header parsing.
#[derive(Debug, PartialEq, Eq)]
pub enum CsvError {
    /// Header is empty.
    EmptyHeader,
    /// Header is not `prefix:term`.
    NotCurie,
    /// Prefix is not `exif`, `dcterms`, or `schema`.
    UnknownPrefix,
    /// `dcterms:` term fails the file register.
    Dcterms(MetadataError),
    /// A quoted field is not closed before the end of the document.
    UnterminatedQuote,
}

/// Parse a single header cell such as `dcterms:title`.
pub fn parse_csv_header(cell: &str) -> Result<CsvHeader, CsvError> {
    let cell = cell.trim();
    if cell.is_empty() {
        return Err(CsvError::EmptyHeader);
    }
    let Some((prefix, term)) = cell.split_once(':') else {
        return Err(CsvError::NotCurie);
    };
    if term.is_empty() {
        return Err(CsvError::NotCurie);
    }
    let namespace = match prefix {
        "exif" => MetadataNamespace::Exif,
        "dcterms" => MetadataNamespace::Dcterms,
        "schema" => MetadataNamespace::SchemaOrg,
        _ => return Err(CsvError::UnknownPrefix),
    };
    if namespace == MetadataNamespace::Dcterms {
        reject_file_dcterms_term(term).map_err(CsvError::Dcterms)?;
    }
    Ok(CsvHeader {
        namespace,
        term: term.to_string(),
    })
}

/// Parse a comma-separated header line after an optional leading `id` column.
pub fn parse_csv_header_line(line: &str) -> Result<Vec<CsvHeader>, CsvError> {
    let rows = parse_records(line)?;
    let Some(cells) = rows.first() else {
        return Ok(Vec::new());
    };
    parse_header_cells(cells)
}

/// Parse header cells, skipping a leading `id`/`object`/`object_id` column.
fn parse_header_cells(cells: &[String]) -> Result<Vec<CsvHeader>, CsvError> {
    let skip = usize::from(cells.first().is_some_and(|cell| is_id_column(cell)));
    cells
        .iter()
        .skip(skip)
        .map(|cell| parse_csv_header(cell))
        .collect()
}

/// Return whether `cell` names the optional leading object-identifier column.
fn is_id_column(cell: &str) -> bool {
    matches!(cell.trim(), "id" | "object" | "object_id")
}

/// Format a header as `prefix:term`.
pub fn format_csv_header(header: &CsvHeader) -> String {
    let prefix = match header.namespace {
        MetadataNamespace::Exif => "exif",
        MetadataNamespace::Dcterms => "dcterms",
        MetadataNamespace::SchemaOrg => "schema",
    };
    format!("{prefix}:{}", header.term)
}

use crate::record::Record;

/// Export one object as CSV (header + one data row). `id` is the first column.
pub fn to_csv(object_id: &str, record: &Record) -> String {
    let mut headers = vec!["id".to_string()];
    let mut values = vec![csv_escape(object_id)];
    for s in &record.statements {
        headers.push(s.curie());
        values.push(csv_escape(&s.value));
    }
    format!("{}\n{}\n", headers.join(","), values.join(","))
}

/// Apply the first data row of a CSV document onto `record`.
///
/// Cells follow RFC 4180: quoted fields MAY contain commas, doubled quotes,
/// and line breaks. Values are stored exactly as unquoted, without trimming.
pub fn apply_csv(text: &str, record: &mut Record) -> Result<(), CsvError> {
    let rows = parse_records(text)?;
    let Some(header_row) = rows.first() else {
        return Ok(());
    };
    let headers = parse_header_cells(header_row)?;
    let Some(data) = rows.get(1) else {
        return Ok(());
    };
    let skip = usize::from(header_row.first().is_some_and(|cell| is_id_column(cell)));
    for (header, value) in headers.iter().zip(data.iter().skip(skip)) {
        record
            .set(header.namespace, &header.term, value.as_str())
            .map_err(CsvError::Dcterms)?;
    }
    Ok(())
}

/// Split a CSV document into records of unquoted cell values (RFC 4180).
///
/// The loop is bounded by the input length. Physically blank lines are
/// skipped; a line holding only a quoted or whitespace value is kept. A quote
/// left open at the end of the text is an error.
fn parse_records(text: &str) -> Result<Vec<Vec<String>>, CsvError> {
    let mut parser = RecordParser::default();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match (parser.in_quotes, c) {
            (true, '"') if chars.peek() == Some(&'"') => {
                parser.cell.push('"');
                chars.next();
            }
            (true, '"') => parser.in_quotes = false,
            (true, _) => parser.cell.push(c),
            (false, '"') => parser.open_quote(),
            (false, ',') => parser.end_cell(),
            (false, '\r') => {}
            (false, '\n') => parser.end_record(),
            (false, _) => parser.push_plain(c),
        }
    }
    if parser.in_quotes {
        return Err(CsvError::UnterminatedQuote);
    }
    parser.end_record();
    Ok(parser.records)
}

/// Mutable state for [`parse_records`].
#[derive(Default)]
struct RecordParser {
    /// Completed records.
    records: Vec<Vec<String>>,
    /// Cells of the record being read.
    record: Vec<String>,
    /// Unquoted text of the cell being read.
    cell: String,
    /// Whether the cursor is inside a quoted field.
    in_quotes: bool,
    /// Whether the current line has any content (a character, comma, or quote).
    has_content: bool,
}

impl RecordParser {
    /// Enter a quoted field.
    fn open_quote(&mut self) {
        self.in_quotes = true;
        self.has_content = true;
    }

    /// Append an unquoted character to the current cell.
    fn push_plain(&mut self, c: char) {
        self.cell.push(c);
        self.has_content = true;
    }

    /// Close the current cell at a comma.
    fn end_cell(&mut self) {
        self.record.push(std::mem::take(&mut self.cell));
        self.has_content = true;
    }

    /// Close the current record, dropping it when the line was physically blank.
    fn end_record(&mut self) {
        self.record.push(std::mem::take(&mut self.cell));
        let finished = std::mem::take(&mut self.record);
        if self.has_content {
            self.records.push(finished);
        }
        self.has_content = false;
    }
}

/// Quote a value when it contains a comma, quote, or line break (RFC 4180).
fn csv_escape(value: &str) -> String {
    if value.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}
