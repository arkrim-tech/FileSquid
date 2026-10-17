//! Unit tests: one counter set behind Prometheus text and JSON (K11).

use filesquid_api::metrics::Metrics;

#[test]
fn prometheus_text_and_json_report_the_same_values() {
    let m = Metrics::default();
    m.journal_entry_written(3);
    m.warning();
    m.error();
    m.error();
    let json = m.snapshot();
    let text = m.to_prometheus();
    for (name, value) in json.as_object().unwrap() {
        let line = text
            .lines()
            .find(|l| l.starts_with(&format!("{name} ")))
            .unwrap_or_else(|| panic!("{name} missing"));
        assert_eq!(
            line.split_whitespace().nth(1).unwrap(),
            value.to_string(),
            "{name}"
        );
    }
    assert_eq!(json["filesquid_errors_total"], 2);
    assert!(text.contains("filesquid_up 1"));
}
