extern crate rocket_contrib;

use self::rocket_contrib::Json;

#[derive(Serialize)]
struct SettingsField {
  name: String,
  alias: String,
  field_type: String,
  value: String
}

#[derive(Serialize)]
struct Settings {
  site_fields: Vec<SettingsField>,
  other_fields: Vec<SettingsField>
}

#[get("/settings")]
fn settings() -> Json<Settings> {
  // TODO: Get settings from DB

  let title_field = SettingsField {
    name: String::from("title"),
    alias: String::from("Site title"),
    field_type: String::from("text"),
    value: String::from("Rustfolio test")
  };

  let favicon_field = SettingsField {
    name: String::from("favicon"),
    alias: String::from("Favicon"),
    field_type: String::from("upload"),
    value: String::from("some picture")
  };

  let owner_field = SettingsField {
    name: String::from("owner"),
    alias: String::from("Owner display name"),
    field_type: String::from("text"),
    value: String::from("Jaka S.")
  };
  
  let mut site_fields = Vec::new();
  site_fields.push(title_field);
  site_fields.push(favicon_field);

  let mut other_fields = Vec::new();
  other_fields.push(owner_field);

  let settings = Settings {
    site_fields: site_fields,
    other_fields: other_fields
  };

  Json(settings)
}
