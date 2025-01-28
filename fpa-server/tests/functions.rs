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

async fn list(token: &String, project: &Uuid, frontier: &Uuid) -> Result<()> {
    let response = reqwest::Client::new()
        .get(format!(
            "{}/{}/frontiers/{}/functions",
            URL, project, frontier
        ))
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

async fn list_by_name(token: &String, project: &Uuid, frontier: &Uuid) -> Result<()> {
    let response = reqwest::Client::new()
        .get(format!(
            "{}/{}/frontiers/{}/functions?name=ALI",
            URL, project, frontier
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

async fn list_by_type(token: &String, project: &Uuid, frontier: &Uuid) -> Result<()> {
    let response = reqwest::Client::new()
        .get(format!(
            "{}/{}/frontiers/{}/functions?type=CE",
            URL, project, frontier
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

async fn create_ali(token: &String, project: &Uuid, frontier: &Uuid) -> Result<Uuid> {
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
        .post(format!(
            "{}/{}/frontiers/{}/functions",
            URL, project, frontier
        ))
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

async fn create_aie(token: &String, project: &Uuid, frontier: &Uuid) -> Result<Uuid> {
    let body = json!({
        "AIE": {
            "name": "AIE Test Name",
            "description": "AIE Test Description",
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
        .post(format!(
            "{}/{}/frontiers/{}/functions",
            URL, project, frontier
        ))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::CREATED);
    assert!(response.headers().get("location").is_some());

    let json = response.json::<serde_json::Value>().await?;
    assert!(json.get("AIE").is_some());
    let value = json.get("AIE").unwrap();

    assert!(value.get("id").is_some());
    let id = Uuid::parse_str(value["id"].as_str().unwrap()).unwrap();

    assert_eq!(value["name"], json!("AIE Test Name"));
    assert_eq!(value["description"], json!("AIE Test Description"));
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

async fn create_ee(token: &String, project: &Uuid, frontier: &Uuid, data: &Data) -> Result<Uuid> {
    let body = json!({
        "EE": {
            "name": "EE Test Name",
            "description": "EE Test Description",
            "alrs": [
                {
                    "type": "ALI",
                    "id": data.function_ali.to_string(),
                }
            ]
        }
    });

    let response = reqwest::Client::new()
        .post(format!(
            "{}/{}/frontiers/{}/functions",
            URL, project, frontier
        ))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::CREATED);
    assert!(response.headers().get("location").is_some());

    let json = response.json::<serde_json::Value>().await?;
    assert!(json.get("EE").is_some());
    let value = json.get("EE").unwrap();

    assert!(value.get("id").is_some());
    let id = Uuid::parse_str(value["id"].as_str().unwrap()).unwrap();

    assert_eq!(value["name"], json!("EE Test Name"));
    assert_eq!(value["description"], json!("EE Test Description"));
    assert!(value["alrs"].is_array());
    assert_eq!(value["alrs"].as_array().unwrap().len(), 1);

    let alrs = value["alrs"].as_array().unwrap();
    assert!(alrs[0].get("ALI").is_some());
    let ali = alrs[0].get("ALI").unwrap();

    assert_eq!(ali["id"], json!(data.function_ali.to_string()));
    assert_eq!(ali["name"], json!("ALI Test Name"));
    assert_eq!(ali["description"], json!("ALI Test Description"));

    Ok(id)
}

async fn create_ce(token: &String, project: &Uuid, frontier: &Uuid, data: &Data) -> Result<Uuid> {
    let body = json!({
        "CE": {
            "name": "CE Test Name",
            "description": "CE Test Description",
            "alrs": [
                {
                    "type": "AIE",
                    "id": data.function_aie.to_string(),
                }
            ]
        }
    });

    let response = reqwest::Client::new()
        .post(format!(
            "{}/{}/frontiers/{}/functions",
            URL, project, frontier
        ))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::CREATED);
    assert!(response.headers().get("location").is_some());

    let json = response.json::<serde_json::Value>().await?;
    assert!(json.get("CE").is_some());
    let value = json.get("CE").unwrap();

    assert!(value.get("id").is_some());
    let id = Uuid::parse_str(value["id"].as_str().unwrap()).unwrap();

    assert_eq!(value["name"], json!("CE Test Name"));
    assert_eq!(value["description"], json!("CE Test Description"));
    assert!(value["alrs"].is_array());
    assert_eq!(value["alrs"].as_array().unwrap().len(), 1);

    let alrs = value["alrs"].as_array().unwrap();
    assert!(alrs[0].get("AIE").is_some());
    let aie = alrs[0].get("AIE").unwrap();

    assert_eq!(aie["id"], json!(data.function_aie.to_string()));
    assert_eq!(aie["name"], json!("AIE Test Name"));
    assert_eq!(aie["description"], json!("AIE Test Description"));

    Ok(id)
}

async fn create_se(token: &String, project: &Uuid, frontier: &Uuid, data: &Data) -> Result<Uuid> {
    let body = json!({
        "SE": {
            "name": "SE Test Name",
            "description": "SE Test Description",
            "alrs": [
                {
                    "type": "AIE",
                    "id": data.function_aie.to_string(),
                },
                {
                    "type": "ALI",
                    "id": data.function_ali.to_string(),
                }
            ]
        }
    });

    let response = reqwest::Client::new()
        .post(format!(
            "{}/{}/frontiers/{}/functions",
            URL, project, frontier
        ))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::CREATED);
    assert!(response.headers().get("location").is_some());

    let json = response.json::<serde_json::Value>().await?;

    assert!(json.get("SE").is_some());
    let value = json.get("SE").unwrap();

    assert!(value.get("id").is_some());
    let id = Uuid::parse_str(value["id"].as_str().unwrap()).unwrap();

    assert_eq!(value["name"], json!("SE Test Name"));
    assert_eq!(value["description"], json!("SE Test Description"));
    assert!(value["alrs"].is_array());
    assert_eq!(value["alrs"].as_array().unwrap().len(), 2);

    let alrs = value["alrs"].as_array().unwrap();
    for alr in alrs.iter() {
        if alr.get("AIE").is_some() {
            let aie = alr.get("AIE").unwrap();
            assert_eq!(aie["id"], json!(data.function_aie.to_string()));
            assert_eq!(aie["name"], json!("AIE Test Name"));
            assert_eq!(aie["description"], json!("AIE Test Description"));
        } else if alr.get("ALI").is_some() {
            let ali = alr.get("ALI").unwrap();
            assert_eq!(ali["id"], json!(data.function_ali.to_string()));
            assert_eq!(ali["name"], json!("ALI Test Name"));
            assert_eq!(ali["description"], json!("ALI Test Description"));
        }
    }

    Ok(id)
}

async fn by_id(token: &String, project: &Uuid, frontier: &Uuid, function: &Uuid) -> Result<()> {
    let response = reqwest::Client::new()
        .get(format!(
            "{}/{}/frontiers/{}/functions/{}",
            URL, project, frontier, function
        ))
        .bearer_auth(token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<serde_json::Value>().await?;

    let types = ["ALI", "AIE", "EE", "SE", "CE"];
    let mut found = false;
    for t in &types {
        if let Some(value) = json.get(*t) {
            found = true;
            let id = value["id"].as_str().unwrap();
            assert_eq!(function, &Uuid::parse_str(id).unwrap());
            break;
        }
    }
    if !found {
        panic!("Function not found.");
    }

    Ok(())
}

async fn update_ali(
    token: &String,
    project: &Uuid,
    frontier: &Uuid,
    function: &Uuid,
) -> Result<()> {
    let body = json!({
        "ALI": {
            "name": "ALI Test Name Updated",
            "description": "ALI Test Description Updated",
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
                    ]
                },
                {
                    "name": "RLR Test Name 02",
                    "description": "RLR Test Description 02",
                    "ders": [
                        {
                            "name": "DER 03 Test Name",
                            "description": "DER Test Description",
                        },
                        {
                            "name": "DER 04 Test Name",
                            "description": "DER Test Description",
                        },
                    ]
                },
            ]
        }
    });
    let response = reqwest::Client::new()
        .put(format!(
            "{}/{}/frontiers/{}/functions/{}",
            URL, project, frontier, function
        ))
        .json(&body)
        .bearer_auth(token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<serde_json::Value>().await?;
    assert!(json.get("ALI").is_some());
    let value = json.get("ALI").unwrap();

    assert!(value.get("id").is_some());
    let id = Uuid::parse_str(value["id"].as_str().unwrap()).unwrap();
    assert_eq!(function, &id);

    assert_eq!(value["name"], json!("ALI Test Name Updated"));
    assert_eq!(value["description"], json!("ALI Test Description Updated"));
    assert!(value["rlrs"].is_array());
    assert_eq!(value["rlrs"].as_array().unwrap().len(), 2);

    let rlrs = value["rlrs"].as_array().unwrap();
    assert_eq!(rlrs[0]["name"], json!("RLR Test Name"));
    assert_eq!(rlrs[0]["description"], json!("RLR Test Description"));
    assert!(rlrs[0]["ders"].is_array());
    assert_eq!(rlrs[0]["ders"].as_array().unwrap().len(), 2);

    let ders = rlrs[0]["ders"].as_array().unwrap();
    assert_eq!(ders[0]["name"], json!("DER 01 Test Name"));
    assert_eq!(ders[0]["description"], json!("DER Test Description"));
    assert_eq!(ders[1]["name"], json!("DER 02 Test Name"));
    assert_eq!(ders[1]["description"], json!("DER Test Description"));

    assert_eq!(rlrs[1]["name"], json!("RLR Test Name 02"));
    assert_eq!(rlrs[1]["description"], json!("RLR Test Description 02"));
    assert!(rlrs[1]["ders"].is_array());
    assert_eq!(rlrs[1]["ders"].as_array().unwrap().len(), 2);

    let ders = rlrs[1]["ders"].as_array().unwrap();
    assert_eq!(ders[0]["name"], json!("DER 03 Test Name"));
    assert_eq!(ders[0]["description"], json!("DER Test Description"));
    assert_eq!(ders[1]["name"], json!("DER 04 Test Name"));
    assert_eq!(ders[1]["description"], json!("DER Test Description"));

    Ok(())
}

async fn update_aie(
    token: &String,
    project: &Uuid,
    frontier: &Uuid,
    function: &Uuid,
) -> Result<()> {
    let body = json!({
        "AIE": {
            "name": "AIE Test Name Updated",
            "description": "AIE Test Description Updated",
            "rlrs": [
                {
                    "name": "RLR Test Name Updated",
                    "description": "RLR Test Description Updated",
                    "ders": [
                        {
                            "name": "DER 01 Test Name Updated",
                            "description": "DER Test Description Updated",
                        },
                    ]
                },
            ]
        }
    });
    let response = reqwest::Client::new()
        .put(format!(
            "{}/{}/frontiers/{}/functions/{}",
            URL, project, frontier, function
        ))
        .json(&body)
        .bearer_auth(token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<serde_json::Value>().await?;
    assert!(json.get("AIE").is_some());
    let value = json.get("AIE").unwrap();

    assert!(value.get("id").is_some());
    let id = Uuid::parse_str(value["id"].as_str().unwrap()).unwrap();
    assert_eq!(function, &id);

    assert_eq!(value["name"], json!("AIE Test Name Updated"));
    assert_eq!(value["description"], json!("AIE Test Description Updated"));
    assert!(value["rlrs"].is_array());
    assert_eq!(value["rlrs"].as_array().unwrap().len(), 1);

    let rlrs = value["rlrs"].as_array().unwrap();
    assert_eq!(rlrs[0]["name"], json!("RLR Test Name Updated"));
    assert_eq!(
        rlrs[0]["description"],
        json!("RLR Test Description Updated")
    );
    assert!(rlrs[0]["ders"].is_array());
    assert_eq!(rlrs[0]["ders"].as_array().unwrap().len(), 1);

    let ders = rlrs[0]["ders"].as_array().unwrap();
    assert_eq!(ders[0]["name"], json!("DER 01 Test Name Updated"));
    assert_eq!(
        ders[0]["description"],
        json!("DER Test Description Updated")
    );

    Ok(())
}

async fn update_ee(token: &String, project: &Uuid, frontier: &Uuid, data: &Data) -> Result<()> {
    let body = json!({
        "EE": {
            "name": "EE Test Name Updated",
            "description": "EE Test Description Updated",
            "alrs": [
                {
                    "type": "AIE",
                    "id": data.function_aie.to_string(),
                }
            ]
        }
    });

    let response = reqwest::Client::new()
        .put(format!(
            "{}/{}/frontiers/{}/functions/{}",
            URL, project, frontier, data.function_ee
        ))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<serde_json::Value>().await?;
    assert!(json.get("EE").is_some());
    let value = json.get("EE").unwrap();

    assert!(value.get("id").is_some());
    let id = Uuid::parse_str(value["id"].as_str().unwrap()).unwrap();
    assert_eq!(data.function_ee, id);

    assert_eq!(value["name"], json!("EE Test Name Updated"));
    assert_eq!(value["description"], json!("EE Test Description Updated"));
    assert!(value["alrs"].is_array());
    assert_eq!(value["alrs"].as_array().unwrap().len(), 1);

    let alrs = value["alrs"].as_array().unwrap();
    assert!(alrs[0].get("AIE").is_some());
    let ali = alrs[0].get("AIE").unwrap();

    assert_eq!(ali["id"], json!(data.function_aie.to_string()));

    Ok(())
}

async fn update_ce(token: &String, project: &Uuid, frontier: &Uuid, data: &Data) -> Result<()> {
    let body = json!({
        "CE": {
            "name": "CE Test Name Updated",
            "description": "CE Test Description Updated",
            "alrs": [
                {
                    "type": "ALI",
                    "id": data.function_ali.to_string(),
                }
            ]
        }
    });

    let response = reqwest::Client::new()
        .put(format!(
            "{}/{}/frontiers/{}/functions/{}",
            URL, project, frontier, data.function_ce
        ))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<serde_json::Value>().await?;
    assert!(json.get("CE").is_some());
    let value = json.get("CE").unwrap();

    assert!(value.get("id").is_some());
    let id = Uuid::parse_str(value["id"].as_str().unwrap()).unwrap();
    assert_eq!(data.function_ce, id);

    assert_eq!(value["name"], json!("CE Test Name Updated"));
    assert_eq!(value["description"], json!("CE Test Description Updated"));
    assert!(value["alrs"].is_array());
    assert_eq!(value["alrs"].as_array().unwrap().len(), 1);

    let alrs = value["alrs"].as_array().unwrap();
    assert!(alrs[0].get("ALI").is_some());
    let aie = alrs[0].get("ALI").unwrap();

    assert_eq!(aie["id"], json!(data.function_ali.to_string()));

    Ok(())
}

async fn update_se(token: &String, project: &Uuid, frontier: &Uuid, data: &Data) -> Result<()> {
    let body = json!({
        "SE": {
            "name": "SE Test Name Updated",
            "description": "SE Test Description Updated",
            "alrs": [
                {
                    "type": "AIE",
                    "id": data.function_aie.to_string(),
                },
                {
                    "type": "ALI",
                    "id": data.function_ali.to_string(),
                }
            ]
        }
    });

    let response = reqwest::Client::new()
        .put(format!(
            "{}/{}/frontiers/{}/functions/{}",
            URL, project, frontier, data.function_se
        ))
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<serde_json::Value>().await?;

    assert!(json.get("SE").is_some());
    let value = json.get("SE").unwrap();

    assert!(value.get("id").is_some());
    let id = Uuid::parse_str(value["id"].as_str().unwrap()).unwrap();
    assert_eq!(data.function_se, id);

    assert_eq!(value["name"], json!("SE Test Name Updated"));
    assert_eq!(value["description"], json!("SE Test Description Updated"));
    assert!(value["alrs"].is_array());
    assert_eq!(value["alrs"].as_array().unwrap().len(), 2);

    let alrs = value["alrs"].as_array().unwrap();
    for alr in alrs.iter() {
        if alr.get("AIE").is_some() {
            let aie = alr.get("AIE").unwrap();
            assert_eq!(aie["id"], json!(data.function_aie.to_string()));
        } else if alr.get("ALI").is_some() {
            let ali = alr.get("ALI").unwrap();
            assert_eq!(ali["id"], json!(data.function_ali.to_string()));
        }
    }

    Ok(())
}

async fn remove(token: &String, project: &Uuid, frontier: &Uuid, function: &Uuid) -> Result<()> {
    let response = reqwest::Client::new()
        .delete(format!(
            "{}/{}/frontiers/{}/functions/{}",
            URL, project, frontier, function
        ))
        .bearer_auth(token)
        .send()
        .await?;
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    Ok(())
}

#[tokio::test]
async fn execute() -> Result<()> {
    let token = tokens::request_token(USERNAME, PASSWORD, Tenant::TENANT_DEFAULT).await?;
    assert!(!token.is_empty());

    let project = selects::project(&token).await?;
    let frontier = selects::frontier(&token, &project).await?;
    let mut data = Data::default();

    list(&token, &project, &frontier).await?;
    list_by_name(&token, &project, &frontier).await?;
    list_by_type(&token, &project, &frontier).await?;

    data.function_ali = create_ali(&token, &project, &frontier).await?;
    data.function_aie = create_aie(&token, &project, &frontier).await?;
    data.function_ee = create_ee(&token, &project, &frontier, &data).await?;
    data.function_ce = create_ce(&token, &project, &frontier, &data).await?;
    data.function_se = create_se(&token, &project, &frontier, &data).await?;

    by_id(&token, &project, &frontier, &data.function_ali).await?;
    by_id(&token, &project, &frontier, &data.function_aie).await?;
    by_id(&token, &project, &frontier, &data.function_ee).await?;
    by_id(&token, &project, &frontier, &data.function_ce).await?;
    by_id(&token, &project, &frontier, &data.function_se).await?;

    update_ali(&token, &project, &frontier, &data.function_ali).await?;
    update_aie(&token, &project, &frontier, &data.function_aie).await?;
    update_ee(&token, &project, &frontier, &data).await?;
    update_ce(&token, &project, &frontier, &data).await?;
    update_se(&token, &project, &frontier, &data).await?;

    remove(&token, &project, &frontier, &data.function_ee).await?;
    remove(&token, &project, &frontier, &data.function_ce).await?;
    remove(&token, &project, &frontier, &data.function_se).await?;
    remove(&token, &project, &frontier, &data.function_ali).await?;
    remove(&token, &project, &frontier, &data.function_aie).await?;

    Ok(())
}
