use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
pub struct ReqInput {
    pub crewtransfercost: Option<i32>,
    pub statusdescription: Option<String>,
    pub deliverydate: Option<String>,
    pub buildingarea: Option<String>,
    pub landarea: Option<String>,
    pub id: i32,
    pub unittype: Option<i32>,
    pub requeststatus: Option<i32>,
    pub area: Option<f64>,
    pub areatype: Option<i32>,
    pub price: Option<f64>,
    pub requestnumber: Option<String>,
    pub createdby: Option<String>,
    pub userid: Option<String>,
    pub addeddate: Option<String>,
    pub modifieddate: Option<String>,
    pub subunittype: Option<i32>,
    pub subunittypearea: Option<f64>,
}
