use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use serde::Deserialize;
use serenity::{
    builder::CreateEmbed,
    model::interactions::application_command::{
        ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
    },
    utils::Color,
};
use sled;

#[derive(Deserialize)]
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
    static ref COURSEDATA: Courses = {
        serde_json::from_slice(
            &std::fs::read(std::env::var("COURSEDATA").unwrap_or("./courses.json".to_string()))
                .unwrap(),
        )
        .unwrap()
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
    if let ApplicationCommandInteractionDataOptionValue::User(user, Some(member)) = options {
        if let Ok(Some(courses_bytes)) = USERDB.get(user.id.as_u64().to_be_bytes()) {
            let courses: Vec<i64> = serde_json::from_slice(&courses_bytes).unwrap_or(vec![]);
            let courses: Vec<&CourseData> = COURSEDATA
                .courses
                .iter()
                .filter(|c| courses.contains(&c.code))
                .collect();
            embed
                .title(member.nick.as_ref().unwrap_or(&user.name))
                .color(Color::from_rgb(0, 255, 0));
            for course in courses {
                let key = format!(
                    "**{}: {}**",
                    course.code,
                    course
                        .name
                        .as_ref()
                        .unwrap_or(&String::from("Unknown Name"))
                );
                let values: Vec<String> = course
                    .times
                    .iter()
                    .map(|t| {
                        format!(
                            "{} | {}-{} | {}",
                            t.day.as_ref().unwrap_or(&String::new()),
                            t.time.0.format("%I:%M %p"),
                            t.time.1.format("%I:%M %p"),
                            t.location.as_ref().unwrap_or(&String::new())
                        )
                    })
                    .collect();
                let value = values.join("\n");
                embed.field(key, value, false);
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
        if let Some(course) = COURSEDATA.courses.iter().filter(|c| c.code == *code).next() {
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
                embed.field(
                    format!(
                        "{} | {}-{} | {}",
                        time.day.as_ref().unwrap_or(&String::new()),
                        time.time.0.format("%I:%M %p"),
                        time.time.1.format("%I:%M %p"),
                        time.location.as_ref().unwrap_or(&String::new())
                    ),
                    "...".to_string(),
                    false,
                );
            }
            return embed;
        }
    }
    unknown_command(embed, command)
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

pub fn unknown_command<'a>(
    embed: &'a mut CreateEmbed,
    _command: &ApplicationCommandInteraction,
) -> &'a mut CreateEmbed {
    embed
        .title("Incorrect Command Usage")
        .description("Use one of 4 commands: `ccupdate`, `ccuser`, `cclookup`, `ccdelete`.")
        .color(Color::from_rgb(255, 0, 0))
}
