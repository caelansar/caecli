use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub aud: String,
    pub exp: usize,
}

pub fn process_jwt_sign(sub: String, aud: String, exp: usize) -> anyhow::Result<String> {
    let claims = Claims { sub, aud, exp };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )?;
    Ok(token)
}

pub fn process_jwt_verify(token: &str) -> anyhow::Result<TokenData<Claims>> {
    let mut validation = Validation::default();
    validation.validate_aud = false;

    // `token` is a struct with 2 fields: `header` and `claims` where `claims` is your own struct.
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &validation,
    )?;
    Ok(token_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_jwt() {
        let token = process_jwt_sign("cae".into(), "device1".into(), 3611294006).unwrap();

        let data = process_jwt_verify(&token).unwrap();
        assert_eq!("cae", data.claims.sub);
        assert_eq!("device1", data.claims.aud);
    }
}
