/*
 * route/api/auth/session.rs
 *
 * thaumiel - Wikidot-like web server to provide pages, forums, and other services
 * Copyright (C) 2019-2020 Ammon Smith
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

use super::prelude::*;
use crate::session::CookieSession;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct LoginInput {
    username_or_email: String,
    password: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct LoginOutput {
    user_id: UserId,
    success: bool,
}

pub async fn api_login(
    req: HttpRequest,
    id: Identity,
    arg: web::Json<LoginInput>,
    deepwell: web::Data<DeepwellPool>,
) -> HttpResponse {
    info!("API v0 /auth/login");

    let LoginInput {
        username_or_email,
        password,
    } = &*arg;

    let address = req.connection_info().remote().map(String::from);
    debug!(
        "Trying to log in as '{}' from '{}'",
        username_or_email,
        match &address {
            Some(ref addr) => addr,
            None => "<unkown>",
        },
    );

    let result = deepwell
        .claim()
        .await
        .login(username_or_email.clone(), password.clone(), address)
        .await;

    match try_io!(result) {
        Ok(session) => {
            info!("Login succeeded, beginning session");

            let cookie = CookieSession {
                session_id: session.session_id(),
                user_id: session.user_id(),
            };

            match cookie.serialize() {
                Ok(data) => id.remember(data),
                Err(resp) => return resp,
            }

            let result = LoginOutput {
                user_id: session.user_id(),
                success: true,
            };

            HttpResponse::Ok().json(Success::from(result))
        }
        Err(error) => {
            warn!("Failed login attempt");

            HttpResponse::Unauthorized().json(error)
        }
    }
}

#[derive(Serialize, Debug)]
pub struct LogoutOutput {
    logged_out: UserId,
    success: bool,
}

pub async fn api_logout(id: Identity, deepwell: web::Data<DeepwellPool>) -> HttpResponse {
    info!("API v0 /auth/logout");

    match id.identity() {
        Some(ref data) => {
            let CookieSession {
                session_id,
                user_id,
            } = match CookieSession::read(data) {
                Ok(cookie) => cookie,
                Err(resp) => return resp,
            };

            debug!("Logging out user ID {} (session {})", user_id, session_id);

            let result = deepwell //
                .claim()
                .await
                .logout(session_id, user_id)
                .await;
            if let Err(error) = try_io!(result) {
                debug!("Failed to end session: {}", error);

                return HttpResponse::InternalServerError().json(error);
            }

            id.forget();

            let result = LogoutOutput {
                logged_out: user_id,
                success: true,
            };

            HttpResponse::Ok().json(Success::from(result))
        }
        None => {
            debug!("Cannot logout, no session cookie");

            HttpResponse::Unauthorized().json(Error::NotLoggedIn.to_sendable())
        }
    }
}
