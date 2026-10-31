//! The product an embedding app presents WordCraft's front end as: its name in window titles and
//! the About box, its version and its About text. WordCraft's own by default; an app built on
//! WordCraft sets its own once at start-up ([`set`]), before the first window opens.

use std::sync::OnceLock;

/// What the front end calls the product.
#[derive(Clone, Copy, Debug)]
pub struct Brand {
    /// The product name: window titles, the About box, the unsaved-changes prompt.
    pub name: &'static str,
    /// The version the About box shows.
    pub version: &'static str,
    /// The About box's description, in place of WordCraft's (shown as given, not translated).
    pub about: Option<&'static str>,
}

impl Default for Brand {
    fn default() -> Brand {
        Brand { name: "WordCraft", version: env!("CARGO_PKG_VERSION"), about: None }
    }
}

static BRAND: OnceLock<Brand> = OnceLock::new();
static ABOUT_TITLE: OnceLock<String> = OnceLock::new();

/// Present the front end as `brand`. Only the first call counts; later ones return `false`.
pub fn set(brand: Brand) -> bool {
    BRAND.set(brand).is_ok()
}

/// The product as set, or WordCraft.
pub fn get() -> Brand {
    BRAND.get().copied().unwrap_or_default()
}

/// The product name.
pub fn name() -> &'static str {
    get().name
}

/// "About <name>", the About box's title.
pub(crate) fn about_title() -> &'static str {
    ABOUT_TITLE.get_or_init(|| format!("About {}", name())).as_str()
}

#[cfg(test)]
mod tests {
    #[test]
    fn wordcraft_unless_set() {
        // The brand is process-wide; tests never set it, so WordCraft's is what they see.
        assert_eq!(super::name(), "WordCraft");
        assert_eq!(super::about_title(), "About WordCraft");
        assert_eq!(super::get().version, env!("CARGO_PKG_VERSION"));
    }
}
