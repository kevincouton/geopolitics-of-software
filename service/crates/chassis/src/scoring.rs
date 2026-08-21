use crate::projects::Project;
use crate::social::Mention;

#[derive(Debug, Clone, Copy, Default)]
pub struct Score {
    pub docs: i32,
    pub platform: i32,
    pub social: i32,
    pub community: i32,
    pub total: i32,
}

pub fn score(project: &Project, mentions: &[Mention]) -> Score {
    let docs = if project.has_chinese_readme { 60 } else { 20 }
        + if project.description.is_some() { 20 } else { 0 }
        + if !project.topics.is_empty() { 20 } else { 0 };

    let platform = if project.has_gitee_mirror { 50 } else { 0 }
        + if !project.topics.is_empty() { 25 } else { 0 }
        + 25; // placeholder for release/discussion checks

    let social = (mentions.len() as i32 * 10).min(100);

    let community = if project.open_issues > 0 { 30 } else { 10 }
        + if project.forks > 0 { 30 } else { 0 }
        + if project.stars > 100 { 40 } else { 20 };

    let total = ((docs as f64) * 0.30
        + (platform as f64) * 0.25
        + (social as f64) * 0.25
        + (community as f64) * 0.20) as i32;

    Score {
        docs,
        platform,
        social,
        community,
        total,
    }
}
