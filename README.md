# KWoC Stats Tracker

A rust script to track student's pull requests and commits to projects for KWoC (Kharagpur Winter of Code). Successor to [v2](https://github.com/kossiitkgp/kwoc-stats-api), written in Go.

## How the script works

The script is periodically run by a cron job (via GitHub Actions). It fetches all projects and students from the database, and then checks for new commits and pull requests for each project and student. The updated statistics are then stored in the database. 

The script also stores a `last_pull_time` field of each project to the latest time a pull request was merged. This is used to determine which pull requests are new and should be considered for the updated statistics.

The following stats are tracked:
- **Per project:**
  - Commit count
  - Pull count
  - Lines added
  - Lines removed
  - List of contributors
  - List of pull requests
- **Per student:**
  - Commit count
  - Pull count
  - Lines added
  - Lines removed
  - Languages used
  - Projects worked on
  - List of pull requests
- **Overall:**
  - Commit count
  - Pull count
  - Lines added
  - Lines removed

## Running Mid-Evals and End-Evals

There are two auxiliary scripts in the `src/bin` directory, for running the mid-evals and end-evals respectively. These scripts are used to update the database with the results of the mid-evals and end-evals.

- Set up the development environment as described below.
- To run the mid-evals, run `cargo run --bin run-mid-evals`.
- To run the end-evals, run `cargo run --bin run-end-evals`.

> [!NOTE]
> These scripts are to be run once per season. They can only be run manually since they require user interaction.

> [!WARNING]
> Note that the tracker script runs periodically and will update the database with information about new PRs that are merged after the mid-evals deadline. So, it is to be ensured that the mid-evals script is run at an appropriate time, or in case of any delay, the timeline is adjusted accordingly.

## Development

1. Clone the repository
2. Create a `.env` file in the root directory with the following content:

```
GITHUB_TOKEN=<your github token>
START_TIME=<start time of KWoC Coding Period>
MID_EVALS_TIME=
END_EVALS_TIME=

DATABASE_HOST=<database host>
DATABASE_PORT=<database port>
DATABASE_NAME=<database name>
DATABASE_USERNAME=<database username>
DATABASE_PASSWORD=<database password>

MID_EVALS_ENDED=false
```

  - The database is the same as the one used by [KWoC Backend](https://github.com/kossiitkgp/KWoC-Backend). 
  - Create a GitHub personal access token, and add it to the `.env` file.
  - The times should be in UTC. (e.g. `2025-12-05T00:00:00Z`)

3. Run `cargo run`

------

>  Please update this documentation if you make changes to the code or any other part of the stats generation process. Future humans will praise you.
