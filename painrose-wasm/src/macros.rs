macro_rules! element {
    ($namespace:expr, $document:ident, $name:expr, {$($attribute:expr => $value:expr),* $(,)?}) => {{
        let elem = $document.create_element_ns($namespace, $name)?;
        $(
            elem.set_attribute($attribute, $value)?;
        )+
        elem
    }};
}

macro_rules! html_element {
    ($document:ident, $name:expr, {$($attribute:expr => $value:expr),* $(,)?}) =>
    { element!(None, $document, $name, {$($attribute => $value),*}) }
}

macro_rules! svg_element {
    ($document:ident, $name:expr, {$($attribute:expr => $value:expr),* $(,)?}) =>
    { element!(Some(crate::SVG_NAMESPACE), $document, $name, {$($attribute => $value),*}) }
}
