use std::collections::HashMap;
use serenity::{
    model::{prelude::{interaction::application_command::{CommandDataOption, CommandDataOptionValue}, command::CommandOptionType}, user::User}
};
use anyhow::{Result, anyhow};

pub struct ParsedOptions<'a> {
    parsed: HashMap<&'a str, &'a CommandDataOption>,
}

impl<'a> ParsedOptions<'a> {
    pub fn parse(options: &'a Vec<CommandDataOption>) -> Self {
        let mut parsed = HashMap::new();

        for option in options {
            parsed.insert(option.name.as_str(), option);
        }

        Self {
            parsed: parsed,
        }
    }

    fn get_value(&self, key: &str) -> Result<&CommandDataOptionValue> {
        let value = self.get(key)?;

        if let Some(value) = &value.resolved {
            return Ok(value)
        } else {
            Err(anyhow!("Command option resolved data was empty"))
        }
    }

    fn get(&self, key: &str) -> Result<&CommandDataOption> {
        if let Some(value) = self.parsed.get(key) {
            Ok(value)
        } else {
            Err(anyhow!("Expected command option with name '{}'", key))
        }
    }

    pub fn get_user(&self, key: &str) -> Result<&User> {
        let value = self.get_value(key)?;

        if let CommandDataOptionValue::User(user, _) = value {
            Ok(user)
        } else {
            Err(anyhow!("Expected command option '{}' to have type User", key))
        }
    }

    pub fn get_boolean(&self, key: &str) -> Result<bool> {
        let value = self.get_value(key)?;

        if let CommandDataOptionValue::Boolean(b) = value {
            Ok(*b)
        } else {
            Err(anyhow!("Expected command option '{}' to have type Boolean", key))
        }
    }

    pub fn get_integer(&self, key: &str) -> Result<i64> {
        let value = self.get_value(key)?;

        if let CommandDataOptionValue::Integer(n) = value {
            Ok(*n)
        } else {
            Err(anyhow!("Expected command option '{}' to have type Integer", key))
        }
    }

    pub fn get_string(&self, key: &str) -> Result<&str> {
        let value = self.get_value(key)?;

        if let CommandDataOptionValue::String(s) = value {
            Ok(s)
        } else {
            Err(anyhow!("Expected command option '{}' to have type Integer", key))
        }
    }

    pub fn get_sub_command(&self) -> Result<&CommandDataOption> {
        for (_, option) in self.parsed.iter() {
            if option.kind == CommandOptionType::SubCommand {
                return Ok(option);
            }
        }

        Err(anyhow!("Expected at least one option with 'option.kind == CommandOptionType::SubCommand'"))
    }
}