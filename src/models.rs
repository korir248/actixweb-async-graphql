use async_graphql::{ComplexObject, SimpleObject};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    #[graphql(name = "_id")]
    pub user_id: i32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Email,
    #[graphql(name = "type")]
    pub user_type: String,
    // pub is_verified: bool,
    pub verification_code: Option<String>,
    pub bio: Option<String>,
    pub blocked: bool,
    pub portfolio: Option<String>,
    pub website: Option<String>,
    pub address: Option<String>,
    pub location: Option<String>,
    #[graphql(name = "cover_pic")]
    pub cover_pic: Option<String>,
    paid_until: Option<String>,
    last_seen: Option<String>,
    login_type: Option<String>,
    reputation: Option<String>,
    gender: Option<String>,
    #[graphql(name = "displayName")]
    display_name: Option<String>,
    #[graphql(name = "profile_pic")]
    profile_pic: Option<String>,
    date: String,
}

#[derive(SimpleObject)]
pub struct Education {
    #[graphql(name = "_id")]
    id: String,
    institution: String,
    major: String,
    #[graphql(name = "start_date")]
    start_date: String,
    #[graphql(name = "end_date")]
    end_date: String,
    current: bool,
    description: String,
}

// impl From<backend::models::User> for User {
//     fn from(user: backend::models::User) -> Self {
//         Self {
//             user_id: user.user_id,
//             username: user.username,
//             password: user.password,
//             email: Email {
//                 address: user.email,
//                 verified: user.is_verified,
//             },
//             user_type: user.user_type,
//             verification_code: user.verification_code,
//             bio: None,
//             blocked: false,
//             portfolio: None,
//             website: None,
//             address: None,
//             location: None,
//             cover_pic: None,
//             display_name: None,
//             gender: None,
//             reputation: None,
//             login_type: None,
//             last_seen: None,
//             paid_until: None,
//             profile_pic: None,
//             date: "today".to_string(),
//         }
//     }
// }

#[ComplexObject]
impl User {
    async fn education(&self) -> Vec<Education> {
        // backend::
        vec![Education {
            id: "345".to_string(),
            institution: "A school".to_string(),
            major: "Eating".to_string(),
            start_date: "last week".to_string(),
            end_date: "today".to_string(),
            current: false,
            description: "school".to_string(),
        }]
    }
    async fn socials(&self) -> Vec<Socials> {
        vec![]
    }
    #[graphql(name = "bnTokens")]
    async fn bn_tokens(&self) -> BnTokens {
        BnTokens {
            wallet_address: None,
            earned: None,
            received: None,
        }
    }
    async fn work(&self) -> Vec<ProfesionalExp> {
        vec![ProfesionalExp {
            id: Some(1),
            company: Some("Artsam Tech".to_string()),
            title: Some("Software dev".to_owned()),
            start_date: Some("yesterday".to_string()),
            end_date: None,
            current: Some(true),
            description: None,
        }]
    }

    async fn honors(&self) -> Vec<Honors> {
        vec![]
    }
    async fn courses(&self) -> Vec<Courses> {
        vec![]
    }
    async fn projects(&self) -> Vec<Projects> {
        vec![]
    }
    async fn skills(&self) -> Vec<Honors> {
        vec![]
    }
    async fn languages(&self) -> Vec<Honors> {
        vec![]
    }
}
#[derive(SimpleObject)]
pub struct Socials {
    social: OSocial,
    profile: Option<String>,
}

#[derive(SimpleObject)]
pub struct OSocial {
    #[graphql(name = "_id")]
    id: Option<String>,
    name: Option<String>,
    image: Option<String>,
}

#[derive(SimpleObject)]
pub struct BnTokens {
    wallet_address: Option<String>,
    earned: Option<i32>,
    received: Option<i32>,
}

#[derive(SimpleObject)]
pub struct Email {
    address: String,
    verified: bool,
}

#[derive(SimpleObject)]
pub struct ProfesionalExp {
    #[graphql(name = "_id")]
    id: Option<i32>,
    company: Option<String>,
    title: Option<String>,
    #[graphql(name = "start_date")]
    start_date: Option<String>,
    #[graphql(name = "end_date")]
    end_date: Option<String>,
    current: Option<bool>,
    description: Option<String>,
}

#[derive(SimpleObject)]
pub struct Honors {
    #[graphql(name = "_id")]
    id: i32,
    organization: String,
    name: String,
    #[graphql(name = "start_date")]
    start_date: String,
    #[graphql(name = "end_date")]
    end_date: String,
    // #[graphql(name = "year")]
    expires: String,
    url: String,
}

#[derive(SimpleObject)]
pub struct Courses {
    #[graphql(name = "_id")]
    id: i32,
    name: String,
    year: String,
}

#[derive(SimpleObject)]
pub struct Projects {
    #[graphql(name = "_id")]
    id: i32,
    name: String,
    year: String,
}
#[derive(Debug, SimpleObject, Clone)]
pub struct UserTest {
    pub id: i32,
    pub name: String,
}
