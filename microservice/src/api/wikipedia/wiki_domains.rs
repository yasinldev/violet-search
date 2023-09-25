/*
    This file contains wikipedia api for countries
*/

use std::collections::HashMap;
use crate::exceptions::exceptions::{throw_violet_search_exception, VioletSearchExceptions};

pub struct WikiDomains;

impl WikiDomains {
    pub fn find_domain(country_code: &str) -> Option<&'static str> {
        // Based on https://en.wikipedia.org/wiki/List_of_Wikipedias
        // About country codes: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2

        let wiki_domains: HashMap<&str, &str> = [
            ("EN", "en.wikipedia.org"),
            ("TR", "tr.wikipedia.org"),
            ("FR", "fr.wikipedia.org"),
            ("DE", "de.wikipedia.org"),
            ("RU", "ru.wikipedia.org"),
            ("ES", "es.wikipedia.org"),
            ("IT", "it.wikipedia.org"),
            ("CN", "zh.wikipedia.org"),
            ("KR", "ko.wikipedia.org"),
            ("JP", "ja.wikipedia.org"),
            ("AZ", "az.wikipedia.org"),
            ("EL", "el.wikipedia.org"),
        ]
            .iter()
            .cloned()
            .collect();

        let country = match wiki_domains.get(country_code) {
            Some(country) => country,
            None => {
                match wiki_domains.get("EN") {
                    Some(country) => country,
                    None => throw_violet_search_exception(
                        VioletSearchExceptions::VioletSearchInvalidParameterException(
                            "Error:    Country code is not correct!".to_string()
                        )
                    )
                }
            },
        };

        Some(country)
    }
}
