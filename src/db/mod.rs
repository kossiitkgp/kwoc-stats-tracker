//! Database connection and functions
use crate::{
    db::{models::{DBProject, DBStudent, Project, Stats, Student}},
    env,
};
use sqlx::postgres::PgPool;

pub mod models;

pub struct Database {
    connection: PgPool,
}

impl Database {
    pub async fn new(env: &env::Env) -> anyhow::Result<Self> {
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            env.db_user, env.db_password, env.db_host, env.db_port, env.db_name
        );

        Ok(Self {
            connection: PgPool::connect(&database_url).await?,
        })
    }

    /// Get all approved projects
    pub async fn get_projects(&self) -> anyhow::Result<Vec<Project>> {
        let mut projects = Vec::new();

        let db_projects: Vec<DBProject> =
            sqlx::query_as("SELECT * FROM projects WHERE project_status = true")
                .fetch_all(&self.connection)
                .await?;

        for db_project in db_projects {
            let project: Result<Project, _> = db_project.into();
            if let Ok(project) = project {
                projects.push(project);
            }
        }

        Ok(projects)
    }

    /// Get all students
    pub async fn get_students(&self) -> anyhow::Result<Vec<Student>> {
        let mut students = Vec::new();

        let db_students: Vec<DBStudent> = sqlx::query_as("SELECT * FROM students")
            .fetch_all(&self.connection)
            .await?;

        for db_student in db_students {
            let student = Student::from(db_student);
            students.push(student);
        }

        Ok(students)
    }

    /// Get overall stats
    pub async fn get_stats(&self) -> anyhow::Result<Stats> {
        let stats: Stats = sqlx::query_as("SELECT * FROM stats")
            .fetch_one(&self.connection)
            .await?;

        Ok(stats)
    }

    /// Update a project. Only updates the fields:
    /// last_pull_time, commit_count, pull_count, lines_added, lines_removed, contributors, pulls
    pub async fn update_project(&self, project: Project) -> anyhow::Result<()> {
        let db_project: DBProject = project.into();
        let query = sqlx::query(
            "UPDATE projects SET last_pull_time = $1, commit_count = $2, pull_count = $3, lines_added = $4, lines_removed = $5, contributors = $6, pulls = $7 WHERE id = $8",
        )
        .bind(db_project.last_pull_time)
        .bind(db_project.commit_count)
        .bind(db_project.pull_count)
        .bind(db_project.lines_added)
        .bind(db_project.lines_removed)
        .bind(db_project.contributors)
        .bind(db_project.pulls)
        .bind(db_project.id);
        query.execute(&self.connection).await?;
        Ok(())
    }

    /// Update a student. Only updates the fields:
    /// passed_mid_evals, passed_end_evals, commit_count, pull_count, lines_added, lines_removed, languages_used, projects_worked, pulls
    pub async fn update_student(&self, student: Student) -> anyhow::Result<()> {
        let db_student: DBStudent = student.into();
        let query = sqlx::query(
            "UPDATE students SET passed_mid_evals = $1, passed_end_evals = $2, commit_count = $3, pull_count = $4, lines_added = $5, lines_removed = $6, languages_used = $7, projects_worked = $8, pulls = $9 WHERE id = $10",
        )
        .bind(db_student.passed_mid_evals)
        .bind(db_student.passed_end_evals)
        .bind(db_student.commit_count)
        .bind(db_student.pull_count)
        .bind(db_student.lines_added)
        .bind(db_student.lines_removed)
        .bind(db_student.languages_used)
        .bind(db_student.projects_worked)
        .bind(db_student.pulls)
        .bind(db_student.id);
        query.execute(&self.connection).await?;
        Ok(())
    }

    /// Update overall stats.
    pub async fn update_stats(&self, stats: Stats) -> anyhow::Result<()> {
        let mut query_sql = "INSERT INTO stats (total_commit_count, total_pull_count, total_lines_added, total_lines_removed) VALUES ($1, $2, $3, $4)";
        if self.get_stats().await.is_ok() {
            query_sql = "UPDATE stats SET total_commit_count = $1, total_pull_count = $2, total_lines_added = $3, total_lines_removed = $4";
        }
        let query = sqlx::query(
            query_sql
        )
        .bind(stats.total_commit_count)
        .bind(stats.total_pull_count)
        .bind(stats.total_lines_added)
        .bind(stats.total_lines_removed);
        query.execute(&self.connection).await?;
        Ok(())
    }

}
