pub trait FileFormat {
    const NAME: &'static str;
    const EXTENSIONS: &'static [&'static str];
}
