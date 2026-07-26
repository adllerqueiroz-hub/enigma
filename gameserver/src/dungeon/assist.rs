use super::*;
use crate::logic::{profile::ProfileManager, social::SocialManager};

pub async fn refresh_assist(
    db: &SqlitePool,
    player_id: i64,
    request: RefreshAssistRequest,
) -> Result<RefreshAssistReply, AppError> {
    let Some(hero_id) = request.ext.as_deref().and_then(|value| value.parse().ok()) else {
        tracing::warn!(
            assist_type = request.assist_type,
            ext = request.ext,
            "assist roster selector is not implemented"
        );
        return Ok(RefreshAssistReply {
            assist_type: request.assist_type,
            assist_hero_careers: vec![],
            ext: request.ext,
        });
    };

    let mut careers = BTreeMap::<i32, Vec<AssistHeroInfo>>::new();
    for (candidate_id, is_friend) in SocialManager::new(player_id)
        .assist_candidates(db, 20)
        .await?
    {
        let Some((career, assist)) = ProfileManager::new(candidate_id)
            .assist_hero(db, hero_id, is_friend)
            .await?
        else {
            continue;
        };
        careers.entry(career).or_default().push(assist);
    }

    Ok(RefreshAssistReply {
        assist_type: request.assist_type,
        assist_hero_careers: careers
            .into_iter()
            .map(|(career, assist_hero_infos)| AssistHeroCareerNo {
                career: Some(career),
                assist_hero_infos,
            })
            .collect(),
        ext: request.ext,
    })
}
