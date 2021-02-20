use std::{env, fs, io};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use unic_langid::{langid, LanguageIdentifier};
use std::str::FromStr;
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
use fluent_bundle::{FluentBundle, FluentResource};

static L10N_RESOURCES: &[&str] = &["simple.ftl"];

fn read_file(path: &Path) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

/// It is expected that every directory inside it
/// has a name that is a valid BCP47 language tag.
fn get_available_locales() -> Result<Vec<LanguageIdentifier>, io::Error> {
    let mut locales = vec![];

    let mut dir = env::current_dir()?;
    dir.push("resources");
    let res_dir = fs::read_dir(dir)?;
    for entry in res_dir {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    if let Some(name) = name.to_str() {
                        let langid = name.parse().expect("Parsing failed.");
                        locales.push(langid);
                    }
                }
            }
        }
    }
    return Ok(locales);
}

pub fn locales(lang: &str) {
    let default_locale = langid!("zh-CN");

    let mut requested = vec![];
    let identifier = LanguageIdentifier::from_str(lang).expect("Parsing locale failed.");
    requested.push(identifier);

    let available = get_available_locales().expect("Retrieving available locales failed.");
    let resolved_locales = negotiate_languages(
        &requested,
        &available,
        Some(&default_locale),
        NegotiationStrategy::Filtering,
    );

    let current_locale = resolved_locales
        .get(0)
        .cloned()
        .expect("At least one locale should match.");

    let mut bundle = FluentBundle::new(resolved_locales.into_iter().cloned().collect());

    for path in L10N_RESOURCES {
        let mut full_path = env::current_dir().expect("Failed to retireve current dir.");
        full_path.push("resources");
        full_path.push(current_locale.to_string());
        full_path.push(path);
        let source = read_file(&full_path).expect("Failed to read file.");
        let resource = FluentResource::try_new(source).expect("Could not parse an FTL string.");
        bundle
            .add_resource(resource)
            .expect("Failed to add FTL resources to the bundle.");
    }

    let mut errors = vec![];
    let msg = bundle
        .get_message("missing-arg-error")
        .expect("Message doesn't exist.");
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(&pattern, None, &mut errors);
    println!("{}", value);
}


#[cfg(test)]
mod test {
    use crate::locales_helper::locales;

    #[test]
    fn should_get_git_tag() {
        locales("zh-CN");
    }
}
