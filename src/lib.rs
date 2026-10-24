use serde::Deserialize;
use std::{collections::HashMap, sync::LazyLock};

const CONFIG_JSON: &str = include_str!("./change_the_subject.json");
static CONFIG: LazyLock<Config> = LazyLock::new(|| serde_json::from_str(CONFIG_JSON).unwrap());

#[derive(Deserialize)]
struct Config {
    main_term: HashMap<String, Replacement>,
    subdivision: HashMap<String, Replacement>,
}

#[derive(Deserialize)]
struct Replacement {
    replacement: ReplacementTerm,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ReplacementTerm {
    Single(String),
    Multiple(Vec<String>),
}

impl ReplacementTerm {
    fn as_strs(&self) -> Vec<&str> {
        match self {
            ReplacementTerm::Single(term) => vec![term.as_str()],
            ReplacementTerm::Multiple(terms) => terms.iter().map(String::as_str).collect(),
        }
    }
}

/// Change outdated terminology to preferred terminology
/// Each term in the iterator can contain many terms, delimited by one of the "separators"
pub fn fix<'a>(terms: &[&'a str], separators: &[&'a str]) -> impl Iterator<Item = String> {
    terms
        .iter()
        .filter(|term| !term.is_empty())
        .flat_map(|term| fix_subdivisions(subdivide(term, separators)))
}

fn fix_subdivisions(subdivisions: Subdivisions) -> impl Iterator<Item = String> {
    subdivisions
        .split()
        .iter()
        .enumerate()
        .fold(vec![String::new()], |acc, (index, subdivision)| {
            let terminology = if index == 0 {
                &CONFIG.main_term
            } else {
                &CONFIG.subdivision
            };
            match search(subdivision, terminology) {
                Some(preferred) => preferred
                    .replacement
                    .as_strs()
                    .iter()
                    .flat_map(|replacement| {
                        acc.iter().map(move |previous_terms| {
                            combine_terms(
                                previous_terms,
                                replacement,
                                subdivisions.separator.unwrap_or_default(),
                            )
                        })
                    })
                    .collect(),
                _ => acc
                    .iter()
                    .map(|previous_terms| {
                        combine_terms(
                            previous_terms,
                            subdivision,
                            subdivisions.separator.unwrap_or_default(),
                        )
                    })
                    .collect(),
            }
        })
        .into_iter()
}

struct Subdivisions<'a> {
    term: &'a str,
    separator: Option<&'a str>,
}

impl<'a> Subdivisions<'a> {
    fn split(&self) -> Vec<&'a str> {
        match self.separator {
            Some(separator) => self.term.split(separator).collect(),
            _ => vec![self.term],
        }
    }
}

fn subdivide<'a>(term: &'a str, separators: &'a [&'a str]) -> Subdivisions<'a> {
    let separator = separators.iter().find(|s| term.contains(*s));
    Subdivisions {
        term,
        separator: separator.map(|v| &**v),
    }
}

fn search<'a>(term: &str, terms: &'a HashMap<String, Replacement>) -> Option<&'a Replacement> {
    terms.get(term.trim_end_matches("."))
}

fn combine_terms(previous: &str, current: &str, separator: &str) -> String {
    if previous.is_empty() {
        current.to_string()
    } else {
        format!("{previous}{separator}{current}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_fix(original: &[&str], fixed: &[&str]) {
        assert_eq!(fix(original, &["—"]).collect::<Vec<_>>(), fixed);
    }

    #[test]
    fn it_suggests_a_replacement() {
        assert_fix(&["Illegal aliens"], &["Undocumented immigrants"]);
        assert_fix(&["Illegal immigration"], &["Undocumented immigrants"]);
        assert_fix(
            &["Women illegal aliens"],
            &["Women undocumented immigrants"],
        );
        assert_fix(
            &["Illegal aliens in literature"],
            &["Undocumented immigrants in literature"],
        );
        assert_fix(
            &["Children of illegal aliens"],
            &["Children of undocumented immigrants"],
        );
        assert_fix(
            &["Illegal alien children"],
            &["Undocumented immigrant children"],
        );
        assert_fix(
            &["Illegal immigration in literature"],
            &["Undocumented immigrants in literature"],
        );
        assert_fix(&["Alien criminals"], &["Noncitizen criminals"]);
        assert_fix(&["Aliens"], &["Noncitizens"]);
        assert_fix(&["Aliens in art"], &["Noncitizens in art"]);
        assert_fix(&["Aliens in literature"], &["Noncitizens in literature"]);
        assert_fix(&["Aliens in mass media"], &["Noncitizens in mass media"]);
        assert_fix(
            &["Church work with aliens"],
            &["Church work with noncitizens"],
        );
        assert_fix(
            &["Officials and employees, Alien"],
            &["Officials and employees, Noncitizen"],
        );
        assert_fix(&["Aliens (Greek law)"], &["Noncitizens (Greek law)"]);
        assert_fix(&["Aliens (Roman law)"], &["Noncitizens (Roman law)"]);
        assert_fix(&["Child slaves"], &["Enslaved children"]);
        assert_fix(&["Indian slaves"], &["Enslaved Indigenous peoples"]);
        assert_fix(&["Older slaves"], &["Enslaved older people"]);
        assert_fix(&["Slaves"], &["Enslaved persons"]);
        assert_fix(&["Women slaves"], &["Enslaved women"]);
        assert_fix(&["East Indians"], &["Indians (India)"]);
        assert_fix(
            &["Indians"],
            &["Indigenous peoples of the Western Hemisphere"],
        );
        assert_fix(
            &["Indians of North America"],
            &["Indigenous peoples of North America"],
        );
        assert_fix(
            &["Gender identity disorders in adolescence"],
            &["Gender dysphoria in adolescence"],
        );
        assert_fix(&["Convict labor"], &["Prison labor"]);
        assert_fix(&["Transsexuals"], &["Transsexual people"]);
        assert_fix(
            &["Christian transsexuals"],
            &["Christian transsexual people"],
        );
        assert_fix(
            &["Female-to-male transsexuals"],
            &["Female-to-male transsexual people"],
        );
        assert_fix(&["Jewish transsexuals"], &["Jewish transsexual people"]);
        assert_fix(
            &["Libraries and transsexuals"],
            &["Libraries and transsexual people"],
        );
        assert_fix(
            &["Male-to-female transsexuals"],
            &["Male-to-female transsexual people"],
        );
        assert_fix(&["Older transsexuals"], &["Older transsexual people"]);
        assert_fix(
            &["Parents of transsexuals"],
            &["Parents of transsexual people"],
        );
        assert_fix(
            &["Transsexuals in literature"],
            &["Transsexual people in literature"],
        );
        assert_fix(
            &["Transsexuals in motion pictures"],
            &["Transsexual people in motion pictures"],
        );
        assert_fix(
            &["Transsexuals' writings"],
            &["Transsexual people's writings"],
        );
        assert_fix(
            &["Transsexuals' writings, American"],
            &["Transsexual people's writings, American"],
        );
        assert_fix(
            &["America, Lake (N.Y. and Ont.)"],
            &["Ontario, Lake (N.Y. and Ont.)"],
        );
        assert_fix(
            &["America, Lake, Region (N.Y. and Ont.)"],
            &["Ontario, Lake, Region (N.Y. and Ont.)"],
        );
        assert_fix(
            &["America, Lake, Watershed (N.Y. and Ont.)"],
            &["Ontario, Lake, Watershed (N.Y. and Ont.)"],
        );
    }

    #[test]
    fn it_suggests_a_replacement_for_subdivisions() {
        assert_fix(
            &["Japanese Americans—Evacuation and relocation, 1942-1945"],
            &["Japanese Americans—Forced removal and internment, 1942-1945"],
        );
        assert_fix(
            &["Aleuts—Evacuation and relocation, 1942-1945"],
            &["Aleuts—Forced removal and internment, 1942-1945"],
        );
        assert_fix(
            &["German Americans—Evacuation and relocation, 1942-1945"],
            &["German Americans—Forced removal and internment, 1942-1945"],
        );
        assert_fix(
            &["Italian Americans—Evacuation and relocation, 1942-1945"],
            &["Italian Americans—Forced removal and internment, 1942-1945"],
        );
    }

    #[test]
    fn it_does_not_change_a_term_without_a_configured_replacement() {
        assert_fix(&["Daffodils"], &["Daffodils"]);
    }

    #[test]
    fn it_can_fix_handle_multiple_terms() {
        assert_fix(
            &["Illegal aliens", "Workplace safety"],
            &["Undocumented immigrants", "Workplace safety"],
        );
    }

    #[test]
    fn it_disregards_empty_terms() {
        assert_fix(&["", "", "Workplace safety"], &["Workplace safety"]);
    }

    #[test]
    fn it_can_change_main_term_and_re_assemble_the_full_heading() {
        assert_fix(
            &["Illegal aliens—United States.", "Workplace safety"],
            &["Undocumented immigrants—United States.", "Workplace safety"],
        );
    }

    #[test]
    fn it_can_change_all_configured_subfields_when_original_term_spans_multiple_subfields() {
        assert_fix(
            &[
                "Japanese Americans—Evacuation and relocation, 1942-1945",
                "Music",
            ],
            &[
                "Japanese Americans—Forced removal and internment, 1942-1945",
                "Music",
            ],
        );
        assert_fix(
            &["Banks (Oceanography)—America, Gulf of."],
            &["Banks (Oceanography)—Mexico, Gulf of"],
        );
    }

    #[test]
    fn it_changes_only_configured_subfields_and_keeps_subsequent_subfield() {
        assert_fix(
            &["Japanese Americans—Evacuation and relocation, 1942-1945—Comic books, strips, etc."],
            &[
                "Japanese Americans—Forced removal and internment, 1942-1945—Comic books, strips, etc.",
            ],
        );
    }

    #[test]
    fn it_removes_trailing_period() {
        assert_fix(&["America, Gulf of."], &["Mexico, Gulf of"]);
    }

    #[test]
    fn it_does_not_remove_periods_for_terms_we_are_not_replacing() {
        assert_fix(&["Y.M.C.A."], &["Y.M.C.A."]);
    }

    #[test]
    fn it_can_handle_alternative_separators() {
        assert_eq!(
            fix(
                &[
                    "Indians of North America || Connecticut",
                    "Indians of North America — New York"
                ],
                &[" || ", " — "]
            )
            .collect::<Vec<_>>(),
            vec![
                "Indigenous peoples of North America || Connecticut",
                "Indigenous peoples of North America — New York"
            ]
        )
    }
}
