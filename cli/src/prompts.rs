#![allow(dead_code)]
use anyhow::{Result, anyhow};
use dialoguer::{FuzzySelect, MultiSelect, theme::ColorfulTheme};
use ghastoolkit::codeql::CodeQLLanguage;

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

pub fn prompt_language<'a>(name: &'a str, items: &'a [CodeQLLanguage]) -> Result<CodeQLLanguage> {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(name)
        .default(0)
        .items(items)
        .interact()?;

    let text = items.get(selection).ok_or(anyhow!("No item selected"))?;

    Ok(text.clone())
}

/// Prompt and select multiple languages
pub fn prompt_languages<'a>(
    name: &'a str,
    items: &'a [CodeQLLanguage],
) -> Result<Vec<CodeQLLanguage>> {
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(name)
        .items(items)
        .interact()?;

    let selected_items: Vec<CodeQLLanguage> = selection
        .iter()
        .filter_map(|&i| items.get(i))
        .cloned()
        .collect();

    Ok(selected_items)
}
