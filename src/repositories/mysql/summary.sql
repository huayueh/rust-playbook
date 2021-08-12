WITH post_count AS
(
    SELECT
    	*,
    	ROW_NUMBER() OVER(ORDER BY id) AS row_num,
        COUNT(id) OVER(ORDER BY id)  AS total_count
    FROM posts
    WHERE id > 0 AND id < 100
)
SELECT *  FROM post_count WHERE row_num > 0 AND row_num < 100;