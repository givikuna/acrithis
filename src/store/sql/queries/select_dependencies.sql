SELECT
    to_hash,
    role
FROM
    dependencies
WHERE
    from_hash = ?1
ORDER BY
    role ASC,
    to_hash ASC;