use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

pub fn prompt_text(name: &str) -> Result<String> {
    let text = dialoguer::Input::<String>::new()
        .with_prompt(name)
        .interact_text()?;

    Ok(text)
}

pub fn prompt_select<'a>(name: &'a str, items: &[&'a str]) -> Result<&'a str> {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(name)
        .default(0)
        .items(items)
        .interact()?;

    let text = items.get(selection).ok_or(anyhow!("No item selected"))?;

    Ok(text)
}
