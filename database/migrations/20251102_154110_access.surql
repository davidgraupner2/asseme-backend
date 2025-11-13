DEFINE ACCESS OVERWRITE record_user ON DATABASE TYPE RECORD 
    SIGNIN (
        SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(password_hash, $password)
    )
    DURATION FOR TOKEN 15m, FOR SESSION 12h;


