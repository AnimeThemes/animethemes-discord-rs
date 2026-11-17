use super::schema;

#[derive(cynic::Enum, Clone, Copy, Debug)]
pub enum VideoOverlap {
    None,
    Trans,
    Over,
}

#[derive(cynic::Enum, PartialEq, Clone, Copy, Debug)]
pub enum ImageFacet {
    SmallCover,
    LargeCover,
    Grill,
    Document,
    Avatar,
    Banner,
}
