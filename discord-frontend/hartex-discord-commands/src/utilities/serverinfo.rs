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

use chrono::{LocalResult, TimeZone, Utc};
use hartex_discord_cdn::Cdn;
use hartex_discord_commands_core::traits::Command;
use hartex_discord_commands_core::CommandMetadata;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::model::http::interaction::InteractionResponse;
use hartex_discord_core::discord::model::http::interaction::InteractionResponseType;
use hartex_discord_core::discord::util::builder::embed::EmbedAuthorBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedFieldBuilder;
use hartex_discord_core::discord::util::builder::embed::ImageSource;
use hartex_discord_core::discord::util::builder::InteractionResponseDataBuilder;
use hartex_discord_core::discord::util::snowflake::Snowflake;
use hartex_discord_entitycache_core::traits::Repository;
use hartex_discord_entitycache_repositories::guild::CachedGuildRepository;
use hartex_discord_utils::CLIENT;
use hartex_localization_core::create_bundle;
use hartex_localization_core::handle_errors;
use hartex_localization_macros::bundle_get;
use miette::IntoDiagnostic;

#[derive(CommandMetadata)]
#[metadata(command_type = 1)]
#[metadata(interaction_only = true)]
#[metadata(minimum_level = 0)]
#[metadata(name = "serverinfo")]
pub struct ServerInfo;

impl Command for ServerInfo {
    async fn execute(&self, interaction: Interaction) -> miette::Result<()> {
        let interaction_client = CLIENT.interaction(interaction.application_id);
        let bundle = create_bundle(
            interaction.locale.and_then(|locale| locale.parse().ok()),
            &["discord-frontend", "commands"],
        )?;

        let guild = CachedGuildRepository
            .get(interaction.guild_id.unwrap())
            .await
            .into_diagnostic()?;

        bundle_get!(bundle."serverinfo-embed-name-field-name": message, out [serverinfo_embed_name_field_name, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."serverinfo-embed-id-field-name": message, out [serverinfo_embed_id_field_name, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."serverinfo-embed-creation-timestamp-field-name": message, out [serverinfo_embed_creation_timestamp_field_name, errors]);
        handle_errors(errors)?;

        let mut author = EmbedAuthorBuilder::new(guild.name.clone());
        if guild.icon.is_some() {
            author = author.icon_url(
                ImageSource::url(Cdn::guild_icon(guild.id, guild.icon.unwrap()))
                    .into_diagnostic()?
            );
        }

        let timestamp = Utc.timestamp_millis_opt(guild.id.timestamp());
        let timestamp_str = match timestamp {
            LocalResult::Single(dt) => dt.to_rfc2822(),
            _ => "unknown".to_string(),
        };

        let embed = EmbedBuilder::new()
            .color(0x41_A0_DE)
            .author(author)
            .field(EmbedFieldBuilder::new(
                serverinfo_embed_name_field_name,
                guild.name,
            ).inline())
            .field(EmbedFieldBuilder::new(
                serverinfo_embed_id_field_name,
                guild.id.get().to_string(),
            ).inline())
            .field(EmbedFieldBuilder::new(
                serverinfo_embed_creation_timestamp_field_name,
                timestamp_str,
            ))
            .validate()
            .into_diagnostic()?
            .build();

        interaction_client
            .create_response(
                interaction.id,
                &interaction.token,
                &InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(
                        InteractionResponseDataBuilder::new()
                            .embeds(vec![embed])
                            .build(),
                    ),
                },
            )
            .await
            .into_diagnostic()?;

        Ok(())
    }
}
