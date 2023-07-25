/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

/// # Limitables
///
/// A limitable is a route that can be ratelimited.
use std::num::NonZeroU32;

use governor::Quota;
use rocket::http::Method;

/// Trait for ratelimiters that can be ratelimited to implement.
pub trait Limitable<'r> {
    /// Evaluate the limit for a specific route.
    #[must_use]
    fn evaluate_limit(method: Method, route: &str) -> Quota;

    /// Create a nonzero u32 for quota.
    #[must_use]
    fn non_zero(i: u32) -> NonZeroU32 {
        NonZeroU32::new(i).unwrap_or(NonZeroU32::new(1).unwrap())
    }
}
