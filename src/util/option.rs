use std::collections::HashMap;
use serenity::{
    model::{prelude::{interaction::application_command::{CommandDataOption, CommandDataOptionValue}}, user::User}
};
use anyhow::{Result, anyhow};

pub struct ParsedOptions<'a> {
    parsed: HashMap<&'a str, &'a Option<CommandDataOptionValue>>,
}

impl<'a> ParsedOptions<'a> {
    pub fn parse(options: &'a Vec<CommandDataOption>) -> Self {
        let mut parsed = HashMap::new();

        for option in options {
            parsed.insert(option.name.as_str(), &option.resolved);
        }

        Self {
            parsed: parsed,
        }
    }

    fn get(&self, key: &str) -> Result<&CommandDataOptionValue> {
        if let Some(value) = self.parsed.get(key) {
            if let Some(value) = value {
                return Ok(value)
            } else {
                Err(anyhow!("Command option resolved data was empty"))
            }
        } else {
            Err(anyhow!("Expected command option with name '{}'", key))
        }
    }

    pub fn get_user(&self, key: &str) -> Result<&User> {
        let value = self.get(key)?;

        if let CommandDataOptionValue::User(user, _) = value {
            Ok(user)
        } else {
            Err(anyhow!("Expected command option '{}' to have type User", key))
        }
    }

    pub fn get_boolean(&self, key: &str) -> Result<bool> {
        let value = self.get(key)?;

        if let CommandDataOptionValue::Boolean(b) = value {
            Ok(*b)
        } else {
            Err(anyhow!("Expected command option '{}' to have type Boolean", key))
        }
    }

    pub fn get_integer(&self, key: &str) -> Result<i64> {
        let value = self.get(key)?;

        if let CommandDataOptionValue::Integer(n) = value {
            Ok(*n)
        } else {
            Err(anyhow!("Expected command option '{}' to have type Integer", key))
        }
    }
}