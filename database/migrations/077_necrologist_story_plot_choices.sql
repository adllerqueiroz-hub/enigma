ALTER TABLE user_necrologist_story_plots
ADD COLUMN selected_options_json TEXT NOT NULL DEFAULT '[]';

ALTER TABLE user_necrologist_story_plots
ADD COLUMN unlock_end_ids_json TEXT NOT NULL DEFAULT '[]';

ALTER TABLE user_necrologist_story_plots
ADD COLUMN last_selected_options_json TEXT NOT NULL DEFAULT '[]';

ALTER TABLE user_necrologist_story_plots
ADD COLUMN last_end_id INTEGER NOT NULL DEFAULT 0;
