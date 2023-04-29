### API Documentation

#### MongoDB Integration

##### MongoDB Connection
1. Storing the OTP
```rust
use mongodb::bson::doc;

async fn store_otp(client: &Client, user_id: &str, otp: &str) -> Result<(), mongodb::error::Error> {
    let db = client.database("otp_verification");
    let collection = db.collection("otps");
    let document = doc! {
        "user_id": user_id,
        "otp": otp,
    };
    collection.insert_one(document, None).await?;
    Ok(())
}
```

2. Verifying the OTP
```rust
async fn verify_otp(client: &Client, user_id: &str, otp: &str) -> Result<bool, mongodb::error::Error> {
    let db = client.database("otp_verification");
    let collection = db.collection("otps");
    let filter = doc! {
        "user_id": user_id,
        "otp": otp,
    };
    let result = collection.find_one(filter, None).await?;
    Ok(result.is_some())
}
```


### 7 hrs dedication XD for this project(Internship Work sucks!)