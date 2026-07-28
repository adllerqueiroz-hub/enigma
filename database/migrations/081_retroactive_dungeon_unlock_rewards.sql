CREATE TABLE user_dungeon_reward_repairs (
    user_id INTEGER NOT NULL,
    episode_id INTEGER NOT NULL,
    star INTEGER NOT NULL,
    repaired_at INTEGER,
    PRIMARY KEY (user_id, episode_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

INSERT INTO user_dungeon_reward_repairs (user_id, episode_id, star)
SELECT dungeon.user_id, dungeon.episode_id, dungeon.star
FROM user_dungeons AS dungeon
JOIN (
    SELECT user_id, created_at
    FROM user_dungeons
    WHERE star > 0
      AND challenge_count = 0
      AND has_record = 0
      AND today_pass_num = 0
    GROUP BY user_id, created_at
    HAVING COUNT(*) >= 3
) AS batch
  ON batch.user_id = dungeon.user_id
 AND batch.created_at = dungeon.created_at
WHERE dungeon.star > 0
  AND dungeon.challenge_count = 0
  AND dungeon.has_record = 0
  AND dungeon.today_pass_num = 0;
