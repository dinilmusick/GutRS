//! Cross-language SOURCE-DERIVED golden cases against actual public Rust API.
//! Every fixture lists the exact pinned TypeScript commit and expected behavior.
use gutrs::{
    arrays, auth_contracts, identity, json_utils, numbers, objects, strings, url_operations, urls,
};
use serde_json::{json, Value};

fn arg_string<'a>(v: &'a Value, name: &str) -> &'a str {
    v[name].as_str().expect("fixture input string")
}

fn int_values(v: &Value) -> Vec<i64> {
    v.as_array()
        .expect("fixture array")
        .iter()
        .map(|n| n.as_i64().expect("fixture integer"))
        .collect()
}

fn str_values(v: &Value) -> Vec<&str> {
    v.as_array()
        .expect("fixture array")
        .iter()
        .map(|n| n.as_str().expect("fixture string"))
        .collect()
}

#[test]
fn source_derived_pinned_public_contracts() {
    let fixture: Value =
        serde_json::from_str(include_str!("../testdata/upstream-golden/portable.json"))
            .expect("valid fixture");
    assert_eq!(
        fixture["source"]["sha"],
        "3e2cbc0990ba29830ce91c5d1a1d552675d5334c"
    );
    for case in fixture["cases"].as_array().expect("cases") {
        let input = &case["input"];
        let actual = match case["symbol"].as_str().expect("fixture symbol") {
            "arrays.chunk" => match arrays::chunk(
                &int_values(&input["values"]),
                input["size"].as_f64().expect("size"),
            ) {
                Ok(v) => json!(v),
                Err(e) => json!({"error":format!("{e:?}")}),
            },
            "arrays.last" => json!(arrays::last(&int_values(&input["values"])).copied()),
            "numbers.clamp" => json!(numbers::clamp(
                input["value"].as_f64().expect("value"),
                input["min"].as_f64().expect("min"),
                input["max"].as_f64().expect("max")
            )
            .expect("clamp")),
            "numbers.inRange" => json!(numbers::in_range(
                input["value"].as_f64().expect("value"),
                input["min"].as_f64().expect("min"),
                input["max"].as_f64().expect("max")
            )
            .expect("range")),
            "numbers.roundTo" => match numbers::round_to(
                input["value"].as_f64().expect("value"),
                input["decimals"].as_f64().expect("precision"),
            ) {
                Ok(v) => json!(v),
                Err(e) => json!({"error":format!("{e:?}")}),
            },
            "numbers.sum" => json!(numbers::sum(
                &input["values"]
                    .as_array()
                    .expect("numbers")
                    .iter()
                    .map(|v| v.as_f64().expect("float"))
                    .collect::<Vec<_>>()
            )),
            "strings.isBlank" => json!(strings::is_blank(arg_string(input, "value"))),
            "strings.startsWithAny" => json!(strings::starts_with_any(
                arg_string(input, "value"),
                &str_values(&input["prefixes"])
            )),
            "strings.endsWithAny" => json!(strings::ends_with_any(
                arg_string(input, "value"),
                &str_values(&input["suffixes"])
            )),
            "strings.truncateWords" => json!(strings::truncate_words(
                arg_string(input, "value"),
                input["maxWords"].as_f64().expect("maxWords"),
                arg_string(input, "suffix")
            )
            .expect("valid word count")),
            "urls.joinUrl" => json!(urls::join_url(
                arg_string(input, "base"),
                &str_values(&input["parts"])
            )),
            "urls.getQueryParam" => json!(url_operations::get_query_param(
                arg_string(input, "url"),
                arg_string(input, "key")
            )
            .expect("url")),
            "urls.setQueryParam" => json!(url_operations::set_query_param(
                arg_string(input, "url"),
                arg_string(input, "key"),
                arg_string(input, "value")
            )
            .expect("url")),
            "json.parseJson" => {
                json!(json_utils::parse_json(arg_string(input, "text")).expect("valid JSON"))
            }
            "json.tryParseJson" => json!(json_utils::try_parse_json(arg_string(input, "text"))),
            "json.stringifyJson" => {
                json!(json_utils::stringify_json(&input["value"], None).expect("serialization"))
            }
            "objects.hasOwn" => json!(objects::has_own_field(
                &input["value"],
                arg_string(input, "key")
            )),
            "objects.pick" => json!(objects::pick_fields(
                &input["value"],
                &str_values(&input["keys"])
            )),
            "auth.parseIdentityId" => json!(identity::parse_identity_id(
                arg_string(input, "kind"),
                &input["id"]
            )),
            "auth.parseIdentityRef" => json!(identity::parse_identity_ref(&input["ref"])),
            "auth.validateOAuthRedirectUri" => match auth_contracts::validate_oauth_redirect_uri(
                &input["url"],
                arg_string(input, "platform"),
                &[],
            ) {
                Ok(v) => json!(v),
                Err(e) => json!({"error":e.0}),
            },
            unknown => panic!("fixture does not exercise public API {unknown}"),
        };
        assert!(
            js_json_equal(&actual, &case["expected"]),
            "golden case {}: actual={actual:?}; expected={:?}",
            case["symbol"],
            case["expected"]
        );
    }
}

/// JavaScript JSON has one Number type; serde_json distinguishes integer and
/// floating representations even for the same numeric value.
fn js_json_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => a.as_f64() == b.as_f64(),
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(x, y)| js_json_equal(x, y))
        }
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len()
                && a.iter().all(|(k, v)| b.get(k).is_some_and(|u| js_json_equal(v, u)))
        }
        _ => left == right,
    }
}
