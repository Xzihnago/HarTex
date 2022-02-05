/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
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

//! A base library for caching within HarTex.

#![deny(clippy::pedantic, warnings)]
#![feature(fundamental)]
#![forbid(unsafe_code)]

use std::{
    future::Future,
    pin::Pin,
    sync::Arc
};

use crate::backend::Backend;

pub mod backend;
pub mod entity;
pub mod relations;
pub mod repository;

/// Cache with a variable backend.
pub trait Cache<B: Backend> {
    /// A reference-counted reference to the cache backend.
    fn backend(&self) -> &Arc<B>;
}

/// A future for updating the cache.
pub type UpdateCacheFuture<'a, B> =
    Pin<Box<dyn Future<Output = Result<(), <B as Backend>::Error>> + Send + 'a>>;