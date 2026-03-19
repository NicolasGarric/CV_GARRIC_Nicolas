// src/components/experience.rs
use crate::data::cv::IconKind;

pub fn icon_modifier(kind: &IconKind) -> &'static str {
    match kind {
        IconKind::Web   => "cv__experience__item__icon--web",
        IconKind::Train => "cv__experience__item__icon--train",
        IconKind::Film  => "cv__experience__item__icon--film",
        IconKind::Post  => "cv__experience__item__icon--post",
        IconKind::Drone => "cv__experience__item__icon--drone",
    }
}
