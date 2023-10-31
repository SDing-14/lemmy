fn main() -> Result<(), Box<dyn std::error::Error>> {
  rosetta_build::config()
    .source("en", "../../../lemmy-ui/lemmy-translations/email/en.json")
    .source("fi", "../../../lemmy-ui/lemmy-translations/email/fi.json")
    .source("ko", "../../../lemmy-ui/lemmy-translations/email/ko.json")
    .source("pt", "../../../lemmy-ui/lemmy-translations/email/pt.json")
    .fallback("en")
    .generate()?;

  Ok(())
}
