DROP TABLE IF EXISTS hero_equip_attributes;
DROP TABLE IF EXISTS hero_sp_attrs;

ALTER TABLE heroes DROP COLUMN base_hp;
ALTER TABLE heroes DROP COLUMN base_attack;
ALTER TABLE heroes DROP COLUMN base_defense;
ALTER TABLE heroes DROP COLUMN base_mdefense;
ALTER TABLE heroes DROP COLUMN base_technic;
ALTER TABLE heroes DROP COLUMN base_multi_hp_idx;
ALTER TABLE heroes DROP COLUMN base_multi_hp_num;
ALTER TABLE heroes DROP COLUMN ex_cri;
ALTER TABLE heroes DROP COLUMN ex_recri;
ALTER TABLE heroes DROP COLUMN ex_cri_dmg;
ALTER TABLE heroes DROP COLUMN ex_cri_def;
ALTER TABLE heroes DROP COLUMN ex_add_dmg;
ALTER TABLE heroes DROP COLUMN ex_drop_dmg;
ALTER TABLE heroes DROP COLUMN nowmal_dmg;
