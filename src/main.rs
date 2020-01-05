/*
 * main.rs
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

#![deny(missing_debug_implementations)]

extern crate actix_files;
extern crate actix_identity;
extern crate actix_rt;
extern crate actix_web;
extern crate bytes;
extern crate color_backtrace;
extern crate futures;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

#[macro_use]
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate str_macro;
extern crate structopt;
extern crate toml;
extern crate wikidot_normalize;
extern crate wikidot_path;

#[cfg(test)]
#[macro_use]
extern crate maplit;

mod config;
mod middleware;
mod request;
mod route;
mod server;

#[cfg(test)]
mod test;

use self::config::Config;
use std::process;

pub type StdResult<T, E> = std::result::Result<T, E>;

#[actix_rt::main]
async fn main() {
    color_backtrace::install();

    let Config {
        hostname,
        http_address,
        keep_alive,
        log_level,
        runtime,
    } = Config::parse_args();

    pretty_env_logger::formatted_builder()
        .filter_level(log_level)
        .init();

    info!("HTTP server starting on {}", http_address);
    let server_result = server::run(hostname, http_address, keep_alive, runtime).await;

    if let Err(error) = server_result {
        error!("Error running actix web server: {}", error);
        process::exit(1);
    }
}
