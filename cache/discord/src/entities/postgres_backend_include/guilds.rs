/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use std::str::FromStr;

use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

impl<'r> FromRow<'r, PgRow> for CachedGuild {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id_string = row.try_get::<String, &str>("id")?;
        let id = Id::new_checked(id_string.parse::<u64>().unwrap()).unwrap();

        Ok(Self {
            id
        })
    }
}
