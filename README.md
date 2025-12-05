# KWoC Stats Tracker

A rust script to track student's pull requests and commits to projects for KWoC (Kharagpur Winter of Code). Successor to [v2](https://github.com/kossiitkgp/kwoc-stats-api), written in Go.

## How the script works

The script is periodically run by a cron job. It fetches all projects and students from the database, and then checks for new commits and pull requests for each project and student. The updated statistics are then stored in the database. 

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

SLACK_WEBHOOK_URL=<slack webhook url>
```

  - The database is the same as the one used by [KWoC Backend](https://github.com/kossiitkgp/KWoC-Backend). 
  - Create a GitHub personal access token, and add it to the `.env` file.
  - The times should be ISO UTC strings. (e.g. `2025-12-04T18:30:00Z` for 5th December 2025 00:00 IST; subtract 05:30 from IST to get UTC)
  - Optionally, add a slack webhook url to receive notifications about the stats updates.


3. Run `cargo run`

## Setting up in production

1. Clone the repository

```bash
git clone https://github.com/kossiitkgp/kwoc-stats-tracker.git
cd kwoc-stats-tracker
```

2. Create a `.env` file in the root directory; follow same steps as in the development section.

3. Set up the cron job to run the script periodically.

```bash
crontab -e
```

Add the following line to the crontab:

```txt
0 */6 * * * cd /path/to/projects/kwoc-stats-tracker && cargo run --release >> kwoc-stats.log 2>&1
```
- `0 */6 * * *` runs the script every 6 hours.
- `/path/to/projects/kwoc-stats-tracker` is the path to the repo directory.
- `>> kwoc-stats.log 2>&1` redirects the output to a log file.
- If `cargo` is not installed, you may need to install it first, and use a path to the cargo binary instead of `cargo`.


## Running Mid-Evals and End-Evals

There are two auxiliary scripts in this repo, for running the mid-evals and end-evals respectively. These scripts are used to update the database with the results of the mid-evals and end-evals.

- To run the mid-evals, run `cargo run --bin run-mid-evals`.
- To run the end-evals, run `cargo run --bin run-end-evals`.

> [!NOTE]
> - These scripts are to be run once per season. They can only be run manually since they require user interaction.
> - After the mid evals are completed, make sure to update the corresponding environment variables, both in this repo and in [KWoC Backend](https://github.com/kossiitkgp/KWoC-Backend).



------

>  Please update this documentation if you make changes to the code. Future humans will praise you.
