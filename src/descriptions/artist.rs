use crate::graphql::schema;
use crate::utils;

pub fn artists_description(
    staffs: Vec<&ArtistDescriptionFragmentSongStaff>,
    role: String,
) -> String {
    let mut array: Vec<String> = vec![];

    let mut groups: Vec<i32> = vec![];

    for staff in staffs.iter().filter(|staff| staff.role == role) {
        if staff.member.is_some() {
            if groups.contains(&staff.artist.id) {
                continue;
            }

            groups.push(staff.artist.id);
        }

        let resolved_alias: &str = staff
            .alias
            .as_deref()
            .filter(|alias| !alias.is_empty())
            .unwrap_or(&staff.artist.name.main);

        let final_name = if let Some(r#as) = &staff.r#as
            && !r#as.is_empty()
        {
            format!("{} (CV: {})", r#as, resolved_alias)
        } else {
            resolved_alias.to_string()
        };

        array.push(format!("[{}]({})", final_name, staff.artist.site_url))
    }

    utils::join_with_last_separator(&array, ", ", " & ").to_string()
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "SongStaff")]
pub struct ArtistDescriptionFragmentSongStaff {
    pub alias: Option<String>,
    #[cynic(rename = "as")]
    pub r#as: Option<String>,
    pub role: String,
    pub artist: Artist,
    pub member: Option<Member>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Artist {
    pub id: i32,
    pub name: ArtistName,
    pub site_url: String,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ArtistName {
    pub main: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Artist")]
pub struct Member {
    pub id: i32,
}
