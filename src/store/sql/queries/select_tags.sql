SELECT
    tag
FROM
    tags
WHERE
    hash = ?1
ORDER BY
    tag ASC;