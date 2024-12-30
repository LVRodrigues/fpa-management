mod shared;

use anyhow::{Ok, Result};
use reqwest::StatusCode;
use serde_json::json;
use shared::{
    selects,
    tokens::{self, Tenant},
    PASSWORD, URL, USERNAME,
};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
struct Data {
    function_ali: Uuid,
    function_aie: Uuid,
    function_ee: Uuid,
    function_ce: Uuid,
    function_se: Uuid,
}

async fn list(token: &String, project: &Uuid, module: &Uuid) -> Result<()> {
    let response = reqwest::Client::new()
        .get(format!("{}/{}/modules/{}/functions", URL, project, module))
        .bearer_auth(token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<serde_json::Value>().await?;

    assert_eq!(json["pages"], json!(1));
    assert_eq!(json["index"], json!(1));
    assert_eq!(json["size"], json!(5));
    assert_eq!(json["records"], json!(5));
    assert!(json["items"].is_array());
    assert_eq!(json["items"].as_array().unwrap().len(), 5);

    Ok(())
}

async fn list_by_name(token: &String, project: &Uuid, module: &Uuid) -> Result<()> {
    let response = reqwest::Client::new()
        .get(format!(
            "{}/{}/modules/{}/functions?name=ALI",
            URL, project, module
        ))
        .bearer_auth(token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<serde_json::Value>().await?;

    assert_eq!(json["pages"], json!(1));
    assert_eq!(json["index"], json!(1));
    assert_eq!(json["size"], json!(1));
    assert_eq!(json["records"], json!(1));
    assert!(json["items"].is_array());
    assert_eq!(json["items"].as_array().unwrap().len(), 1);

    let items = json["items"].as_array().unwrap();
    assert!(items[0].get("ALI").is_some());
    let value = items[0].get("ALI").unwrap();
    assert!(value["name"].as_str().unwrap().contains("ALI"));

    assert!(value["rlrs"].is_array());
    assert_eq!(value["rlrs"].as_array().unwrap().len(), 1);

    let ders = value["rlrs"].as_array().unwrap();
    assert_eq!(ders[0]["ders"].as_array().unwrap().len(), 5);

    Ok(())
}

async fn list_by_type(token: &String, project: &Uuid, module: &Uuid) -> Result<()> {
    let response = reqwest::Client::new()
        .get(format!(
            "{}/{}/modules/{}/functions?type=CE",
            URL, project, module
        ))
        .bearer_auth(token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<serde_json::Value>().await?;

    assert_eq!(json["pages"], json!(1));
    assert_eq!(json["index"], json!(1));
    assert_eq!(json["size"], json!(1));
    assert_eq!(json["records"], json!(1));
    assert!(json["items"].is_array());
    assert_eq!(json["items"].as_array().unwrap().len(), 1);

    let items = json["items"].as_array().unwrap();
    assert!(items[0].get("CE").is_some());
    let value = items[0].get("CE").unwrap();
    assert!(value["name"].as_str().unwrap().contains("CE"));

    assert!(value["alrs"].is_array());
    assert_eq!(value["alrs"].as_array().unwrap().len(), 1);

    let alrs = value["alrs"].as_array().unwrap();
    assert!(alrs[0].get("AIE").is_some());

    Ok(())
}

async fn createALI(token: &String, project: &Uuid, module: &Uuid) -> Result<Uuid> {
    let body = json!({
        "ALI": {
            "name": "ALI Test Name",
            "description": "ALI Test Description",
            "rlrs": [
                {
                    "name": "RLR Test Name",
                    "description": "RLR Test Description",
                    "ders": [
                        {
                            "name": "DER 01 Test Name",
                            "description": "DER Test Description",
                        },
                        {
                            "name": "DER 02 Test Name",
                            "description": "DER Test Description",
                        },
                        {
                            "name": "DER 03 Test Name",
                            "description": "DER Test Description",
                        },
                    ]
                },
            ]
        }
    });

    let response = reqwest::Client::new()
        .post(format!("{}/{}/modules/{}/functions", URL, project, module))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::CREATED);
    assert!(response.headers().get("location").is_some());

    let json = response.json::<serde_json::Value>().await?;
    assert!(json.get("ALI").is_some());
    let value = json.get("ALI").unwrap();

    assert!(value.get("id").is_some());
    let id = Uuid::parse_str(value["id"].as_str().unwrap()).unwrap();

    assert_eq!(value["name"], json!("ALI Test Name"));
    assert_eq!(value["description"], json!("ALI Test Description"));
    assert!(value["rlrs"].is_array());
    assert_eq!(value["rlrs"].as_array().unwrap().len(), 1);

    let rlrs = value["rlrs"].as_array().unwrap();
    assert_eq!(rlrs[0]["name"], json!("RLR Test Name"));
    assert_eq!(rlrs[0]["description"], json!("RLR Test Description"));
    assert!(rlrs[0]["ders"].is_array());
    assert_eq!(rlrs[0]["ders"].as_array().unwrap().len(), 3);

    let ders = rlrs[0]["ders"].as_array().unwrap();
    assert_eq!(ders[0]["name"], json!("DER 01 Test Name"));
    assert_eq!(ders[0]["description"], json!("DER Test Description"));
    assert_eq!(ders[1]["name"], json!("DER 02 Test Name"));
    assert_eq!(ders[1]["description"], json!("DER Test Description"));
    assert_eq!(ders[2]["name"], json!("DER 03 Test Name"));
    assert_eq!(ders[2]["description"], json!("DER Test Description"));

    Ok(id)
}

#[tokio::test]
async fn execute() -> Result<()> {
    let token = tokens::request_token(USERNAME, PASSWORD, Tenant::TENANT_DEFAULT).await?;
    assert!(!token.is_empty());

    let project = selects::project(&token).await?;
    let module = selects::module(&token, &project).await?;
    let mut data = Data::default();

    list(&token, &project, &module).await?;

    list_by_name(&token, &project, &module).await?;

    list_by_type(&token, &project, &module).await?;

    data.function_ali = createALI(&token, &project, &module).await?;

    Ok(())
}
