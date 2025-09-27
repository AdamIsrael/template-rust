// SPDX-License-Identifier: {{ license }}

mod i18n;

fn main() {
    // Get the system's preferred languages.
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();

    // Enable localizations to be applied.
    i18n::init(&requested_languages);

    println!("Hello, world!");
}
