DEFINE ACCESS OVERWRITE user ON DATABASE TYPE RECORD 
    SIGNIN (
        RETURN fn::signin($email, $password)
    );