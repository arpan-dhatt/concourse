use std::collections::HashMap;

use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use serde::Deserialize;
use serenity::{
    builder::CreateEmbed,
    client::Context,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    },
    utils::Color,
};
use sled;

#[derive(Deserialize, PartialEq)]
struct CourseTime {
    day: Option<String>,
    time: (DateTime<Utc>, DateTime<Utc>),
    location: Option<String>,
}

#[derive(Deserialize)]
struct CourseData {
    code: i64,
    link: Option<String>,
    name: Option<String>,
    times: Vec<CourseTime>,
    instruction_mode: Option<String>,
    instructor: Option<String>,
    status: Option<String>,
    flags: Vec<String>,
}

#[derive(Deserialize)]
struct Courses {
    courses: Vec<CourseData>,
}

lazy_static! {
    static ref USERDB: sled::Db =
        { sled::open(std::env::var("USERDB").unwrap_or("./user.db".to_string())).unwrap() };
    static ref COURSEDATA: HashMap<i64, CourseData> = {
        let courses: Courses = serde_json::from_slice(
            &std::fs::read(std::env::var("COURSEDATA").unwrap_or("./courses.json".to_string()))
                .unwrap(),
        )
        .unwrap();
        let mut map = HashMap::new();
        for course in courses.courses {
            map.insert(course.code, course);
        }
        map
    };
}

pub fn ccupdate<'a>(
    embed: &'a mut CreateEmbed,
    command: &ApplicationCommandInteraction,
) -> &'a mut CreateEmbed {
    let options = command
        .data
        .options
        .get(0)
        .expect("Expected codes option")
        .resolved
        .as_ref()
        .expect("Expected string value");
    if let ApplicationCommandInteractionDataOptionValue::String(codes) = options {
        let codes: Vec<i64> = codes
            .split(",")
            .map(|s| s.trim())
            .filter_map(|s| s.parse().ok())
            .collect();
        USERDB
            .insert(
                command.user.id.as_u64().to_be_bytes(),
                serde_json::to_vec(&codes[..(10.min(codes.len()))]).unwrap(),
            )
            .unwrap();
        embed
            .title("Success")
            .description("Use the `ccuser` command to see your schedule.")
            .color(Color::from_rgb(0, 255, 0));
        return embed;
    }
    unknown_command(embed, command)
}

pub fn ccuser<'a>(
    embed: &'a mut CreateEmbed,
    command: &ApplicationCommandInteraction,
) -> &'a mut CreateEmbed {
    let options = command
        .data
        .options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user value");
    let issuer_course_codes: Vec<i64> = match USERDB.get(command.user.id.as_u64().to_be_bytes()) {
        Ok(Some(ivec)) => serde_json::from_slice(&ivec).unwrap_or(vec![]),
        _ => vec![],
    };
    let issuer_courses: Vec<&CourseData> = issuer_course_codes
        .iter()
        .filter_map(|c| COURSEDATA.get(c))
        .collect();
    let mut issuer_times = vec![];
    for course in &issuer_courses {
        for time in &course.times {
            issuer_times.push(time);
        }
    }
    if let ApplicationCommandInteractionDataOptionValue::User(user, Some(member)) = options {
        if let Ok(Some(courses_bytes)) = USERDB.get(user.id.as_u64().to_be_bytes()) {
            let target_courses: Vec<i64> = serde_json::from_slice(&courses_bytes).unwrap_or(vec![]);
            let target_courses: Vec<&CourseData> = target_courses
                .iter()
                .filter_map(|c| COURSEDATA.get(c))
                .collect();
            embed
                .title(member.nick.as_ref().unwrap_or(&user.name))
                .color(Color::from_rgb(0, 255, 0));
            for course in target_courses {
                let course_code_matches = issuer_course_codes.contains(&course.code);
                let key = match course_code_matches && command.user.id != user.id {
                    true => format!(
                        "__**{}: {}**__",
                        course.code,
                        course
                            .name
                            .as_ref()
                            .unwrap_or(&String::from("Unknown Name"))
                    ),
                    false => format!(
                        "**{}: {}**",
                        course.code,
                        course
                            .name
                            .as_ref()
                            .unwrap_or(&String::from("Unknown Name"))
                    ),
                };
                let values: Vec<String> = course
                    .times
                    .iter()
                    .map(|t| {
                        match issuer_times.contains(&t)
                            && command.user.id != user.id
                            && !course_code_matches
                        {
                            true => format!(
                                "__{} | {}-{} | {}__",
                                t.day.as_ref().unwrap_or(&String::from("-")),
                                t.time.0.format("%I:%M %p"),
                                t.time.1.format("%I:%M %p"),
                                t.location.as_ref().unwrap_or(&String::from("-"))
                            ),
                            false => format!(
                                "{} | {}-{} | {}",
                                t.day.as_ref().unwrap_or(&String::from("-")),
                                t.time.0.format("%I:%M %p"),
                                t.time.1.format("%I:%M %p"),
                                t.location.as_ref().unwrap_or(&String::from("-"))
                            ),
                        }
                    })
                    .collect();
                let value = values.join("\n");
                if values.len() > 0 {
                    embed.field(key, value, false);
                } else {
                    embed.field(key, "No times", false);
                }
                embed.footer(|footer| {
                    footer.text("Classes or locations common to you are underlined.")
                });
            }
            return embed;
        } else {
            embed
                .title(member.nick.as_ref().unwrap_or(&user.name))
                .description("No data available. This user doesn't have any course codes stored.")
                .color(Color::from_rgb(255, 85, 0));
            return embed;
        }
    }
    unknown_command(embed, command)
}

fn get_users_in_location(course_time: &CourseTime) -> Vec<u64> {
    let mut out = vec![];
    for (user_id_bytes, course_codes_bytes) in USERDB.iter().filter_map(|d| d.ok()) {
        let mut buf = [0u8; 8];
        for i in 0..8 {
            buf[i] = *(user_id_bytes.get(i).unwrap_or(&0));
        }
        let user_id = u64::from_be_bytes(buf);
        let course_codes: Vec<i64> = serde_json::from_slice(&course_codes_bytes).unwrap_or(vec![]);
        let courses: Vec<&CourseData> = course_codes
            .iter()
            .filter_map(|c| COURSEDATA.get(c))
            .collect();
        let mut course_times = vec![];
        for course in courses {
            for time in &course.times {
                course_times.push(time);
            }
        }
        if course_times.contains(&course_time) {
            out.push(user_id);
        }
    }
    out
}

pub fn cclookup<'a>(
    embed: &'a mut CreateEmbed,
    command: &ApplicationCommandInteraction,
) -> &'a mut CreateEmbed {
    let options = command
        .data
        .options
        .get(0)
        .expect("Expected code option")
        .resolved
        .as_ref()
        .expect("Expected integer value");
    if let ApplicationCommandInteractionDataOptionValue::Integer(code) = options {
        if let Some(course) = COURSEDATA.get(code) {
            embed
                .title(course.code)
                .description(format!(
                    "{}",
                    course
                        .name
                        .as_ref()
                        .unwrap_or(&String::from("Unknown Name"))
                ))
                .color(Color::from_rgb(0, 255, 0));
            for time in &course.times {
                let users_here: Vec<String> = get_users_in_location(time)
                    .iter()
                    .map(|c| format!("<@{}>", c))
                    .collect();
                embed.field(
                    format!(
                        "{} | {}-{} | {}",
                        time.day.as_ref().unwrap_or(&String::from("-")),
                        time.time.0.format("%I:%M %p"),
                        time.time.1.format("%I:%M %p"),
                        time.location.as_ref().unwrap_or(&String::from("-"))
                    ),
                    match users_here.len() > 0 {
                        true => users_here.join(""),
                        false => "No students found".to_string(),
                    },
                    false,
                );
            }
            return embed;
        }
    }
    unknown_command(embed, command)
}

pub async fn ccfind(command: ApplicationCommandInteraction, ctx: Context) -> serenity::Result<()> {
    command
        .create_interaction_response(ctx.http, |response| {
            response
                .kind(serenity::model::interactions::InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    if let Ok(Some(courses_bytes)) = USERDB.get(command.user.id.as_u64().to_be_bytes()) {
                        let courses: Vec<i64> = serde_json::from_slice(&courses_bytes).unwrap_or(vec![]);
                        if courses.len() == 0 {
                            message.create_embed(|embed| {
                                embed.title("Insufficient Information")
                                    .description("Make sure you've entered your data into the system. Otherwise this command does not work. Check `/cchelp` for more information.")
                                    .color(Color::from_rgb(255, 0, 0))
                            })
                        }
                        else {
                            for code in courses {
                                message.create_embed(|embed| {
                                    if let Some(course) = COURSEDATA.get(&code) {
                                        embed
                                            .title(course.code)
                                            .description(format!(
                                                "{}",
                                                course
                                                    .name
                                                    .as_ref()
                                                    .unwrap_or(&String::from("Unknown Name"))
                                            ))
                                            .color(Color::from_rgb(0, 255, 0));
                                        for time in &course.times {
                                            let users_here: Vec<String> = get_users_in_location(time)
                                                .iter()
                                                .map(|c| format!("<@{}>", c))
                                                .collect();
                                            embed.field(
                                                format!(
                                                    "{} | {}-{} | {}",
                                                    time.day.as_ref().unwrap_or(&String::from("-")),
                                                    time.time.0.format("%I:%M %p"),
                                                    time.time.1.format("%I:%M %p"),
                                                    time.location.as_ref().unwrap_or(&String::from("-"))
                                                ),
                                                match users_here.len() > 0 {
                                                    true => users_here.join(""),
                                                    false => "No students found".to_string(),
                                                },
                                                false,
                                            );
                                        }
                                        embed
                                        } else {
                                            embed.title(code)
                                                .description("This code is not found in the database. Make sure it's a valid unique class code. If it is, then report this to the developer. (check bot's about).")
                                        }
                                });
                            }
                            message
                        }
                    }
                    else {
                        message.create_embed(|embed| {
                                embed.title("Insufficient Information")
                                    .description("Make sure you've entered your data into the system. Otherwise this command does not work. Check `/cchelp` for more information.")
                                    .color(Color::from_rgb(255, 0, 0))
                            })
                    }
                })
        })
        .await
}

pub fn ccdelete<'a>(
    embed: &'a mut CreateEmbed,
    command: &ApplicationCommandInteraction,
) -> &'a mut CreateEmbed {
    if let Ok(Some(_)) = USERDB.remove(&command.user.id.as_u64().to_be_bytes()) {
        embed
            .title("Success")
            .description("Your data has been successfully removed.")
            .color(Color::from_rgb(0, 255, 0));
        return embed;
    } else {
        embed
            .title("Failure")
            .description("No data was removed since yours wasn't found. It's already deleted or was never there.")
            .color(Color::from_rgb(255, 85, 0));
        return embed;
    }
}

pub fn cchelp<'a>(
    embed: &'a mut CreateEmbed,
    _command: &ApplicationCommandInteraction,
) -> &'a mut CreateEmbed {
    embed
        .title("Concourse Help Page")
        .color(Color::from_rgb(0,255,0))
        .description("Concourse is a bot built for UT that is meant to replace sending pictures of your schedule. It allows you to input your unique course codes and compare them to other students. You can also lookup unique course codes to see who is in the classes. This bot can show if you have lectures with other students, even if unique course codes are different (multiple unique codes usually share lectures).\nCommands:")
        .field("`/ccupdate`", "Get started by using this command. Use comma-separated course codes, like this `/ccupdate codes:12349,56789,98765`.", false)
        .field("`/ccuser`", "If this user has entered their courses already, you can see them and the times/locations, if available for the course. If you've entered your courses already using `/ccupdate` it will underline similarities.", false)
        .field("`/ccfind`", "Lists all your classes you're attending by their location, and every student in that class.", false)
        .field("`/cclookup`", "Lookup a certain class code to see if anyone is taking it (async classes won't show people for now). This will list the course's times and if anyone who has entered the codes they will be listed.", false)
        .field("`/ccdelete`", "Deletes your course codes from the bot's database, in case you don't want them there at any point.", false)
}

pub fn unknown_command<'a>(
    embed: &'a mut CreateEmbed,
    _command: &ApplicationCommandInteraction,
) -> &'a mut CreateEmbed {
    embed
        .title("Incorrect Command Usage")
        .description("Use one of 5 commands: `ccupdate`, `ccuser`, `cclookup`, `ccfind`, `ccdelete`, `cchelp`, and make sure your input values are valid.")
        .color(Color::from_rgb(255, 0, 0))
}
