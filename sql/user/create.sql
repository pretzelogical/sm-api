INSERT INTO sm_user (name, pass)
VALUES ($1, $2)
RETURNING sm_user.id, sm_user.name, sm_user.pass;
