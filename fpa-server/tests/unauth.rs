use anyhow::Result;
use reqwest::StatusCode;

const TOKEN: &str = "eyJhbGciOiJSUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICJRVnpxaHJDbWhMT0RaMTBxM3ljVkFNekU3cWtqU2twMzhybFdadW9lMHVRIn0.eyJleHAiOjE3MDE3MjkxNjIsImlhdCI6MTcwMTcyODg2MiwianRpIjoiYzhiZjlmNzgtOTAyNi00M2YzLTlhYTgtN2ZjNzQ1OWQxNWMwIiwiaXNzIjoiaHR0cDovL2xvY2FsaG9zdDo4MDgwL3JlYWxtcy90ZW5hbnQtMDEiLCJhdWQiOiJhY2NvdW50Iiwic3ViIjoiN2MzOTc0ZDMtZWEzZi00ZTRhLTg4ZWUtMThlMzBiOWIzNGJjIiwidHlwIjoiQmVhcmVyIiwiYXpwIjoiZnBhLW1hbmFnZW1lbnQiLCJzZXNzaW9uX3N0YXRlIjoiZTczZTU5YTctMzAzMy00ZjFiLTk0MzctMzRkOWY2MDc0NmEwIiwiYWNyIjoiMSIsImFsbG93ZWQtb3JpZ2lucyI6WyJodHRwOi8vbG9jYWxob3N0OjUwMDAiXSwicmVhbG1fYWNjZXNzIjp7InJvbGVzIjpbImRlZmF1bHQtcm9sZXMtdGVuYW50LTAxIiwib2ZmbGluZV9hY2Nlc3MiLCJ1bWFfYXV0aG9yaXphdGlvbiIsInVzZXIiXX0sInJlc291cmNlX2FjY2VzcyI6eyJhY2NvdW50Ijp7InJvbGVzIjpbIm1hbmFnZS1hY2NvdW50IiwibWFuYWdlLWFjY291bnQtbGlua3MiLCJ2aWV3LXByb2ZpbGUiXX19LCJzY29wZSI6InByb2ZpbGUgZW1haWwiLCJzaWQiOiJlNzNlNTlhNy0zMDMzLTRmMWItOTQzNy0zNGQ5ZjYwNzQ2YTAiLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwibmFtZSI6IlVzZXIgU3lzdGVtIiwicHJlZmVycmVkX3VzZXJuYW1lIjoidXNlciIsImdpdmVuX25hbWUiOiJVc2VyIiwiZmFtaWx5X25hbWUiOiJTeXN0ZW0iLCJlbWFpbCI6InVzZXJAbmFvLmNvbS5iciJ9.dAi0SSL6__5NCIgK4_rg0B2VX1a2xdKCZ3yL5Ozxpk5HHrv1edry2kcJ69_gR1omfKA_zenuveMuucL-ypU3USRT_0AWa8WecULuut1nEr_dZu66NIuvbrB9an3qDn1iGYAZGMgqVvMjEnN_HcpOkwXw3sbtoLxx2VExXduhMLV6HrW8u8LnsO3J-doSRlbDcL2_ab3eaK29viZD_3IdVp1_3D_lewO4Nuk0X9Pcy2Sd6MZN9BjW0cQ4tvlB0O4i2x23poXvzqsXKpa8QMIdzZpFJqi9KSwSZdIS4WkYMIBFHMo6tfwl5tIjHtanpl9g31R1g3LmueAAn4-pUY2S9A";

#[tokio::test]
async fn unauth() -> Result<()> {
    let response = reqwest::Client::new()
        .get("http://localhost:5000/api/health")
        .bearer_auth(TOKEN)
        .send()
        .await?;
    assert!(response.status() == StatusCode::UNAUTHORIZED);

    Ok(())
}
