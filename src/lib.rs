use zed_extension_api as zed;

struct ElsaExtension;

impl zed::Extension for ElsaExtension {
    fn new() -> Self {
        ElsaExtension
    }
}

zed::register_extension!(ElsaExtension);
