DEFINE FUNCTION OVERWRITE fn::signin($email: string, $password: string) {
    LET $user = SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(password_hash, $password);
    return $user;
};