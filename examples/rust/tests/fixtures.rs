//! Checks the generated types against the docs examples in examples/fixtures
//! (see tools/make_fixtures.py).
//!   - "ok" cases must decode and re-encode to exactly `known_json`: the example with
//!     only the fields the schema defines. That catches wrong JSON names and types,
//!     which plain decoding would silently ignore.
//!   - "stale" cases (old examples missing newer fields) must fail on one of the
//!     fields listed in `missing`, because required fields aren't `Option` here.
//!   - "docs-mistake" cases must fail to decode or change when re-encoded, so a fixed
//!     docs example shows up as a failure until the fixtures are regenerated.

mod registry;

use registry::Outcome;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct Fixture {
    cases: Vec<Case>,
}

#[derive(Deserialize)]
struct Case {
    name: String,
    kind: String,
    #[serde(rename = "type")]
    title: Option<String>,
    status: String,
    #[serde(default)]
    notes: Vec<String>,
    json: Value,
    known_json: Option<Value>,
    #[serde(default)]
    missing: Vec<String>,
}

/// Where got and want differ (None if they don't). Integers are compared exactly,
/// other numbers with a small relative tolerance (formatting may differ).
fn diff(got: &Value, want: &Value, path: &str) -> Option<String> {
    match (got, want) {
        (Value::Object(g), Value::Object(w)) => {
            let mut keys: Vec<&String> = g.keys().chain(w.keys()).collect();
            keys.sort();
            keys.dedup();
            keys.into_iter().find_map(|k| {
                let p = format!("{path}/{k}");
                match (g.get(k), w.get(k)) {
                    (Some(_), None) => Some(format!("{p}: written by the generated type but not in the example")),
                    (None, Some(_)) => Some(format!("{p}: in the example but lost by the generated type")),
                    (Some(gv), Some(wv)) => diff(gv, wv, &p),
                    (None, None) => None,
                }
            })
        }
        (Value::Array(g), Value::Array(w)) if g.len() == w.len() => {
            g.iter().zip(w).enumerate().find_map(|(i, (gv, wv))| diff(gv, wv, &format!("{path}/{i}")))
        }
        (Value::Number(g), Value::Number(w)) => {
            let same = match (g.as_u64(), w.as_u64(), g.as_i64(), w.as_i64()) {
                (Some(a), Some(b), _, _) => a == b,
                (_, _, Some(a), Some(b)) => a == b,
                _ => {
                    let (a, b) = (g.as_f64().unwrap(), w.as_f64().unwrap());
                    (a - b).abs() <= 1e-12 * b.abs().max(1.0)
                }
            };
            (!same).then(|| format!("{path}: {g} != {w}"))
        }
        _ if got == want => None,
        _ => Some(format!("{path}: {got} != {want}")),
    }
}

fn check(c: &Case, outcome: Outcome) -> Result<(), String> {
    match (c.status.as_str(), outcome.result) {
        ("ok", Err(e)) => Err(format!("decode: {e}")),
        ("ok", Ok(back)) => {
            // empty-object schema: quicktype makes a map, which keeps every key
            let want = if outcome.is_map { &c.json } else { c.known_json.as_ref().unwrap_or(&c.json) };
            match diff(&back, want, "") {
                Some(d) => Err(format!("re-encoded, the example differs at {d}")),
                None => Ok(()),
            }
        }
        ("stale", Ok(_)) => Err(format!("decoded, but should miss one of {:?}", c.missing)),
        ("stale", Err(e)) => {
            let msg = e.to_string();
            let expected = c.missing.iter().any(|m| {
                let field = m.rsplit('/').next().unwrap_or(m);
                msg.starts_with(&format!("missing field `{field}`"))
            });
            if expected { Ok(()) } else { Err(format!("failed with {msg:?}, expected one of {:?}", c.missing)) }
        }
        ("docs-mistake", Ok(back)) if diff(&back, &c.json, "").is_none() => {
            Err(format!("docs example no longer shows the mistake {:?}; regenerate the fixtures", c.notes))
        }
        ("docs-mistake", _) => Ok(()),
        (s, _) => Err(format!("unknown status {s:?}")),
    }
}

fn run(file: &str, lookup: fn(&str, &Value) -> Option<Outcome>) {
    let path = format!("{}/../fixtures/{file}", env!("CARGO_MANIFEST_DIR"));
    let raw = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let fixture: Fixture = serde_json::from_str(&raw).unwrap();
    let (mut checked, mut failures) = (0, Vec::new());
    for c in &fixture.cases {
        let Some(title) = c.title.as_deref() else { continue };
        if c.kind != "request" && c.kind != "response" {
            continue;
        }
        checked += 1;
        let result = match lookup(title, &c.json) {
            None => Err(format!("no generated Rust type for {title}")),
            Some(outcome) => check(c, outcome),
        };
        if let Err(e) = result {
            failures.push(format!("{}/{} ({title}, {}): {e}", c.name, c.kind, c.status));
        }
    }
    println!("{file}: {checked} typed cases checked");
    assert!(checked > 0, "no typed cases in {path}");
    assert!(failures.is_empty(), "{} of {checked} cases failed:\n{}", failures.len(), failures.join("\n"));
}

#[test]
fn daemon_docs_examples() {
    run("daemon.json", registry::monerod);
}

#[test]
fn wallet_docs_examples() {
    run("wallet.json", registry::walletrpc);
}
