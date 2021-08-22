mod handlers;

use std::env;

use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        interactions::{
            application_command::{
                ApplicationCommand,
                ApplicationCommandOptionType,
            },
            Interaction, InteractionResponseType,
        },
    },
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if let Err(why) = match command.data.name.as_str() {
                "ccfind" => handlers::ccfind(command, ctx).await,
                _ => command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.create_embed(|embed| {
                                match command.data.name.as_str() {
                                    "ccupdate" => handlers::ccupdate(embed, &command),
                                    "ccuser" => handlers::ccuser(embed, &command),
                                    "cclookup" => handlers::cclookup(embed, &command),
                                    "ccdelete" => handlers::ccdelete(embed, &command),
                                    "cchelp" => handlers::cchelp(embed, &command),
                                    _ => handlers::unknown_command(embed, &command)
                                }
                            })
                        })
                })
                .await
            }
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let commands = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command
                        .name("ccupdate")
                        .description("Update your courses")
                        .create_option(|option| {
                            option
                                .name("codes")
                                .description("Comma separated unique course codes")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                })
                .create_application_command(|command| {
                    command
                        .name("ccuser")
                        .description("Compare your courses to a user")
                        .create_option(|option| {
                            option
                                .name("user")
                                .description("User to compare against")
                                .kind(ApplicationCommandOptionType::User)
                                .required(true)
                        })
                })
                .create_application_command(|command| {
                    command
                        .name("cclookup")
                        .description("Get a course's times and attendees")
                        .create_option(|option| {
                            option
                                .name("code")
                                .description("Valid course code")
                                .kind(ApplicationCommandOptionType::Integer)
                                .required(true)
                        })
                })
                .create_application_command(|command| {
                    command
                        .name("ccfind")
                        .description("Find at students in all your classes")
                })
                .create_application_command(|command| {
                    command
                        .name("ccdelete")
                        .description("Delete your course information")
                })
                .create_application_command(|command| {
                    command
                        .name("cchelp")
                        .description("Learn about the bot and its commands")
                })
        })
        .await;

        println!(
            "I now have the following global slash commands: {:#?}",
            commands
        );
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // The Application Id is usually the Bot User Id.
    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("application id is not a valid id");

    // Build our client.
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
