// src/components/header.rs
use crate::data::cv::ContactKind;

pub fn contact_icon_modifier(kind: &ContactKind) -> &'static str {
    match kind {
        ContactKind::Phone    => "cv__header__contact--phone",
        ContactKind::Email    => "cv__header__contact--email",
        ContactKind::Location => "cv__header__contact--location",
        ContactKind::Linkedin => "cv__header__contact--linkedin",
        ContactKind::Github   => "cv__header__contact--github",
    }
}
