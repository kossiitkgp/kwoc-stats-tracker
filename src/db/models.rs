//! Database models
use reqwest::Url;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct DBProject {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub tags: String,
    pub repo_link: String,
    pub comm_channel: String,
    pub readme_link: String,
    pub project_status: bool, // approved or not
    pub status_remark: String,

    pub last_pull_time: i64,
    pub commit_count: i64,
    pub pull_count: i64,
    pub lines_added: i64,
    pub lines_removed: i64,
    pub contributors: String, // comma separated
    pub pulls: String, // comma separated
    pub mentor_id: i64,
    pub secondary_mentor_id: Option<i64>
}

#[derive(Clone)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub tags: String,
    pub repo_link: String,
    pub comm_channel: String,
    pub readme_link: String,
    pub project_status: bool,
    pub status_remark: String,

    pub last_pull_time: chrono::DateTime<chrono::Utc>,
    pub commit_count: i64,
    pub pull_count: i64,
    pub lines_added: i64,
    pub lines_removed: i64,
    pub contributors: Vec<String>,
    pub pulls: Vec<String>,

    pub mentor_id: i64,
    pub secondary_mentor_id: Option<i64>,

    pub repo_owner: String,
    pub repo_name: String,
}

impl From<DBProject> for Project {
    fn from(db_project: DBProject) -> Self {
        let repo_url= Url::parse(&db_project.repo_link).unwrap();
        let mut segments = repo_url.path_segments().unwrap();
        let repo_owner = segments.next().unwrap();
        let repo_name = segments.last().unwrap();
        Self {
            id: db_project.id,
            name: db_project.name,
            description: db_project.description,
            tags: db_project.tags,
            repo_link: db_project.repo_link,
            comm_channel: db_project.comm_channel,
            readme_link: db_project.readme_link,
            project_status: db_project.project_status,
            status_remark: db_project.status_remark,
            last_pull_time: chrono::DateTime::<chrono::Utc>::from_timestamp(db_project.last_pull_time, 0).unwrap(),
            commit_count: db_project.commit_count,
            pull_count: db_project.pull_count,
            lines_added: db_project.lines_added,
            lines_removed: db_project.lines_removed,
            contributors: db_project.contributors.split(',').map(|s| s.to_string()).collect(),
            pulls: db_project.pulls.split(',').map(|s| s.to_string()).collect(),
            mentor_id: db_project.mentor_id,
            secondary_mentor_id: db_project.secondary_mentor_id,
            repo_owner: repo_owner.to_string(),
            repo_name: repo_name.to_string(),
        }
    }
}

impl From<Project> for DBProject {
    fn from(project: Project) -> Self {
        Self {
            id: project.id,
            name: project.name,
            description: project.description,
            tags: project.tags,
            repo_link: project.repo_link,
            comm_channel: project.comm_channel,
            readme_link: project.readme_link,
            project_status: project.project_status,
            status_remark: project.status_remark,
            last_pull_time: project.last_pull_time.timestamp(),
            commit_count: project.commit_count,
            pull_count: project.pull_count,
            lines_added: project.lines_added,
            lines_removed: project.lines_removed,
            contributors: project.contributors.iter().map(|s| s.as_str()).filter(|l| l.len() > 0).collect::<Vec<&str>>().join(","),
            pulls: project.pulls.iter().map(|p| p.as_str()).filter(|p| p.len() > 0).collect::<Vec<&str>>().join(","),
            mentor_id: project.mentor_id,
            secondary_mentor_id: project.secondary_mentor_id,
        }
    }
}

#[derive(FromRow, Debug)]
pub struct Stats {
    pub total_commit_count: i64,
    pub total_pull_count: i64,
    pub total_lines_added: i64,
    pub total_lines_removed: i64,
}

#[derive(FromRow)]
pub struct DBStudent {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub college: String,
    pub username: String,
    pub passed_mid_evals: bool,
    pub passed_end_evals:  bool,
    pub blog_link: Option<String>,
    pub commit_count: i64,
    pub pull_count: i64,
    pub lines_added: i64,
    pub lines_removed: i64,
    pub languages_used: String,
    pub projects_worked: String,
    pub pulls: String,
}

#[derive(Clone)]
pub struct Student {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub college: String,
    pub username: String,
    pub passed_mid_evals: bool,
    pub passed_end_evals:  bool,
    pub blog_link: Option<String>,
    pub commit_count: i64,
    pub pull_count: i64,
    pub lines_added: i64,
    pub lines_removed: i64,
    pub languages_used: Vec<String>,
    pub projects_worked: Vec<i64>,
    pub pulls: Vec<String>
}


impl From<DBStudent> for Student {
    fn from(db_student: DBStudent) -> Self {
        Self {
            id: db_student.id,
            name: db_student.name,
            email: db_student.email,
            college: db_student.college,
            username: db_student.username,
            passed_mid_evals: db_student.passed_mid_evals,
            passed_end_evals: db_student.passed_end_evals,
            blog_link: db_student.blog_link,
            commit_count: db_student.commit_count,
            pull_count: db_student.pull_count,
            lines_added: db_student.lines_added,
            lines_removed: db_student.lines_removed,
            languages_used: db_student.languages_used.split(',').map(|s| s.to_string()).collect(),
            projects_worked: db_student.projects_worked.split(',').filter_map(|s| s.parse::<i64>().ok()).collect(),
            pulls: db_student.pulls.split(',').map(|s| s.to_string()).collect(),
        }
    }
}

impl From<Student> for DBStudent {
    fn from(student: Student) -> Self {
        Self {
            id: student.id,
            name: student.name,
            email: student.email,
            college: student.college,
            username: student.username,
            passed_mid_evals: student.passed_mid_evals,
            passed_end_evals: student.passed_end_evals,
            blog_link: student.blog_link,
            commit_count: student.commit_count,
            pull_count: student.pull_count,
            lines_added: student.lines_added,
            lines_removed: student.lines_removed,
            languages_used: student.languages_used.iter().map(|s| s.as_str()).filter(|l| l.len() > 0).collect::<Vec<&str>>().join(","),
            projects_worked: student.projects_worked.iter().map(|p| p.to_string()).filter(|p| p.len() > 0).collect::<Vec<String>>().join(","),
            pulls: student.pulls.iter().map(|p| p.as_str()).filter(|p| p.len() > 0).collect::<Vec<&str>>().join(","),
        }
    }
}