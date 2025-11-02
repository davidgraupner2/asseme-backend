DEFINE ACCESS OVERWRITE user ON DATABASE TYPE RECORD 
	SIGNIN (
		SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(password_hash, $password)
	)
	SIGNUP (
		CREATE user CONTENT {
			name: $name,
			email: $email,
			password: crypto::argon2::generate($password)
		}
	);

select * from $auth_id;