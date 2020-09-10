use goose::prelude::*;

async fn loadtest_uniques(user: &GooseUser) -> GooseTaskResult {
    let _goose = user.get("/uniques/").await?;
    Ok(())
}

async fn loadtest_ping(user: &GooseUser) -> GooseTaskResult {
    let _goose = user.get("/ping/").await?;
    Ok(())
}

fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_taskset(
            taskset!("LoadtestTasks")
                .register_task(task!(loadtest_uniques)),
        )
        .register_taskset(
            taskset!("ComparablePingTasks")
                .register_task(task!(loadtest_ping)),
        )
        .execute()?
        .print();
    Ok(())
}
