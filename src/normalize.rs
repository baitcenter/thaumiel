/*
 * normalize.rs
 *
 * kant-router - Wikidot-compatible router for web applications
 * Copyright (C) 2019 Ammon Smith
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use regex::Regex;

lazy_static! {
    static ref NON_URL: Regex = Regex::new(r"([^a-z0-9\-]+|-{2,})").unwrap();
    static ref DASHES: Regex = Regex::new(r"(^-+)|(-+)$").unwrap();
}

/// Converts an arbitrary string into Wikidot normalized form.
///
/// This will convert non-alphanumeric characters to dashes and
/// makes it lowercase.
///
/// Examples:
/// * `Big Cheese Horace` -> `big-cheese-horace`
/// * `bottom--Text` -> `bottom-text`
/// * `Tufto's Proposal` -> `tufto-s-proposal`
/// * `-test-` -> `test`
pub fn normalize(name: &mut String) {
    // Lowercase
    name.make_ascii_lowercase();

    // Convert non-URL characters to dashes
    while let Some(mtch) = NON_URL.find(name) {
        let start = mtch.start();
        let end = mtch.end();
        name.replace_range(start..end, "-");
    }

    // Remove leading and trailing dashes
    while let Some(mtch) = DASHES.find(name) {
        let start = mtch.start();
        let end = mtch.end();
        name.replace_range(start..end, "");
    }
}

/// Determines if an arbitrary string is already in Wikidot normalized form.
pub fn is_normal(name: &str) -> bool {
    println!("> {}", name);
    // Is all lowercase
    let lowercase = name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_digit(10) || c == '-');
    if !lowercase {
        return false;
    }

    // No special characters
    if let Some(_) = NON_URL.find(name) {
        return false;
    }

    // Has leading or trailing dashes
    if let Some(_) = DASHES.find(name) {
        return false;
    }

    true
}

#[test]
fn test_normalize() {
    macro_rules! check {
        ($input:expr, $expected:expr) => {{
            let mut text = str!($input);
            normalize(&mut text);
            assert_eq!(text, $expected, "Normalized text doesn't match expected");
        }};
    }

    check!("", "");
    check!("Big Cheese Horace", "big-cheese-horace");
    check!("bottom--Text", "bottom-text");
    check!("Tufto's Proposal", "tufto-s-proposal");
    check!("-test-", "test");
    check!("End of Death Hub", "end-of-death-hub");
    check!("$100 is a lot of money", "100-is-a-lot-of-money");
    check!("snake_case", "snake-case");
    check!("long__snake__case", "long-snake-case");
    check!(" <[ TEST ]> ", "test");
    check!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!", "");
}

#[test]
fn test_is_normal() {
    macro_rules! check {
        ($expected:expr, $input:expr) => {{
            assert_eq!(
                is_normal($input),
                $expected,
                "Normalization test failed: {}",
                $input,
            );
        }};
    }

    check!(true, "");
    check!(true, "big-cheese-horace");
    check!(false, "Big Cheese Horace");
    check!(true, "bottom-text");
    check!(false, "bottom-Text");
    check!(false, "-test-");
    check!(true, "scp-1000");
    check!(true, "end-of-death-hub");
    check!(false, "End of Death Hub");
    check!(false, "$200 please");
    check!(false, "snake_case");
    check!(true, "kebab-case");
    check!(false, "<[ TEST ]>");
    check!(false, " <[ TEST ]> ");
    check!(false, "!!!!!!!!!!!!");
}