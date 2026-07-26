use anyhow::Result;
use serde::{Deserialize, Serialize};
use sonettobuf;
use sqlx::{FromRow, SqlitePool};

#[async_trait::async_trait]
pub trait EquipmentModel<T>: Send + Sync {
    async fn get(&self, equip_uid: i64) -> Result<T>;
    async fn get_all(&self) -> Result<Vec<T>>;
    async fn break_level(&self, equip_uid: i64) -> Result<bool>;
    async fn lock(&self, equip_uid: i64, is_lock: bool) -> Result<bool>;
    async fn refine_level(&self, equip_uid: i64, level: i32) -> Result<bool>;
    async fn delete(&self, equip_uid: i64) -> Result<()>;
    async fn strengthen(
        &self,
        target_uid: i64,
        consume_items: Vec<(i64, i32)>,
    ) -> Result<(i32, Vec<i32>)>;
}

pub struct UserEquipmentModel {
    user_id: i64,
    pool: SqlitePool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Equipment {
    pub uid: i64,
    pub user_id: i64,
    pub equip_id: i32,
    pub level: i32,
    pub exp: i32,
    pub break_lv: i32,
    pub count: i32,
    pub is_lock: bool,
    pub refine_lv: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Equipment> for sonettobuf::Equip {
    fn from(e: Equipment) -> Self {
        sonettobuf::Equip {
            equip_id: Some(e.equip_id),
            uid: Some(e.uid),
            level: Some(e.level),
            exp: Some(e.exp),
            break_lv: Some(e.break_lv),
            count: Some(e.count),
            is_lock: Some(e.is_lock),
            refine_lv: Some(e.refine_lv),
        }
    }
}

impl UserEquipmentModel {
    pub fn new(user_id: i64, pool: SqlitePool) -> Self {
        Self { user_id, pool }
    }

    pub async fn get_equip(&self, equip_uid: i64) -> Result<Equipment> {
        EquipmentModel::<Equipment>::get(self, equip_uid).await
    }

    pub async fn update_equipment_lock(&self, equip_uid: i64, lock: bool) -> Result<bool> {
        EquipmentModel::<Equipment>::lock(self, equip_uid, lock).await
    }

    pub async fn strengthen_equip(
        &self,
        target_uid: i64,
        consume_items: Vec<(i64, i32)>,
    ) -> Result<(i32, Vec<i32>)> {
        EquipmentModel::<Equipment>::strengthen(self, target_uid, consume_items).await
    }

    pub async fn break_equip(&self, equip_uid: i64) -> Result<bool> {
        EquipmentModel::<Equipment>::break_level(self, equip_uid).await
    }

    pub async fn refine_equip(&self, equip_uid: i64, level: i32) -> Result<bool> {
        EquipmentModel::<Equipment>::refine_level(self, equip_uid, level).await
    }

    pub async fn delete_equip(&self, equip_uid: i64) -> Result<()> {
        EquipmentModel::<Equipment>::delete(self, equip_uid).await
    }
}

#[async_trait::async_trait]
impl EquipmentModel<Equipment> for UserEquipmentModel {
    async fn get(&self, equip_uid: i64) -> Result<Equipment> {
        let equip = sqlx::query_as::<_, Equipment>(
            r#"
            SELECT uid, user_id, equip_id, level, exp, break_lv, count, is_lock, refine_lv, created_at, updated_at
            FROM equipment
            WHERE uid = ? AND user_id = ?
            "#,
        )
        .bind(equip_uid)
        .bind(self.user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(equip)
    }

    async fn get_all(&self) -> Result<Vec<Equipment>> {
        let equips = sqlx::query_as::<_, Equipment>(
            r#"
            SELECT uid, user_id, equip_id, level, exp, break_lv, count, is_lock, refine_lv, created_at, updated_at
            FROM equipment
            WHERE user_id = ? AND count > 0 ORDER BY equip_id
            "#,
        )
        .bind(self.user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(equips)
    }

    async fn break_level(&self, equip_uid: i64) -> Result<bool> {
        let equip_data = self.get(equip_uid).await?;

        let now = common::time::ServerTime::now_ms();
        let new_level = sqlx::query(
            "UPDATE equipment
             SET break_lv = ?, updated_at = ?
             WHERE uid = ? AND user_id = ?",
        )
        .bind(equip_data.break_lv + 1)
        .bind(now)
        .bind(equip_uid)
        .bind(self.user_id)
        .execute(&self.pool)
        .await?;

        Ok(new_level.rows_affected() > 0)
    }

    async fn lock(&self, equip_uid: i64, is_lock: bool) -> Result<bool> {
        let now = common::time::ServerTime::now_ms();

        let rows_affected = sqlx::query(
            "UPDATE equipment SET is_lock = ?, updated_at = ? WHERE uid = ? AND user_id = ?",
        )
        .bind(is_lock)
        .bind(now)
        .bind(equip_uid)
        .bind(self.user_id)
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }

    async fn refine_level(&self, equip_uid: i64, level: i32) -> Result<bool> {
        let now = common::time::ServerTime::now_ms();
        let new_level = sqlx::query(
            "UPDATE equipment
             SET refine_lv = ?, updated_at = ?
             WHERE uid = ? AND user_id = ?",
        )
        .bind(level)
        .bind(now)
        .bind(equip_uid)
        .bind(self.user_id)
        .execute(&self.pool)
        .await?;

        Ok(new_level.rows_affected() > 0)
    }

    async fn delete(&self, equip_uid: i64) -> Result<()> {
        sqlx::query("DELETE FROM equipment WHERE uid = ? AND user_id = ?")
            .bind(equip_uid)
            .bind(self.user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn strengthen(
        &self,
        target_uid: i64,
        consume_items: Vec<(i64, i32)>,
    ) -> Result<(i32, Vec<i32>)> {
        let mut target = self.get(target_uid).await?;
        let game_data = config::configs::get();

        let target_equip_data = game_data
            .equip
            .get(target.equip_id)
            .ok_or_else(|| anyhow::anyhow!("Equipment data not found"))?;

        let mut total_exp = 0i32;
        let mut equips_to_update = Vec::new();
        let now = common::time::ServerTime::now_ms();

        for (eat_uid, consume_count) in &consume_items {
            let eat_equipment = self
                .get(*eat_uid)
                .await
                .map_err(|_| anyhow::anyhow!("equipment {eat_uid} not found"))?;

            if eat_equipment.is_lock {
                anyhow::bail!("equipment {eat_uid} is locked");
            }

            if *eat_uid == target_uid {
                anyhow::bail!("target equipment cannot strengthen itself");
            }

            if *consume_count <= 0 || *consume_count > eat_equipment.count {
                anyhow::bail!("invalid consume count for equipment {eat_uid}");
            }

            let eat_equip_data = game_data.equip.get(eat_equipment.equip_id).ok_or_else(|| {
                anyhow::anyhow!(
                    "Equipment data not found for equip_id={}",
                    eat_equipment.equip_id
                )
            })?;

            let mut exp_per_equip = eat_equipment.exp;

            match eat_equipment.equip_id {
                1002 => exp_per_equip += 400,
                1003 => exp_per_equip += 1000,
                1004 => exp_per_equip += 4000,
                1005 => exp_per_equip += 10000,
                _ => {
                    if eat_equipment.level == 1 {
                        if let Some(lv2_cost) = game_data
                            .equip_strengthen_cost
                            .iter()
                            .find(|c| c.rare == eat_equip_data.rare && c.level == 2)
                        {
                            exp_per_equip += lv2_cost.exp;
                        }
                    } else {
                        for lvl in 2..=eat_equipment.level {
                            if let Some(cost) = game_data
                                .equip_strengthen_cost
                                .iter()
                                .find(|c| c.rare == eat_equip_data.rare && c.level == lvl)
                            {
                                exp_per_equip += cost.exp;
                            }
                        }
                    }
                }
            }

            total_exp = total_exp
                .checked_add(
                    exp_per_equip
                        .checked_mul(*consume_count)
                        .ok_or_else(|| anyhow::anyhow!("equipment experience overflow"))?,
                )
                .ok_or_else(|| anyhow::anyhow!("equipment experience overflow"))?;
            equips_to_update.push((
                *eat_uid,
                *consume_count,
                eat_equipment.count,
                eat_equipment.equip_id,
            ));
        }

        if total_exp <= 0 {
            anyhow::bail!("equipment consume list grants no experience");
        }

        let expected_level = target.level;
        let expected_exp = target.exp;
        target.exp += total_exp;
        let rare = target_equip_data.rare;

        let max_level = game_data
            .equip_break_cost
            .iter()
            .find(|e| e.rare == rare && e.break_level == target.break_lv)
            .map(|e| e.level)
            .unwrap_or(60);

        tracing::info!(
            "Target equipment: level={}, break_lv={}, max_level={}, exp={}, total_exp_to_add={}",
            target.level,
            target.break_lv,
            max_level,
            target.exp,
            total_exp
        );

        while target.level < max_level {
            let next_level = target.level + 1;
            let exp_required = game_data
                .equip_strengthen_cost
                .iter()
                .find(|e| e.rare == rare && e.level == next_level)
                .map(|c| c.exp)
                .unwrap_or(999999);

            tracing::info!(
                "Trying to level {} -> {}: have {} exp, need {} exp",
                target.level,
                next_level,
                target.exp,
                exp_required
            );

            if target.exp >= exp_required {
                target.exp -= exp_required;
                target.level += 1;
            } else {
                break;
            }
        }

        let mut affected_equip_ids = vec![target.equip_id];
        for (_, _, _, equip_id) in &equips_to_update {
            if !affected_equip_ids.contains(equip_id) {
                affected_equip_ids.push(*equip_id);
            }
        }

        let mut tx = self.pool.begin().await?;
        let target_updated = sqlx::query(
            "UPDATE equipment
                 SET level = ?, exp = ?, updated_at = ?
                 WHERE uid = ? AND user_id = ? AND level = ? AND exp = ?",
        )
        .bind(target.level)
        .bind(target.exp)
        .bind(now)
        .bind(target_uid)
        .bind(self.user_id)
        .bind(expected_level)
        .bind(expected_exp)
        .execute(&mut *tx)
        .await?;
        if target_updated.rows_affected() != 1 {
            return Err(anyhow::anyhow!("equipment changed during strengthen"));
        }

        for (eat_uid, consume_count, current_count, equip_id) in equips_to_update {
            let new_count = current_count - consume_count;
            let is_stackable = game_data
                .equip
                .get(equip_id)
                .is_some_and(|equip| equip.is_exp_equip == 1);

            if new_count <= 0 {
                if is_stackable {
                    let result = sqlx::query(
                        "UPDATE equipment SET count = 0, updated_at = ?
                             WHERE uid = ? AND user_id = ? AND count = ? AND is_lock = 0",
                    )
                    .bind(now)
                    .bind(eat_uid)
                    .bind(self.user_id)
                    .bind(current_count)
                    .execute(&mut *tx)
                    .await?;
                    if result.rows_affected() != 1 {
                        return Err(anyhow::anyhow!("equipment changed during strengthen"));
                    }
                } else {
                    let result = sqlx::query(
                        "DELETE FROM equipment
                         WHERE uid = ? AND user_id = ? AND count = ? AND is_lock = 0",
                    )
                    .bind(eat_uid)
                    .bind(self.user_id)
                    .bind(current_count)
                    .execute(&mut *tx)
                    .await?;
                    if result.rows_affected() != 1 {
                        return Err(anyhow::anyhow!("equipment changed during strengthen"));
                    }
                }
            } else {
                let result = sqlx::query(
                    "UPDATE equipment SET count = ?, updated_at = ?
                     WHERE uid = ? AND user_id = ? AND count = ? AND is_lock = 0",
                )
                .bind(new_count)
                .bind(now)
                .bind(eat_uid)
                .bind(self.user_id)
                .bind(current_count)
                .execute(&mut *tx)
                .await?;
                if result.rows_affected() != 1 {
                    return Err(anyhow::anyhow!("equipment changed during strengthen"));
                }
            }
        }
        tx.commit().await?;

        tracing::info!(
            "User {} strengthened equipment uid={} to level {} (exp: {})",
            self.user_id,
            target_uid,
            target.level,
            target.exp
        );

        Ok((total_exp, affected_equip_ids))
    }
}
