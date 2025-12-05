//! The main code to update the stats.

use std::collections::HashSet;

use chrono::{DateTime, Utc};

use crate::{db::Database, db::models, env::Env, github::GitHub};

pub async fn update_stats(
    env: &Env,
    database: &Database,
    gh: &GitHub,
    deadline: DateTime<Utc>,
) -> anyhow::Result<(i32, i32)> {
    // Fetch info from the database
    let mut projects = database.get_projects().await?;
    let mut students = database.get_students().await?;
    if env.mid_evals_ended {
        students.retain(|s| s.passed_mid_evals);
        println!("Only including students who passed the mid-evals");
    }
    let mut stats = if let Ok(stats) = database.get_stats().await {
        stats
    } else {
        models::Stats {
            total_commit_count: 0,
            total_pull_count: 0,
            total_lines_added: 0,
            total_lines_removed: 0,
        }
    };

    let mut total_pull_count = 0;
    let mut projects_with_new_prs = HashSet::new();

    // For each project, fetch the pull requests and update the stats
    for project in &mut projects {
        let mut max_merge_time = project.last_pull_time;

        let existing_pulls: HashSet<String> = HashSet::from_iter(project.pulls.iter().cloned());

        let mut page = 1;
        loop {
            let mut done = false;

            // Fetch all PRs for the project
            let pull_requests = gh
                .get_project_pull_requests(&project.repo_owner, &project.repo_name, page)
                .await?;
            if pull_requests.len() == 0 {
                break;
            }
            for pr in pull_requests {
                if pr.merged_at.is_none() {
                    continue;
                }

                let url = pr.html_url;

                if existing_pulls.contains(&url) {
                    continue;
                }

                // If the PR is created after the start time,
                // and merged before the deadline, then update the stats for the project
                let created_at: DateTime<Utc> = pr.created_at.parse().unwrap();
                let merged_at: DateTime<Utc> = pr.merged_at.map(|s| s.parse().unwrap()).unwrap();
                if created_at > env.start_time
                    && merged_at < deadline
                    && let Some(student) = students.iter_mut().find(|s| s.username == pr.user.login)
                {
                    total_pull_count += 1;
                    if !projects_with_new_prs.contains(&project.id) {
                        projects_with_new_prs.insert(project.id);
                    }

                    let mut lines_added = 0;
                    let mut lines_removed = 0;
                    let mut languages = vec![];
                    for file in gh
                        .get_pull_request_files(&project.repo_owner, &project.repo_name, pr.number)
                        .await?
                    {
                        if let Some(language) = file.filename.split('.').last() {
                            let language = language.to_string();
                            if !languages.contains(&language) {
                                languages.push(language);
                            }
                        }
                        lines_added += file.additions;
                        lines_removed += file.deletions;
                    }

                    let commits = gh
                        .get_pull_request_commits(
                            &project.repo_owner,
                            &project.repo_name,
                            pr.number,
                        )
                        .await?;
                    let mut commit_count = 0;
                    for commit in commits {
                        if commit.commit.author.email == student.email {
                            commit_count += 1;
                        }
                    }

                    let lines_added = lines_added as i64;
                    let lines_removed = lines_removed as i64;

                    // update project
                    project.commit_count += commit_count;
                    project.pull_count += 1;
                    project.lines_added += lines_added;
                    project.lines_removed += lines_removed;
                    if !project.contributors.contains(&student.username) {
                        project.contributors.push(student.username.clone());
                    }
                    project.pulls.push(url.clone());

                    // update student
                    student.commit_count += commit_count;
                    student.pull_count += 1;
                    student.lines_added += lines_added;
                    student.lines_removed += lines_removed;
                    if !student.projects_worked.contains(&project.id) {
                        student.projects_worked.push(project.id);
                    }
                    for language in languages {
                        if !student.languages_used.contains(&language) {
                            student.languages_used.push(language);
                        }
                    }
                    student.pulls.push(url.clone());

                    // update stats
                    stats.total_commit_count += commit_count;
                    stats.total_pull_count += 1;
                    stats.total_lines_added += lines_added;
                    stats.total_lines_removed += lines_removed;

                    max_merge_time = max_merge_time.max(merged_at);

                    println!("[KWoC-Stats] Updated new pull request for project {} by {}: {}", project.name, student.username, url);
                }

                if created_at < env.start_time {
                    done = true;
                    break;
                }
            }
            if done {
                break;
            }
            page += 1;
        }

        // Update the last pull time for the project
        project.last_pull_time = max_merge_time;
    }

    // Commit all the changes to the database

    for project in &projects {
        database.update_project(project.clone()).await?;
    }

    for student in &students {
        database.update_student(student.clone()).await?;
    }

    database.update_stats(stats).await?;

    Ok((projects_with_new_prs.len() as i32, total_pull_count))
}
